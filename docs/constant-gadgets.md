# NumBuild and Set: concrete extension inventory

Research date: 2026-10-11. Sources: full author PDF, current author explanation HTML/JS, Wizards rules, and the project's pinned Card-Forge scripts. Current author explanation HTML SHA-256: `69261db4d41fa65af257d56fdf061b6a4d0357f071fdb13a0175517888c27ed4` (downloaded this date).

**Status: source-backed recipes and candidate repairs; no complete NumBuild or Set runtime has been executed in this research.** The existing engine-backed Add fixture does not establish these instructions. The full author PDF and current webpage differ materially. The inventories below make those differences explicit rather than silently claiming paper fidelity. [Full PDF B.2/B.21][paper], [current explanation][live], [rendering/source conventions][js]

The first ordinary source program to target is:

```
NumBuild 2   -> 10 0 2
Set r1 r0   ->  5 1 0
NumBuild 3   -> 10 0 3
Set r2 r0   ->  5 2 0
Add r1 r2   ->  0 1 2
HaltD       ->  4 0 0
```

All registers start at zero. Expected final values: r0=3, r1=5, r2=3. Each NumBuild is separated from the next by Set, so this program does not require the consecutive-NumBuild base-144 multiplier. Its queue needs eighteen actual symbol lands, with seventeen initially in Alice's library and the last Plains as the program permanent. This is a narrower construction profile than an implementation of arbitrary NumBuild. [Author instruction table and example programs][paper]

## Shared repairs before these instructions

The current Add/Halt representation cannot simply append the author's gadget list:

- **Registers need late indestructible/vigilance counters.** During Alice's second NumBuild combat, Z has not been read and r0 is still an inactive Saproling land. It must survive Excruciator's damage. On Bob's turn it must attack without tapping so it can block on the next Alice turn. Place these counters **after final Blood Sun**, keeping them in layer 6 despite land suppression. Fighter reach counters stay **before** Blood Sun so inactive Fighters cannot block. This repairs the paper's generic default-T1 ordering for the extension; it is not presented as a timestamp explicitly supplied by the paper. [NumBuild narrative][paper], [counter and timestamp rules][cr], [Blood Sun implementation][sun]
- **Khorvath/Sylvia's Rhino grants must follow Blood Sun.** Active X10 sources must grant flying/haste/double strike even to inactive-land r0. Put their source/effect timestamps after Sun. Their X10 suppression still removes their own abilities when inactive. Khorvath's text edit is **Knight→Rhino**; Sylvia's is **Dragon→Rhino**. These are different original words. [Khorvath][khor], [Sylvia][sylvia], [rules][cr]
- **Stormtide needs no extension.** Its printed abilities include **islandwalk, not flying or reach**. As an ordinary Leviathan outside the conditional land selectors, it retains islandwalk. It cannot block flying Dream Fighter, Excruciator or Shape Stealer, even while untapped, and its Bob-turn islandwalk attack is unblockable because Alice controls an Island. The previous proposal to add Beast/Reflection and a second Fishliver Oil was unnecessary and is withdrawn. If Beast/Reflection have already been added, they do not change these conclusions: both types are outside this profile's land selectors, flying still excludes it as a blocker, and islandwalk still excludes every Alice blocker. Remove those edits from the minimum inventory rather than depending on them. [Stormtide script and Oracle][storm], CR 702.9b, 702.14c/e [rules][cr]
- **Source Riders must not itself be Saproling.** The declared prepared-board repair uses a Croaking Counterpart 1/1 green Frog copy of Riders, with a fresh Zubera choice, then Human→Saproling text editing. Because its type line is Frog, the edit rewrites its protection ability without creating an early Saproling or suppressing its own protection. See `legal-setup.md`. [Croaking][croak], [Riders][riders], [copy/type-edit rules][cr]

These are rules-derived proposed amendments. Their interaction with the whole board, particularly Forest program symbols and continuous-effect dependency, remains an engine verification requirement.

## Set / Zero

The literal author inventory adds six objects to the reader/register scaffold:

| Role | Controller | Permanent and exact changes |
|---|---|---|
| Copier | Bob | Halana and Alena, Partners; copiable 0/2; add X5, Elf, Reflection, Yeti; green |
| Clear enabler | Alice | Archpriest of Iona; retain Human Cleric; add X5, YA=Metathran, Reflection, Yeti; green |
| Clear attacker | Alice | Shape Stealer; copiable 3/3; retain Shapeshifter Spirit; add X5 and Metathran; one −1/−0 counter; flying counter; green **and** black |
| Party warrior | Alice | 0/2 Warrior token, no flying/reach/abilities; green |
| Party rogue | Alice | 0/2 Rogue token, no flying/reach/abilities; green |
| Party wizard | Alice | 0/2 Wizard token, no flying/reach/abilities; green |

The text edits/additions are noncopiable unless explicitly called copiable. Shape Stealer's flying counter precedes Sun, so it cannot attack while inactive. Its two-color Lace effect must survive when active; Life/Limb makes it only green while inactive. Retain both active colors rather than the prepared constructor's automatic only-Green default. [Full B.2][paper], [current Set row][live], [Shape Stealer][shape], [Archpriest][arch], [Halana][halana]

Concrete preparation follows `legal-setup.md`: Halana 0/2 by Belbe/Drana/Saw; Shape Stealer 3/3 by adjusting an original to actual 6/6 then Saw; copy before adding conditional types/colors/counters. For example original Shape Stealer 1/1, Belbe X=5 gives −4/6, and Enrage X=10 gives 6/6. Its −1/−0 counter comes from the Jabari/Resourceful Defense process. Rotlung Reanimator edited Zombie→the desired party type creates 2/2 tokens; Belbe X=2 gives actual 0/4, then Saw produces copiable 0/2 tokens with that type and no abilities. Color them green. [Armor][armor], [Enrage][enrage], [Saw rulings][saw], [Rotlung][rotlung]

**Combined NumBuild/Set profile can reuse two party roles:** core Tetsuko is a Human **Rogue**; each Kangee is still a **Wizard** after Bird→Nomad. Archpriest supplies Cleric, so one 0/2 Warrior token completes the party. Conditional land suppression does not remove those creature types or creature status. The party calculation is automatic, not a player choice. The combined profile therefore adds four Set objects rather than six, provided those reused creatures remain present. This is an explicit inventory reduction supported by their types and CR 700.8; it is not the author's literal six-object list. [Tetsuko][tetsuko], [Kangee][kangee], [party rules][cr]

Expected runtime evidence:

1. In Alice's third combat, X5 and YA have both enabled Archpriest. A full party enables its mandatory trigger; Reflection+Yeti leaves only rY legal. It grants rY +1/+1 and flying until end of turn, after Sun's removal effect.
2. Shape Stealer attacks. rY can now block flying. Reader creatures have shadow; other potential blockers must be excluded by shadow/protection or ability suppression. Verify the **entire** blocker set, including native Stormtide and both Halana copies; see the blocker audit below.
3. Shape Stealer's block trigger sets its base power to rY's actual power, V+1. The −1/−0 counter makes its actual power V. Its active black color gives wither through Alice's Corrosive Mentor.
4. Its V damage puts V −1/−1 counters on rY, canceling the old V +1/+1 counters. At Bob's combat, the X5 Elf Halana gets rZ's value as power and adds that many counters to rY. For Y=Z the source was also cleared, so the result is zero.

Capture party count, sole legal target, sole blocker, trigger ordering, actual power, wither counter cancellation, and source/destination values. Test clearing nonzero to zero and to another nonzero, Y=Z, and values larger than the attacker's initial 3/3. [Author Set explanation][paper], [Shape][shape], [Archpriest][arch], [Halana][halana]

## NumBuild full family

Using the **current live** Darksteel Myr rows and one Marauder, the full family has 30 creature/permanent roles plus one islandwalk Aura:

| Count | Controller | Role / required changes |
|---:|---|---|
| 1 | Alice | Excruciator: X10=Kavu and XA=Monkey; original 7/7 Avatar; flying, indestructible, vigilance counters **before Sun**; green |
| 1 | Alice | Extra Dream Fighter: copiable 2/2; X10; flying counter before Sun; blue; lasting protection from red |
| 1 | Bob | Vigor: X10 and Beast; reach counter before Sun; islandwalk Aura; green; retain original 6/6 Elemental Incarnation/trample/prevention |
| 1 | Bob | Khorvath Brightflame: X10; remove its own flying; Knight→Rhino text; late source timestamp; green |
| 1 | Bob | Sylvia Brightspear: X10; Dragon→Rhino text; late source timestamp; green |
| 1 | Alice | Mangara's Equity animated as a creature: X10; white→green trigger edit; enter choice **red**; green and black; **no reach** |
| 12 | Alice | Darksteel Myr indexed by Y: copiable base **12Y/1** and creature types **Myr Nomad**; noncopiable X10 and Yn, reach counter before Sun, green; retain artifact creature and printed indestructible |
| 11 | Alice | Kangee indexed by Z=1..11: Zn; Bird→Nomad text (also changes own Bird type); flying removed; **Z feather counters**; green; retain Wizard |
| 1 | Bob | Angrath's Marauders: X10 and ZA=Masticore; original 4/4 Human Pirate; green |
| 1 | Alice | Fishliver Oil enchanting Bob's Vigor, granting islandwalk |

The single Oil enchants Vigor; Stormtide already has islandwalk. Together with the four reused-party Set objects this adds **35 battlefield objects**, excluding the existing reader/register scaffold. For all conditional added abilities, use actual counters or concrete lasting effects and store their timestamps. A family inventory cannot substitute for a complete legal prepared-state certificate. [Live NumBuild row][live], [full B.21][paper], [Vigor][vigor], [Excruciator][excr], [Kangee][kangee], [Marauders][maraud], [Equity][equity], [Oil][oil]

### Important source disagreements and omitted details

- The PDF lists Myrs at **12Y/2** without Nomad/reach. The live page lists **12Y/1**, Nomad and reach. Nomad is required for Kangee's Z increment; reach is required to block flying r0. Use the current live recipe explicitly.
- The PDF gives Equity 0/1, Nomad and reach; the live row specifies neither size/Nomad nor reach. Giving Equity reach creates a second forced blocker of r0 on Bob's turn, making the damage/assignment ambiguous and potentially adding another Z. Use **no reach**. A candidate explicit animation is Urza's 1/1 Soldier enchantment-creature copy, keeping no artifact addition and adding X10 afterward; its exact original creature type/size is not supplied in the live row, so freeze that proposed choice in the extension manifest.
- The PDF repeats the Marauders line around the Kangee list. Use **one**, matching the live row and the narrative's factor-two damage increase. Two independent doublers produce a different multiplication.
- The paper's unspecified default T1 cannot cover register survival/vigilance and Rhino grants for inactive r0. The late-grant repair above must be declared and tested.
- Kangee's feather counters are **not copied** by Mythos. Kick a source Kangee to seed feathers, then use Resourceful Defense to duplicate/move exact counts onto final Alice-created copies, removing counter-multiplication helpers before cleanup. All eleven final sources need their own 1..11 counts. Remove flying through the lasting Mist Dragon/Blade technique **after** final copies.

These discrepancies are visible in primary texts; the proposed amendments are our derivations, not an author-verified erratum. [Full PDF][paper], [live HTML][live], [live tooltip logic][js], [Kangee][kangee], [Defense][defense], [copy rules][cr]

### Making the Myr copies legally

A naïve Saw target fails because printed Darksteel Myr is indestructible. A concrete additional helper is **Turn to Frog**: it removes abilities until end of turn without changing copiable information. On an original Myr, resolve Turn to Frog (actual 1/1), Belbe X=1 (actual 0/2), Enrage X=24Y (actual 24Y/2; omit Enrage for Y=0), then Saw. The now-mortal original dies; the two copies are copiably Darksteel Myr **12Y/1**, retaining printed indestructible, artifact creature Myr, mana cost, and its original colorless baseline. Frog/blue/ability-removal effects are not copied. The live row places Nomad before `ec`, making it **copiable**. Cast Glasspool Mimic, edit Shapeshifter→Nomad and Rogue→Myr before it resolves, and copy the sized Myr. It becomes an artifact creature Myr Nomad; the repeated Myr type does not create a distinct second subtype. Create final tokens with Mythos, then add X10/Yn noncopiably, green Lace and the early reach counter. [Glasspool][glasspool] This is a new explicit preparation route, not one quoted from B.21. [Turn to Frog][frog], [Armor][armor], [Enrage][enrage], [Saw][saw], [copy rules][cr]

Protect the 1-toughness Myrs while adding the remaining Olivia selector types: their printed indestructible works before conditional-land suppression. Do not install the Life/Sun suppression scaffold until those pings/copies are complete.

### Runtime contract and consecutive state

First NumBuild: in Alice combat two, Vigor uniquely blocks the blue Dream Fighter and both phase out; red r0 cannot block that Fighter. r0 blocks Excruciator. With Vigor absent, Equity's wither retaliation clears r0. On Bob's turn Vigor returns, r0 attacks without tapping, and the only eligible blocker is selected-Y Myr. Selected-Z Kangee gives it Z additional power, so Vigor converts its **12Y+Z** damage into counters on r0. Vigor's own islandwalk attack is unblockable. [Author NumBuild narrative][live]

Consecutive NumBuild: Vigor remains tapped through the next Alice turn because it attacked on the preceding Bob turn. It cannot block the blue Fighter, so it does not phase out; its prevention converts Equity retaliation into growth. Alice combat two doubles r0 twice, V→4V. Combat three enables the one Marauder, making each retaliation twice as large: two tripling steps give 4V→36V. Bob's turn suppresses the ZA Marauder again; r0's two damage/retaliation steps yield 36V→144V, then the Myr contributes 12Y+Z. Persist actual Vigor tapped/phased state; an ISA-level `previous_opcode` flag is not physical evidence. [Live narrative][live], [Vigor][vigor], [Marauders][maraud]

The two-digit construction must be tested with zero, Z=0, Y=11/Z=11, a nonzero old r0, consecutive pairs representing numbers above 143, and a intervening non-NumBuild instruction that resets the consecutive-state mechanism. Enumerate all blockers/targets and replacement choices, not just the final integer.

## Complete blocker and combat-damage audit

**Scope:** the Add/Halt scaffold plus the NumBuild/Set inventory above, including the declared Frog source-Rider repair, early Fighter/Shape/Myr reach or flying counters, late register survival counters, and late Rhino grants. This is a primary-source rules derivation to check against a full engine trace, not an executed full-board proof. Check the state **at declare blockers**, before Dream/Shape triggers resolve. A creature with zero power can still block; later changes to evasion do not undo a legal block. Grand Melee requires the maximum feasible blocks without overriding flying, shadow, protection, landwalk, tapped status or phased status. CR 509.1b/c, 702.9b, 702.16f, 702.28b [rules][cr]; [Grand Melee][melee], [keyword enforcement][keywordcode], [block validation][combatcode].

| Combat / attacker | Entire legal blocker set in this profile | Why remaining bodies cannot block |
|---|---|---|
| Each Alice reader combat: matching 3/2 flying, shadow gate | That layer's active Bob Dream Fighter | Only that Fighter has the required shadow and reach. Other active gate attackers are 2/1 and unblocked by Tetsuko; the 1/1 Vaevictis/Bloodthirster bodies are also unblocked. Register/Vigor flying or reach does not bypass shadow. |
| NumBuild, Alice combat two: extra 2/2 blue flying Dream Fighter | Bob Vigor if untapped and phased in; empty set if Vigor is tapped in the consecutive case | Red r0 is excluded by protection from red. Reader Fighters have shadow. Halana copies are inactive, and Reflection would independently exclude them against this blue attacker. Stormtide, Khorvath with its own flying removed, Sylvia, Preserver, Mentor, Frog source Rider and Celestial have neither usable flying nor reach; target Riders are noncreature Vehicles. |
| NumBuild, Alice combats two and three: green flying Excruciator | r0, with the late Khorvath Rhino flying grant | Vigor is a Beast, excluded by Excruciator's protection from Beasts even when present. Reader Fighters have shadow. All other Bob bodies lack usable flying/reach, as above. The blue Fighter does not attack again in combat three: it is either phased out or tapped after its second-combat attack. Excruciator has vigilance. |
| NumBuild, Bob combat: red flying r0 | The one active, untapped selected-Y Darksteel Myr with reach | Other Myrs have suppressed reach. Alice reader gates/Vaevictis/Bloodthirster have shadow (and the latter two are Beasts, independently excluded by red r0's protection). Excruciator is suppressed by returned XA, losing early flying; extra Dream is phased out or tapped. Shape Stealer is X5-inactive. Equity has no reach. Kangees have their own flying removed, and party/Gorgon/Guardian/Sliver/Mentor/Archpriest bodies have no flying/reach. Animated program/gate lands likewise have no usable flying/reach. |
| Bob combat: Stormtide, plus Vigor during NumBuild | Empty set for each islandwalker | Alice's program land is an Island under Stormtide. Native Stormtide islandwalk and Vigor's one Fishliver Oil prohibit **every** block, including a Myr block. Vigor's trample supplies no blocked-damage assignment here because Vigor is unblocked. |
| Set, Alice combat three: green-and-black flying Shape Stealer | The sole rY, granted flying by Archpriest before attackers | Other registers lack flying. Set Halana has reach but is Reflection, excluded by green Shape Stealer's protection; Add Halana is X0-inactive and loses reach. Vigor is X10-inactive and loses early reach; Khorvath is inactive and has its own flying removed. Reader Fighters have shadow. Every other Bob body is excluded as in the previous rows. Stormtide cannot block regardless of optional Beast/Reflection. |

The reader row applies only after the preceding wildcard enables that layer; unreleased later layers have their flying/reach suppressed. The selected gate's attached Scythe makes it 3/2 at declaration, while Tetsuko excludes every other gate with toughness one. The compiled subset has no planeswalkers, battles, extra attacking destinations, banding, abilities to block additional creatures, or paid attack/block costs. Alice's live types and the edited Gorgons are essential: green/blue creatures have protection from Reflection; green/red creatures have protection from Beast. The reference table enumerates all bodies in the specified profile rather than assuming that a high-power attacker cannot have an extra blocker. [Author reader and register mechanism][paper], [current construction][live], [Tetsuko][tetsuko], [Shadow Sliver][shadow], [Gorgon before its declared edits][gorgon], [Stormtide][storm], [Halana][halana], [Khorvath][khor], [Vigor][vigor].

**Forced assignment follows from these singleton sets.** Grand Melee forces each listed available blocker to its sole available attacker. Every blocked attacker and blocker has exactly one combat counterpart: CR 510.1c/d therefore assign **all** its positive combat damage to that counterpart; CR 510.1a assigns none for nonpositive power. The other attackers are unblocked and assign to their defending player under CR 510.1b. There is no split, lethal-damage ordering, or voluntary damage-to-player allocation for a blocked instruction attacker: r0, Excruciator, Shape Stealer and Myr have no trample, and no listed pair has banding. The two Dream pairs leave combat before damage. CR 510.2 fixes simultaneous dealing after assignment. [Rules][cr], [combat legality][combatcode], [printed creature scripts][excr], [Shape][shape], [Myr][myr], [Vigor][vigor].

For Set with V=0, Shape's actual power becomes zero only **after** the block and trigger resolution. That does not undo the block or permit reassignment. For Y=Z, Z-reader phasing removes the relevant Dralnu +1/+1 while restoring the register's 2/2 base; those changes balance, so Archpriest's +1/+1 still makes the register's power V+1 when Shape reads it. Use actual P/T after the mandatory APNAP reader resolution, not a frozen value from declaration. [Shape][shape], [Dralnu][dralnu], CR 509.1b, 613.4, 603.3b [rules][cr].

**Remaining decisions:** none is introduced by native or Beast/Reflection-added Stormtide in these blocker/damage rows. This does not establish the whole construction is choice-free: the full trace must separately enforce sole legal Vaevictis/Archpriest/Halana targets, APNAP trigger groups, per-event replacement applicability, no optional activations and every asserted timestamp/phase boundary. An extra eligible flying/reach creature, missing protection, retained inactive reach, unintended second Nomad/reach Equity, or an attack/block cost invalidates the singleton proof and must be rejected rather than resolved by a preferred controller choice. The scheduling/prevention audit below treats the expected trigger and replacement events; physically reaching all modifications remains a separate preparation task. [Target restrictions][paper], [Dream][dream], CR 508.1d, 509.1b/c, 510.1, 603.3b, 616.1 [rules][cr].

## Smallest next conservative demonstration

For the six-instruction 2+3 program above, narrow the full family to six NumBuild singletons, only the **Y0 Myr**, only **Z2/Z3 Kangees**, and the Vigor Oil: ten new objects. Omit the Marauder because no NumBuild instructions are consecutive. Add the four Set objects using Tetsuko/Kangee party reuse: **14 additional battlefield objects**, with late keyword/source timestamps and no Stormtide modifications. This is the smallest explicitly enumerated conservative candidate in this note, not a proof of global minimality. Omitting unused banks is only valid if the compiler constrains the backend to these operands and rejects unsupported instructions.

First verify that exact profile using the real engine: r0=2; r1=2; r0=3; r2=3; r1=5; then actual HaltD. Verify the full board and chronological copy/counter history before claiming legally reached preparation. Only broaden to all Myrs/Kangees and consecutive base-144 once a full-family fixture proves their extra objects introduce no choices.

[arch]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/archpriest_of_iona.txt
[armor]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/belbes_armor.txt
[cr]: https://media.wizards.com/2026/downloads/MagicCompRules%2020260925.txt
[croak]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/c/croaking_counterpart.txt
[defense]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/resourceful_defense.txt
[enrage]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/e/enrage.txt
[equity]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/mangaras_equity.txt
[excr]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/e/excruciator.txt
[frog]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/t/turn_to_frog.txt
[halana]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/h/halana_and_alena_partners.txt
[js]: https://cyh31.neocities.org/amepscimtg/explanation.js
[kangee]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/k/kangee_aerie_keeper.txt
[khor]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/k/khorvath_brightflame.txt
[live]: https://cyh31.neocities.org/amepscimtg/explanation
[maraud]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/angraths_marauders.txt
[oil]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/f/fishliver_oil.txt
[paper]: https://www.toothycat.net/~hologram/Magic/Magic_Microcontroller_full.pdf
[riders]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/riders_of_gavony.txt
[rotlung]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/rotlung_reanimator.txt
[saw]: https://magic.wizards.com/en/news/feature/unfinity-release-notes-2022-10-07
[shape]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/shape_stealer.txt
[storm]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/stormtide_leviathan.txt
[sun]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/blood_sun.txt
[sylvia]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/sylvia_brightspear.txt
[tetsuko]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/t/tetsuko_umezawa_fugitive.txt
[vigor]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/v/vigor.txt
[glasspool]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/g/glasspool_mimic_glasspool_shore.txt

## Scheduling and prevention audit (2026-10-11; rules-derived, not executed)

This audit concerns the complete, corrected NumBuild profile above. It does **not** justify choosing arbitrary orders for a general MTG stack or accepting every set of replacement effects. The actual profile largely has mandatory APNAP ordering and disjoint replacement applicability. Missing late register counters or the specified flying/reach suppression invalidates the arguments below. Native Stormtide needs neither a second Oil nor Beast/Reflection edits.

### Inactive r0 power is restored by the Y wildcard's phasing

Dralnu's Crusade has **two separate static abilities** in the pinned script: one gives +1/+1; the other adds Saproling and changes color. Their treatment under Sun differs. Its type-changing ability has already applied in layer 4 and can continue doing so even though its source loses abilities in layer 6. Its separate power/toughness ability does not begin until layer 7c, so a land gate that has lost that ability does **not** grant +1/+1. [Dralnu script][dralnu], CR 613.1, 613.4, 613.6 [rules][cr]

For register value V, the precise Alice-turn timeline is:

| Boundary | Z0 gate | r0 actual P/T before damage | Explanation |
|---|---|---|---|
| Alice turn start and combat one | Metathran makes it Saproling/land; Sun removes its abilities | **(V−1)/(V+1)** | Life/Limb 1/1, two −1/−0 counters, V +1/+1 counters; Z0 gate's +1/+1 ability absent |
| After X reader resolves | Still Metathran/Saproling/land | **(V−1)/(V+1)** | XA wildcard releases Y gates, not Z gates |
| Combat two, blocks declared | Still land until the Y-reader trigger resolves | **(V−1)/(V+1)** | Late Khorvath/Sylvia grants already permit r0 to block Excruciator; power need not be positive to block |
| After Y-reader trigger finishes | Y Fighter and its attached YA wildcard phased out; Z gates regain abilities | **V/(V+2)** | Z0 gate is now a nonland and grants its separate +1/+1; r0 remains an inactive Saproling land |
| Combat three, if Z=0 | Z0 gate itself phases out | **V/(V+2)** | r0 becomes active: lasting 2/2 replaces the inactive Life/Limb 1/1, offsetting loss of the gate's +1/+1 |
| Combat three, if Z≠0 | Z0 gate stays present and nonland | **V/(V+2)** | r0 stays inactive with Life 1/1 plus its restored +1/+1 |

The **value counters** remain V before damage; the temporary V−1 power is real and must not be replaced with a register-level invariant throughout the turn. V=0 gives −1 power and **positive one toughness**, so no zero-toughness SBA is caused by that intermediate state. [Life/Limb][life], [Dralnu][dralnu], [Sun][sun], [author reader and NumBuild sequence][live]

The Y-reader Dream Fighter resolution first phases the selected Y gate, then phases the Fighter itself. Its YA Equipment phases out **indirectly** with that Fighter. That second operation releases all Metathran Z gates. The pinned Dream script represents the two phasing operations as parent ability and subability; rules perform no SBA check between steps inside that one resolution. Check statics as the effects require, but place the SBA/priority boundary after the whole triggered ability. [Dream script][dream], [phasing effect][phasecode], CR 702.26g, 704.4 [rules][cr]

### The only extra simultaneous blocking triggers have fixed APNAP order

In a first/nonconsecutive NumBuild's Alice combat two, declarations produce:

| Trigger | Controller | Event and resolution |
|---|---|---|
| Extra blue Dream Fighter | **Alice** | Becomes blocked by Bob's Vigor; phases Vigor and itself |
| Y-reader Dream Fighter | **Bob** | Blocks the selected Y gate; phases gate and itself, including YA Equipment |

Alice is active. CR 603.3b puts Alice's trigger on the stack first, then Bob's above it. **Bob's Y-reader trigger resolves first; Alice's extra-Dream trigger resolves second.** There is one trigger per controller in this group, so neither controller has an ordering choice. The pinned `MagicStack` actually partitions pending triggers by activating player in APNAP order before calling `orderAndPlaySimultaneousSa`. A global list of length two is not evidence of a player-choice group of length two. [Stack code][stackcode], [Dream script][dream], [author sequence][live], [rules][cr]

SBAs are checked after each resolution. Both selected Y gate and its Fighter are absent before that first check; their indirect attachments are absent too. Restored Z gates have their positive copiable toughness (plus any Scythe boost); r0 has V+2 toughness; no relevant survivor has lethal damage yet. The following extra-Dream resolution removes Vigor and the blue Fighter, with Oil phasing indirectly with Vigor. There is still no damage between these resolutions. Both complete before the combat damage step. [Phasing code][phasecode], [Oil][oil], CR 704.3–4 [rules][cr]

Reversing these two resolutions would also end with both pairs absent and the r0 boost restored **under those no-damage/positive-toughness assumptions**, but that reversal is **not legally selectable** in this game and need not be permitted by the controller. Follow APNAP instead of relying on that counterfactual confluence.

In a consecutive NumBuild Vigor is tapped, cannot block, and the blue Fighter stays unblocked. Thus there is **no extra-Dream blocking trigger**; the Y-reader trigger is alone. Combat three has only the Z-reader phasing trigger in this family. Bloodthirster's two combat-adding triggers occur in **different combat-damage steps**, first strike and regular, with a priority boundary between; they are not two simultaneous ordering choices. Set's third-combat Shape Stealer trigger is Alice-controlled while the Z-reader trigger is Bob-controlled, so the same APNAP reasoning puts the reader above Shape Stealer. [Bloodthirster script][blood], [Shape][shape], [Dream][dream], [rules][cr]

Unexpected same-controller duplicate Dream triggers mean the blocker set was not unique—for example an additional flying/reach creature also blocked the blue Fighter. Stormtide cannot create that duplicate: it has neither flying nor reach. Reject that state; don't sort those triggers to conceal an invalid construction. Dream's wording triggers once per blocking creature, so a second blocker matters independently. [Dream][dream], CR 700.1 [rules][cr]

### Marauders and Vigor never compete on the same intended damage

Both are Bob-controlled, but their tests use **different sides of the damage event**:

- Marauders doubles damage whose **source Bob controls**.
- Vigor prevents damage whose **recipient is another creature Bob controls**.
- Equity retaliation is dealt by **Alice's Equity itself**, not by the damaged r0. Its defined recipient is the earlier damage source; that does not change the controller of the new damage source.

Consequently the intended events have this applicability table:

| Event | Marauders | Vigor | Result |
|---|---|---|---|
| r0 (Bob) → Excruciator (Alice), Alice combat two | Inactive because ZA not released | Recipient not Bob's creature | Ordinary damage to indestructible Excruciator; Equity triggers |
| r0 (Bob) → Excruciator (Alice), Alice combat three | **Applies** in consecutive full profile | Recipient not Bob's creature | Double damage; Equity records that doubled amount |
| Equity (Alice) → r0 (Bob) | Source not Bob's | **Applies when Vigor present** | Prevented damage creates +1/+1 counters; no wither counters from prevented damage |
| Excruciator (Alice) → r0 (Bob) | Source not Bob's | Applicable prevention cannot prevent Excruciator's damage | Actual damage remains, zero Vigor counters; late indestructible keeps r0 alive |
| r0 (Bob) → selected Myr (Alice), Bob combat | **Inactive:** ZA wildcard has returned in Bob's untap | Recipient not Bob's creature | Ordinary damage to indestructible Myr; Equity triggers |
| selected Myr (Alice) → r0 (Bob), Bob combat | Source not Bob's | **Applies** | 12Y+Z damage is prevented and converted to counters |

This is an applicability proof, **not a confluence argument for two competing replacements**. If the correct NumBuild profile presents Marauders and Vigor for the same source/recipient event, diagnose controller/type/suppression state instead of choosing an order. [Marauders script][maraud], [Vigor script][vigor], [Equity script][equity], [Excruciator][excr]

CR 615.12 still applies prevention effects to unpreventable damage, but they prevent nothing; Vigor counts only actually prevented damage, so gives zero counters for Excruciator's hit. The pinned handler sets `PreventedAmount=0` in this case and its buffered replacement side-effect runner uses the prevented amount, only resolving a positive sum. It does not incorrectly grant seven counters merely because Vigor was considered. [Replacement handler][replacementcode], [Vigor][vigor], [rules][cr]

Worship modifies **damage's life-loss result**, not the creature-damage prevention event. The pinned Worship script uses `LifeReduced`, while Vigor/Marauders use `DamageDone`. It does not reduce the damage quantity recorded for Equity or Bloodthirster. Protection can prevent damage independently, so verify that intended Equity/Myr/Excruciator sources have no protected Beast/Reflection/Yeti/Zubera type matching the recipient. [Worship script][worship], [replacement handler][replacementcode], CR 120.4, 702.16 [rules][cr]

### Damage timing, counter updates and actual choice boundaries

On Bob's normal damage step, r0's damage to Myr and Myr's damage to r0 are assigned using the **pre-event powers**, then dealt together. Vigor's counters from Myr's hit can increase r0's power during processing but do not retroactively increase r0's already assigned damage. Equity's later trigger contains the original amount r0 actually dealt. Perform the resulting SBAs before putting the trigger on the stack, then resolve the retaliation with its own prevention processing and another SBA boundary. The selected Myr remains indestructible; r0 has late indestructible; the single selected Myr stays in combat. [Damage trigger amount capture][damagecode], [Vigor][vigor], CR 510.1–3, 120.4, 704.3 [rules][cr]

For the consecutive case, start Bob's combat at 36V:

- First-strike r0 damage is 36V. Its one Equity retaliation is prevented, giving r0 another 36V → 72V.
- Normal-step assigned r0 damage is 72V, while Myr assigns D=12Y+Z. Vigor converts D to counters. Equity subsequently retaliates for the recorded 72V, bringing the final value to **144V+D**.

With V=0, zero damage produces no damage event, replacement increase or Equity trigger (CR 120.8). The only normal-step growth is D from Myr. Do not manufacture a zero-damage Equity trigger merely to keep an ISA trace uniform. [Equity][equity], [Vigor][vigor], [rules][cr]

General replacements **can** change the output: if a Bob-controlled source dealt D to another Bob creature while both effects were active, applying Marauders first would yield 2D Vigor counters; applying Vigor first would prevent D and give D counters, leaving no damage event for Marauders to double. CR 616.1 genuinely permits that affected controller's choice; the validated NumBuild profile avoids such an event. Do not add a global rule that these effects commute. [Marauders][maraud], [Vigor][vigor], [rules][cr]

The handler's damage batching can collect candidate effects across multiple source/recipient entries for the same decider. A list with more than one effect can therefore describe disjoint events instead of competition on one event. Accepting it requires verifying **each event's applicability and side effects**, including per-event controller and unpreventable flag; cardinality alone is insufficient. Its source has an explicit per-entity ordering TODO. The bounded backend should retain rejection for unknown or overlapping choices and log the expected one-trigger-per-controller APNAP group, instead of broadly weakening strict mode. [Replacement handler][replacementcode], [Stack code][stackcode]

[dralnu]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/d/dralnus_crusade.txt
[life]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/l/life_and_limb.txt
[dream]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/d/dream_fighter.txt
[blood]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/bloodthirster.txt
[worship]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/w/worship.txt
[phasecode]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-game/src/main/java/forge/game/ability/effects/PhasesEffect.java
[stackcode]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-game/src/main/java/forge/game/zone/MagicStack.java#L855
[replacementcode]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-game/src/main/java/forge/game/replacement/ReplacementHandler.java#L344
[damagecode]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-game/src/main/java/forge/game/trigger/TriggerDamageDone.java

[combatcode]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-game/src/main/java/forge/game/combat/CombatUtil.java#L906
[keywordcode]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-game/src/main/java/forge/game/card/CardFactoryUtil.java#L3910
[melee]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/g/grand_melee.txt
[gorgon]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/masked_gorgon.txt
[shadow]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/shadow_sliver.txt
[myr]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/d/darksteel_myr.txt

## Bob-untap power defect and proposed phasing compensation (2026-10-11)

**This section supersedes the unconditional Bob-turn power and base-144 claims above.** The cited author recipes do not supply the compensation below. The missing power is now observed in the running first-literal Forge trace; the positive-value consecutive result is a rules-derived prediction, not an executed consecutive fixture. The full constants program is still underway and is not claimed to pass.

### What returns, and what loses its buff

The author gives every third-layer Z-gate creature Y* (Metathran in the paper mapping). The Y* Dralnu wildcard is noncreature Equipment attached to Bob's Y Dream Fighter. Its indirect phasing follows that Bob-controlled Fighter: it phases back in during **Bob's** untap, despite being Alice-controlled. A selected Z gate directly phased under Alice's control stays out until Alice's untap. Thus during Bob's turn, **every present, nonselected Z gate is again a Saproling land**, with its separate +1/+1 ability removed by Blood Sun. Its type-changing ability continues across layers, so it still makes its corresponding register a Saproling. CR 502.1, 702.26a/g, 613.6 [rules][cr]; [author third-reader construction and cross-turn wildcard][live], [PDF §§4.4–4.5][paper], [two distinct Crusade abilities][dralnu].

Consequently, with C +1/+1 counters on r0:

- **Z=0:** selected Z0 remains directly phased out; r0 stays active with lasting base 2/2 and two −1/−0 counters, giving power **C**.
- **Z≠0:** Z0 returns to its land/suppressed state; r0 remains inactive with Life/Limb base 1/1 and two −1/−0 counters, giving power **C−1**, toughness C+1.

The paper §4.8 and live register explanation assert that inactive registers get their compensating +1/+1 from a Dralnu's Crusade. That explanation omits the Bob-turn suppression of the **source** ability. CR 613.6 preserves only an effect that already started applying; it does not combine Crusade's separate type/color and P/T abilities into one effect. Neither the PDF B.21 nor the current live NumBuild row contains an alternate buff that covers this boundary. This is a defect in the literal described profile under these rules, not evidence that Magic cannot implement NumBuild. [Paper §§4.7–4.8/B.21][paper], [current register and NumBuild rows][live], [Crusade][dralnu], [Sun][sun], [Life/Limb][life], [rules][cr].

**Observed read-only runtime evidence:** `target/demo-constant-add/trace.jsonl`, snapshot SHA-256 `44eb4f707f7665b27ab4077ede0a09fc321d8ff14f1f1e32ceb08c5b2c90e163`, 319 complete JSON lines / 4,180,895 bytes, ending at checkpoint/event 318 in Bob turn two's combat beginning. All quoted checkpoints have zero r0 +1/+1 counters:

| Trace checkpoint | Boundary | YA wildcard | Z0 type | r0 P/T |
|---:|---|---|---|---|
| 159 | Alice turn one, before Y-reader resolution | Present | Enchantment creature land, Saproling/Metathran/Forest | −1/1 |
| 168 | After Y-reader resolution | Phased out | Enchantment creature, Sliver/Metathran | 0/2 |
| 292 | Alice cleanup | Phased out | Nonland creature | 0/2 |
| 302/303 | Bob turn two, untap | Present | Enchantment creature land, Saproling/Metathran/Forest | **−1/1** |

At those Bob checkpoints r0 retains the proposed late flying/haste/double strike/indestructible/vigilance grants. Therefore the discrepancy is the missing P/T source, rather than a failure to install those keywords. The trace is a local generated primary observation and can grow or be regenerated; its snapshot hash and event IDs identify the inspected prefix. It proves this boundary, not successful completion or the entire published construction.

### Consecutive arithmetic without compensation

Let V be positive r0 value at the start of a consecutive NumBuild, and D=12Y+Z with **Z≠0**. Alice's second and third combats still reach 36V counters because YA is absent and the Z0 Crusade buff operates throughout those damage steps. At Bob's untap the power drops to 36V−1:

| Bob step | Damage / prevention | r0 counters after the relevant effects |
|---|---|---:|
| First-strike damage and Equity retaliation | r0 deals 36V−1; Vigor converts Equity's equal retaliation | 72V−1 |
| Normal damage assignment | r0 assigns **72V−2** to its sole Myr; Myr assigns D to r0 simultaneously | 72V−1+D after Myr prevention |
| Later Equity retaliation | Equity retaliates for recorded 72V−2, independently of the D already added | **144V−3+D** |

For example V=1 and D=2 predicts **143 counters instead of 146**. For Z=0, r0 is active and the missing-one defect does not occur; for a cleared/nonconsecutive V=0, r0's nonpositive power assigns no damage and produces no Equity trigger, so the Myr can still give exactly D counters. This explains why NumBuild 2 / Set / NumBuild 3 / Set may work while arbitrary consecutive literals remain unsupported. The intended ISA equation cannot replace actual assignment powers. CR 510.1–2, 120.8 [rules][cr]; [Equity][equity], [Vigor][vigor], [author base-144 narrative][live].

### Proposed one-object compensation

Add one **Alice-owned, Bob-controlled, white, zero-mana-value noncreature artifact** copied from Joraga Warcaller through the existing embalmed-Vizier / Imposter-Mech route. Its copiable artifact subtype is Vehicle, its copiable rules retain crew 3, and it has no creature subtypes. After all final copies, apply **Elf→Saproling** with Artificial Evolution and put **exactly one +1/+1 counter** on this artifact. With Brawl it is Equipment; equip it to the **same Y Dream Fighter** as YA. Its retained static ability reads: “Other Saproling creatures you control get +1/+1 for each +1/+1 counter on Joraga Warcaller.” [Warcaller][warcaller], [Vizier][vizierconstant], [Mech][mechconstant], [Artificial Evolution][evolutionconstant], [Brawl][brawlconstant].

This is a parent-proposed repair with a rules derivation, **not an author erratum or executed setup**. The self-name reads the counter on this very artifact, not an r0 register with the same name: CR 201.5. A noncreature permanent can hold a +1/+1 counter; that counter need not modify its own P/T to be counted by its static ability. The static ability functions on the battlefield without requiring its source to be a creature (CR 113.6, 122.1, 208.3). The pinned Warcaller uses `Affected$ Creature.Elf+Other+YouCtrl` and `Count$CardCounters.P1P1`; after text editing the affected subtype is Saproling, while the counter lookup remains on the source. [Warcaller script][warcaller], [rules][cr].

| Boundary | Compensation artifact | Z-gate +1/+1 | Inactive register power |
|---|---|---|---|
| Alice start / before Y-reader resolution | Present, buffs Bob's Saprolings | Suppressed | 1−2+C+1 = **C** |
| After Y-reader resolution through Alice cleanup | Indirectly phased out | Restored on each present Z gate | 1−2+C+1 = **C** |
| Bob untap / Bob combat | Returns with Y Fighter, buffs Bob's Saprolings | Suppressed on present Z gates | 1−2+C+1 = **C** |

The selected active rZ is not Saproling, so it receives no helper buff; its selected Crusade is absent and its power stays 2−2+C=C. This includes selected r0 when Z=0. The two compensating sources switch together with YA/Y Fighter; there is no priority or SBA boundary partway through the one phasing resolution or simultaneous untap phasing action. The helper is not a land and has no selector creature subtype, so Sun does not suppress its own ability. CR 613.1/4/6, 702.26g, 704.4 [rules][cr]; [Dream][dream], [Crusade][dralnu], [Sun][sun].

**Preparation, with concrete ownership and copy boundaries:**

1. Alice embalms Vizier, choosing ordinary Joraga Warcaller: the resulting creature copy is white and has no mana cost, with Zombie added. Donate this temporary creature to Bob.
2. Alice resolves Imposter Mech copying that opposing creature. Its copy exception produces a white, no-cost artifact Vehicle with crew 3 and no creature subtypes. Alice makes the final token with Mythos; the token is Alice-owned. None of the original source's counters or earlier text edits is copied.
3. After that final copy, target it with Artificial Evolution Elf→Saproling. Because its type line has no Elf, it gains no Saproling subtype and creates no early Saproling. Before the target-shroud locks are installed, use Alice's Resourceful Defense activated ability to move **one** +1/+1 counter from an Alice-controlled seed permanent onto this Alice-controlled final artifact. An ordinary Warcaller kicked exactly once is a concrete one-counter seed. The zero-cost final copy itself was not kicked and is not asserted to enter with a counter.
4. With Brawl installed and before Stony Silence, equip this artifact to Alice's Y Fighter while both are Alice-controlled, then Donate the artifact and Fighter to Bob (in either order after attachment). Donate changes control, not ownership, and does not detach Equipment. Alternatively Bob may equip it after both Donations, during the unlocked preparation period. Multiple Equipment can equip that Fighter; its mana value zero adds no Brawl power. Remove the seed/Defense/copy helpers through the existing cleanup closure, removing Defense before any counter-bearing helper leaves.

The Guardian must be absent or tapped while targeting the artifact for text edit/counter placement/Donate. At runtime the untapped Guardian gives noncreature artifacts shroud. Stony Silence prevents both crew and equip activations; the static Warcaller buff stays available. Copy modifiers make white/no-cost/artifact values copiable, while final Evolution and counter placement are not copied. CR 301.5b/d, 707.2/9b [rules][cr]; [Defense][defense], [Guardian][guardianconstant], [Mythos][mythosconstant], [Donate][donateconstant], [Stony Silence][stonyconstant], [Brawl][brawlconstant].

**Conflicts and affected bodies:** it also gives +1/+1 to inactive Bob Saproling Fighters, Halana copies, Vigor, Khorvath/Sylvia, Marauders and Celestial. It does not restore their abilities, flying, reach or islandwalk, and hence introduces no new eligible attackers/blockers in the audited profile. Alice's reader gates and party creatures are not Bob-controlled and get no buff. Active Elf Halana and rZ are not Saprolings, so the Add/Set transfer remains tied to the same source counters. Source Rider must retain the declared Frog repair; this artifact does not fix an erroneously early Saproling source Rider. Its white noncreature body is shrouded at runtime, so it supplies no new Vaevictis target, no Archpriest/Halana creature target and no party member. Ensure both it and YA phase indirectly with the same Fighter; detachment, becoming a creature, wrong controller or a second +1/+1 counter invalidates the proof. [Guardian][guardianconstant], [Warcaller][warcaller], [Brawl][brawlconstant], [author targeting mechanism][paper], [rules][cr].

Relative to the **unrepaired original Add/Halt scaffold**, this compensation raises the sparse demo additions from 14 to **15**, and the full NumBuild/Set additions from 35 to **36**. Those added objects are a Warcaller Vehicle, not a second Stormtide Oil. If the compensation is adopted into the common scaffold, the gadget-only counts remain 14/35 relative to that repaired scaffold. Engine fixtures must verify the compensation at all three rows, selected Z=0 versus nonzero, original Add/Set target sets, and a positive consecutive example before promoting arbitrary base-144 NumBuild to supported status.

[warcaller]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/j/joraga_warcaller.txt
[vizierconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/v/vizier_of_many_faces.txt
[mechconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/i/imposter_mech.txt
[evolutionconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/artificial_evolution.txt
[brawlconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/bludgeon_brawl.txt
[guardianconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/spectral_guardian.txt
[mythosconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/mythos_of_illuna.txt
[donateconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/d/donate.txt
[stonyconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/stony_silence.txt

### Bounded compensation fixture now passes; full runtime remains separate

The parent execution reported the new `MtgConstantBoardTest.firstNumberHasUniqueBlockersAndLateRegisterKeywords` fixture passing against the prepared-v2 candidate. Its checked source imports the **complete sparse board**, then directly phases the relevant gates/Fighters and recomputes statics. It asserts initial r0 power 0; after X10 release the sole blockers are r0 for Excruciator and Vigor for the blue Fighter, with surviving flying/double strike; after Y release the compensation artifact is indirectly phased and r0 stays at power 0; after Z2 release selected Y0 Myr is 2/3. It then assigns two test counters to r0, phases Y Fighter back in, and asserts the artifact returns, Z0 is a land again, and **r0 stays at power 2**. [Fixture source](/home/jay/dev/Rust/mtg-computer/tools/forge/MtgConstantBoardTest.java:17), [candidate representation](/home/jay/dev/Rust/mtg-computer/src/construction/constants.rs:15).

This is real engine evidence for continuous-effect/phasing behavior and complete-board second-combat blocker sets. Its direct `Card.phase` calls and explicit fixture counter assignment are test inputs, not an executed instruction sequence, legal card-resolution setup, actual combat addition, automatic Bob untap, or positive consecutive NumBuild. The first unrepaired full-game run had only the first NumBuild 2 established at this update; the repaired full constants runtime is still in progress. Selected Z0 and a positive consecutive case still require their own fixture/runtime evidence before base-144 is declared supported.

The legal-state certificate must also preserve **no mana cost** from embalmed Vizier, rather than a printed {0} mana cost; both give mana value zero for Brawl here, but are different copiable characteristics. In the inspected constructor/importer, `mana_cost: "0"` is passed to `new ManaCost(string)` and really means printed {0}, so exact copy fidelity requires a distinct no-cost representation. The pinned parser treats generic −1 as no cost, and the engine provides `ManaCost.NO_COST`; an explicit schema sentinel can select that value. [Importer](/home/jay/dev/Rust/mtg-computer/tools/forge/MtgBoard.java:138), [mana representation][manacostconstant], [parser][manaparserconstant]. This caveat also applies to the twelve similarly prepared Riders. Cast the final Mythos without spending both red and green, avoiding its conditional fight instruction. Counter placement must occur on the final copy, before target shroud/activation locks; do not copy the source's one counter or leave Resourceful Defense and the kicked seed in runtime zones. [Vizier][vizierconstant], [Mech][mechconstant], [Mythos][mythosconstant], [Defense][defense], CR 202.3, 707.2/9b [rules][cr].

[manacostconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-core/src/main/java/forge/card/mana/ManaCost.java#L49
[manaparserconstant]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-core/src/main/java/forge/card/mana/ManaCostParser.java
