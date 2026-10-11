# Existing work and online implementations

Inspected on 2026-10-10 using local filesystem searches and the authenticated GitHub account JayMcArthur. No existing MTG checkout was found in /home/jay/dev or the searched home/Desktop/Documents paths. Relevant remote repositories were cloned into this project's ignored references directory for comparison. These searches were targeted, not a claim that every mounted filesystem was indexed.

| Repository | Snapshot and role |
| --- | --- |
| [JayMcArthur/MTG-to-computer](https://github.com/JayMcArthur/MTG-to-computer) | `ca5cc02`; original work, last push 2021-04-26. Two Python scripts explore tag systems and Rogozhin UTM(2,18). The UTM script has a hardcoded historical Windows output path, a 10,000-step cap, and uses the same message for cap exhaustion and a real halt. It records tape changes but lacks a readable-source compiler or unified provenance trace. |
| [JayMcArthur/mtg-turing-machine](https://github.com/JayMcArthur/mtg-turing-machine) | `2772c80`; fork of Cerno-b, identical to upstream master at inspection. Existing pipeline: arbitrary TM → binary TM → 2-tag → UTM(2,18) tape → specialized MTG simulation. The README explicitly warns about enormous simulation costs. Its existing 21 Python tests passed locally. |
| [Cerno-b/mtg-turing-machine](https://github.com/Cerno-b/mtg-turing-machine) | The upstream implementation behind the fork. Useful as a historical differential baseline and reference for the original construction. Fork and upstream are not independent correctness evidence. |
| [Snowman-s/Mtg-Turing-Machine](https://github.com/Snowman-s/Mtg-Turing-Machine) | `5f74103`; Java, last push 2022-06-10; inspected Game.java and Main.java, not built. Processing graphical board and log window, with an original-construction card model, stack, phasing, effects, and state-based actions. Useful UI and event-model reference; current completeness/rules correctness was not established. |
| [2024 author simulator](https://www.toothycat.net/~hologram/Magic/MTGProgSimulatorText.html) | Browser ISA interpreter and sample programs. Downloaded HTML/JS under references; separate from full Magic rules verification. |
| [2024 author explanation](https://cyh31.neocities.org/amepscimtg/explanation) | Detailed gadget recipes with interactive terminology/card references. Use alongside the published ISA and versioned Oracle/rules data. |

The GitHub account also contains Computer-From-Scratch, described as a NAND-gate computer; the two MTG repositories are the relevant starting points for this task. Existing repos were not edited or pushed. The new Rust repository has no remote and no commits yet.
