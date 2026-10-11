# Forge gate experiment

The pinned Forge source builds here with Java 21 and Maven 3.9.12. The first two executed fixtures pass:

- Edited Crusade → Life and Limb → Blood Sun removes the gadget's combat trigger. Resolving Dream Fighter's phase ability restores it. Phasing preserves the gate's object ID, battlefield zone and timestamp.
- Bludgeon Brawl makes the artifact wildcard Crusade an Equipment. It phases indirectly with Dream Fighter and returns still attached. Both suppression gates stop applying while phased out.
- An active Joraga Warcaller supplies three power to a modified 0/2 Halana and Alena. The inactive destination's Warcaller ability is suppressed by the actual gate chain. Resolving the actual counter ability changes destination counters from two to five and leaves the source at three.

All twelve basic-land symbols pass: actual Strata Scythe imprints and Tetsuko leave exactly the matching 3/2 gate blockable by Dream Fighter. Celestial Convergence's counter removal and real draw also pass. A focused lasting-artifact-before-Brawl fixture passes.

The full Rust-generated prepared board imports into Forge and derives suppressed Add/Halt gadgets, valid attachments, a noncreature program land and the initial register properties. The actual turn loop now executes the first X=Plains replacement and read: Dream Fighter blocks the matching gate, phases it and its Scythe out, and indirectly phases the attached wildcard out. Bloodthirster's two actual damage triggers create the additional combats. Y/Z and the complete Add/Halt outcome are still being checked.

The turn loop caught an importer bug that the isolated reader fixture (using intrinsic Reach) missed: `GameEntity.setCounters(type, count)` changes counts but creates no keyword effect. Import through `Card.setCounters(Multiset)` creates the actual Reach counter ability. A full-board unique-blocker regression protects this boundary. The trace observer also now checks stack emptiness before querying the resolving ability during transient stack-removal events.

Recorded engine JSONL has a source-bearing prepared manifest, rules/engine/hash provenance, resolution parent IDs and board checkpoints with effective types, power/toughness, counters, keyword abilities, control, zone, timestamp, tapping and phasing. The offline viewer packages these facts without running rules itself. The full159-checkpoint Add/Halt engine trace passes the browser checks for decoding, board inspection, phase stepping, playback and causal links. Incomplete recordings remain labeled explicitly.

The full import exposed two preparation details: Bludgeon Brawl must follow the lasting artifact changes for Forge to retain Equipment, and effective inactive register power before the reader opens is counters minus one because the Z gates themselves lose their separate power boost. Register decoding uses actual counters. Stormtide is assigned to Bob to keep its forced attack harmless against Alice's Island. data/README.md records these explicit fixture choices.

Reproduce with `bash tools/forge/run-gate.sh`. Authored fixtures are in `tools/forge/MtgGateTest.java`; the downloaded engine stays in ignored `references/forge`. The engine revision is `5222f05e569da8e33c85858e6fe1698901548d64`.

## Architecture decision

Use Forge for the bounded game backend, with Rust owning compilation, manifest generation, trace contracts and independent comparisons. The executed shared-gate evidence justifies continuing this integration. It does not establish complete construction support. Numeric bounds and the strict runtime-choice controller remain required work. The production backend will reject unsupported bounds rather than overflow Java counters.

The isolated fixtures directly resolve existing abilities; the new complete-board experiment executes turn progression and trigger scheduling. Neither executes legal setup spells or proves that every meaningful choice is forced. These boundaries remain visible.

## Complete Add trace and second-instruction failure

The first complete turn-loop experiment read `[0,1,2]` through three actual combats. The matching X0/Y1/Z2 gates phased out with their Scythes; Y1 also indirectly phased its cross-controller target Rider. On Bob turn2, the fighter/wildcard gates returned while Alice’s numeric gates remained out. The actual Halana trigger added three +1/+1 counters to r1 (2→5), leaving r2 at3. This is actual Forge addition, not an ISA mutation.

The same run did **not** finish HaltD. On Alice turn3, Forest entered as the next program land; after X4 was read it still had Forest/Saproling/creature types and green protection, so the next Vaevictis trigger had no legal Alice target. The paper’s Terrain Island→Island overwrite should prevent this. An isolated Stormtide/Terrain/Life and Limb/suppressed Terrain test passes; the complete-board failure was traced to the ordinary source Rider: Human→Saproling also rewrote its own body, creating an intrinsic Saproling before the reader gates and suppressing its own protection. The declared repair uses a Croaking Counterpart 1/1 Green Frog copy before that text edit. Both complete-board Forest and keyword-counter fixtures now pass; Forest remains a noncreature Island. No Forge core patch was required. The complete audited run is being repeated. The recorded result is `failed`, not a draw.

The schema2 importer now represents early keyword-counter timestamps, copied Green reader creatures, White no-mana Vehicle/Crew3 target Riders and token Scythes. A two-boundary import check passes and ends deliberately at `host-limit`; it is not a game-outcome test. Forge counter identifiers are case sensitive: `Double Strike` creates the actual keyword counter; `Double strike` is a custom counter and is rejected for timestamp import.

## Audited-controller counter callback (2026-10-11)

The repaired full board reached a real Celestial Convergence draw on Bob turn4, but the independently predicted r1=5 check failed: the destination retained2 counters. A small actual-engine regression reproduced this without the reader. Forge's CountersPutEffect asks chooseCounterType even when only P1P1 is legal. The strict controller threw an unsupported callback exception; CountersPutEffect catches all exceptions and silently returns. The ability consequently appeared resolved without adding counters.

The controller now permits exactly one counter type and records that mandatory decision. Every rejected or unsupported controller decision also persists in the trace observer; the next checkpoint fails even if an engine effect catches the exception. The regression tests confirm actual3-counter addition2→5 and a rejected two-counter-type choice surviving Forge's catch. All19 focused gate/recording tests pass. The complete engine run reached a real draw with r1=5, r2=3 and all12 counters matching. The Rust command then caught Gson omitting null manifest fields; serializeNulls and an exact-manifest regression fix that recording bug. The complete command rerun is pending.

## Constants candidate evidence (2026-10-11)

An unrepaired sparse constant game executed its first NumBuild from zero to r0=2 and its first Set from r1=0 to2 through the actual turn loop. That candidate was stopped after Bob turn4 to rerun the revised profile; its trace has no Magic outcome and is not a completed program.

Its Bob-untap checkpoints confirm the inactive-r0 power defect described in constant-gadgets.md. The revised profile adds a zero-mana-value Warcaller Vehicle with one +1/+1 counter and Elf→Saproling text editing, attached to the Y Fighter. A complete-board fixture proves the unique NumBuild blockers, late register keywords, selected Myr2/3, indirect helper phasing and value2 power across Bob's Y-Fighter return. It uses direct phase/counter test inputs, so it does not establish a completed NumBuild instruction or legal preparation. The revised ordinary-assignment program starts from zero and is currently running.
