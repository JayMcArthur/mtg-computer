package forge.ai;

import com.google.gson.JsonParser;
import forge.GuiDesktop;
import forge.game.card.CounterType;
import forge.gui.GuiBase;
import forge.localinstance.properties.ForgePreferences.FPref;
import forge.model.FModel;

import java.nio.file.Files;
import java.nio.file.Path;

/** Runs the actual engine from a prepared board and checks independently predicted counters. */
public final class MtgRunner {
    public static void main(String[] args) {
        try {
            execute(args);
            System.exit(0);
        } catch (Exception error) {
            // Forge's desktop uncaught handler opens a modal bug-report dialog.
            // The command-line host must return a failure status instead.
            System.err.println("MTG run failed: " + error);
            System.exit(1);
        }
    }

    private static void execute(String[] args) throws Exception {
        if (args.length != 3) throw new IllegalArgumentException("MtgRunner MANIFEST TRACE MAX_BOUNDARIES");
        int limit = Integer.parseInt(args[2]);
        if (limit < 1) throw new IllegalArgumentException("MAX_BOUNDARIES must be positive");
        GuiBase.setInterface(new GuiDesktop());
        FModel.initialize(null, preferences -> {
            preferences.setPref(FPref.LOAD_CARD_SCRIPTS_LAZILY, false);
            preferences.setPref(FPref.UI_LANGUAGE, "en-US");
            return null;
        });
        MtgBoard board = new MtgBoard(JsonParser.parseString(Files.readString(Path.of(args[0]))).getAsJsonObject());
        run(board, Path.of(args[1]), limit);
    }

    public static void run(MtgBoard board, Path path, int limit) throws Exception {
        var game = board.game;
        try (MtgTrace trace = new MtgTrace(game, board.objects, board.manifest, path)) {
            for (var player : game.getPlayers()) {
                player.dangerouslySetController(new MtgController(game, player, player.getLobbyPlayer(), trace));
            }
            trace.checkpoint();
            game.getPhaseHandler().setupFirstTurn(game.getPlayers().get(1), null);
            int steps = 0;
            try {
                while (!game.isGameOver() && steps < limit) {
                    game.getPhaseHandler().mainLoopStep();
                    trace.checkpoint();
                    steps++;
                }
                if (!game.isGameOver()) {
                    throw new IllegalStateException("host boundary limit reached without a Magic outcome");
                }
                verify(board);
                trace.finish("verified", "actual Magic draw and all 12 register counters agree with the ISA");
            } catch (RuntimeException error) {
                trace.finish(!game.isGameOver() && steps == limit ? "host-limit" : "failed", error.toString());
                throw error;
            }
        }
    }

    private static void verify(MtgBoard board) {
        if (!board.game.getOutcome().isDraw()) throw new IllegalStateException("expected HaltD draw");
        var expected = board.manifest.getAsJsonArray("expected_registers");
        for (int r = 0; r < 12; r++) {
            int actual = board.objects.get("r" + r).getCounters(CounterType.getType("P1P1"));
            if (actual != expected.get(r).getAsInt()) {
                throw new IllegalStateException("r" + r + ": expected " + expected.get(r) + ", actual " + actual);
            }
        }
    }
}
