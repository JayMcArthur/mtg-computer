package forge.ai;

import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import forge.game.Game;
import forge.game.card.Card;
import forge.game.card.CounterType;
import forge.game.combat.Combat;
import forge.game.combat.CombatUtil;
import org.testng.annotations.Test;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.LinkedHashMap;
import java.util.Map;

import static org.testng.Assert.*;

/** Imports the Rust prepared manifest; no ISA operation mutates this game. */
public class MtgPreparedBoardTest extends AITest {
    private final Map<String, Card> objects = new LinkedHashMap<>();
    private JsonObject manifest;

    @Test
    public void completeBoardKeepsProgramForestANoncreatureLand() throws Exception {
        MtgBoard board = prepare();
        Card forest = addCard("Forest", board.game.getPlayers().get(1));
        board.game.getAction().checkStaticAbilities();
        assertEquals(forest.getType().getLandTypes(), java.util.Set.of("Island"),
                "Terrain must overwrite the basic Forest subtype in the complete environment");
        assertFalse(forest.isCreature());
        assertFalse(board.objects.get("source-rider").isLand(),
                "rewriting Human in the Rider's ability must not create an intrinsic Saproling body");
        assertTrue(board.objects.get("source-rider").getStaticAbilities().stream().anyMatch(a -> a.isIntrinsic()));
    }

    @Test
    public void preparedBoardDerivesSuppressedGadgetsAndRegisterValues() throws Exception {
        MtgBoard board = prepare();
        Game game = board.game;
        assertFalse(game.isGameOver());
        Card fighter = objects.get("fighter-X");
        assertTrue(fighter.hasKeyword("Reach"), "a keyword counter must create its actual keyword effect");
        assertTrue(objects.get("bloodthirster").hasKeyword("Double Strike"),
                "the imported keyword counter must create the two combat damage steps");
        assertFalse(objects.get("fighter-Y").hasKeyword("Reach"),
                "Blood Sun must suppress an earlier keyword counter while the fighter is a land");
        Combat combat = new Combat(game.getPlayers().get(1));
        for (int n = 0; n < 12; n++) {
            Card gate = objects.get("gate-X-" + n);
            combat.addAttacker(gate, game.getPlayers().get(0));
        }
        for (int n = 0; n < 12; n++) {
            assertEquals(CombatUtil.canBlock(objects.get("gate-X-" + n), fighter, combat), n == 0,
                    "complete board must retain the unique printed-land match");
        }
    }

    @Test
    public void fullReaderAddAndHaltUsesTheActualGameLoop() throws Exception {
        MtgBoard board = prepare();
        Game game = board.game;
        MtgRunner.run(board, Path.of(System.getProperty("mtg.trace")), 300);
        assertTrue(game.isGameOver(), "host limit reached; this is not a game result");
        assertTrue(game.getOutcome().isDraw());
        assertEquals(objects.get("r1").getCounters(CounterType.getType("P1P1")), 5);
        assertEquals(objects.get("r2").getCounters(CounterType.getType("P1P1")), 3);
    }

    private MtgBoard prepare() throws Exception {
        manifest = JsonParser.parseString(Files.readString(
                Path.of(System.getProperty("mtg.fixture")))).getAsJsonObject();
        MtgBoard board = new MtgBoard(manifest);
        objects.clear();
        objects.putAll(board.objects);
        assertTrue(objects.get("program-5").isLand());
        assertFalse(objects.get("program-5").isCreature());
        assertTrue(objects.get("add").getTriggers().isEmpty());
        assertTrue(objects.get("halt").getTriggers().isEmpty());
        for (int r = 0; r < 12; r++) {
            Card register = objects.get("r" + r);
            assertTrue(register.isLand());
            assertFalse(register.getStaticAbilities().stream().anyMatch(a -> a.isIntrinsic()));
            assertEquals(register.getNetPower(), r == 1 ? 1 : r == 2 ? 2 : -1);
        }
        return board;
    }
}
