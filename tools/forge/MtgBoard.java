package forge.ai;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.common.collect.HashMultiset;
import com.google.common.collect.Multiset;
import forge.card.CardType;
import forge.card.ColorSet;
import forge.card.GamePieceType;
import forge.card.mana.ManaCost;
import forge.game.Game;
import forge.game.GameRules;
import forge.game.GameStage;
import forge.game.GameType;
import forge.game.Match;
import forge.game.ability.AbilityFactory;
import forge.game.trigger.TriggerHandler;
import forge.deck.Deck;
import forge.game.card.Card;
import forge.game.card.CounterType;
import forge.game.player.Player;
import forge.game.player.RegisteredPlayer;
import forge.game.zone.ZoneType;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import forge.model.FModel;

/** One prepared-state importer shared by the command-line runner and its tests. */
public final class MtgBoard {
    public final JsonObject manifest;
    public final Map<String, Card> objects = new LinkedHashMap<>();
    public final Game game;

    public MtgBoard(JsonObject manifest) {
        this.manifest = manifest;
        game = prepare();
    }

    private static void require(boolean condition, String message) {
        if (!condition) throw new IllegalArgumentException(message);
    }

    private Game prepare() {
        List<RegisteredPlayer> players = List.of(
                new RegisteredPlayer(new Deck()).setPlayer(new LobbyPlayerAi("Bob", null)),
                new RegisteredPlayer(new Deck()).setPlayer(new LobbyPlayerAi("Alice", null)));
        GameRules rules = new GameRules(GameType.Constructed);
        Game game = new Match(rules, players, "MTG microcontroller").createGame();
        game.setAge(GameStage.Play);
        require(manifest.get("schema").getAsInt() == 3, "unsupported manifest schema");
        Player alice = game.getPlayers().get(1);
        Player bob = game.getPlayers().get(0);
        alice.setLife(1, null);
        bob.setLife(1, null);
        for (JsonElement entry : manifest.getAsJsonArray("cards")) {
            JsonObject spec = entry.getAsJsonObject();
            Player controller = spec.get("controller").getAsString().equals("Alice") ? alice : bob;
            Player owner = spec.get("owner").getAsString().equals("Alice") ? alice : bob;
            Card card;
            if (spec.getAsJsonObject("copiable").get("blank_rules").getAsBoolean()) {
                require(spec.get("token").getAsBoolean(), "blank rules require a token");
                card = new Card(game.nextCardId(), game);
                card.setName(spec.get("name").getAsString());
                card.setOwner(owner);
            } else {
                var paper = FModel.getMagicDb().getCommonCards().getCard(spec.get("name").getAsString());
                require(paper != null, "unknown card " + spec.get("name"));
                card = Card.fromPaperCard(paper, owner);
            }
            card.setController(controller, spec.get("timestamp").getAsLong());
            card.setGameTimestamp(spec.get("timestamp").getAsLong());
            game.dangerouslySetTimestamp(Math.max(game.getTimestamp(), spec.get("timestamp").getAsLong()));
            if (spec.get("token").getAsBoolean()) card.setGamePieceType(GamePieceType.TOKEN);
            copyValues(card, spec.getAsJsonObject("copiable"));
            for (JsonElement change : spec.getAsJsonArray("modifications")) {
                modify(card, change.getAsJsonObject());
                game.dangerouslySetTimestamp(Math.max(game.getTimestamp(), change.getAsJsonObject().get("timestamp").getAsLong()));
            }
            Multiset<CounterType> counters = HashMultiset.create();
            spec.getAsJsonObject("counters").entrySet().forEach(counter ->
                    counters.add(CounterType.getType(counter.getKey()), counter.getValue().getAsInt()));
            card.setCounters(counters);
            spec.getAsJsonObject("counter_timestamps").entrySet().forEach(counter -> {
                require(CounterType.getType(counter.getKey()).isKeywordCounter(),
                        "timestamp belongs to non-keyword counter " + counter.getKey() + " on " + spec.get("id"));
                var ability = card.getHiddenStaticAbilities().stream()
                        .filter(a -> a.hasParam("AddKeyword") && a.getParam("AddKeyword").equals(counter.getKey()))
                        .findFirst().orElseThrow(() -> new IllegalArgumentException("missing keyword-counter effect " + counter.getKey()));
                ability.putParam("Timestamp", counter.getValue().getAsString());
            });
            List<String> choices = strings(spec.get("chosen_types"));
            if (!choices.isEmpty()) card.setChosenType(choices.get(0));
            if (choices.size() == 2) card.setChosenType2(choices.get(1));
            card.setChosenColors(strings(spec.get("chosen_colors")));
            card.setTapped(spec.get("tapped").getAsBoolean());
            card.setSickness(false);
            controller.getZone(ZoneType.valueOf(spec.get("zone").getAsString())).add(card);
            objects.put(spec.get("id").getAsString(), card);
        }
        game.getAction().checkStaticAbilities();
        attach(manifest, alice);
        game.getAction().checkStaticAbilities();

        return game;
    }

    private void attach(JsonObject manifest, Player alice) {
        for (JsonElement entry : manifest.getAsJsonArray("cards")) {
            JsonObject spec = entry.getAsJsonObject();
            Card card = objects.get(spec.get("id").getAsString());
            if (!spec.get("imprint").isJsonNull()) {
                Card imprint = objects.get(spec.get("imprint").getAsString());
                imprint.setExiledWith(card);
                card.addImprintedCard(imprint);
            }
            if (!spec.get("attached_to").isJsonNull()) {
                Card host = objects.get(spec.get("attached_to").getAsString());
                // Existing attachments can be suppressed lands in the prepared state.
                require(host.cantBeAttachedMsg(card, null) == null, "invalid attachment " + spec.get("id")
                        + " " + card.getType() + " -> " + host.getType());
                card.attachToEntity(host, null);
                require(card.getEntityAttachedTo() == host, "attachment failed " + spec.get("id"));
            }
            if (!spec.get("enchanting_player").isJsonNull()) {
                var player = spec.get("enchanting_player").getAsString().equals("Alice") ? alice
                        : game.getPlayers().get(0);
                card.attachToEntity(player, null);
            }
            card.setLayerTimestamp(spec.get("timestamp").getAsLong());
        }
    }

    private void copyValues(Card card, JsonObject copy) {
        if (!copy.get("colors").isJsonNull()) card.setColor(ColorSet.fromNames(strings(copy.get("colors"))));
        if (!copy.get("card_types").isJsonNull()) card.setType(new CardType(strings(copy.get("card_types")), false));
        if (!copy.get("creature_types").isJsonNull()) card.setCreatureTypes(strings(copy.get("creature_types")));
        if (!copy.get("base_pt").isJsonNull()) {
            card.setBasePower(copy.getAsJsonArray("base_pt").get(0).getAsInt());
            card.setBaseToughness(copy.getAsJsonArray("base_pt").get(1).getAsInt());
        }
        if (!copy.get("mana_cost").isJsonNull()) {
            String cost = copy.get("mana_cost").getAsString();
            card.setManaCost(cost.equals("no cost") ? ManaCost.NO_COST : new ManaCost(cost));
        }
        card.addIntrinsicKeywords(strings(copy.get("keywords")));
    }

    private void modify(Card card, JsonObject change) {
        long timestamp = change.get("timestamp").getAsLong();
        switch (change.get("kind").getAsString()) {
            case "TypeWord" -> card.addChangedTextTypeWord(change.get("from").getAsString(), change.get("to").getAsString(), timestamp, 0);
            case "ColorWord" -> card.addChangedTextColorWord(change.get("from").getAsString(), change.get("to").getAsString(), timestamp, 0);
            case "AddTypes" -> card.addChangedCardTypes(new CardType(strings(change.get("types")), true),
                    new CardType(true), false, Set.of(), timestamp, 0, true, false);
            case "Color" -> card.addColor(ColorSet.fromNames(strings(change.get("colors"))), false, timestamp, null);
            case "BasePt" -> card.addNewPT(change.get("power").getAsInt(), change.get("toughness").getAsInt(), timestamp, 0);
            case "Keyword" -> card.addChangedCardKeywords(List.of(change.get("keyword").getAsString()), List.of(), false, timestamp, null);
            case "RemoveKeyword" -> card.addChangedCardKeywords(List.of(), List.of(change.get("keyword").getAsString()), false, timestamp, null);
            case "CombatDraw" -> {
                var trigger = TriggerHandler.parseTrigger("Mode$ DamageDone | ValidSource$ Card.Self | ValidTarget$ Player | CombatDamage$ True | TriggerZones$ Battlefield | TriggerDescription$ Whenever CARDNAME deals combat damage to a player, draw a card.", card, false);
                trigger.setOverridingAbility(AbilityFactory.getAbility("DB$ Draw | Defined$ You | NumCards$ 1", card));
                card.addChangedCardTraits(List.of(), List.of(trigger), List.of(), List.of(), trait -> false, timestamp, 0);
            }
            default -> throw new IllegalArgumentException("unsupported prepared modification " + change);
        }
    }

    private List<String> strings(JsonElement array) {
        List<String> result = new ArrayList<>();
        for (JsonElement item : array.getAsJsonArray()) result.add(item.getAsString());
        return result;
    }
}
