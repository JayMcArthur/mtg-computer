package forge.ai;

import com.google.common.collect.ListMultimap;
import com.google.common.collect.Multimap;
import forge.LobbyPlayer;
import forge.card.ColorSet;
import forge.card.ICardFace;
import forge.card.mana.ManaCost;
import forge.card.mana.ManaCostShard;
import forge.deck.Deck;
import forge.deck.DeckSection;
import forge.game.*;
import forge.game.ability.effects.RollDiceEffect;
import forge.game.card.*;
import forge.game.card.sticker.Sticker;
import forge.game.combat.Combat;
import forge.game.cost.*;
import forge.game.keyword.KeywordInterface;
import forge.game.mana.Mana;
import forge.game.mana.ManaConversionMatrix;
import forge.game.mana.ManaCostBeingPaid;
import forge.game.replacement.ReplacementEffect;
import forge.game.spellability.*;
import forge.game.staticability.StaticAbility;
import forge.game.staticability.StaticAbilityMustAttack;
import forge.game.staticability.StaticAbilityMustBlock;
import forge.game.trigger.WrappedAbility;
import forge.game.zone.PlayerZone;
import forge.game.zone.ZoneType;
import forge.item.PaperCard;
import forge.util.ITriggerEvent;
import forge.util.collect.FCollectionView;
import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.commons.lang3.tuple.Pair;
import java.util.Collection;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.function.Predicate;
import forge.game.player.Player;
import forge.game.player.PlayerView;
import forge.game.player.DelayedReveal;
import forge.game.player.PlayerActionConfirmMode;
import forge.game.player.PlayerController;
import forge.game.player.PlaySpellAbility;
import forge.game.combat.CombatUtil;
import java.util.LinkedHashMap;

/** Decisions for the prepared profile; unsupported engine callbacks stop execution. */
public class MtgController extends PlayerController {
    private final MtgTrace trace;

    public MtgController(Game game, Player player, LobbyPlayer lobby, MtgTrace trace) {
        super(game, player, lobby);
        this.trace = trace;
    }

    private UnsupportedOperationException unsupported(String detail) {
        var error = new UnsupportedOperationException(detail);
        trace.rejectDecision(error);
        return error;
    }

    private IllegalStateException reject(String detail) {
        var error = new IllegalStateException(detail);
        trace.rejectDecision(error);
        return error;
    }

    private CardCollection requiredCards(CardCollectionView options, int min, int max, boolean optional) {
        if (optional || min != max || options.size() > min) {
            throw reject("divergent card choice " + options + " range " + min + ".." + max);
        }
        trace.decision(getPlayer(), "required-cards", options.stream().map(trace::cardId).toList(),
                "mandatory selection exhausts the legal options; requested " + min);
        return new CardCollection(options);
    }

    @Override
    public CardCollectionView chooseCardsForEffect(CardCollectionView options, SpellAbility ability,
            String title, int min, int max, boolean optional, Map<String, Object> params) {
        return requiredCards(options, min, max, optional);
    }

    @Override
    public CardCollectionView choosePermanentsToSacrifice(SpellAbility ability, int min, int max,
            CardCollectionView options, String message) {
        return requiredCards(options, min, max, false);
    }

    @Override
    public CardCollectionView choosePermanentsToDestroy(SpellAbility ability, int min, int max,
            CardCollectionView options, String message) {
        return requiredCards(options, min, max, false);
    }

    @Override
    public CardCollectionView orderMoveToZoneList(CardCollectionView cards, ZoneType zone, SpellAbility ability) {
        if (cards.size() > 1) throw reject("unaudited zone order " + cards);
        trace.decision(getPlayer(), "zone-order", cards.stream().map(trace::cardId).toList(), "at most one object");
        return new CardCollection(cards);
    }

    @Override
    public CardCollection orderBlockers(Card attacker, CardCollection blockers) {
        if (blockers.size() > 1) throw reject("unaudited blocker order " + blockers);
        return blockers;
    }

    @Override
    public CardCollection orderAttackers(Card blocker, CardCollection attackers) {
        if (attackers.size() > 1) throw reject("unaudited attacker order " + attackers);
        return attackers;
    }

    @Override
    public List<SpellAbility> chooseSpellAbilityToPlay() {
        for (ZoneType zone : List.of(ZoneType.Battlefield, ZoneType.Hand, ZoneType.Graveyard, ZoneType.Exile)) {
            for (Card card : getPlayer().getCardsIn(zone)) {
                for (SpellAbility ability : card.getSpellAbilities()) {
                    ability.setActivatingPlayer(getPlayer());
                    if (ability.canPlay() && ability.checkRestrictions(getPlayer())) {
                        throw reject("optional priority action " + ability);
                    }
                }
            }
        }
        trace.decision(getPlayer(), "priority", List.of(), "no playable spell or activated ability in profile zones");
        return null;
    }

    // These callbacks only display information or manage GUI waiting; they make no game decision.
    @Override
    public void reveal(CardCollectionView cards, ZoneType zone, Player owner, String prefix, boolean suffix) { }
    @Override
    public void reveal(List<CardView> cards, ZoneType zone, PlayerView owner, String prefix, boolean suffix) { }
    @Override
    public void notifyOfValue(SpellAbility ability, GameObject object, String value) { }
    @Override
    public void autoPassCancel() { }
    @Override
    public void awaitNextInput() { }
    @Override
    public void cancelAwaitNextInput() { }

    @Override
    public void declareAttackers(Player attacker, Combat combat) {
        List<Player> defenders = getGame().getPlayers().stream().filter(p -> p != attacker).toList();
        if (defenders.size() != 1) throw reject("profile requires two players");
        Player defender = defenders.get(0);
        for (Card card : attacker.getCreaturesInPlay()) {
            if (CombatUtil.canAttack(card, defender)) {
                if (!StaticAbilityMustAttack.entitiesMustAttack(card).contains(card)) {
                    throw reject("optional attack " + card);
                }
                combat.addAttacker(card, defender);
                trace.decision(attacker, "attack", List.of(trace.cardId(card)), "must attack; sole player defender");
            }
        }
        if (!CombatUtil.validateAttackers(combat)) throw reject("invalid forced attack declaration");
    }

    @Override
    public void declareBlockers(Player defender, Combat combat) {
        for (Card blocker : defender.getCreaturesInPlay()) {
            List<Card> options = combat.getAttackers().stream()
                    .filter(attacker -> CombatUtil.canBlock(attacker, blocker, combat)).toList();
            if (options.size() > 1) throw reject("divergent blocker choice: " + blocker + " " + options);
            if (options.size() == 1) {
                if (!StaticAbilityMustBlock.blocksEachCombatIfAble(blocker)) {
                    throw reject("optional block " + blocker);
                }
                combat.addBlocker(options.get(0), blocker);
                trace.decision(defender, "block", List.of(trace.cardId(blocker), trace.cardId(options.get(0))),
                        "must block; one legal attacker");
            }
        }
        String invalid = CombatUtil.validateBlocks(combat, defender);
        if (invalid != null) throw reject(invalid);
    }

    @Override
    public void orderAndPlaySimultaneousSa(List<SpellAbility> abilities) {
        if (abilities.size() > 1) throw reject("unaudited trigger order " + abilities);
        trace.decision(getPlayer(), "trigger-order", abilities.stream().map(a -> String.valueOf(a.getId())).toList(),
                "at most one simultaneous trigger");
        for (SpellAbility ability : abilities) {
            chooseTargetsFor(ability);
            getGame().getStack().add(ability);
        }
    }

    @Override
    public boolean chooseTargetsFor(SpellAbility ability) {
        if (!ability.usesTargeting()) return true;
        if (ability.hasParam("TargetsForEachPlayer")) {
            for (Player controller : getGame().getPlayers()) {
                List<Card> options = controller.getCardsIn(ZoneType.Battlefield).stream()
                        .filter(ability::canTarget).toList();
                if (options.size() != 1) throw reject("target choice for " + ability + ": " + options);
                ability.getTargets().add(options.get(0));
                trace.decision(getPlayer(), "target", List.of(trace.cardId(options.get(0))), "one legal target for " + controller.getName());
            }
        } else {
            List<Card> options = getGame().getCardsIn(ZoneType.Battlefield).stream()
                    .filter(ability::canTarget).toList();
            if (options.size() != 1) throw reject("target choice for " + ability + ": " + options);
            ability.getTargets().add(options.get(0));
            trace.decision(getPlayer(), "target", List.of(trace.cardId(options.get(0))), "one legal battlefield target");
        }
        return true;
    }

    @Override
    public Map<Card, Integer> assignCombatDamage(Card attacker, CardCollectionView blockers,
            CardCollectionView remaining, int damage, GameEntity defender, boolean overrideOrder) {
        if (blockers.size() > 1 || !blockers.isEmpty() && attacker.hasKeyword("Trample")) {
            throw reject("unaudited combat damage assignment " + attacker + " " + blockers);
        }
        Map<Card, Integer> result = new LinkedHashMap<>();
        Card recipient = blockers.isEmpty() ? null : blockers.getFirst();
        result.put(recipient, damage);
        trace.decision(getPlayer(), "combat-damage", recipient == null ? List.of() : List.of(trace.cardId(recipient)),
                "all " + damage + " damage to the sole recipient");
        return result;
    }

    @Override
    public ReplacementEffect chooseSingleReplacementEffect(List<ReplacementEffect> options) {
        if (options.size() != 1) throw reject("unaudited replacement choice " + options);
        trace.decision(getPlayer(), "replacement", List.of(options.get(0).toString()), "one applicable replacement");
        return options.get(0);
    }

    @Override
    public void playSpellAbilityNoStack(SpellAbility ability, boolean chooseNewTargets) {
        if (!PlaySpellAbility.playSpellAbilityNoStack(this, getPlayer(), ability, !chooseNewTargets)) {
            throw reject("mandatory effect failed: " + ability);
        }
    }

    @Override
    public SpellAbility getAbilityToPlay(Card hostCard, List<SpellAbility> abilities, ITriggerEvent triggerEvent) {
        throw unsupported("unsupported decision: getAbilityToPlay");
    }

    @Override
    public List<SpellAbility> orderSimultaneousSa(List<SpellAbility> activePlayerSAs) {
        throw unsupported("unsupported decision: orderSimultaneousSa");
    }

    @Override
    public boolean playTrigger(Card host, WrappedAbility wrapperAbility, boolean isMandatory) {
        throw unsupported("unsupported decision: playTrigger");
    }

    @Override
    public boolean playSaFromPlayEffect(SpellAbility tgtSA) {
        throw unsupported("unsupported decision: playSaFromPlayEffect");
    }

    @Override
    public List<PaperCard> sideboard(final Deck deck, GameType gameType, String message) {
        throw unsupported("unsupported decision: sideboard");
    }

    @Override
    public List<PaperCard> chooseCardsYouWonToAddToDeck(List<PaperCard> losses) {
        throw unsupported("unsupported decision: chooseCardsYouWonToAddToDeck");
    }

    @Override
    public Map<GameEntity, Integer> divideShield(Card effectSource, Map<GameEntity, Integer> affected, int shieldAmount) {
        throw unsupported("unsupported decision: divideShield");
    }

    @Override
    public Map<Byte, Integer> specifyManaCombo(SpellAbility sa, ColorSet colorSet, int manaAmount, boolean different) {
        throw unsupported("unsupported decision: specifyManaCombo");
    }

    @Override
    public Integer announceRequirements(SpellAbility ability, int min, int max, String announce) {
        throw unsupported("unsupported decision: announceRequirements");
    }

    @Override
    public TargetChoices chooseNewTargetsFor(SpellAbility ability, Predicate<GameObject> filter, boolean optional) {
        throw unsupported("unsupported decision: chooseNewTargetsFor");
    }

    @Override
    public Pair<SpellAbilityStackInstance, GameObject> chooseTarget(SpellAbility sa, List<Pair<SpellAbilityStackInstance, GameObject>> allTargets) {
        throw unsupported("unsupported decision: chooseTarget");
    }

    @Override
    public boolean helpPayForAssistSpell(ManaCostBeingPaid cost, SpellAbility sa, int max, int requested) {
        throw unsupported("unsupported decision: helpPayForAssistSpell");
    }

    @Override
    public Player choosePlayerToAssistPayment(FCollectionView<Player> optionList, SpellAbility sa, String title, int max) {
        throw unsupported("unsupported decision: choosePlayerToAssistPayment");
    }

    @Override
    public CardCollection chooseCardsForEffectMultiple(Map<String, CardCollection> validMap, SpellAbility sa, String title, boolean isOptional) {
        throw unsupported("unsupported decision: chooseCardsForEffectMultiple");
    }

    @Override
    public <T extends GameEntity> T chooseSingleEntityForEffect(FCollectionView<T> optionList, DelayedReveal delayedReveal, SpellAbility sa, String title, boolean isOptional, Player relatedPlayer, Map<String, Object> params) {
        throw unsupported("unsupported decision: chooseSingleEntityForEffect");
    }

    @Override
    public <T extends GameEntity> List<T> chooseEntitiesForEffect(FCollectionView<T> optionList, int min, int max, DelayedReveal delayedReveal, SpellAbility sa, String title, Player relatedPlayer, Map<String, Object> params) {
        throw unsupported("unsupported decision: chooseEntitiesForEffect");
    }

    @Override
    public List<SpellAbility> chooseSpellAbilitiesForEffect(List<SpellAbility> spells, SpellAbility sa, String title, int num, Map<String, Object> params) {
        throw unsupported("unsupported decision: chooseSpellAbilitiesForEffect");
    }

    @Override
    public SpellAbility chooseSingleSpellForEffect(List<SpellAbility> spells, SpellAbility sa, String title, Map<String, Object> params) {
        throw unsupported("unsupported decision: chooseSingleSpellForEffect");
    }

    @Override
    public boolean confirmAction(SpellAbility sa, PlayerActionConfirmMode mode, String message, List<String> options, Card cardToShow, Map<String, Object> params) {
        throw unsupported("unsupported decision: confirmAction");
    }

    @Override
    public boolean confirmBidAction(SpellAbility sa, PlayerActionConfirmMode bidlife, String string, int bid, Player winner) {
        throw unsupported("unsupported decision: confirmBidAction");
    }

    @Override
    public boolean confirmReplacementEffect(ReplacementEffect replacementEffect, SpellAbility effectSA, GameEntity affected, String question) {
        throw unsupported("unsupported decision: confirmReplacementEffect");
    }

    @Override
    public boolean confirmStaticApplication(Card hostCard, PlayerActionConfirmMode mode, String message, String logic) {
        throw unsupported("unsupported decision: confirmStaticApplication");
    }

    @Override
    public boolean confirmTrigger(WrappedAbility sa) {
        throw unsupported("unsupported decision: confirmTrigger");
    }

    @Override
    public List<Card> exertAttackers(List<Card> attackers) {
        throw unsupported("unsupported decision: exertAttackers");
    }

    @Override
    public List<Card> enlistAttackers(List<Card> attackers) {
        throw unsupported("unsupported decision: enlistAttackers");
    }

    @Override
    public CardCollection orderBlocker(final Card attacker, final Card blocker, final CardCollection oldBlockers) {
        throw unsupported("unsupported decision: orderBlocker");
    }

    @Override
    public ImmutablePair<CardCollection, CardCollection> arrangeForScry(CardCollection topN) {
        throw unsupported("unsupported decision: arrangeForScry");
    }

    @Override
    public ImmutablePair<CardCollection, CardCollection> arrangeForSurveil(CardCollection topN) {
        throw unsupported("unsupported decision: arrangeForSurveil");
    }

    @Override
    public boolean willPutCardOnTop(Card c) {
        throw unsupported("unsupported decision: willPutCardOnTop");
    }

    @Override
    public CardCollectionView chooseCardsToDiscardFrom(Player playerDiscard, SpellAbility sa, CardCollection validCards, int min, int max, CardCollectionView visibleToChooser) {
        throw unsupported("unsupported decision: chooseCardsToDiscardFrom");
    }

    @Override
    public CardCollectionView chooseCardsToDiscardUnlessType(int min, CardCollectionView hand, String[] unlessTypes, SpellAbility sa) {
        throw unsupported("unsupported decision: chooseCardsToDiscardUnlessType");
    }

    @Override
    public CardCollectionView chooseCardsToDiscardToMaximumHandSize(int numDiscard) {
        throw unsupported("unsupported decision: chooseCardsToDiscardToMaximumHandSize");
    }

    @Override
    public CardCollectionView chooseCardsToDelve(int genericAmount, CardCollection grave) {
        throw unsupported("unsupported decision: chooseCardsToDelve");
    }

    @Override
    public Map<Card, ManaCostShard> chooseCardsForConvokeOrImprovise(SpellAbility sa, ManaCost manaCost, CardCollectionView untappedCards, boolean artifacts, boolean creatures, Integer maxReduction) {
        throw unsupported("unsupported decision: chooseCardsForConvokeOrImprovise");
    }

    @Override
    public List<Card> chooseCardsForSplice(SpellAbility sa, List<Card> cards) {
        throw unsupported("unsupported decision: chooseCardsForSplice");
    }

    @Override
    public CardCollectionView chooseCardsToRevealFromHand(int min, int max, CardCollectionView valid) {
        throw unsupported("unsupported decision: chooseCardsToRevealFromHand");
    }

    @Override
    public List<SpellAbility> chooseSaToActivateFromOpeningHand(List<SpellAbility> usableFromOpeningHand) {
        throw unsupported("unsupported decision: chooseSaToActivateFromOpeningHand");
    }

    @Override
    public Player chooseStartingPlayer(boolean isFirstGame) {
        throw unsupported("unsupported decision: chooseStartingPlayer");
    }

    @Override
    public PlayerZone chooseStartingHand(List<PlayerZone> zones) {
        throw unsupported("unsupported decision: chooseStartingHand");
    }

    @Override
    public Mana chooseManaFromPool(List<Mana> manaChoices) {
        throw unsupported("unsupported decision: chooseManaFromPool");
    }

    @Override
    public String chooseSomeType(String kindOfType, SpellAbility sa, Collection<String> validTypes, boolean isOptional) {
        throw unsupported("unsupported decision: chooseSomeType");
    }

    @Override
    public String chooseSector(Card assignee, String ai, List<String> sectors) {
        throw unsupported("unsupported decision: chooseSector");
    }

    @Override
    public Sticker chooseSticker(List<Sticker> options, Card target, SpellAbility sa, boolean isOptional) {
        throw unsupported("unsupported decision: chooseSticker");
    }

    @Override
    public int chooseStickerNamePosition(Sticker sticker, Card target) {
        throw unsupported("unsupported decision: chooseStickerNamePosition");
    }

    @Override
    public Card chooseCardToKeepStickers(CardCollectionView options) {
        throw unsupported("unsupported decision: chooseCardToKeepStickers");
    }

    @Override
    public List<Card> chooseContraptionsToCrank(List<Card> contraptions) {
        throw unsupported("unsupported decision: chooseContraptionsToCrank");
    }

    @Override
    public int chooseSprocket(Card assignee, List<Integer> sprockets) {
        throw unsupported("unsupported decision: chooseSprocket");
    }

    @Override
    public PlanarDice choosePDRollToIgnore(List<PlanarDice> rolls) {
        throw unsupported("unsupported decision: choosePDRollToIgnore");
    }

    @Override
    public Integer chooseRollToIgnore(List<Integer> rolls) {
        throw unsupported("unsupported decision: chooseRollToIgnore");
    }

    @Override
    public List<Integer> chooseDiceToReroll(List<Integer> rolls) {
        throw unsupported("unsupported decision: chooseDiceToReroll");
    }

    @Override
    public Integer chooseRollToModify(List<Integer> rolls) {
        throw unsupported("unsupported decision: chooseRollToModify");
    }

    @Override
    public RollDiceEffect.DieRollResult chooseRollToSwap(List<RollDiceEffect.DieRollResult> rolls) {
        throw unsupported("unsupported decision: chooseRollToSwap");
    }

    @Override
    public String chooseRollSwapValue(List<String> swapChoices, Integer currentResult, int power, int toughness) {
        throw unsupported("unsupported decision: chooseRollSwapValue");
    }

    @Override
    public Object vote(SpellAbility sa, String prompt, List<Object> options, ListMultimap<Object, Player> votes, Player forPlayer, boolean optional) {
        throw unsupported("unsupported decision: vote");
    }

    @Override
    public boolean mulliganKeepHand(Player player, int cardsToReturn) {
        throw unsupported("unsupported decision: mulliganKeepHand");
    }

    @Override
    public CardCollectionView tuckCardsViaMulligan(CardCollectionView hand, int cardsToReturn) {
        throw unsupported("unsupported decision: tuckCardsViaMulligan");
    }

    @Override
    public boolean playChosenSpellAbility(SpellAbility sa) {
        throw unsupported("unsupported decision: playChosenSpellAbility");
    }

    @Override
    public List<AbilitySub> chooseModeForAbility(SpellAbility sa, List<AbilitySub> possible, int min, int num, boolean allowRepeat) {
        throw unsupported("unsupported decision: chooseModeForAbility");
    }

    @Override
    public int chooseNumberForCostReduction(final SpellAbility sa, final int min, final int max) {
        throw unsupported("unsupported decision: chooseNumberForCostReduction");
    }

    @Override
    public int chooseNumberForKeywordCost(SpellAbility sa, Cost cost, KeywordInterface keyword, String prompt, int max) {
        throw unsupported("unsupported decision: chooseNumberForKeywordCost");
    }

    @Override
    public int chooseNumber(SpellAbility sa, String title, int min, int max) {
        throw unsupported("unsupported decision: chooseNumber");
    }

    @Override
    public int chooseNumber(SpellAbility sa, String title, List<Integer> values, Player relatedPlayer) {
        throw unsupported("unsupported decision: chooseNumber");
    }

    @Override
    public boolean chooseBinary(SpellAbility sa, String question, BinaryChoiceType kindOfChoice, Boolean defaultChoice) {
        throw unsupported("unsupported decision: chooseBinary");
    }

    @Override
    public boolean chooseFlipResult(SpellAbility sa, Player flipper, boolean call) {
        throw unsupported("unsupported decision: chooseFlipResult");
    }

    @Override
    public byte chooseColor(String message, SpellAbility sa, ColorSet colors) {
        throw unsupported("unsupported decision: chooseColor");
    }

    @Override
    public byte chooseColorAllowColorless(String message, Card c, ColorSet colors) {
        throw unsupported("unsupported decision: chooseColorAllowColorless");
    }

    @Override
    public ColorSet chooseColors(String message, SpellAbility sa, int min, int max, ColorSet options) {
        throw unsupported("unsupported decision: chooseColors");
    }

    @Override
    public ICardFace chooseSingleCardFace(SpellAbility sa, String message, Predicate<ICardFace> cpp, String name) {
        throw unsupported("unsupported decision: chooseSingleCardFace");
    }

    @Override
    public ICardFace chooseSingleCardFace(SpellAbility sa, List<ICardFace> faces, String message) {
        throw unsupported("unsupported decision: chooseSingleCardFace");
    }

    @Override
    public CardState chooseSingleCardState(SpellAbility sa, List<CardState> states, String message, Map<String, Object> params) {
        throw unsupported("unsupported decision: chooseSingleCardState");
    }

    @Override
    public boolean chooseCardsPile(SpellAbility sa, CardCollectionView pile1, CardCollectionView pile2, String faceUp) {
        throw unsupported("unsupported decision: chooseCardsPile");
    }

    @Override
    public CounterType chooseCounterType(List<CounterType> options, SpellAbility sa, String prompt, Map<String, Object> params) {
        if (options.size() != 1) throw reject("divergent counter type " + options);
        trace.decision(getPlayer(), "counter-type", options.stream().map(Object::toString).toList(), "one legal counter type");
        return options.get(0);
    }

    @Override
    public String chooseKeywordForPump(List<String> options, SpellAbility sa, String prompt, Card tgtCard) {
        throw unsupported("unsupported decision: chooseKeywordForPump");
    }

    @Override
    public boolean confirmPayment(CostPart costPart, String string, SpellAbility sa) {
        throw unsupported("unsupported decision: confirmPayment");
    }

    @Override
    public StaticAbility chooseSingleStaticAbility(List<StaticAbility> possibleReplacers) {
        throw unsupported("unsupported decision: chooseSingleStaticAbility");
    }

    @Override
    public String chooseProtectionType(SpellAbility sa, List<String> choices) {
        throw unsupported("unsupported decision: chooseProtectionType");
    }

    @Override
    public void revealAnte(String message, Multimap<Player, PaperCard> removedAnteCards) {
        throw unsupported("unsupported decision: revealAnte");
    }

    @Override
    public void revealAISkipCards(String message, Map<Player, Map<DeckSection, List<? extends PaperCard>>> deckCards) {
        throw unsupported("unsupported decision: revealAISkipCards");
    }

    @Override
    public void revealUnsupported(Map<Player, List<PaperCard>> unsupported) {
        throw unsupported("unsupported decision: revealUnsupported");
    }

    @Override
    public List<OptionalCostValue> chooseOptionalCosts(SpellAbility choosen, List<OptionalCostValue> optionalCostValues) {
        throw unsupported("unsupported decision: chooseOptionalCosts");
    }

    @Override
    public List<CostPart> orderCosts(List<CostPart> costs) {
        throw unsupported("unsupported decision: orderCosts");
    }

    @Override
    public boolean payCostToPreventEffect(Cost cost, SpellAbility sa, boolean alreadyPaid, FCollectionView<Player> allPayers) {
        throw unsupported("unsupported decision: payCostToPreventEffect");
    }

    @Override
    public boolean payCostDuringRoll(Cost cost, SpellAbility sa) {
        throw unsupported("unsupported decision: payCostDuringRoll");
    }

    @Override
    public boolean payCombatCost(Card card, Cost cost, SpellAbility sa, String prompt) {
        throw unsupported("unsupported decision: payCombatCost");
    }

    @Override
    public boolean payManaCost(ManaCost toPay, CostPartMana costPartMana, SpellAbility sa, String prompt, ManaConversionMatrix matrix, boolean effect) {
        if (!toPay.isZero()) throw unsupported("unsupported nonzero mana payment " + toPay);
        trace.decision(getPlayer(), "mana-payment", List.of(), "mandatory effect has zero mana cost");
        return true;
    }

    @Override
    public boolean applyManaToCost(ManaCostBeingPaid toPay, SpellAbility ability, String prompt, ManaConversionMatrix matrix, boolean effect) {
        throw unsupported("unsupported decision: applyManaToCost");
    }

    @Override
    public CardCollectionView chooseCardsForCost(CardCollectionView optionList, SpellAbility sa, CostPartWithList cpl, int amount, boolean isOptional, String prompt) {
        throw unsupported("unsupported decision: chooseCardsForCost");
    }

    @Override
    public CostDecisionMakerBase getCostDecisionMaker(Player player, SpellAbility ability, boolean effect, String prompt) {
        Cost costs = ability.getPayCosts();
        if (costs != null && !CostAdjustment.adjust(costs, ability, effect).isFree()) {
            throw unsupported("unsupported nonzero effect cost " + costs);
        }
        // The engine still requests a payer for cost 0; the guard permits no payment choice.
        return new AiCostDecision(player, ability, effect);
    }

    @Override
    public String chooseCardName(SpellAbility sa, Predicate<ICardFace> cpp, String valid, String message) {
        throw unsupported("unsupported decision: chooseCardName");
    }

    @Override
    public String chooseCardName(SpellAbility sa, List<ICardFace> faces, String message) {
        throw unsupported("unsupported decision: chooseCardName");
    }

    @Override
    public Card chooseSingleCardForZoneChange(ZoneType destination, List<ZoneType> origin, SpellAbility sa, CardCollection fetchList, DelayedReveal delayedReveal, String selectPrompt, boolean isOptional, Player decider) {
        throw unsupported("unsupported decision: chooseSingleCardForZoneChange");
    }

    @Override
    public List<Card> chooseCardsForZoneChange(ZoneType destination, List<ZoneType> origin, SpellAbility sa, CardCollection fetchList, int min, int max, DelayedReveal delayedReveal, String selectPrompt, Player decider) {
        throw unsupported("unsupported decision: chooseCardsForZoneChange");
    }
}
