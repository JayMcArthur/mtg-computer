# Prepared Forge execution

The backend uses Forge revision `5222f05e569da8e33c85858e6fe1698901548d64`, Java21 and Maven. Source and build artifacts stay in ignored `references/` and `target/`.

From the project root:

```bash
cargo run --quiet -- fixture > data/construction/add-halt-prepared.json
bash tools/forge/run-gate.sh
cargo run -- replay target/forge/add-halt.jsonl target/forge/add-halt.html
```

`MtgBoard` imports a prepared state; `MtgRunner` advances the actual game loop; `MtgController` rejects ambiguous or unsupported decisions; `MtgTrace` records engine events, decision checks and board checkpoints. A host boundary limit is a failed run without a Magic outcome. No ISA operation updates the Forge game. A completed run must end in an actual draw with all12 counter values matching the independent Rust ISA result.

The tests cover reader gates, printed-land matching, attachments/phasing, real addition, the draw ability, keyword-counter import, and Terrain's Forest overwrite in the complete environment. The full audited Add/Halt command succeeds with a real draw and exact recorded-manifest/counter verification. See [executed evidence](../../docs/engine-experiment.md).

The new standalone command is:

```bash
cargo run -- board examples/add-halt.mtg target/demo
```

It writes `manifest.json`, `trace.jsonl` and an offline `replay.html`. On first use `run-board.sh` builds the pinned engine and records its Maven classpath; later runs recompile the authored helpers against that build. The standalone reactor build succeeds. Inputs, Add and final HaltD use the audited reader profile. Assignments export the new live-recipe NumBuild/Set candidate profile, whose complete run is still under verification. Loops require additional gadgets.

Prepared-state execution and controller decision checks do not establish legal starting decks or setup reachability. [Physical preparation research](../../docs/legal-setup.md) records concrete copy routes and declared repairs; no complete setup action replay exists yet.
