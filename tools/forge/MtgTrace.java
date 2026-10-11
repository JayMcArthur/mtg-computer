package forge.ai;

import com.google.common.eventbus.Subscribe;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonObject;
import forge.game.Game;
import forge.game.card.Card;
import forge.game.card.CardView;
import forge.game.event.*;
import forge.game.spellability.SpellAbility;
import forge.game.player.Player;
import forge.game.zone.ZoneType;

import java.io.BufferedWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.nio.charset.StandardCharsets;
import java.util.HexFormat;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/** Observes engine events and checkpoint state without changing the game. */
public final class MtgTrace implements AutoCloseable {
    private final Game game;
    private final Map<Integer, String> names = new LinkedHashMap<>();
    private final Map<Integer, Long> casts = new LinkedHashMap<>();
    private final BufferedWriter writer;
    private final Gson gson = new GsonBuilder().serializeNulls().create();
    private IOException failure;
    private RuntimeException eventFailure;
    private long sequence;

    public MtgTrace(Game game, Map<String, Card> objects, JsonObject manifest, Path path) throws IOException {
        this.game = game;
        objects.forEach((name, card) -> names.put(card.getId(), name));
        Files.createDirectories(path.toAbsolutePath().getParent());
        writer = Files.newBufferedWriter(path);
        Map<String, Object> header = new LinkedHashMap<>();
        header.put("kind", "header");
        header.put("schema", 1);
        header.put("evidence", "forge-prepared-game");
        header.put("manifest", manifest);
        try {
            header.put("manifest_sha256", HexFormat.of().formatHex(MessageDigest.getInstance("SHA-256")
                    .digest(manifest.toString().getBytes(StandardCharsets.UTF_8))));
        } catch (NoSuchAlgorithmException error) {
            throw new IllegalStateException(error);
        }
        write(header);
        game.subscribeToEvents(this);
    }

    private String id(CardView card) {
        return names.getOrDefault(card.getId(), "engine-" + card.getId());
    }

    public String cardId(Card card) {
        return names.getOrDefault(card.getId(), "engine-" + card.getId());
    }

    public void decision(Player player, String decision, List<String> options, String reason) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("kind", "decision");
        row.put("player", player.getName());
        row.put("decision", decision);
        row.put("options", options);
        row.put("reason", reason);
        write(row);
    }

    public void rejectDecision(RuntimeException error) {
        if (eventFailure == null) eventFailure = error;
    }

    @Subscribe
    public void event(GameEvent event) {
        // Guava logs subscriber exceptions rather than returning them to the game loop.
        try {
            recordEvent(event);
        } catch (RuntimeException error) {
            eventFailure = error;
        }
    }

    private void recordEvent(GameEvent event) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("kind", event.getClass().getSimpleName());
        if (game.getStack().isResolving() && !game.getStack().isEmpty()) {
            SpellAbility resolving = game.getStack().peekAbility();
            if (resolving != null) row.put("parent", casts.get(resolving.getId()));
        }
        if (event instanceof GameEventTurnPhase phase) {
            row.put("player", phase.playerTurn().getName());
            row.put("phase", phase.phase().name());
            System.out.println("MTG turn " + game.getPhaseHandler().getTurn() + " " + phase);
        } else if (event instanceof GameEventCardCounters counters) {
            row.put("card", id(counters.card()));
            row.put("counter", counters.type().toString());
            row.put("before", counters.oldValue());
            row.put("after", counters.newValue());
        } else if (event instanceof GameEventCardPhased phased) {
            row.put("card", id(phased.card()));
            row.put("phased_out", phased.phaseState());
        } else if (event instanceof GameEventCardDamaged damage) {
            row.put("card", id(damage.card()));
            row.put("source", id(damage.source()));
            row.put("amount", damage.amount());
            row.put("damage_type", damage.type().name());
        } else if (event instanceof GameEventPlayerDamaged damage) {
            row.put("player", damage.target().getName());
            row.put("source", id(damage.source()));
            row.put("amount", damage.amount());
            row.put("combat", damage.combat());
        } else if (event instanceof GameEventCardChangeZone zone) {
            row.put("card", id(zone.card()));
            row.put("from", zone.from() == null ? null : zone.from().zoneType().name());
            row.put("to", zone.to() == null ? null : zone.to().zoneType().name());
        } else if (event instanceof GameEventSpellAbilityCast cast) {
            row.put("card", id(cast.sa().getHostCard()));
            row.put("ability", cast.sa().getId());
            row.put("targets", cast.targetDescription());
            casts.put(cast.sa().getId(), sequence);
        } else if (event instanceof GameEventSpellResolved resolved) {
            row.put("card", id(resolved.spell().getHostCard()));
            row.put("ability", resolved.spell().getId());
            row.put("fizzled", resolved.hasFizzled());
            row.put("description", resolved.stackDescription());
        } else if (!(event instanceof GameEventGameOutcome)) {
            return;
        }
        row.put("description", row.getOrDefault("description", event.toString()));
        write(row);
    }

    public void checkpoint() throws IOException {
        if (failure != null) throw failure;
        if (eventFailure != null) throw eventFailure;
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("kind", "checkpoint");
        row.put("evidence", "forge-prepared-game");
        List<Map<String, Object>> cards = new ArrayList<>();
        for (ZoneType zone : List.of(ZoneType.Battlefield, ZoneType.Library, ZoneType.Hand, ZoneType.Graveyard, ZoneType.Exile)) {
            for (Card card : game.getCardsIncludePhasingIn(zone)) {
                Map<String, Object> state = new LinkedHashMap<>();
                state.put("id", names.getOrDefault(card.getId(), "engine-" + card.getId()));
                state.put("name", card.getName());
                state.put("object", card.getGameTimestamp());
                state.put("controller", card.getController().getName());
                state.put("owner", card.getOwner().getName());
                state.put("zone", zone.name());
                state.put("type", card.getType().toString());
                state.put("power", card.getNetPower());
                state.put("toughness", card.getNetToughness());
                var power = card.getNetPowerBreakdown();
                var toughness = card.getNetToughnessBreakdown();
                state.put("power_breakdown", Map.of("current", power.currentValue,
                        "boost", power.tempBoost, "counters", power.bonusFromCounters));
                state.put("toughness_breakdown", Map.of("current", toughness.currentValue,
                        "boost", toughness.tempBoost, "counters", toughness.bonusFromCounters));
                state.put("tapped", card.isTapped());
                state.put("phased_out", card.isPhasedOut());
                Map<String, Integer> counters = new LinkedHashMap<>();
                card.getCounters().entrySet().forEach(entry -> counters.put(entry.getElement().toString(), entry.getCount()));
                state.put("counters", counters);
                state.put("keywords", card.getKeywords().stream().map(keyword -> keyword.getOriginal()).toList());
                cards.add(state);
            }
        }
        row.put("cards", cards);
        row.put("players", game.getPlayers().stream().map(player -> Map.of(
                "name", player.getName(), "life", player.getLife(),
                "mana", player.getManaPool().totalMana())).toList());
        write(row);
        writer.flush();
    }

    public void finish(String status, String detail) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("kind", "result");
        row.put("status", status);
        row.put("detail", detail);
        if (game.isGameOver()) row.put("draw", game.getOutcome().isDraw());
        write(row);
    }

    private void write(Map<String, Object> row) {
        row.put("id", sequence++);
        row.put("turn", game.getPhaseHandler().getTurn());
        row.put("phase", String.valueOf(game.getPhaseHandler().getPhase()));
        try {
            writer.write(gson.toJson(row));
            writer.newLine();
        } catch (IOException error) {
            failure = error;
        }
    }

    @Override
    public void close() throws IOException {
        writer.close();
        if (failure != null) throw failure;
    }
}
