# MTG computer investigation and Rust starting point

Requested outcome: assess whether newer cards/constructions improve on the forced MTG computer; choose a practical compilation route; make the layers from readable source to MTG observable.

## Investigation plan

1. [done] Locate local MTG work and authenticated GitHub repositories. Found JayMcArthur/MTG-to-computer and JayMcArthur/mtg-turing-machine; cloned under references without modifying upstream sources.
2. [done] Download the 2019 paper, 2024 published paper, and 2024 full construction under papers.
3. [done] Read the old compiler pipeline and execute its existing tests: 21 passed.
4. [done] Compare original restrictions with the 2024 architecture, inspect newer-card candidates against primary sources, and distinguish demonstrated improvements from hypotheses in research.md.
5. [decision open] Whether 60 cards is a hard limit: user preference requested. The comparison and starter remain explicitly conditional; no reduced deck has been claimed.
6. [done] Define the Rust architecture, trace contract, independent execution checks, and what would constitute actual MTG execution in design.md.
7. [done] Build a small runnable Rust compiler/ISA simulator that emits real 2024 land-card encodings and preserves source mappings. This is an executable starting point, not a full MTG rules engine or a proof of setup legality.
8. [done] Compare the compiler output with the published cube sample and authors' simulator; check loops, constants, aliases and source diagnostics. Seven Rust tests pass; all snapshots of the three examples agree with the author's ISA engine (6, 78 and 21 instructions).
9. [done] Re-scan code, docs and artifacts for unsupported claims and unresolved work. The fresh residual plan below records the findings.

No remote repository will be created or published as part of this investigation. PDFs, reference checkouts, research notes and Rust source stay local.

## Residual plan

1. [done] Correct the README's historical PDF page count: pdfinfo establishes 8 pages, not 10. Published 2024 is 19 pages; full manuscript is 37. Downloaded titles verified; all seven local documentation links resolve.
2. [done] Review the compiler's relocation and special semantics against tests and the author's ISA interpreter; zero-distance jumps, long jumps, self-copy and arbitrary-precision constants pass. README and design explicitly distinguish ISA checks from rules certification.
3. [done] Final cargo fmt --check, cargo clippy --all-targets -- -D warnings, cargo test, cargo build and Node reference comparison pass. New text artifacts have no trailing whitespace. References are ignored. A deliberate two-instruction execution limit returned a failure and partial output rather than claiming a halt.
4. [documented future scope] Current Oracle snapshot, full rules-event backend, all-choice confluence, physical deck/setup and visual debugger are subsequent engineering milestones in design.md. They are not implemented or certified in this research-and-starter deliverable.
5. [documented research limit] New-card review is targeted through the cutoff, not exhaustive. No post-2024 complete replacement was established. Investigating every eligible card and proving a negative result remain outside this review.
