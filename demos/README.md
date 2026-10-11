# Recorded Magic demonstration

Open [add-halt.html](add-halt.html) directly in a browser. It is self-contained and makes no network requests.

This is the completed159-checkpoint Forge run of examples/add-halt.mtg: preparation establishes a=2/b=3, actual Magic executes the addition, and Celestial Convergence produces a real draw. Both the Java runner and Rust backend checked all12 register counters. The recorded manifest matches the compiled board exactly. Rules and engine provenance, prepared values, effective board values and decision/event causes are embedded.

Use Instruction stepping to follow source→three symbols→land cards→reader→gadget. Select a card or event for its details; expand the full battlefield when needed. The run demonstrates prepared-state execution. It does not prove legal starting-deck setup or global all-choice confluence. Runtime assignments and loops have separate implementation milestones.

Reproduce with cargo run -- board examples/add-halt.mtg target/demo. Java21 and Maven are required; the pinned engine is built on first use.
