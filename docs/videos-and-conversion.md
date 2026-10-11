# What the older videos demonstrate

Reviewed 2026-10-10 using primary YouTube metadata, downloaded English automatic captions, repository source and the published papers. No video frames were inspected. Captions may mistranscribe card and machine names; technical conclusions below were checked against source code. Metadata and caption files are local under ignored references/videos, not redistributed as repository content.

## The links

| Video | Primary identification | Why it matters here |
| --- | --- | --- |
| [I Built a COMPUTER in Magic: The Gathering](https://www.youtube.com/watch?v=pdmODVYPDLA) | Because Science, 2019-10-31; 22:54. | Explains a physical board, setup, and a cycle of the original construction. The useful visual idea is to show interactions and the encoded tape together. This demonstration is separate from establishing a general source compiler. |
| [How to run any program in a Magic: The Gathering Turing Machine](https://www.youtube.com/watch?v=YzXoFldEux4) | Cerno_b, 2019-12-01; 22:10. Its description links [Cerno-b/mtg-turing-machine](https://github.com/Cerno-b/mtg-turing-machine), the upstream of Jay's identical fork. | This is the compiler video the user recalled. It explains conversion expansion and works back from encoded output to the original answer. |

Useful chapters in the compiler walkthrough: [conversion, 7:37](https://www.youtube.com/watch?v=YzXoFldEux4&t=457s), [unary adder, 13:31](https://www.youtube.com/watch?v=YzXoFldEux4&t=811s), [compilation, 14:19](https://www.youtube.com/watch?v=YzXoFldEux4&t=859s), and [simulation, 18:06](https://www.youtube.com/watch?v=YzXoFldEux4&t=1086s). The narration reports that its ordinary compiled 2+3 example did not finish after 72 hours. It then switches to a manually binary 1+1 example and reports roughly two hours. Those are the author's historical measurements, not new benchmarks or forecasts for Rust.

The 2024 paper cites Jan Biel's compiler video, connecting the newer work directly to this earlier effort. [Published references](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FUN.2024.31).

## What the old implementation actually emulates

The Python pipeline is real and includes inverse decoding:

`TM → binary TM → 2-tag system → Rogozhin UTM(2,18) tape → specialized MTG simulation → decoded result`

However, its MTG class encodes consequences of known interactions. `step_cleansing_beam` directly increases matching tape tokens' power/toughness; `step_soul_snuffers` explicitly simplifies counter updates to the tape tokens; `trigger_controllers` folds token creation, control changes and boundary extension into a routine. Setup is not executed as game actions. This is a useful specialized simulator, not independently complete coverage of all relevant legal choices or a general rules engine. [Python implementation](https://github.com/Cerno-b/mtg-turing-machine/blob/master/mtg_turing_machine/classes/mtg_turing_machine.py).

The Java visual implementation has a stack and card effect model. In Game.java, `toNext` groups triggers by controller and delegates ordering to `sortByPlayer`; that helper throws when a group contains more than one entry. It does not explore alternative legal orders. Effects and state-based actions are similarly specialized. A stack and a graphical board do not by themselves certify the embedding. [Java Game.java](https://github.com/Snowman-s/Mtg-Turing-Machine/blob/master/src/main/java/snowesamosc/mtgturing/Game.java).

## How to improve the project

These are proposed engineering decisions, not claims made by the videos.

1. **Keep the conversion explanation.** Show source, intermediate representation, target instructions/cards and reverse decoding with stable provenance. The 2024 target has fewer computational layers, but each still needs an inspectable mapping.
2. **Change the compiler target.** The 2024 instruction set directly expresses arithmetic, control flow and memory. This removes the old interpreter cascade from the compiled program. Changing Python to Rust alone would not do that.
3. **Make game execution independent.** The board backend should execute abilities and game events, then decode counters/tokens. Compare its result with a separate ISA interpreter; do not let the interpreter dictate board mutations.
4. **Explain causality visually.** Clicking a changed counter should show the resolved ability, its source's computed power, its legal target, and the source instruction that activated the gadget.
5. **Measure costs at each layer.** Track instruction and event counts, turns per player, setup/program deck cards, active permanents and maximum counters. Compare the same input, output and halt convention across machines. Host time is a separate metric.
6. **Optimize after equivalence checks.** Constant folding and small immediate branches can reduce real instruction count. Checkpoints and certified phase skipping can reduce host work. Label skipped groups in traces; do not present omitted events as individually simulated ones.

The first deliverable should be a short complete board execution, followed by a source-compiled loop. A claim to have implemented every instruction of the 2024 paper should wait for corresponding gadget and full-board coverage.
