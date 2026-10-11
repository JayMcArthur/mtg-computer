# Research and plan: an observable MTG emulator

Requested on 2026-10-10: investigate the two older videos, determine what exists for the 2024 microcontroller, and plan a genuine emulator with a clear visual representation. This phase changes documentation only; the existing Rust starter remains an instruction simulator.

## Research work plan

1. [done] Identify both videos from primary metadata/captions and the linked implementation; videos-and-conversion.md records the source and access boundary. Actual video frames were not inspected.
2. [done] Inventory the author's simulator/artifact and search public board-emulation/physical-build evidence; emulator-landscape.md records inspected sources and the bounded negative result.
3. [done] Inspect old Python/Java abstractions and pinned Forge/XMage source. Forge has the text-change/layer ingredients and merits the first feasibility experiment; integration remains untested.
4. [done] Define the fidelity contract below, including legal choices, timestamps, phasing, object identity, combat, prepared state and output decoding.
5. [done] Select Add/HaltD from a prepared state and enumerate its dependency groups. A complete expanded construction manifest is explicitly a future implementation task.
6. [done] Specify source-to-board navigation/replay in visual-debugger.md, with evidence labels separating current ISA traces from future board events.
7. [done] Define ordered future implementation phases, concrete artifacts and acceptance evidence below.
8. [done] Re-scan the research/design artifacts and create the fresh residual review below. No new emulator code was implemented during this research phase.

## Working decision

Research the full 2024 construction before minimizing its deck. The theoretical model uses a prepared game state; reproducing that state and proving execution is separate from constructing it legally from decks. Exact 60-card feasibility remains a later decision. No finding that a simulator is missing will be phrased as proof that nobody has made one.

The user requested research and planning, not another implementation pass. The tasks below are future implementation work; none are represented as already completed.

## Feasibility and recommended direction

Yes: a computer can emulate any bounded execution of the construction by applying its game rules. The undecidability result concerns deciding arbitrary halting/outcomes in advance, not computing the next step. A finite host cannot realize unlimited memory, so memory/event budgets must stop the emulator without substituting a game result.

Keep this a **Rust project for compiling, tracing and explaining the 2024 construction**, alongside its ISA reference interpreter. Before choosing the game execution backend, test Forge's existing rules machinery against the difficult shared gate. A specialized Rust rules core is the alternative if adapting the existing engine proves unsuitable. Either backend must execute actual phase/stack/combat transitions and account for every relevant effect and choice in the selected board. Implementing every card in Magic is not a prerequisite.

The recommended first implementation experiment is a bounded Forge fixture with a strict instrumented controller. The [landscape review](emulator-landscape.md) records actual text-change/layer/phasing support, controller gaps and integer limits. If reuse wins on correctness, trace visibility and supported bounded values, retain the Rust compiler/trace system and use a headless adapter. If it fails, classify the smallest failing fixture before choosing a scoped engine patch or the Rust core. Make one architecture choice from this evidence; do not permanently maintain two production engines.

The 2024 compiler target improves actual program work by removing the old conversion cascade. The project can also improve observability and reproducibility even if another emulator exists. Novelty is not a prerequisite for building a useful system.

## Fidelity contract

- Freeze one identified rules version, card-text snapshot, construction recipe and creature-type mapping. Resolve the documented historical Cephalid change explicitly. A historical reproduction and a current-rules port are different claims.
- Model zones, owners/controllers, physical card identities and game-object identities. Moving between zones normally creates a new object; phasing has its own identity/attachment rules.
- Track copiable values separately from later modifications, timestamps and duration. Evaluate the continuous-effect layers and dependencies that determine types, abilities, power and toughness.
- Generate and resolve actual triggers. Observe APNAP, delayed untap-step triggers, target legality at announcement/resolution, replacement effects and state-based actions to a fixed point.
- Generate legal attackers/blockers under requirements and restrictions, extra combat phases, damage steps including double strike, damage prevention, wither and counter cancellation.
- Enumerate meaningful legal choices. A deterministic policy can replay one branch; it does not establish forced execution. Prove a unique option or convergence of all relevant alternatives at each audited boundary.
- Begin with a prepared board snapshot. Executing the setup combo and checking deck reachability is a later independent deliverable. Do not claim a manually imported state has already been constructed legally.
- Decode register/flag/program state from game objects. The rules backend must never invoke an ISA operation to obtain its next state. Compare decoded values against the independent instruction evaluator after the actual instruction boundary.
- Stop on an unsupported active ability, divergent choice branch, invalid construction invariant, resource budget or comparison failure; retain the preceding trace and specific evidence.

## First complete execution

Use a prepared board with r1=2 and r2=3 and the cyclic program:

| Instruction | Symbols | Program cards |
| --- | --- | --- |
| Add r1 r2 | `0 1 2` | Plains, Island, Swamp |
| HaltD | `4 0 0` | Forest, Plains, Plains |

Six program cards are required; support/setup cards are additional. Set the starting program-permanent/library positions so that the first reader action consumes the first symbol, with no off-by-one rotation.

The prepared inputs are source parameters established during setup. This first demonstration does not pretend to execute literal assignments at runtime. It proves the complete reader→gate→arithmetic→decode→game-outcome path for two instructions. The final r1 value is five, and Celestial Convergence actually resolves to end the game in a draw.

The physical recipe basis is the [2024 full construction](https://www.toothycat.net/~hologram/Magic/Magic_Microcontroller_full.pdf), sections 4 and B.1/B.16, supplemented by the [author's live explanation](https://cyh31.neocities.org/amepscimtg/explanation).

### Required dependency groups

| Group | Why it must execute |
| --- | --- |
| Program movement | Vaevictis Asmadi advances the program; Wheel of Sun and Moon recycles sacrificed cards; Tajuru Preserver prevents Bob's sacrifice. |
| Three symbol readers | Strata Scythe distinguishes basic-land names; Tetsuko constrains blockers; Dream Fighter phases the matching gate; Bloodthirster's double-strike damage supplies extra combats. |
| Conditional gates | Modified Dralnu's Crusade, Life and Limb and Blood Sun enable the selected X/Y/Z machinery. Gate attachments and phasing must persist correctly. |
| Register selection | Joraga Warcaller counters supply the value; register gates, modified creature types and protection restrict legal targets. |
| Arithmetic | The active Halana and Alena combat trigger adds the source power as counters to the destination. |
| Game outcome | Celestial Convergence remains inactive until the halt selector enables it, then uses its omen counter and actual life totals. |
| Global constraints | Grand Melee, Stormtide Leviathan, Worship, shroud/protection, ability suppression, draw-step control and the other relevant environment permanents constrain legal actions. |

This is a dependency-group checklist, not a complete deck list. The construction-manifest task must expand it into every permanent, modification, attachment and effect needed by the selected board. Omitting a global card without proving its irrelevance would weaken the claim.

## Implementation phases and acceptance evidence

| Phase | Concrete work and artifacts | Completion evidence |
| --- | --- | --- |
| 0 — Freeze and inventory | `data/construction/` manifest for the Add/Halt fixture; `data/rules/` versioned rules and selected card text; dependency report; recipe discrepancies recorded and resolved. | Every permanent has a source, owner/controller, copiable values, modifications and timestamp order. No active instruction relies on an unresolved recipe. |
| 1 — Test the hardest shared mechanism | A small fixture with the text-edited Crusade → Life and Limb → Blood Sun chain and the Dream Fighter phase-out/re-entry. Inspect an existing engine's handling of the same fixture if practical. | Derived types/abilities change at the correct layers; phasing does not become exile/re-entry; trigger existence changes as expected. Architecture choice is recorded with actual findings. |
| 2 — Board semantics | Implement the selected backend: a headless Forge runner and Rust adapter if phase 1 establishes suitability; otherwise cohesive `src/board/` modules for world/zones, effects, stack, combat and state-based actions. Keep manifests and trace/comparison interfaces in Rust. | Focused cases for actual required rules interactions pass, including target loss and competing replacements. Runtime choices are audited rather than supplied by a first-option controller. Unsupported abilities and numeric limits stop explicitly. |
| 3 — Reader | Execute program movement and all three combats from the prepared board; expose X/Y/Z selection and gate state in structured events. | The full six-card cyclic sequence is read in order over two instructions; exactly the intended selectors activate, and protection/attack/block evidence explains all actions. |
| 4 — First program | Execute Add and HaltD on the complete selected board; decode from game objects; compare with ISA state at agreed boundaries. | r1 changes from 2 to 5 through the resolved ability; source register stays 3; the actual game result is a draw. All reached legal-choice branches converge or expose a specific failure. |
| 5 — Visual replay | Local trace viewer following visual-debugger.md; source/program/board/stack panels, causal selection, step sizes and checkpoint replay. | A viewer can follow the first instruction from source to three land reads to phasing to power/trigger/counters; replay restores identical state hashes. No synthetic board events. |
| 6 — Runtime constants and copy | Add NumBuild and Set/Zero gadgets; independent readable-source evaluator and source-boundary comparison. | A fully compiled `a = 2; b = 3; a += b; halt` starts with zero registers and ends with a=5. Consecutive NumBuild uses the actual tapped/phased Vigor mechanism; self-copy is never compiled as Zero. |
| 7 — Control flow | Add FIsZero, conditional decrement and direct/computed branch gadgets; preserve the paper's flag-combining and advance-before-jump behavior. | A source-compiled bounded summation loop agrees at every source/instruction boundary, including zero input and taken/untaken jumps. Its trace shows real library movement for branching. |
| 8 — Physical construction | Export deck inventory, initial-state inventory, modifications, setup sequence and printable run transcript; check current format legality if that profile is claimed. | Every runtime object can be reached through the documented setup. Setup choices and prepared inputs are explicit; runtime choice audit remains intact. |
| 9 — Wider ISA | Add multiplication, memory, division, calls and returns one verified gadget family at a time. Resolve draft inconsistencies and author-simulator bugs first. | Coverage report names every supported opcode/operand restriction; paper samples pass through board events where supported. Never claim full 2024 emulation from only the arithmetic subset. |

Run the visual replay phase once the first full board execution exists. A preliminary layout can use ISA traces earlier, but must label its evidence level. This avoids delaying useful visual feedback until every instruction is implemented.

## Verification and performance

Use three independent comparisons: readable-source evaluator versus ISA interpreter; opcode/card encoding round-trip; ISA versus decoded game state. The author's JS interpreter is an additional comparator for its reliable subset, not the authority for known-broken call/return behavior.

Adversarial fixtures should vary zero and large register values, operand aliasing, gate combinations, timestamps, target legality changes, trigger permutations, replacements, tapped/phased objects, and program wrap. Record the explored choice branches and their normal forms. Passing tests on bounded runs supports those runs; a universal forced-execution claim also needs invariant arguments.

Start with explicit execution. Later optimize by caching derived properties with correct invalidation, grouping identical visual objects, checkpointing and accelerating certified commuting event groups. Compare expanded execution and accelerated execution on bounded fixtures. A phase skipped symbolically must appear as such in the trace, with the rule that justifies it.

Measure host time and trace size separately from instruction count, MTG turns, triggered ability resolutions, active objects, counters and deck size. A simplified graphic does not reduce logical object count; a faster host simulator does not reduce Magic turns.

## Scope and stopping points

This research deliverable supplies an implementation plan, not an implemented full emulator. The first useful goal is the complete two-instruction prepared-state run and its causal visual replay. The next goal is a genuinely compiled source loop. Full paper coverage, legal physical setup and a 60-card reduction are later distinct milestones.

Exact 60-card feasibility remains unresolved and does not prevent emulator research. A larger prepared board can demonstrate semantics while the eventual physical deck is evaluated separately. Preserve automatic, deterministic execution for the initial profile; interactive and random opcodes remain outside that profile.

## Residual research review

1. [done] Implementation plan aligned with source findings: Forge feasibility first, then one execution-backend choice. Neither a Rust core nor a Java adapter is represented as selected or built.
2. [done] Evidence and outcome review complete: the negative search claim is bounded; HaltD is a draw; initial parameters are setup inputs; metadata/captions access is distinguished from watching frames. Java trigger-order limitations were checked against the actual throwing helper.
3. [done] All eighteen local documentation links resolve; documentation has no trailing whitespace. README/design navigation updated. Existing Rust source, examples and tests were not edited; no code tests were rerun for these documentation changes. Video metadata/captions remain in ignored references/videos.
4. [future execution unknown] Whether Forge runs the gate, reader and first program can only be resolved by the planned build/fixture experiment. No engine was built in this research phase.
5. [future certification] Full rules coverage, all-reachable-choice invariants, expanded board manifest, source loop, physical reachability and sixty-card reduction remain separate implementation milestones. Their absence does not prevent completing the requested research and plan.
