package forge.ai;

import forge.game.Game;
import forge.game.ability.AbilityKey;
import forge.game.ability.AbilityUtils;
import forge.game.card.Card;
import forge.game.card.CounterEnumType;
import forge.game.combat.CombatUtil;
import forge.game.player.Player;
import forge.game.spellability.SpellAbility;
import forge.game.trigger.Trigger;
import forge.game.zone.ZoneType;
import forge.card.CardType;
import org.testng.annotations.Test;
import org.testng.annotations.DataProvider;

import static org.testng.Assert.*;

/** Prepared-state experiment; this does not execute the setup spells. */
public class MtgGateTest extends AITest {
    private static final String[] LANDS = {
        "Plains", "Island", "Swamp", "Mountain", "Forest", "Wastes",
        "Snow-Covered Plains", "Snow-Covered Island", "Snow-Covered Swamp",
        "Snow-Covered Mountain", "Snow-Covered Forest", "Snow-Covered Wastes"
    };

    @DataProvider(name = "symbols")
    public Object[][] symbols() {
        Object[][] inputs = new Object[12][1];
        for (int i = 0; i < inputs.length; i++) inputs[i][0] = i;
        return inputs;
    }

    @Test
    public void terrainOverwritesForestBeforeLifeAndLimbEvenWhenSuppressed() {
        Game game = initAndCreateGame();
        Player alice = game.getPlayers().get(1);
        addCard("Stormtide Leviathan", alice);
        Card terrain = addCard("Illusionary Terrain", alice);
        terrain.setChosenType("Island");
        terrain.setChosenType2("Island");
        Card forest = addCard("Forest", alice);
        game.getAction().checkStaticAbilities();
        assertTrue(forest.getType().hasSubtype("Island"));
        assertFalse(forest.getType().hasSubtype("Forest"), "setting Island must overwrite Forest");
        Card life = addCard("Life and Limb", alice);
        game.getAction().checkStaticAbilities();
        assertFalse(forest.isCreature(), "the program Forest must remain a noncreature land");
        terrain.addType("Creature");
        terrain.setCreatureTypes(java.util.List.of("Masticore"));
        terrain.setBasePower(1);
        terrain.setBaseToughness(1);
        Card gate = addCard("Dralnu's Crusade", alice);
        gate.addChangedTextTypeWord("Goblin", "Masticore", game.getNextTimestamp(), 0);
        gate.addChangedTextTypeWord("Zombie", "Saproling", game.getNextTimestamp(), 0);
        addCard("Blood Sun", alice);
        life.setGameTimestamp(game.getNextTimestamp());
        game.getAction().checkStaticAbilities();
        assertTrue(terrain.isLand());
        assertFalse(forest.isCreature(), "layer 4 Terrain effect must survive layer 6 ability removal");
    }

    @Test
    public void editedGateSuppressesAbilitiesAndPhasesWithoutZoneChange() {
        Game game = initAndCreateGame();
        Player alice = game.getPlayers().get(1);
        Player bob = game.getPlayers().get(0);
        Card gate = addCard("Dralnu's Crusade", alice);
        gate.addType("Creature");
        gate.setCreatureTypes(java.util.List.of("Sliver"));
        gate.setBasePower(2);
        gate.setBaseToughness(1);
        gate.addChangedTextTypeWord("Goblin", "Elf", game.getNextTimestamp(), 0);
        gate.addChangedTextTypeWord("Zombie", "Saproling", game.getNextTimestamp(), 0);
        Card arithmetic = addCard("Halana and Alena, Partners", bob);
        arithmetic.setCreatureTypes(java.util.List.of("Elf", "Rabbit"));
        addCard("Life and Limb", alice);
        addCard("Blood Sun", alice);
        Card fighter = addCard("Dream Fighter", bob);
        Card attachment = addCard("Dralnu's Crusade", bob);
        attachment.addType("Artifact");
        attachment.addChangedTextTypeWord("Goblin", "Rabbit", game.getNextTimestamp(), 0);
        attachment.addChangedTextTypeWord("Zombie", "Saproling", game.getNextTimestamp(), 0);
        addCard("Bludgeon Brawl", alice);
        game.getAction().checkStaticAbilities();
        assertTrue(attachment.isEquipment());
        attachment.attachToEntity(fighter, null);

        assertTrue(arithmetic.isLand(), "edited Elf gate supplies Saproling, then Forest");
        assertEquals(arithmetic.getNetPower(), 3, "1/1 plus two Crusades");
        assertTrue(arithmetic.getTriggers().isEmpty(), "Blood Sun removes the combat trigger");
        assertTrue(CombatUtil.canBlock(gate, fighter));
        int objectId = gate.getId();
        long timestamp = gate.getGameTimestamp();
        Trigger blocked = fighter.getTriggers().iterator().next();
        SpellAbility phase = blocked.ensureAbility();
        phase.setActivatingPlayer(bob);
        phase.setTriggeringObject(AbilityKey.Attacker, gate);
        phase.setTriggeringObject(AbilityKey.Blocker, fighter);
        AbilityUtils.resolve(phase);
        game.getAction().checkStaticAbilities();

        assertTrue(gate.isPhasedOut());
        assertTrue(fighter.isPhasedOut());
        assertTrue(attachment.isPhasedOut(), "wildcard gate phases indirectly with Fighter");
        assertSame(attachment.getEntityAttachedTo(), fighter);
        assertFalse(arithmetic.isLand());
        assertFalse(arithmetic.getTriggers().isEmpty(), "phased gate no longer suppresses ability");
        assertEquals(arithmetic.getNetPower(), 2);
        assertSame(gate.getZone(), alice.getZone(ZoneType.Battlefield));
        assertEquals(gate.getId(), objectId);
        assertEquals(gate.getGameTimestamp(), timestamp);

        gate.phase(true);
        fighter.phase(true);
        game.getAction().checkStaticAbilities();
        assertFalse(gate.isPhasedOut());
        assertTrue(arithmetic.isLand());
        assertTrue(arithmetic.getTriggers().isEmpty());
        assertFalse(attachment.isPhasedOut());
        assertSame(attachment.getEntityAttachedTo(), fighter);
        assertEquals(gate.getId(), objectId);
        assertEquals(gate.getGameTimestamp(), timestamp);
    }

    @Test
    public void addAbilityUsesDerivedWarcallerPowerAndActualCounters() throws Exception {
        Game game = initAndCreateGame();
        Player bob = game.getPlayers().get(0);
        Card source = addCard("Joraga Warcaller", bob);
        source.setCreatureTypes(java.util.List.of("Camarid", "Rabbit"));
        source.setCounters(CounterEnumType.P1P1, 3);
        Card destination = addCard("Joraga Warcaller", bob);
        destination.setCreatureTypes(java.util.List.of("Brushwagg", "Rabbit"));
        destination.setCounters(CounterEnumType.P1P1, 2);
        Card inactiveGate = addCard("Dralnu's Crusade", bob);
        inactiveGate.addChangedTextTypeWord("Goblin", "Brushwagg", game.getNextTimestamp(), 0);
        inactiveGate.addChangedTextTypeWord("Zombie", "Saproling", game.getNextTimestamp(), 0);
        addCard("Life and Limb", bob);
        addCard("Blood Sun", bob);
        Card arithmetic = addCard("Halana and Alena, Partners", bob);
        arithmetic.setCreatureTypes(java.util.List.of("Elf"));
        arithmetic.setBasePower(0);
        arithmetic.setBaseToughness(2);
        game.getAction().checkStaticAbilities();
        assertTrue(destination.getStaticAbilities().isEmpty());
        assertEquals(arithmetic.getNetPower(), 3);

        SpellAbility add = arithmetic.getTriggers().iterator().next().ensureAbility();
        add.setActivatingPlayer(bob);
        add.getTargets().add(destination);
        try (MtgTrace trace = new MtgTrace(game, java.util.Map.of("destination", destination),
                new com.google.gson.JsonObject(), java.nio.file.Path.of(System.getProperty("mtg.trace") + ".isolated"))) {
            MtgController controller = new MtgController(game, bob, bob.getLobbyPlayer(), trace);
            bob.dangerouslySetController(controller);
            controller.playSpellAbilityNoStack(add, false);
            trace.checkpoint();
        }
        game.getAction().checkStaticAbilities();
        assertEquals(destination.getCounters(CounterEnumType.P1P1), 5);
        assertEquals(source.getCounters(CounterEnumType.P1P1), 3);
    }

    @Test
    public void rejectedCounterChoiceSurvivesTheEngineCatch() throws Exception {
        Game game = initAndCreateGame();
        Player bob = game.getPlayers().get(0);
        Card arithmetic = addCard("Halana and Alena, Partners", bob);
        Card destination = addCard("Joraga Warcaller", bob);
        SpellAbility ability = arithmetic.getTriggers().iterator().next().ensureAbility();
        ability.setActivatingPlayer(bob);
        ability.getTargets().add(destination);
        ability.putParam("CounterType", "P1P1,M1M1");
        try (MtgTrace trace = new MtgTrace(game, java.util.Map.of("destination", destination),
                new com.google.gson.JsonObject(), java.nio.file.Path.of(System.getProperty("mtg.trace") + ".rejection"))) {
            bob.dangerouslySetController(new MtgController(game, bob, bob.getLobbyPlayer(), trace));
            // Forge catches controller exceptions in CountersPutEffect; the host must still fail.
            AbilityUtils.resolve(ability);
            assertEquals(destination.getCounters(CounterEnumType.P1P1), 0);
            assertThrows(IllegalStateException.class, () -> trace.checkpoint());
        }
    }

    @Test
    public void recordingRetainsTheExactPreparedManifest() throws Exception {
        Game game = initAndCreateGame();
        var manifest = com.google.gson.JsonParser.parseString("{\"copiable\":{\"power\":null},\"attached_to\":null}").getAsJsonObject();
        var path = java.nio.file.Path.of(System.getProperty("mtg.trace") + ".manifest");
        try (MtgTrace trace = new MtgTrace(game, java.util.Map.of(), manifest, path)) {
            trace.checkpoint();
        }
        var first = java.nio.file.Files.readAllLines(path).get(0);
        assertEquals(com.google.gson.JsonParser.parseString(first).getAsJsonObject().get("manifest"), manifest);
    }

    @Test(dataProvider = "symbols")
    public void strataAndTetsukoSelectExactlyTheMatchingLand(int symbol) {
        Game game = initAndCreateGame();
        Player alice = game.getPlayers().get(1);
        Player bob = game.getPlayers().get(0);
        addCard(LANDS[symbol], alice);
        addCard("Tetsuko Umezawa, Fugitive", alice);
        addCard("Shadow Sliver", alice);
        Card fighter = addCard("Dream Fighter", bob);
        fighter.addType("Sliver");
        fighter.addIntrinsicKeyword("Reach");
        Card[] gates = new Card[12];
        for (int i = 0; i < gates.length; i++) {
            Card gate = addCard("Dralnu's Crusade", alice);
            gate.addType("Creature");
            gate.setCreatureTypes(java.util.List.of("Sliver"));
            gate.setBasePower(2);
            gate.setBaseToughness(1);
            gate.addIntrinsicKeyword("Flying");
            Card scythe = addCard("Strata Scythe", alice);
            Card imprint = addCardToZone(LANDS[i], alice, ZoneType.Exile);
            imprint.setExiledWith(scythe);
            scythe.addImprintedCard(imprint);
            scythe.attachToEntity(gate, null);
            gates[i] = gate;
        }
        game.getAction().checkStaticAbilities();
        for (int i = 0; i < gates.length; i++) {
            assertEquals(gates[i].getNetPower(), i == symbol ? 3 : 2);
            assertEquals(gates[i].getNetToughness(), i == symbol ? 2 : 1);
            assertEquals(CombatUtil.canBlock(gates[i], fighter), i == symbol,
                    "legal blocker options must match the printed land name");
        }
    }

    @Test
    public void celestialConvergenceResolvesToAnActualDraw() {
        Game game = initAndCreateGame();
        Player alice = game.getPlayers().get(1);
        Player bob = game.getPlayers().get(0);
        alice.setLife(1, null);
        bob.setLife(1, null);
        Card convergence = addCard("Celestial Convergence", bob);
        convergence.addType("Creature");
        convergence.setCreatureTypes(java.util.List.of("Eldrazi", "Antelope"));
        convergence.setBasePower(1);
        convergence.setBaseToughness(1);
        convergence.setCounters(CounterEnumType.OMEN, 1);
        game.getAction().checkStaticAbilities();
        assertFalse(game.isGameOver());
        SpellAbility upkeep = convergence.getTriggers().iterator().next().ensureAbility();
        upkeep.setActivatingPlayer(bob);
        AbilityUtils.resolve(upkeep);
        assertEquals(convergence.getCounters(CounterEnumType.OMEN), 0);
        assertTrue(game.isGameOver());
        assertTrue(game.getOutcome().isDraw());
    }

    @Test
    public void lastingArtifactChangeBeforeBrawlRetainsEquipmentSubtype() {
        Game game = initAndCreateGame();
        Player alice = game.getPlayers().get(1);
        Card wildcard = addCard("Dralnu's Crusade", alice);
        wildcard.addChangedCardTypes(new CardType(java.util.List.of("Artifact"), true),
                new CardType(true), false, java.util.Set.of(), game.getNextTimestamp(), 0, true, false);
        addCard("Bludgeon Brawl", alice);
        game.getAction().checkStaticAbilities();
        assertTrue(wildcard.isEquipment());
        assertTrue(wildcard.isArtifact());
        assertTrue(wildcard.isEnchantment());
    }
}
