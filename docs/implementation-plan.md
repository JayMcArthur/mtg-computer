# End-to-end implementation

Active objective: make the Rust MTG computer end to end, with the readable program, conversion layers, actual MTG execution and visual replay. The earlier research plan remains evidence, not a completed implementation.

## Execution plan

Only one item is in progress. Each completion needs the evidence in the final column.

| State | Work | Acceptance evidence |
| --- | --- | --- |
| done | Build pinned Forge and test the edited Crusade/Life and Limb/Blood Sun gate with phasing. | Two fixtures pass, including wildcard attachment and actual addition; engine-experiment.md records the bounded Forge decision. |
| done | Freeze rules/card text and expand the complete prepared Add/HaltD board manifest. | Rust exports the board; 44 engine scripts and current rules have source/hash provenance; the complete import passes. Official Oracle audit and legal setup are still separate requirements. |
| done | Fix the complete-board Terrain/Forest interaction and execute the prepared Forge turn loop with audited decisions. | All Add X/Y/Z reads, queue replacement, indirect wildcard/Scythe/Rider phasing and Bob addition 2→5 are recorded. Halt stopped when Forest remained a green creature despite Terrain; isolated Terrain regression passes. Complete environment regression now passes with the declared Frog-copy source Rider repair. The audited engine now records Add2→5 and a real Celestial draw with all12 counters matching. The standalone reactor build succeeds. Rust caught null-field loss in Gson; its regression passes and the corrected complete Rust command succeeds. |
| done | Record versioned engine events and board checkpoints with source-bearing manifest. | Actual events carry engine causes during resolution; keyword-counter import regression fixed; header records rules, engine and manifest hash. |
| done | Drive prepared Add/Halt manifests from readable inputs and instructions. | Arbitrary Add sequences plus final Halt generate cyclic program cards and actual initial counters; unverified gadgets and engine overflow bounds reject. |
| done | Execute all three symbol-reader combats and library cycling. | Six-card Add/HaltD sequence, matching gates and object identity verified. |
| done | Execute Add/HaltD and compare decoded game state against the independent ISA. | Actual counters2+3=5, source3 and Celestial draw are recorded; exact manifest validation passes. |
| done | Build local linked source/program/battlefield/stack visual replay. | Offline HTML generation and browser checks pass for the partial engine trace, counter decoding, full-board inspection and checkpoint navigation. The full159-checkpoint engine trace now passes causal/stepping/playback/browser checks. |
| in progress | Execute NumBuild and Set/Zero from initially zero registers. | Sparse live-recipe inventory, late keyword effects, unique blockers and synchronized inactive-register boost fixture pass. First unrepaired NumBuild2 and Set2 ran through Forge. Revised full a=2; b=3; a+=b; halt, positive consecutive digit and Z0 checks remain. |
| pending | Execute conditional decrement, flag and branch gadgets. | Compiled sum loop, zero/taken/untaken/wrap cases; actual library movement. |
| pending | Complete remaining deterministic paper instructions and resolve recipe inconsistencies. | Per-family board/reference comparison and explicit opcode coverage. |
| pending | Export physical inventory, setup and printable transcript. | Reachability documented separately from prepared-state execution; constraints and legal profile explicit. |
| done | Produce a self-contained guided visual demonstration from the verified trace. | demos/add-halt.html embeds the completed trace;159-checkpoint browser validation passes. |
| pending | Verify, audit authored implementation and synchronize documentation. | Focused integration checks, actual audit measurements or named unavailable analyzers. |
| pending | Fresh residual sweep and follow-on plan. | Every remaining meaningful gap gets an explicit item and is addressed before completion. |

## Rules for completion

The game backend must derive state from actual abilities and rules events. It must not apply an ISA operation and decorate its result with card names. ISA traces, illustrated recipes and board execution have distinct evidence labels. Limits stop host execution without becoming a Magic outcome. Prepared-state import does not establish legal setup reachability. Choice selection alone does not prove forced execution. The direct controller rejects ambiguous/unsupported decisions and records bounded legal options; its complete-run audit is still pending.

## Current evidence

The existing compiler and ISA interpreter are a baseline, not the board backend. Java 21 and Maven 3.9.12 are available. Forge revision `5222f05e569da8e33c85858e6fe1698901548d64` builds and passes the shared-gate/attachment and arithmetic experiments. Continue with this one bounded backend. Full construction suitability is still being tested. Current rules and explicit creature-type mapping are frozen under data/.
