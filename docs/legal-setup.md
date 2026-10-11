# Legal preparation of the Add/Halt profile

Research date: 2026-10-11. Target: `add-halt-prepared-v1` in `src/construction.rs` and `data/construction/add-halt-prepared.json`, with X2 = **Capybara**, Stormtide Leviathan controlled by **Bob**, and Bludgeon Brawl at **T3**. This note covers preparation, not the runtime engine tests.

**Status: concrete preparation routes for the individual roles, not a replayed legal setup or an exact certificate for the current JSON.** The paper's Appendix C supplies modification techniques and an illustrative Vaevictis sequence; the longer sequence below is our proposed composition. No complete starting-decks → prepared-board execution was performed in this research. The importer assigns states directly. Passing runtime fixtures therefore proves neither legal reachability nor the correspondence between printed physical cards and imported roles. [Author's full construction, Appendix C/D][paper]

The smallest immediate discrepancies are copiable colors and Vehicle/Crew metadata, real keyword counters and their timestamps, chronological timestamps for attachment/global installation, and the impossible default Green overrides on exiled imprint lands. The closing sequence below also needs a concrete engine replay and a complete starting-deck inventory before it can be called a legal setup. These are preparation issues, not evidence that the Add/Halt runtime mechanism fails.

## Sources and interpretation

Use the Wizards Comprehensive Rules dated 2026-09-25 for the legal derivations. Specific rule numbers below refer to that document. Card-Forge links are pinned to the engine revision actually used by this project; their `Oracle` fields and scripts are primary evidence about **the selected engine**, not a substitute authority for future Oracle updates. Wizards release notes independently confirm the important copy exceptions for Urza, Imposter Mech, Saw in Half, and Kathril. [Rules][cr], [Urza][bro], [Imposter Mech][neo], [Saw in Half][unf], [Kathril][iko]

Preparation allows cooperative choices, target selection, votes, copying, searching, and library manipulation. The no-choice requirement begins after preparation. Arbitrarily much mana means a sufficiently large **finite** chosen number of loop repetitions for this finite board, not declaring an infinite quantity in a game.

Names below denote roles with their final copied characteristics. A physical Council Guardian that becomes Sterling Grove remains the same physical card underneath its copy effect. A legal certificate must preserve that provenance instead of treating it as a printed Sterling Grove with an unexplained keyword.

## Exact creature and artifact recipes

Carry out these operations before protection/shroud, Cursed Totem, Stony Silence, and final Blood Sun obstruct them. Create final tokens as Alice, then transfer control where required. An Alice-created token remains Alice-owned after Donate. Do not bounce final modified objects: changing zones discards the lasting edits, and a bounced token cannot return. CR 111.2, 400.7, 707.2, 707.9. [Rules][cr]

### 1. The 36 flying 2/1 Sliver Dralnu gates

1. Cast an ordinary Dralnu's Crusade. It is an enchantment and a legal noncreature target for Astral Dragon.
2. Cast Astral Dragon. **Before that spell resolves**, cast Artificial Evolution targeting it, changing Dragon to Sliver. The spell's text change follows it onto the battlefield under CR 400.7a. Its ensuing enter trigger creates two copies of Dralnu's Crusade with copiable 3/3, Sliver, creature in addition to enchantment, and flying. Editing Astral Dragon after its enter trigger is already on the stack would be too late to edit that independent trigger.
3. On one such 3/3 token, activate Drana with X=1, making it 3/2 until end of turn; cast Enrage with X=1, making its actual size 4/2.
4. Cast Saw in Half targeting that token. It dies, and Alice creates two tokens with copiable **2/1**, enchantment creature, Sliver, flying, and Dralnu's original mana cost and colors. The temporary Drana/Enrage effects are not copied. Their modified actual size is the input to Saw's copy exception.
5. Use Mythos of Illuna on this unedited template enough times to create the 36 final tokens. Pay without **both** red and green; otherwise Mythos adds an enter/fight ability absent from the prepared profile. Remove spare templates and helpers later.
6. Only after making each final copy, apply Artificial Evolution to replace Goblin with its stage selector and Zombie with Saproling. Add Monkey to the Y-stage gates and Metathran to the Z-stage gates using edited Olivia as described below. Apply Prismatic Lace to make final gates green before shroud.

The token's flying is **copiable**, not a flying counter. The black/red original colors survive this route until a later noncopiable Lace effect. Artificial Evolution edits applied to a template do not survive a subsequent ordinary copy, which is why step 6 is last. [Astral Dragon][astral], [Artificial Evolution][ae], [Drana][drana], [Enrage][enrage], [Saw][unf], [Mythos][mythos], [copy rules][cr]

**Imprint cards retain their ordinary colorless card characteristics.** The current manifest's default Green effects on the 36 lands in exile have no provided legal source: Scythe moves actual land cards directly from library to exile, Lace targets permanents, and a prior battlefield modification would not follow a card through those zone changes. Clear these effects as is already done for library program cards; their colors do not affect Scythe's name count. [Scythe][scythe], [Lace][lace], CR 400.7 [rules][cr]

The selector substitution uses the checked-in mapping, including Capybara for X2. Capybara is not used in another selected Add/Halt role. Do not silently substitute the paper's obsolete Cephalid mapping or the older inventory's Octopus mapping.

### 2. Vaevictis and Bloodthirster

For each original creature:

1. Cast Croaking Counterpart targeting it. While that spell is on the stack, Artificial Evolution changes Frog to Sliver.
2. The resulting token is a copiable **1/1 green Sliver**, with the original creature's abilities, including flying.
3. Cast Glasspool Mimic. Before it resolves, edit Shapeshifter to Beast and Rogue to Reflection. On resolution, choose the Sliver token as its copy. The copy exception adds Beast and Reflection; the copied result is 1/1 green Sliver Beast Reflection.
4. Create final token copies with Mythos, avoiding the red-and-green bonus. Remove the original Mimic and extra intermediates.
5. Give the Y Vaevictis Monkey and the Z Vaevictis Metathran with lasting Olivia effects. Grant Bloodthirster one **double strike counter** through Kathril; it is not conditionally made a land in this subset, so its ordering against Blood Sun is immaterial here. No extra flying counter is required.

This is the Appendix C Vaevictis example applied also to Bloodthirster. A legal profile must record the tokens' **copiable green color**. Merely importing the original card colors and a later Green modification yields the same present color but different copyable information. [Full construction Appendix C][paper], [Croaking][croak], [Glasspool][glasspool], [Mythos][mythos]

### 3. The twelve Y protection Riders

The mana-value-zero and noncreature requirements can be composed without asking Vizier to copy a noncreature:

1. Put Vizier of Many Faces into Alice's graveyard and activate embalm. As the token enters, choose an ordinary Riders of Gavony creature. Choose **Yeti** for its fresh enter choice. The embalmed copy has no mana cost and is white, with Zombie added.
2. Donate that creature to Bob. This is now a creature an opponent controls.
3. Cast Imposter Mech as Alice, choosing that opposing Riders copy as it enters. This copy is a **Vehicle artifact**, retains **crew 3**, has no mana cost, and loses creature and its creature subtypes. Make its fresh chosen type Yeti.
4. Make the twelve final Alice-owned token copies with Mythos; choose Yeti for **each** copy as it enters. An earlier chosen type is not a copiable value. Give each final copy the appropriate Human→Zn Artificial Evolution edit and green Lace effect.
5. With Bludgeon Brawl active, equip each final Rider to Alice's corresponding Y gate while both objects are Alice-controlled. Then Donate the Rider to Bob. Its mana value zero produces no Brawl power increase.

This leaves no Zombie/Human/Knight creature subtypes on the noncreature artifact, but does leave the artifact subtype **Vehicle** and printed **crew 3**. Stony Silence later prevents activating crew. The current schema's artifact-only override without Vehicle/Crew is a projection of this outcome, not its full copiable state. Retain these fields in any literal legal-state certificate. [Vizier][vizier], [Wizards Imposter Mech rulings][neo], [Brawl][brawl], [Mythos][mythos], CR 205.1a, 707.2, 707.9 [rules][cr]

**Declared source-Rider repair:** make the source-protection Rider an Alice-created Croaking Counterpart **1/1 green Frog** copy of ordinary Riders, choose **Zubera afresh on entry**, then edit Human→Saproling and Donate it to Bob. Its copiable Frog type line has no Human, so the edit affects the protection ability without making the Rider itself a Saproling. Retain the original mana cost, abilities and vigilance. No flying/islandwalk is added, so Stormtide prevents its attack. This corrects the ordinary printed Human→Saproling route, which also edits its own Human type, creates a Saproling before the T2 gates, makes it lose its own protection to Sun, and can create the Stormtide/Life-and-Limb dependency that invalidates Terrain's Island overwrite. This is a proposed rules-derived fix, not the paper's literal source body; the complete-board Forest regression must verify the dependency outcome. [Croaking][croak], [Riders][riders], [copy/type and dependency rules][cr]

### 4. Animated Celestial and Illusionary Terrain

**Celestial:** make an original Celestial Convergence an artifact using Memnarch. Edit Urza, Prince of Kroog's Soldier word to Eldrazi, then activate Urza targeting that artifact. The copy is an enchantment creature, 1/1 Eldrazi. Memnarch's noncopy artifact addition is **not copied**; it does not become an artifact simply because Urza could target the original. Cast Glasspool Mimic, edit Shapeshifter→Antelope and Rogue→Eldrazi before resolution, and copy this token. Make the final token with Mythos. Its copiable creature types are Eldrazi Antelope and base size 1/1. It enters with **seven** omen counters; while Alice still controls it, use Resourceful Defense to move exactly six to a disposable permanent. Color the final creature green and Donate it to Bob. A legal fixture cannot skip that seven→one operation. [Wizards Urza rulings][bro], [Glasspool][glasspool], [Celestial][celestial], [Defense][defense]

**Terrain:** similarly animate an artifact-made original Illusionary Terrain with Urza edited Soldier→Masticore. Make the final 1/1 enchantment creature Masticore copy. Select **Island** for both land-type choices on its own entry, then color it green. Its final type is not artifact. Remove the unanimated originals/intermediates. Arrange suppression before its next Alice upkeep so cumulative upkeep cannot intervene. [Urza][bro], [Terrain][terrain], [full construction §4.7][paper]

### 5. Add's Halana

Start with Halana and Alena, Partners at 2/3. Belbe's Armor X=2 makes its actual size 0/5; Drana X=1 makes it 0/4. Saw in Half creates two copies with copiable base **0/2**, retaining Human Ranger and the original abilities. Make the final token green. Edited Olivia adds Aetherborn (X0), Elf, Reflection and Yeti, leaving the original Human/Ranger types intact. It needs no extra flying, reach or keyword counter. [Halana][halana], [Armor][armor], [Drana][drana], [Saw][unf]

### 6. Registers and lasting 2/2

For each original Joraga Warcaller copy, use Croaking Counterpart edited Frog→Zn to create a **1/1 green Zn** token, replacing its Elf type. Final Warcaller tokens cannot retain Elf. Add Rabbit, and also Rhino to r0, with Olivia. To obtain a lasting, **noncopiable** 2/2 without permanently adding Gigantoplasm's activated ability:

1. Have a helper Gigantoplasm copy an ordinary Warcaller, retaining Gigantoplasm's added size-setting activated ability.
2. Equip Blade of Shared Souls to a final register and choose that helper as its temporary copy.
3. Activate the register's copied size-setting ability with X=2. The resulting base-2/2 effect has no duration.
4. Equip Blade to a helper, ending its copy effect on the register. The register restores its own copiable Zn/1/1 characteristics and loses Gigantoplasm's added ability; the separately resolved 2/2 effect remains.

Place actual indestructible and vigilance counters **after** Blood Sun if this profile is to support NumBuild; retain that explicit retiming in the extension certificate. Give each register two −1/−0 counters using the seed/multiplication procedure below. Put two +1/+1 counters on r1 and three on r2; none on the others for the sample Add/Halt board. Apply the final red Lace effects **after** Life and Limb. [Croaking][croak], [Gigantoplasm][giga], [Blade][blade], [Defense][defense], [Kathril][iko], CR 611.2a/b [rules][cr]

## Noncopiable types, keyword counters, and numeric counters

### Olivia requires survival, not just enough mana

Edit Olivia Voldaren's Vampire word to the desired type, then activate her damage/type ability. The lasting type addition is not copied. Re-editing Olivia later does not rewrite an already resolved type-adding effect. Before pinging a 2/1 gate once, use Belbe X=1 to make it 1/2. Before pinging a 1/1 Fighter twice for Sliver plus a stage type, use Belbe X=2 to make it −1/3. Before the four Halana additions, use Belbe X=4 to make it −4/6. Resolve the pings individually. Registers can receive their indestructible counters first. These temporary changes prevent state-based deaths between pings; cleanup removes damage and until-end-of-turn boosts together. No lasting power change is intended. [Olivia][olivia], [Armor][armor], CR 514.2, 704.5g [rules][cr]

### Physical keyword counters and timestamps

Discard Darksteel Myr (indestructible), Questing Beast (vigilance), Sylvia Brightspear (double strike), and Halana and Alena (reach) into Alice's graveyard. Repeated Kathril entries distribute the appropriate counters to Alice-controlled final recipients; all other counters from those triggers go onto disposable helpers. Mirror Gallery avoids the legend rule while multiple legendary helpers coexist. Twelve entries suffice to put one indestructible and one vigilance counter on each register; three of those entries give reach to the three Fighters, and one gives double strike to Bloodthirster. Remove excess helpers before runtime. [Kathril rules][iko], [engine Kathril script][kathril], [Mirror Gallery][mirror]

**Use per-role ordering, not one keyword-counter timestamp band.** Fighters' reach counters should precede final Blood Sun, so inactive conditional Fighters lose reach. Registers' indestructible/vigilance counters should instead follow Blood Sun for a NumBuild-capable extension: r0 is still an inactive Saproling land during Alice's second combat and must survive Excruciator damage, and it must attack without tapping on Bob's turn so it can block the next Alice turn. Late counters keep those two abilities despite inactive-land suppression. The current Add/Halt subset does not exercise those register attack/damage requirements, so its early imported effects do not settle the extension question. This is an explicit repair of the paper's generic default-T1 statement, not a claim that the published text specified late counters. Bloodthirster is unconditional and not a land here; either placement survives. Moving/adding another same-kind counter resets that kind's timestamps. Numeric counter placement does not share this layer-6 consequence. CR 122.1b, 613.7c [rules][cr], [Blood Sun][sun], [NumBuild requirements in B.21][paper]

These conclusions revise the first draft's overly broad early-keyword recommendation. A future NumBuild fixture must also place Khorvath/Sylvia's Rhino flying/haste/double-strike granting effects after Blood Sun, so those grants remain on inactive-land r0. See `constant-gadgets.md` for the source mismatch and candidate profile repairs.

### Concrete −1/−0 seed and multiplication

On Alice's first turn, cast a nonartifact, nonblack seed creature such as Ballot Broker, Donate it to Bob, and leave Stormtide/Grand Melee uninstalled. On Bob's first turn it has been continuously controlled since his turn began; Bob attacks Alice with it. After that combat, Alice casts Jabari's Influence: the attacker is a legal target, changes control back to Alice, and gains one −1/−0 counter. Alice's life must survive that attack. This is why the seed requires an actual combat, not an arbitrary counter assignment. [Jabari][jabari], [Ballot Broker][ballot]

Create **24 Alice-controlled Resourceful Defense copies**. Bounce the seed while all registers are still Alice-controlled. Its one counter causes 24 separate Defense triggers. Assign two to each register; each resolving trigger places one −1/−0 counter. The original printed Ballot Broker is reusable later, but its counter disappeared when it left the battlefield. Obtain the sample's +1/+1 counters by moving two and three from a Kathril onto r1/r2 using Defense's activated ability. **Remove all Defense copies before bouncing any remaining counter-bearing helpers**, or their mandatory counter-transfer triggers can corrupt the final board. Then Donate registers to Bob. [Defense][defense], [Kathril][iko], CR 400.7 [rules][cr]

For Celestial's six unwanted omen counters, use Defense's activated move while both Celestial and its disposable receiver are Alice-controlled. Remove Defense before removing that receiver too.

## Bob's Grove and cross-control attachments

The Appendix C suggestion to obtain blue protection and **then** target Council Guardian with blue True Polymorph needs an order correction: blue protection would prevent that target.

1. Alice makes her intended original Sterling Grove an artifact with Memnarch; Spectral Guardian and the other shroud/protection locks are not installed yet.
2. Alice casts Council Guardian, and its enter vote trigger goes on the stack. Give that object a lasting green Lace effect if required, while still unprotected.
3. Before the vote trigger resolves, Donate Council Guardian to Bob, then cast True Polymorph targeting that now-Bob-controlled Guardian and Alice's artifact Grove. Both are legal targets. It becomes an ordinary enchantment Grove; the original's noncopiable artifact addition is not copied.
4. Resolve the earlier vote trigger. Alice controls Ballot Broker and casts both her votes for blue; Bob casts his one vote for any other permitted color. Blue wins, so the **same object**, now the Bob Grove, gains protection from blue. Its prior type/name change does not break the enter trigger's self-reference.
5. Attach Suppression Bonds before finishing any remaining locks. Bonds is white before its later green Lace, so protection from blue does not bar it.

This route produces a Council Guardian physical card copying Sterling Grove with a resolved protection effect; it does not literally modify a printed Grove into that object. The importer must represent the copy/provenance or explicitly call it a role projection. [Council][council], [Ballot Broker][ballot], [True Polymorph][poly], [Memnarch][memnarch], CR 201.5, 707.2 [rules][cr]

For each wildcard Crusade, use Memnarch to add artifact; edit Goblin→Monkey/Metathran/Masticore and Zombie→Saproling. With Brawl installed, equip it to its **Alice-controlled** Fighter, then Donate the Fighter to Bob. Likewise equip Y Riders while Alice controls both Equipment and gate, then Donate the Riders. Equip cannot target an opponent-controlled creature, but changing control after attachment does not detach it. When a host phases out, its attachments phase out indirectly and phase in with that host, even across controllers: the Alice wildcard returns with Bob's Fighter, and the Bob Rider returns with Alice's Y gate. CR 702.6a, 301.5d, 702.26g. [Brawl][brawl], [Donate][donate], [rules][cr]

## Chronology that preserves the runtime mechanism

The paper's circled bands are comparisons of effects, not literal globally interchangeable object dates. A physically reachable preparation should use this schedule, assigning distinct real event timestamps inside each row:

| Relative phase | Operations | Required ordering |
|---|---|---|
| A, early | Cast/Donate Stormtide; final Terrain; make base copies, lasting type edits, 2/2 register effects and green creature colors | Stormtide before Terrain; both before Life/Limb |
| A, early keyword grant | Kathril places Fighter reach, and optionally unconditional Bloodthirster double strike | Fighter reach before final Blood Sun |
| B, gates | Final Dralnu gates/wildcards and their individual edits | Before Life/Limb |
| B, final Sun | Install final Blood Sun after Fighter reach; place register indestructible/vigilance afterward for the NumBuild-capable extension | Sun follows Fighter reach but precedes register survival/vigilance grants |
| C, early T3 | Install the **final** Brawl once, equip all hosts while Alice controls them; then Donate the relevant objects | Equipment's **new attachment timestamp** follows Brawl and the host's entry; no later Brawl removal |
| C, late T3 | Install Life/Limb, edited Gorgons, final untapped Spectral Guardian, both Groves/protection/shroud | Life/Limb after **all wildcard attachment timestamps**, so its Green follows their Black; protection/shroud after setup targets |
| D, final registers | Prismatic Lace makes registers red | After Life/Limb; do before protection/shroud forbids the target |
| E, no-mechanism-order locks | Root Maze, Choke, masks, Recycle, Stony Silence, Cursed Totem, final Grand Melee | Late installation is needed for setup; verify retiming does not change this subset's runtime |

Prismatic Lace can target registers after the Gorgon protection effects: protection from Reflection/Beast creatures does not prohibit a type-less instant, and the Groves shroud enchantments rather than these registers. Perform every other target-sensitive setup operation before its applicable lock.

**Do not remove Brawl after attaching its animated Equipment.** Losing the Equipment subtype detaches the wildcard/Rider artifacts under CR 301.5c and 704.5p. Install final Brawl early within T3, attach first, then install Life/Limb late within T3. Wildcard Crusades receive new object timestamps when attached; Life/Limb must follow those timestamps to preserve its Green-after-Black ordering. The final engine fixture should use these actual relative comparisons. [Brawl][brawl], [rules][cr]

Do not copy the JSON's numeric timestamps into a claimed physical history. A permanent cannot receive a Lace effect before it exists; each attachment gives Equipment a new timestamp; and late shutdown objects cannot also have been installed before the equipment operations they forbid. Keep the comparisons actually needed by the runtime and separately record the real chronological sequence. Counter timestamps require explicit data or replay history: a `counters` count map alone is not sufficient evidence. CR 613.7b–e [rules][cr]

## Program, starting decks, and complete cleanup: proposed route

This is a concrete **candidate** closing procedure, not an engine-tested setup. It uses controlled phasing to avoid two otherwise illegal cleanup operations: an untapped Spectral Guardian shrouds the bootstrap artifacts, and Alice's Grove shrouds helper enchantments. A plain Capsize cleanup after those locks is insufficient. [Guardian][guardian], [Grove][grove]

1. **Bootstrap:** a possible Alice opening seven is Ancient Tomb, Grim Monolith, Power Artifact, two Lotus Petals, Gemstone Array and Dimir Guildmage. Tomb pays for Monolith; the Petals supply UU for Power Artifact. Monolith's tap produces three colorless while its reduced untap costs two. Repeat finitely; Array converts the resource to colored mana; Guildmage draws required cards, never from an empty library. Cast Leyline of Anticipation from the drawn cards. Keep a Spellbook through Alice turn two's cleanup so the remaining helper hand does not require unwanted discards. [Monolith][monolith], [Power Artifact][power], [Array][array], [Guildmage][guild], [Leyline][leyline], [Spellbook][spellbook]
2. **Turn boundary:** make/Donate the Jabari seed on Alice turn one, perform its Bob-turn-one attack, and build the bulk of the board on Alice turn two. Final Alice attackers remain Alice-controlled through the intervening Bob turn before runtime. Grand Melee must be absent or phased out for Bob turn two's combat, so Bob can choose not to attack with his final Stormtide.
3. **Life:** establish Worship and a surviving creature for each player. Before Terrain's basic-land overwrite removes Ancient Tomb's printed damage/mana ability, repeatedly tap/untap Tomb until Alice is at one, Donate it to Bob, and repeat for Bob. Worship makes the last lethal damage leave one. Do this before masks/shroud obstruct targets. Keep Monolith/Array through Bob's final setup main phase; mana cannot be banked across phase/step boundaries. [Tomb][tomb], [Worship][worship], [Twiddle][twiddle], [Donate][donate], [Terrain][terrain]
4. **Bob's zones:** on Alice's turn, before Bob Mask/Recycle, use Guildmage to make Bob draw his whole initial deck, then discard it all; an Alice-controlled Tormod's Crypt exiles his graveyard. Bob requires no personal mana engine for these targeted abilities. Create/Donate Bob's final permanents, including Recycle, before the final queue is ordered, keeping sufficient Alice library cards for any draw triggered while Alice briefly controls Recycle. The Add/Halt subset has no flag, so Bob's final library/hand are empty. Record his actual starting deck and extra public exiled cards outside the role manifest. [Guildmage][guild], [Crypt][crypt], [Recycle][recycle]
5. **Open a cleanup window on Alice turn two:** after final Blood Sun, cast the final Alice Grove (giving its shroud a T3 timestamp), add artifact/color, and finish the Council→Bob Grove recipe. Tap final Spectral Guardian with Twiddle, so artifact targets are available. Alice's artifact Grove does not shroud itself; Reality Ripple can now phase it out. Its absence removes Alice's enchantment shroud through Bob turn two. **Before final Brawl is installed**, cast Wheel enchanting Alice, color it green, use Argent Mutation to make it an artifact until end of turn, then Reality Ripple it out. A player-enchanting Aura must not become Equipment under Brawl: CR 704.5n would detach it. Wheel must be phased out first. Spells that reached Alice's library while Wheel was active are drawn and dealt with before queue ordering. [Guardian][guardian], [Grove][grove], [Wheel][wheel], [Mutation][mutation], [Ripple][ripple], [Twiddle][twiddle], [rules][cr]
6. **Phase the late locks instead of reconstructing them:** cast/color Alice Recycle while enough library remains for all mandatory draws. Artifact it temporarily with Argent Mutation, then phase it out with Reality Ripple; its hand-size/draw effects are inactive thereafter. Similarly create/color and temporarily artifact/phase Alice Mask, Stony Silence, Cursed Totem and Grand Melee. Argent Mutation is a spell, so it works even while Stony/Cursed temporarily shuts off setup abilities; bank mana before that interval. Mutation always draws a card: perform every such operation **before** the final program queue. Non-Aura locks becoming temporary Equipment need not be attached. Finish final Brawl, equipment attachment/control transfers, late Life/Limb, and register red effects in the chronological order above. Keep the tapped Guardian from accidentally untapping. [Mutation][mutation], [Ripple][ripple], [Recycle][recycle], [Brawl][brawl]
7. **Scythes:** one printed Strata Scythe is a copying template, not 36 printed copies. Create **36 final token copies**, each with a fresh enter trigger that searches for its specified actual basic land, exiles it, and shuffles. Linked imprints are independent. The template's own search may fail to find a land; its imprint is not copied. Finish **all** searches/shuffles before ordering the program, and remove the printed template during cleanup. [Scythe][scythe], CR 607.2a, 707.2, 701.19b [rules][cr]
8. **Queue:** draw the remaining setup/program cards with Guildmage, then discard the five library program lands while Wheel is phased out. Reclaim them in reverse final order, recovering Reclaim between casts through Archaeomancer/Capsize: placements Plains, Forest, Swamp, Island, Plains yield top-to-bottom **Plains, Island, Swamp, Forest, Plains**. Play the sixth Plains as the tapped program permanent during Alice turn two after Root Maze is active. No further spell may shuffle or draw Alice's library. [Reclaim][reclaim], [Archaeomancer][archaeomancer], [Capsize][capsize], [Root Maze][root]
9. **Bob turn two, after combat:** bank sufficient finite colored mana. While Alice's Grove, Wheel, Mask, Recycle, Stony, Cursed and Grand remain phased out and Guardian remains tapped, Capsize every remaining helper with buyback, including Spellbook, bootstrap artifacts and Leyline. Remove Resourceful Defenses before counter-bearing helpers. Untap final Guardian with a last Twiddle once artifact-targeting cleanup is complete; its shroud does not prevent sacrificing Crypt as a cost. Cast One with Nothing to discard Alice's remaining helper hand. Activate Alice's Tormod's Crypt targeting Alice; it sacrifices itself before resolution, then exiles Alice's graveyard including One, Twiddle, Capsize-recovery spells, discarded helpers and Crypt. Wheel/Mask/Recycle are still phased out, so this neither pollutes the program library nor requires a forbidden player target nor draws a program card. No helper cards remain in hand, graveyard or battlefield. [Capsize][capsize], [Twiddle][twiddle], [One with Nothing][one], [Crypt][crypt]
10. **Boundary:** allow Bob cleanup to clear remaining temporary effects and floating mana. Alice-controlled directly phased-out locks phase in at Alice's next untap, before her upkeep. Their temporary artifact additions expired during cleanup (usually Alice turn two's cleanup); lasting Lace colors, original object timestamps, counters and attachments persist. The directly phased Wheel returns attached to Alice despite the now-active Mask. The artifact Alice Grove returns and final Guardian is untapped. Compare the full board **after this untap**, then begin Alice's first runtime upkeep/combat. Final Alice attackers have been continuously controlled since before that turn began. CR 702.26a/f/i [rules][cr], [Ripple][ripple]

No complete action log verifies these phases yet. In particular, record the exact library counts around Recycle and every Mutation draw, the order of Grove/Wheel phasing before Brawl, the timestamp of the persistent Alice Grove, every extra exiled original, and both starting-deck lists. This route adds **Argent Mutation, Reality Ripple, Spellbook, Reclaim, One with Nothing and Tormod's Crypt** as concrete cleanup helpers beyond the paper's named modification toolkit. Check their current format legality as part of the eventual starting-deck certificate.

## What remains before a legal setup claim

1. **Freeze a physical-state schema:** remove impossible Green modifications on exiled imprint cards; retain copiable Green on the relevant creature copies, Vehicle/Crew, actual keyword counters with their placement timestamps, and Council-under-Grove provenance. Provide the actual target-legal coloring/phasing operations for noncreature Green overrides, rather than importing them before the permanents exist. Keep the original-to-copy map and all extra exiled deck cards.
2. **Replay attachment/timestamp closure:** create a real chronological history satisfying the runtime comparisons, particularly uninterrupted early-T3 Brawl, attachment timestamps followed by late-T3 Life/Limb, Sun after Fighter reach but before extension register keywords, Terrain after Stormtide, and Red after Life/Limb. The table is a dependency schedule, not a fully expanded exact action trace.
3. **Execute one complete starting-game setup:** select full legal starting decks, draws and card counts; expand each reusable spell cycle; run the proposed phased-lock cleanup and following untap through the real engine; and compare every final field/zone/counter/attachment and next-turn eligibility. Only that replay can promote this note from rules-derived candidate construction to an actual setup certificate.

Until then report the result as **a prepared-board Add/Halt execution backed by real rules-engine fixtures, with researched preparation routes**, not a physically completed or legally replayed full construction.

[paper]: https://www.toothycat.net/~hologram/Magic/Magic_Microcontroller_full.pdf
[cr]: https://media.wizards.com/2026/downloads/MagicCompRules%2020260925.txt
[bro]: https://magic.wizards.com/en/news/feature/the-brothers-war-release-notes
[neo]: https://magic.wizards.com/en/news/feature/kamigawa-neon-dynasty-release-notes-2022-02-09
[unf]: https://magic.wizards.com/en/news/feature/unfinity-release-notes-2022-10-07
[iko]: https://magic.wizards.com/en/news/feature/ikoria-lair-behemoths-and-commander-2020-edition-release-notes-2020-04-10
[astral]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/astral_dragon.txt
[ae]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/artificial_evolution.txt
[drana]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/d/drana_kalastria_bloodchief.txt
[enrage]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/e/enrage.txt
[mythos]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/mythos_of_illuna.txt
[croak]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/c/croaking_counterpart.txt
[glasspool]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/g/glasspool_mimic_glasspool_shore.txt
[vizier]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/v/vizier_of_many_faces.txt
[brawl]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/bludgeon_brawl.txt
[celestial]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/c/celestial_convergence.txt
[defense]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/resourceful_defense.txt
[terrain]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/i/illusionary_terrain.txt
[halana]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/h/halana_and_alena_partners.txt
[armor]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/belbes_armor.txt
[giga]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/g/gigantoplasm.txt
[blade]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/blade_of_shared_souls.txt
[olivia]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/o/olivia_voldaren.txt
[kathril]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/k/kathril_aspect_warper.txt
[mirror]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/mirror_gallery.txt
[sun]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/blood_sun.txt
[jabari]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/j/jabaris_influence.txt
[ballot]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/ballot_broker.txt
[council]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/c/council_guardian.txt
[poly]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/t/true_polymorph.txt
[memnarch]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/memnarch.txt
[donate]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/d/donate.txt
[monolith]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/g/grim_monolith.txt
[power]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/p/power_artifact.txt
[array]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/g/gemstone_array.txt
[guild]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/d/dimir_guildmage.txt
[leyline]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/l/leyline_of_anticipation.txt
[tomb]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/ancient_tomb.txt
[worship]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/w/worship.txt
[twiddle]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/t/twiddle.txt
[scythe]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/strata_scythe.txt
[crypt]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/t/tormods_crypt.txt
[reclaim]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/reclaim.txt
[archaeomancer]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/archaeomancer.txt
[capsize]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/c/capsize.txt
[root]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/root_maze.txt
[one]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/o/one_with_nothing.txt
[wheel]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/w/wheel_of_sun_and_moon.txt
[recycle]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/recycle.txt
[mutation]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/argent_mutation.txt
[ripple]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/reality_ripple.txt
[spellbook]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/spellbook.txt
[guardian]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/spectral_guardian.txt
[grove]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/sterling_grove.txt
[riders]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/riders_of_gavony.txt
[lace]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/p/prismatic_lace.txt
