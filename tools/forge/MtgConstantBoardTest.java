package forge.ai;

import com.google.gson.JsonParser;
import forge.game.card.Card;
import forge.game.combat.Combat;
import forge.game.combat.CombatUtil;
import forge.game.player.Player;
import org.testng.annotations.Test;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import static org.testng.Assert.*;

/** Complete-board legality checks for the new sparse constant inventory. */
public class MtgConstantBoardTest extends AITest {
    @Test
    public void firstNumberHasUniqueBlockersAndLateRegisterKeywords() throws Exception {
        MtgBoard board = new MtgBoard(JsonParser.parseString(Files.readString(
                Path.of(System.getProperty("mtg.constants")))).getAsJsonObject());
        Card register = board.objects.get("r0");
        assertTrue(register.isLand());
        assertTrue(register.hasKeyword("Indestructible"));
        assertTrue(register.hasKeyword("Vigilance"));
        assertTrue(board.objects.get("inactive-register-boost").getManaCost().isNoCost());
        assertFalse(register.hasKeyword("Flying"));
        assertEquals(register.getNetPower(), 0);
        phase(board, "gate-X-10", "fighter-X");
        assertTrue(register.hasKeyword("Flying"));
        assertTrue(register.hasKeyword("Double Strike"));
        assertBlockers(board, "num-excruciator", "r0");
        assertBlockers(board, "num-fighter", "num-vigor");
        phase(board, "gate-Y-0", "fighter-Y");
        assertEquals(register.getNetPower(), 0);
        assertTrue(board.objects.get("inactive-register-boost").isPhasedOut());
        phase(board, "gate-Z-2", "fighter-Z");
        assertEquals(board.objects.get("num-myr-0").getNetPower(), 2);
        assertEquals(board.objects.get("num-myr-0").getNetToughness(), 3);
        assertEquals(board.objects.get("num-equity").getChosenColors(), List.of("Red"));
        assertFalse(board.objects.get("set-party-Warrior").getStaticAbilities().stream().anyMatch(a -> a.isIntrinsic()));
        register.setCounters(forge.game.card.CounterEnumType.P1P1, 2);
        board.game.getAction().checkStaticAbilities();
        assertEquals(register.getNetPower(), 2);
        // Bob's Fighter returns with both attachments; Z0 loses its separate boost.
        board.objects.get("fighter-Y").phase(false);
        board.game.getAction().checkStaticAbilities();
        assertFalse(board.objects.get("inactive-register-boost").isPhasedOut());
        assertTrue(board.objects.get("gate-Z-0").isLand());
        assertEquals(register.getNetPower(), 2, "the synchronized helper must preserve the counter-value power");
    }

    private void phase(MtgBoard board, String... ids) {
        for (String id : ids) board.objects.get(id).phase(true);
        board.game.getAction().checkStaticAbilities();
    }

    private void assertBlockers(MtgBoard board, String attackerId, String... expected) {
        Card attacker = board.objects.get(attackerId);
        Player defender = board.game.getPlayers().stream().filter(p -> p != attacker.getController()).findFirst().orElseThrow();
        Combat combat = new Combat(attacker.getController());
        combat.addAttacker(attacker, defender);
        List<String> actual = defender.getCreaturesInPlay().stream()
                .filter(c -> CombatUtil.canBlock(attacker, c, combat))
                .map(c -> board.objects.entrySet().stream().filter(e -> e.getValue() == c).findFirst().orElseThrow().getKey()).toList();
        assertEquals(actual, List.of(expected), "complete-board blockers for " + attackerId);
    }
}
