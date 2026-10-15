# MTG computer

A Rust starting point for compiling readable natural-number programs to the instruction set in Yin and Churchill's 2024 Magic microcontroller.

The compiler preserves source lines, emits the twelve-basic-land encoding, and runs an arbitrary-precision instruction simulator. A bounded Forge integration now tests actual MTG gates, attachments, blocker selection, arithmetic and the draw ability, and imports a Rust-generated prepared board. **The complete game-loop runner and visual debugger are being implemented.** Prepared-state import does not establish legal setup reachability or forced execution.

## Papers and findings

- [2024 published paper](papers/2024-mtg-programming-language.pdf), 19 pages.
- [2024 full construction](papers/2024-mtg-microcontroller-full.pdf), 37 pages, with instruction gadgets.
- [2019 original paper](papers/2019-mtg-turing-complete.pdf), 8 pages.
- [Research and card comparison](docs/research.md).
- [Repository inventory](docs/repositories.md).
- [Current project plan](http://localhost:8313/yoss/Projects/MTGComputer/Plan).
- [Active execution task and verification](http://localhost:8313/yoss/Projects/MTGComputer/Tasks/ExecutionResume20261012).
- [Prepared construction inventory](docs/construction-inventory.md).
- [Executed engine experiments](docs/engine-experiment.md).
- [Physical preparation research and declared repairs](docs/legal-setup.md).
- [Constant-building gadgets and source discrepancies](docs/constant-gadgets.md).

Follow-up research and planning:

- [Older videos and conversion](docs/videos-and-conversion.md).
- [Visible execution feature](http://localhost:8313/yoss/Projects/MTGComputer/Features/VisibleExecution).
- [Visual debugger specification](docs/visual-debugger.md).
- [Existing emulator landscape](docs/emulator-landscape.md).

The 2024 construction is a substantially more practical compiler target, but its published example deck has 360 cards. A strict 60-card requirement needs a separate solution. The starter explores the 2024 route without claiming that deck-size question is settled.

Open the [completed offline Magic demonstration](demos/add-halt.html) directly in a browser. It shows prepared inputs2/3, actual addition to5 and the real HaltD draw.

## Try it

```bash
cd /home/jay/dev/Rust/mtg-computer
cargo run -- compile examples/cube.mtg
cargo run -- run examples/cube.mtg
cargo run -- run examples/sum.mtg
cargo run -- trace examples/sum.mtg > /tmp/mtg-sum.jsonl
cargo run --quiet -- fixture > data/construction/add-halt-prepared.json
bash tools/forge/run-gate.sh
cargo run -- replay target/forge/add-halt.jsonl target/forge/add-halt.html
```

Cube finishes with `cube=1000`; sum finishes with `total=55`. `compile` displays source → numeric instructions → actual program card names. `trace` emits a JSON object per instruction, including source line, addresses, land names, every register before/after, flag and halt state. Large values are decimal strings, so JSON consumers cannot lose integer precision.

`fixture` exports the prepared Add/Halt board with r1=2 and r2=3; these inputs are established during preparation. The Forge checks require Java 21 and Maven and download the pinned engine on first use. `fixture SOURCE` accepts inputs, addition, assignments and final halt. Assignments now export the NumBuild/Set candidate profile; its complete run is under verification. Loops and other unverified families still reject. The new `board SOURCE OUTPUT_DIR [MAX_BOUNDARIES]` command wires compilation, Forge recording and offline replay, with a verified standalone build path; the complete Add/Halt command verifies a real draw and all12 counters. See [frozen inputs](data/README.md) for rules, card scripts, type port and timestamp choices.

```text
count = 10
total = 0
loop:
if count == 0 goto done
total += count
count -= 1
goto loop
done:
halt
```

## Language and semantics

Use one statement per line, with whitespace between tokens. `#` starts a comment. Statements supported:

| Source | Meaning |
| --- | --- |
| `input name = NUMBER` | Declare a prepared register input; emits no runtime instruction. |
| `name = NUMBER` | Set a variable at runtime to a nonnegative decimal integer. |
| `name = other` | Copy a variable. Self-assignment has no effect. |
| `name += other` | Add into a variable. |
| `name *= other` | Multiply into a variable, including squaring. |
| `name -= 1` | Decrement if nonzero; otherwise leave at zero and set the failure flag. |
| `label:` | Name the following instruction. |
| `goto label` | Jump to the label. |
| `if name == 0 goto label` | Jump when the variable is zero. |
| `halt` | Execute HaltD: the represented Magic game ends in a draw. |

The compiler assigns eleven named variables to r1–r11 in declaration order. r0 is scratch space for constants and branch distances. Registers initially contain zero except for explicit prepared inputs. Inputs and assignments introduce their destinations; other references must refer to a previously declared variable. A branch can skip an assignment's execution, leaving its allocated register at its initial value. There is no interactive input, randomness, negative arithmetic, memory spilling, division, or function-call syntax in this initial subset.

The program is cyclic, matching the paper: reaching the end without halt wraps to the beginning. The default execution budget is 10,000 instructions; pass a third argument to `run` or `trace` to change it. Exhausting this budget returns a failure status and partial trace, **not** a Magic halt or a proof of nontermination. The board command defaults to10,000 engine boundaries; this is a host limit, not an instruction count or game result. Program card counts exclude the supporting construction and setup cards.

Every branch builds its distance in r0, allowing arbitrary program lengths. This simple lowering costs extra instructions; small immediate-distance branch optimization can follow once full trace and rules checks exist.

## Verification

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo build
node tools/check-reference.mjs
```

The Rust tests check the published cube opcodes, summation across inputs, large integers, aliasing, branch relocation, cyclic execution, decrement failure, source mappings and diagnostics. The optional Node check compares all instruction snapshots for three examples with the authors' downloaded interpreter. It needs `references/MTGProgSimulatorText.js`, currently downloaded locally; this third-party code is excluded from Git. Its calls and returns have known defects, so agreement on this subset is supporting evidence, not a general correctness proof.

Original PDF sources: [2019](https://arxiv.org/pdf/1904.09828), [2024 published](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FUN.2024.31), [2024 full](https://www.toothycat.net/~hologram/Magic/Magic_Microcontroller_full.pdf). The PDFs retain the authors' notices; the published 2024 work is CC BY 4.0. Reference checkouts are for inspection; no upstream code was copied into the Rust implementation.
