# Prepared construction inventory: Add/HaltD, then source programs

Research date: 2026-10-10. This is an extracted preparation specification, **not an executed legal setup or validated rules-engine fixture**. Sources: [full 2024 manuscript](https://www.toothycat.net/~hologram/Magic/Magic_Microcontroller_full.pdf), especially §§4.1–4.11, B.1–B.5, B.16, B.19, B.21–B.28, B.33–B.34 and Appendix C; [live construction](https://cyh31.neocities.org/amepscimtg/explanation), [its notation implementation](https://cyh31.neocities.org/amepscimtg/explanation.js), [its card-text dataset](https://cyh31.neocities.org/amepscimtg/cards.json); [published paper](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.FUN.2024.31); [author ISA interpreter](https://www.toothycat.net/~hologram/Magic/MTGProgSimulatorText.js). Freeze downloaded bytes with the fixture. The live page and manuscript disagree in places listed below; neither should silently overwrite the other.

## Conventions and deliberate fixture choices

A/B mean Alice/Bob control; “both” means one each. Copies are separate Magic objects, not shared references. Copiable base values, text changes, lasting noncopiable changes, counters and temporary effects must remain separate. In live notation, `ec` ends the copiable portion; conditional prefixes such as X0 say to add a type but do not by themselves specify its setup provenance. `ad-cf` means an Astral Dragon token copy made a creature with flying. The live explanation's controller-less globals do not fix ownership: the proposed prepared fixture assigns those globals to Alice unless the table says otherwise. That is a declared completion of omitted metadata, not a quotation from the paper.

All support permanents are green unless a different color is specified. Use Prismatic Lace lasting color effects where necessary. All runtime permanents begin phased in, with their attachments, under the listed controllers since before the next Alice turn; no summoning sickness. Begin just before Alice's untap step. Alice/Bob have life 1, empty hands, no mana, and no additional playable objects. Exact legal reachability of these preparation choices remains a separate setup milestone.

Timestamp bands: **T1** default continuous effects, with Stormtide before Illusionary Terrain; **T2** Dralnu effects and Blood Sun; **T3** Life and Limb and protection/shroud effects; **T4** register red color effects. These are ordered effect timestamps, not just a field on the printed card. Preserve dependencies as well as timestamps. A source does not order every effect inside each band; test that omitted internal order is immaterial rather than assuming it.

### Type aliases

| Slot | X | Y | Z |
| --- | --- | --- | --- |
| 0 | Aetherborn | Antelope | Aurochs |
| 1 | Beeble | Basilisk | Brushwagg |
| 2 | **Capybara** | Camel | Camarid |
| 3 | Drake | Dauthi | Druid |
| 4 | Eldrazi | Efreet | Elephant |
| 5 | Faerie | Fox | Ferret |
| 6 | Gremlin | Gnome | Graveborn |
| 7 | Homarid | Hippo | Hamster |
| 8 | Illusion | Inkling | Imp |
| 9 | Juggernaut | Jellyfish | Jackal |
| 10 | Kavu | Kor | Kithkin |
| 11 | Lhurgoyf | Lammasu | Licid |
| wildcard (*) | Monkey | Metathran | Masticore |

The manuscript uses Cephalid for X2. Current [CR 205.3m](https://media.wizards.com/2026/downloads/MagicCompRules%2020260925.txt) omits Cephalid and includes Capybara. **Capybara is the project's frozen replacement**, matching `data/construction/types.json`; it is absent from the 2024 recipe. Octopus was rejected because the Output gadget uses it. Every X2 gate and conditional object must use the replacement consistently. `tR=Reflection`, `tU=Beast`, `trY=Reflection+Yeti`, `trZ=Reflection+Zubera`. These combinations restrict damage, blocking and attachments as well as targeting.

## Add/HaltD globals

Source: full §§4.2–4.7, 4.11 and corresponding live Structure subsections. Default printed characteristics apply where the table specifies no modification.

| Quantity / controller | Permanent and preparation | Runtime purpose |
| --- | --- | --- |
| 1 A | Grand Melee | Attack/block requirements |
| 1 B | Stormtide Leviathan, T1 | Add Island to lands; flying/islandwalk attack restriction. Bob's own islandwalk attacker is harmless against Alice's Island. |
| 1 A | Life and Limb, T3 | Saproling/Forest type, color and base P/T effects |
| 1 A | Blood Sun, T2 | Remove **nonmana** abilities from lands |
| 1 A | Bludgeon Brawl | Noncreature artifacts become Equipment; permits wildcard/Riders attachments |
| 1 A | Shadow Sliver | Slivers have shadow; isolates reader combat |
| 1 A | Tetsuko Umezawa, Fugitive | Alice's low-P/T attackers cannot be blocked |
| 1 A | Wheel of Sun and Moon enchanting Alice | Program graveyard movement instead goes to library bottom |
| 1 B | Tajuru Preserver | Bob cannot sacrifice to Vaevictis |
| 1 A | Masked Gorgon: change **white→blue**, **Gorgon→Reflection**; T3 protection | Green/blue creatures protected from Reflections |
| 1 A | Second Masked Gorgon: **white→red**, **Gorgon→Beast**; T3 | Green/red creatures protected from Beasts |
| 1 A | Spectral Guardian, **untapped**, T3 shroud | Noncreature artifacts have shroud |
| 1 A | Sterling Grove, additionally artifact | Other Alice enchantments shrouded; this Grove shrouded by Guardian |
| 1 B | Sterling Grove with lasting protection from blue | Sole Bob permanent legal for Vaevictis; Bob Preserver prevents sacrifice |
| 1 A | Suppression Bonds attached to Bob's Grove | Disable its sacrifice/search activation |
| 2 both | Ivory Mask | Players cannot be targeted |
| 1 A | Illusionary Terrain, animated creature with added Z*, T1 after Stormtide; chosen types **Island→Island** | Remove other basic land subtypes from basic program land, preventing Forest animation; cumulative upkeep disabled while inactive |
| 1 A each | Root Maze; Choke | Program lands enter tapped and remain tapped |
| 1 A each | Stony Silence; Cursed Totem | Artifact/creature activated abilities disabled |
| 2 both | Recycle | Skip both draw steps |
| 1 A | Mirror Gallery | Ignore legend-rule clashes between copies |
| 2 both | Worship | Damage still dealt; life remains 1 |
| 2 both | Corrosive Mentor | Black creatures have wither; needed by expanded gadgets, harmless here |

All active creatures must continue to provide Worship's condition. No extra land named like a program symbol may be on either battlefield: Strata Scythe counts lands on **both** battlefields. The later conditional protections must remain after Blood Sun removes abilities; preserve their prescribed T3 order and layer dependencies. Do not flatten these globals to “all undesired targets illegal.”

## Three-stage reader: exact multiplicities and attachment graph

Source: full §§4.3–4.5 and live Structure/conditional mechanism. Symbols 0–11 are Plains, Island, Swamp, Mountain, Forest, Wastes, then the six snow-covered variants in that order. The post-MH3 fixture uses actual Snow-Covered Wastes, avoiding Persistent Petitioners/Infinite Reflection machinery.

For **each** stage S=X,Y,Z and **each** n=0…11:

1. Alice controls one token copy of Dralnu's Crusade, made an enchantment creature with copiable flying, **copiable base 2/1 and creature type Sliver**. Edit **Goblin→Sn**, **Zombie→Saproling** at T2. Its affected creatures gain +1/+1, become black and become Saprolings before Life and Limb's later green effect.
2. Attach one Strata Scythe to that token, with its own **distinct exiled actual basic land card** named for symbol n. This means **36 gate tokens, 36 Scythes, 36 exiled imprint cards**; imprints are not the cyclic program cards.
3. Y-stage gate creatures additionally have X*; Z-stage gate creatures additionally have Y*. The previous wildcard suppresses flying until that stage is ready.

For each stage, Bob controls a Dream Fighter with additional Sliver type and a reach counter. Add X* to the Y-stage Fighter and Y* to the Z-stage Fighter. Each retains printed 1/1 base and its phasing trigger. Alice controls one **noncreature artifact** Dralnu's Crusade edited Goblin→S*, Zombie→Saproling, attached via Bludgeon Brawl to that stage's Bob Fighter. Its original mana cost is retained: it may boost its host through Bludgeon Brawl, which does not change the block legality argument. If imposing mana value 0 instead, that is an additional proposed modification requiring provenance; the reader wildcard recipe itself does not demand it.

Alice controls three token copies of Vaevictis Asmadi, the Dire, each **copiable 1/1 Sliver+Beast+Reflection** retaining flying and its attack trigger. Add X* to stage Y, Y* to stage Z. Alice also controls one 1/1 Sliver+Beast+Reflection Bloodthirster with double strike; retain flying. Its two damage events add two later combats. It untaps itself but cannot attack Bob again that turn.

**Phasing ownership matters:** numeric Dralnu creatures are Alice-controlled and return in Alice's next untap. Bob Fighters return in Bob's next untap; their wildcard Equipment returns with them. Bob-controlled Riders attached to Alice's numeric gates return with their hosts, not merely at Bob's untap. Do not return every gate together.

### Initial six-card program queue

Prepared program: `0 1 2 / 4 0 0` (Add r1 r2; HaltD). Allocate six distinct cards: Plains, Island, Swamp, Forest, Plains, Plains. Initially the **last Plains** is Alice's tapped program permanent; her library top-to-bottom is **Plains, Island, Swamp, Forest, Plains**. Thus each instruction begins by replacing the previous instruction's Z card and reading the next X. Alice's hand/graveyard are empty. Record card identity through each zone change using CR object-identity rules.

## Registers and forced target selection

Source: full §4.8 and live Registers. Bob controls twelve token copies of Joraga Warcaller:

- Replace printed Elf/Shaman creature types with **Zn** in copiable values. Add Rabbit, indestructible and vigilance; noncopiably set base P/T to **2/2** using the source's Gigantoplasm technique. Add **two -1/-0 counters**. At T4 make the register red. Put its numeric value in +1/+1 counters. Add Rhino to r0.
- Start r1 with 2 +1/+1 counters, r2 with 3; other registers zero. Their derived power should equal the stored value both when active and inactive. Inactive base 1 plus gate +1 minus two -1/-0 equals zero before value counters; active base 2 minus two equals zero.
- Bob controls twelve Riders of Gavony whose copiable card type is **only artifact**, mana value **0**, printed Human text edited Human→Zn, ETB chosen creature type **Yeti**. Each is attached as Equipment to Alice's corresponding **Y-stage** numeric Dralnu. Each gives Bob's Zn creatures protection from Yetis; exactly the selected Y attachment phases out. Do not accidentally use X or Z hosts.
- Bob also controls one Riders of Gavony edited **Human→Saproling**, choosing **Zubera**; it protects inactive registers. This ordinary Rider retains printed Human/Knight creature types and vigilance unless otherwise modified.

The Y-artifact Rider recipe in the source uses Imposter Mech and Vizier of Many Faces for card type and mana value, then retains its chosen type through the necessary copying/setup. A prepared property list is not proof that the sequence preserved that choice; verify ETB/copy provenance before claiming physical setup.

## Add and actual halt

Source: full B.1/B.16 and live matching instructions.

| Quantity/controller | Preparation | Derived execution |
| --- | --- | --- |
| 1 B | Halana and Alena, Partners; copiable base **0/2**, retain Human/Ranger; add **X0, Elf, Reflection, Yeti** | Active after X=0; active rZ adds value through Warcaller Elf effect. At beginning of Bob combat its mandatory trigger's only legal target is rY; add its current power in +1/+1 counters and grant haste. |
| 1 B | Celestial Convergence animated creature, add **X4,Y0**, **exactly 1 omen counter** | Once X4 and Y0 are enabled, beginning of Bob upkeep removes its last counter and compares life totals; both at 1 yields a real Magic draw. No synthetic ISA halt event. |

For Celestial's otherwise unspecified animated baseline, freeze the prepared object as a **token Enchantment Creature — Eldrazi Antelope, copiable base 1/1**, with printed Convergence abilities and no retained Soldier/Myr/Human/Shapeshifter types. This is an explicit fixture completion of the manuscript's X4,Y0 requirements. Appendix C supports Urza token animation (edit Soldier→Eldrazi before token creation), followed by Glasspool Mimic text edits Shapeshifter→Antelope and Rogue→Eldrazi and copying the animated Celestial, then token copying. The temporary artifact change on the Urza target is not itself copied. This is a proposed legal preparation route, not a replayed proof: copy/ETB resolution, resetting seven initial omen counters to one, final token creation and removing intermediate objects all need setup validation. Alternatively retaining Urza's native Soldier plus separately added X4,Y0 is a declared prepared variant, not the frozen choice here.

Expected causal run: Alice combat 1 Vaevictis replaces previous Plains with X=Plains; unique Scythe boost makes X0 gate blockable; Bob Fighter phases it and X* wildcard out. Combat 2 similarly reads Island/Y1 and releases Y*; combat 3 reads Swamp/Z2, activates r2 and phases out the Y1-attached protection Rider. Bob untap returns wildcard gates, preserving selected numeric gates phased out. Bob combat Halana power=3 and target r1 adds 3 counters, decoding r1=5. Next Alice untap restores numeric gates/Riders; her three combats read Forest/Plains/Plains. Bob upkeep then resolves Celestial's actual draw condition.

Inventory is complete at the **named gadget-role level** for this prepared Add/Halt subset. Unspecified token animation subtypes, global ownership, internal timestamp ties and ETB-copy provenance are explicit remaining preparation details, not grounds for claiming full physical completion.

## Extension: flag environment

Source: full §4.10 and live flag. Before Sub1/flag/conditional-branch support, add one inert flag card to Bob's library for false (hand for true), and no other cards to those zones. Proposed fixture flag is an ordinary basic Plains; never place it on battlefield. Add Wheel of Sun and Moon enchanting Bob, Bob's Tomorrow, Azami's Familiar, and Bob's Aggressive Mining made an artifact (land play disabled; artifact activations suppressed). If using a nonland flag, instead supply named Nevermore appropriately. Bob draws with Tomorrow's replacement: with zero/one library card there is no actual library-order choice, and no empty-library draw loss. Preserve all replacement events. Add/Halt does not require these gadgets; a full-profile fixture may include them.

## Extension: Set/Zero and constants

Source: full B.2–B.3/B.19/B.21, Appendix C and live instructions.

**Set/Zero** adds Bob Halana 0/2 with X5, Elf, Reflection,Yeti; Alice Archpriest of Iona with X5,Y*,Reflection,Yeti; Alice Shape Stealer copiable 3/3, X5,Y*, flying, green+black and one -1/-0 counter. Alice needs a full party: the manuscript specifies separate 0/2 **Warrior, Rogue and Wizard tokens**, with Archpriest as Cleric. They remain green and cannot attack under Stormtide. At Alice third beginning of combat Archpriest targets rY, grants +1/+1 and flying. rY is forced to block Shape Stealer; Shape Stealer copies its base P/T until end of turn then its -1/-0 cancels the temporary power increment. Wither damage cancels the old value counters. Bob Halana adds rZ. Y=Z yields Zero by erasing before reading the addition power.

**Add1** is one Bob Halana copiably 1/1 with X4,Y1,Reflection,Zubera. Beginning Bob combat targets rZ and adds one counter. For small literals, repeated Add1 is a simpler initial backend than full NumBuild, at the cost of more real turns. That is a compiler choice, not the paper's constant-building gadget.

**NumBuild** dependency inventory:

| Controller | Additional object and changes |
| --- | --- |
| A | Excruciator X10,X*, flying/indestructible/vigilance |
| A | Dream Fighter X10, copiable 2/2, flying, blue, protection from red |
| B | Vigor X10,Beast; reach and islandwalk |
| B | Khorvath Brightflame X10, flying removed, Dragon text→Rhino |
| B | Sylvia Brightspear X10, Dragon text→Rhino |
| A | Mangara's Equity animated; X10, Nomad, 0/1, reach, green+black; choose red; damage-to-white text→green |
| A | 12 Darksteel Myrs, X10,Yn, **Nomad**, base power 12n, reach; source toughness conflict below |
| A | 11 Kangee, Aerie Keepers, Zn for n=1…11, Bird text→Nomad, flying removed, n feather counters |
| B | Angrath's Marauders X10,Z* |
| B in manuscript only | Ojutai, Soul of Winter X10,Beast; flying removed; Dragon text→Rhino |

The Myrs' Nomad type is explicit in live notation and required by the Kangee interaction, but omitted from the full manuscript's Myr bullet list. Consecutive NumBuild detection depends on Vigor still being tapped during the next **Alice** second combat after its previous Bob attack. A non-NumBuild intervening instruction suppresses its islandwalk, stopping the intervening Bob attack and resetting this history. Do not store a free-standing `last_opcode` bit in the board backend as a substitute.

## Extension: Sub1, FIsZero and jumps

**Sub1** needs the complete SubCond family (full B.4–B.5): Alice Necrogen Mists X1; Bob 1/1 Halfdane X1,Reflection,Yeti with islandwalk, equipped with **Alice-controlled** Tanuki Transplanter; Bob copiable 0/1 Furtive Homunculus X1,Elf,Dragon,Reflection, flying, green+black, added combat-damage-to-player draw ability; Alice 0/1 Omnath X1 reach/indestructible; Bob Wrathful Red Dragon X1,Reflection,Zubera with flying removed; Bob Belligerent Brontodon X1; Bob Riders with creature type overwritten X1 and text Human→Rabbit, choice Yeti, **T1** protection. Selected rZ retains early Yeti protection while inactive registers lose it to Blood Sun, leaving Halfdane without a legal target when Y=Z; its baseline power stays 1. The Furtive/Omnath blocking, mana, damage, wither, Wrathful and draw machinery must still run to subtract/set underflow flag.

**FIsZero** needs full FLess/flag-combining family (B.33–B.34): Alice Necrogen Mists X11; Bob Halfdane 1/1 X11,Reflection,Yeti,flying,+1/+0 counter; Bob Okk 1/1 X11,Elf,flying,draw-on-player-damage ability; Bob Bishop of Binding type overwritten X11, edited Dinosaur→Fractal; Alice Tamiyo,Collector of Tales card type overwritten artifact creature Fractal; Bob Willbreaker X11 with **Mountain added T4**; Alice Shimmers choosing Mountain with conditions X2,Y2; X2,Y3; and X3 respectively; Bob Riders type overwritten X11, Human→Rabbit, choiceYeti,T1. Tamiyo changes control on targeted Bishop trigger to preserve OR-combining flag, and Willbreaker phasing ends that control duration. If the compiler uses Sub1 underflow flags instead of FIsZero, this family can be omitted; document changed loop semantics and compile control flow accordingly.

**Direct forward/backward jump** family (B.22–B.24): Bob Fiery Emancipation copiable0/1 X3,Dragon,Nomad,flying,indestructible,blue+black,firststrike; Bob Chains of Mephistopheles copiable0/1 X3,Sliver,artifact,green+black,**islandwalk per PDF**; Alice-controlled Moonsilver Spear attached to Chains, Angel→Scarecrow; Alice Reaper King X3,Reflection,Beast,green+blue+red; Alice Spellbane Centaur X3,blue text→red; Alice Shadow Sliver X3,Y0,Sliver→Dragon; Alice Melira's Keepers copiable0/1 X3,Sliver,Beast,reach,indestructible,lifelink, Spiteful Shadows attached; Alice Empyrial Archangel X3,Dragon,indestructible,flying removed,green+black; Alice Lich X3 animated. Kangee/Nomad constants from NumBuild supply nonzero Z distance. Computed Z=0 additionally needs Bob Akron Legionnaire copiable0/1 X3,Z0,Dragon,Elf,flying,indestructible,blue+black,firststrike.

Backward adds Alice Joraga Warcaller X3, Elf text→Scarecrow, **P−5** value counters; Alice Justice X3,red text→white,lifelink; Alice Rotlung Reanimator creature type overwritten X3, Cleric/Zombie text→Scarecrow/Sand and black→white. P is full cyclic program length in cards (at least6; smaller programs repeat to first multiple≥6). Program movement is replacement-backed **one-card milling repeatedly**, not one bulk reorder: otherwise library-bottom order introduces a player choice. Setup must place keyword counters on Melira's Keepers while its ability is disabled; Lich setup needs the manuscript's Platinum Angel/life-restoration precautions before runtime.

## Conditional jump discrepancy and proposed explicit repair

Full B.25–B.28 and live conditional-jump text agree with one another but contradict the ISA headers and interpreter. No correction was found elsewhere in the inspected full/published text or live explanation. The published version gives ISA semantics without the detailed recipes; it therefore does not settle the recipe problem.

| Source recipe | Flag | Derived described event | Bob combat/jump |
| --- | --- | --- | --- |
| Labeled NF: 1/1 Blinding Angel X3,Y2 orY3; Bob Meishin 0/3 X3,Y*,reach | 0 | Angel remains1/1; Tetsuko makes it unblockable; playerdamage trigger | **Skipped; no jump** |
| Same labeled NF | 1 | Meishin reduces Angel power0; no playerdamage | **Runs; jump** |
| Labeled F: 2/2 Angel X3,Y4 orY5,protectionblack; sameMeishin | 0 | Angel2/2 blockable; Meishin forced to block | **Runs; jump** |
| Same labeled F | 1 | Angel1/2 madeunblockable byTetsuko; playerdamage | **Skipped; no jump** |

This is an inference from their stated rules transitions, **not an executed engine result**. Short source snippets establishing it: NF prose says “so no jump takes place” for flag0; F prose says “skip Bob’s combat phase” for flag1. Author JS `case2/3` jumps under `!flag`, `case4/5` under `flag`, matching intended ISA, not these recipes.

Proposed canonical fixture correction **conditional-gates-swap-v1**: keep ISA numbers, swap only the conditional Blinding Angel-family gate assignments **Y2↔Y4, Y3↔Y5**. The 2/2/protectionblack Angels then serve NF; 1/1 Angels serve F. Swap corresponding conditional forward Dragon-shadow Slivers **Y2↔Y4** as well, preserving forward/backward routing; unconditional forward Y0 Sliver staysY0. Keep shared Bob Meishin X3,Y*. Freeze this as a deliberate project correction with a provenance record, and require both flag values for each opcode in a real board fixture before claiming validated correction. Do not change every Yn globally: doing so would also relabel program operands and destroy the targeted repair.

## Source discrepancies requiring fixture decisions

- **NumBuild Myr toughness:** PDF12Y/2; live12Y/1. Both give required power, but engine tests must check survival and legal blockers rather than assuming equivalence. Use PDF `/2` provisionally and state that choice.
- **NumBuild Mangara animation:** PDF specifies0/1,Nomad,reach; live omits these fields, retaining only colors/text edits. Use complete PDF recipe and separately track upkeep disabling.
- **NumBuild Ojutai:** PDF lists Bob's Ojutai, live omits it and its card dataset lacks it. Its necessity is unestablished; tapped-history argument alone does not need it. Do not add guessed interactions.
- **Jump Chains attack ability:** PDF explicitly islandwalk; live grammar `fly`. Both allow attacking, but blockers differ because shadow alone does not guarantee it cannot be blocked. Prefer PDF islandwalk and test the actual forced combat.
- **Backward first-strike damage:** PDF B.24 prose says Z′ but derives P−3Z′, while live says3Z′, consistent with Fiery Emancipation's tripling. Treat the single Z′ sentence as conflicting prose and derive actual damage from rules.
- **Masked text/type assignments and missing Cephalid:** preserve all text-edit effects separately; use the valid replacement alias above consistently rather than generating illegal Artificial Evolution choices.
- **Conditional prefix provenance:** the webpage's prefix templates indicate adding condition types, while `ec` marks which explicitly listed subsequent changes are copiable. It does not resolve every token-copy chain or type-overwrite ordering. Construction manifests must preserve actual setup steps, not infer copiable status from convenience.

## Next verification evidence

First freeze the Add/Halt named inventory, fill the declared metadata choices into a prepared fixture, and enumerate its legal targets and trigger orders. Require the full six actual program cards and reader, real Warcaller-derived Halana power, counter addition, ownership-sensitive phasing, upkeep counter removal and actual drawn game outcome. Compare decoded r1=5 with ISA only at instruction boundaries.

Then choose repeated Add1 versus full NumBuild for source constants; validate Set/Zero separately. Implement loop arithmetic with Sub1 and real flags first if that avoids the larger FIsZero family. Conditional-jump repair remains explicit and provisional until physical-rule traces match both truth-table rows. Prepared runtime success proves neither the physical setup chain nor all paper instructions.
