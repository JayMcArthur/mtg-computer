# MTG computer research

Research date: 2026-10-10. Scope: the original forced two-player construction, the 2024 microcontroller, and a targeted review of newer physical cards. This is a source-backed architecture recommendation, not a certification of every card interaction under today's rules or an exhaustive search of every release.

## Decision

Use the **2024 microcontroller as the preferred compiler target if larger tournament-legal decks are acceptable**. Keep the 2019/2020 UTM as a historical comparison or a target for a strict 60-card constraint. A later paper already addresses the main practical problem: programming MTG without the enormous expansion through tag systems. Post-2024 cards reviewed here do not establish a further complete replacement. Candidate improvements need independent gadget proofs.

### Original baseline

Churchill, Biderman, and Herrick encode Rogozhin's two-state, eighteen-symbol UTM. Creature types encode symbols; green/white tokens and toughness encode tape position. Death triggers perform transitions. Phasing selects machine state. The four recurring spells are Infest, Cleansing Beam, Coalition Victory, and Soul Snuffers. A computational step takes three or four Alice turns, interleaved with Bob turns. Their appendix supplies a 60-card Legacy deck. Execution begins from a deliberately prepared state; its setup is not forced. Wild Evocation's random reveal has a one-card hand, so its outcome is deterministic. The result does not depend on unknown information, even though libraries and hands remain Magic zones. [Original paper, sections III–IV and appendix](https://arxiv.org/pdf/1904.09828).

### Established improvement

Yin and Churchill's FUN 2024 construction provides twelve unbounded natural-number registers, unbounded sparse memory, a Boolean flag, arithmetic, branching, and calls. Each instruction is three symbols. Usually one Alice/Bob turn cycle executes an instruction. Twelve basic-land names encode symbols; three Alice combats read them. Registers use Joraga Warcaller token counters. A memory cell is a Mouse with base size equal to its value and counters equal to its address; absence means zero. Their sample Legacy deck has **360 cards**, rather than 60. Optional input and randomness instructions must be excluded from a fully forced deterministic profile. This is an improvement in programming and execution, not an increase beyond Turing completeness. [Published paper, sections 2–4 and 6, appendix C](https://drops.dagstuhl.de/storage/00lipics/lipics-vol291-fun2024/LIPIcs.FUN.2024.31/LIPIcs.FUN.2024.31.pdf).

## Primary sources and versions

| Source | Role |
| --- | --- |
| [2019 preprint](https://arxiv.org/pdf/1904.09828) | Original construction and 60-card baseline. |
| [2024 published paper](https://drops.dagstuhl.de/storage/00lipics/lipics-vol291-fun2024/LIPIcs.FUN.2024.31/LIPIcs.FUN.2024.31.pdf) | Canonical published ISA and architecture. |
| [2024 full manuscript](https://www.toothycat.net/~hologram/Magic/Magic_Microcontroller_full.pdf) | Detailed instruction gadgets in appendix B; contains draft inconsistencies. |
| [Author's live explanation](https://cyh31.neocities.org/amepscimtg/explanation) | Concrete permanents and updated gadget descriptions. |
| [Author's simulator](https://www.toothycat.net/~hologram/Magic/MTGProgSimulatorText.html), [JavaScript](https://www.toothycat.net/~hologram/Magic/MTGProgSimulatorText.js) | Instruction semantics and examples, not a comprehensive-rules engine. |
| [Official rules](https://magic.wizards.com/en/rules), [2026-09-25 text](https://media.wizards.com/2026/downloads/MagicCompRules%2020260925.txt) | Rules snapshot found on the official rules page during this review. |
| [Tournament rules](https://media.wizards.com/ContentResources/WPN/MTG_MTR_2026_Feb27_EN.pdf) | Official WPN download linked when reviewed; February 27, 2026 edition. |

Version findings from reading the actual sources:

- The full manuscript's Halve gadget labels Catacomb Dragon with `Y1`, whereas its instruction is `4 2 Z`; the live explanation uses `Y2`. This is a specific source inconsistency, not grounds to reject the entire construction.
- The full manuscript's division prose mixes Hornet Nest/Phantom Steed and Polyraptor/Gruul Ragebeast descriptions. Defer division gadget certification until the live construction and Oracle text are reconciled. [Full manuscript, B.7 and B.20](https://www.toothycat.net/~hologram/Magic/Magic_Microcontroller_full.pdf), [live explanation](https://cyh31.neocities.org/amepscimtg/explanation).
- The JavaScript uses expressions such as `3 * this.registers[0]` in call implementations, although registers are BigInt. JavaScript rejects mixing Number and BigInt. Return passes a second argument to `jumpInstructions`, whose declaration accepts one argument. Both defects were reproduced by running the downloaded engine in Node: CallFwd throws TypeError; Return leaves the program counter unchanged. Use it as a comparison, not a correctness oracle. [Simulator implementation](https://www.toothycat.net/~hologram/Magic/MTGProgSimulatorText.js).

## A concrete card-level starting point

The author describes conditional execution through modified Dralnu's Crusades: selected creature types become Saprolings; Life and Limb makes them lands; Blood Sun removes their abilities. Dream Fighter phases out the selected gate. Vaevictis Asmadi, the Dire advances the land program; Wheel of Sun and Moon recycles it. Grand Melee forces available combat actions, while other restrictions constrain attackers, blockers, and targets.

For Add, Bob's Halana and Alena, Partners is modified to base 0/2 with the instruction gate, Elf, and target-selector types. The active Joraga Warcaller raises its power by the source register's value. Its combat trigger adds that many counters to the uniquely targetable destination. Add1 instead uses a base 1/1 copy. Set clears the destination through Archpriest of Iona and Shape Stealer combat with wither, then adds the source value. [Author explanation, global setup and Add/Move/Add1 gadgets](https://cyh31.neocities.org/amepscimtg/explanation).

These are actual proposed Magic actions. A simulator that directly changes a register counter and prints these names has illustrated the construction; it has not verified the trigger, combat, layer, and state-based-action derivation.

## Newer physical cards reviewed

These dates describe worldwide product release; prerelease and format-legality dates can differ.

| Card | Release evidence | Finding |
| --- | --- | --- |
| Bloodthirster | [Warhammer decks, October 7, 2022](https://magic.wizards.com/en/news/announcements/warhammer-40000-commander-decklists) | Already used by the 2024 backend to obtain extra combats. It is a post-original ingredient, not a newly proposed post-2024 optimization. |
| Taii Wakeen, Perfect Shot | [Thunder Junction, April 19, 2024](https://magic.wizards.com/en/news/announcements/outlaws-of-thunder-junction-arrives-april-19-2024), [official text/rulings](https://magic.wizards.com/en/news/feature/outlaws-of-thunder-junction-release-notes) | Already in the published deck and live Halve gadget. Exact noncombat damage equal to toughness triggers a draw. Its optional activated ability needs to remain disabled. |
| Snow-Covered Wastes | [MH3 release June 14, 2024](https://magic.wizards.com/en/news/feature/modern-horizons-3-mechanics), [June 7–13 prerelease](https://magic.wizards.com/en/news/announcements/play-on-the-modern-horizons), [official rules text](https://media.wizards.com/2024/downloads/MH3_Release_Notes_9d01wihakukg/EN_MTGMH3_ReleaseNotes_20240515.pdf) | Completes twelve distinct basic-land names without the four-copy restriction. Established usefulness in the 2024 alphabet. The draft's June 7 footnote should not be described as worldwide release. |
| Innkeeper's Talent | [Bloomburrow release August 2, 2024](https://media.wizards.com/2024/wpn/marketing_materials/blb/blb_product_guide_en.pdf), [official text](https://magic.wizards.com/en/news/feature/bloomburrow-release-notes) | Mandatory combat counter addition could implement increment with one legal target; level three doubles counters. **Candidate only:** level two adds ward, creating payment/countering interactions, and level activations create choices unless shut off. The existing backend already has Add1. No demonstrated deck or runtime reduction. |
| Ouroboroid | [Edge of Eternities launch August 1, 2025](https://company.wizards.com/en/news/find-magic-among-the-stars-in-magic-the-gathering-edge-of-eternitites), [official text/rulings](https://magic.wizards.com/en/news/feature/edge-of-eternities-release-notes) | Mandatory combat trigger adds counters equal to its power to every friendly creature. **Candidate only:** may support arithmetic in an isolated architecture; in the existing backend it also changes other registers and machinery. It is not a drop-in replacement for targeted addition. |
| Sphinx's Approach | [Reality Fracture release October 2, 2026](https://www.magic.wizards.com/en/news/announcements/reality-fracture-delays-in-select-regions), [official text/rulings](https://www.magic.wizards.com/en/news/feature/reality-fracture-release-notes) | Any-number deck clause gives another repeatable card name, but it is an instant, so Vaevictis cannot use it as a land-program permanent. Its optional search/shuffle is unsuitable for the proposed forced runtime. No demonstrated representation gain. |

This targeted review reached official material for the latest release before the cutoff. It does **not** imply all 2025–2026 cards were exhaustively screened. A defensible negative result here is “no further complete improvement established in this review,” not “no better construction exists.” Official release notes also warn that later rules updates may change rulings; do not call old release notes a complete current Oracle snapshot.

## Restrictions and unresolved rules obligations

The official September 25, 2026 rules retain choices relevant to these machines: `603.3b` lets a controller order simultaneous triggers; `616.1` selects applicable replacement/prevention effects; `118.8c` permits declining an “if able” cast with particular hidden-zone additional costs. APNAP fixes order **between players**, not all ordering within one player's triggers. `104.3a` always permits concession. `205.3m` no longer lists Cephalid, used in both historical symbol/gate vocabularies. `510.1c–d` require examining multi-creature damage assignments. [Comprehensive Rules](https://media.wizards.com/2026/downloads/MagicCompRules%2020260925.txt).

Consequent project requirements, proposed here:

1. Define forced execution as no outcome-changing in-game decisions after setup, excluding concession. Distinguish unique legal action from multiple legal actions that provably converge.
2. Enumerate legal trigger permutations and replacement choices. Never make an arbitrary stable sort and label that forced MTG execution.
3. Remap historical Cephalid to a valid unused type such as Octopus, with an explicit collision check throughout the selected construction.
4. Reject input and random instructions in the deterministic profile. Fixed input is established during setup.
5. For a Wild Evocation backend, inspect modal choices, optional costs, additional costs, targets, and mana sources. The original four recurring spells do not themselves supply the hidden-zone-additional-cost loophole; adding new spells can.
6. Preserve unbounded mathematical integers with arbitrary-precision values and sparse memory. Execution budgets are simulator limits, never hardware halts or MTG draws.

Constructed deck rules specify a minimum of sixty cards and no maximum, subject to physical shuffling requirements; basic lands are exempt from the four-copy limit. Tournament legality therefore does not imply a sixty-card implementation. Tournament loops and slow-play provisions also constrain actual events. Theoretical legal setup, a runnable casual demonstration, and a match finished within tournament time are separate achievements. [Tournament Rules, 4.4, 5.5 and 6.1](https://media.wizards.com/ContentResources/WPN/MTG_MTR_2026_Feb27_EN.pdf).

The 2024 library length grows with encoded program length. If exactly sixty cards must be preserved, do not silently use its full deck. Investigate a specialized smaller ISA/deck or keep the original fixed UTM with optimized conversion. Finite deck size does not itself bound token/counter memory.

## Recommended Rust implementation boundary

The following is a proposed engineering design, not a result established by the papers.

Build one chain with source provenance at every boundary:

`readable natural-number program → typed control-flow IR → 2024 instructions → land cards → annotated Magic events → decoded state`

Keep source spans, IR IDs, instruction addresses, card positions, permanent IDs, and rule/ability references in the trace. A viewer should select a source statement and see its generated instructions, then step into combats, triggers, phasing, counters, and zone movement. Preserve timestamps and copiable versus noncopiable modifications in board data.

Start with constants, copy/zero, addition/increment, conditional decrement, direct conditional jumps, and halt. This supports actual finite programs and loops without requiring the full instruction set. Source variables beyond the register budget can later use memory. Defer division, calls, and interactive programs until the documented inconsistencies are resolved.

Selected ISA encodings to check against both spec and tests:

| Symbols | Meaning |
| --- | --- |
| `10 Y Z` | Build `12Y+Z` in r0; consecutive builds append base-144 digits. |
| `5 Y Z` | Copy rZ into rY; equal operands zero rY. |
| `0 Y Z` | Add rZ into rY. |
| `4 1 Z` | Increment rZ. |
| `1 Z Z` | Decrement rZ if nonzero; flag reports failure. |
| `3 0/1 Z` | Forward/backward jump; Z=0 takes distance from r0. |
| `3 2/3 Z` | Forward/backward jump if flag false. |
| `3 4/5 Z` | Forward/backward jump if flag true. |
| `4 0 0/1/2` | Draw/Alice win/Bob win. |

The source interpreter advances past the current instruction before applying jumps; comparisons can start OR-combining flag state. Do not substitute conventional branch offsets or flag semantics. [Author's instruction engine](https://www.toothycat.net/~hologram/Magic/MTGProgSimulatorText.js).

Maintain explicit verification stages:

- **Language checked:** source evaluation and instruction evaluation agree.
- **Encoding checked:** opcode/card conversion round-trips and reports program card counts.
- **Construction illustrated:** named card events are generated from the cited gadget recipe.
- **Rules verified for a selected gadget:** a separate state transition model checks legality, every legal choice, layers, state-based actions, and decoded-state agreement.
- **Physical demonstration prepared:** complete deck list, setup recipe, board inventory, modifications, and an executable human trace exist.

Only the final two justify claiming a program ran in MTG rules or is ready for physical play. A JavaScript or Rust ISA interpreter does not independently certify the embedding.

For each next gadget, test the boundary cases that expose semantics: zero source/destination, same operands, failed decrement, large constants, a conditional jump taken and untaken, cyclic program wrap, and repeated NumBuild. Compare independent evaluators rather than tests that repeat an implementation's own arithmetic. Add an actual full-board choice audit before advertising “all moves forced.”
