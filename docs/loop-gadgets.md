# Physical loop gadgets: checked decrement, flags and jumps

Research date: 2026-10-11. This is a source-derived, implementable **candidate prepared profile**, not a completed rules-engine run or legally replayed setup. Primary construction sources are the [full FUN 2024 manuscript][paper], the [author's live explanation][live] and its [tooltip/preparation code][explainjs], plus the pinned Card-Forge revision `5222f05e569da8e33c85858e6fe1698901548d64`. The live HTML snapshot SHA-256 is `69261db4d41fa65af257d56fdf061b6a4d0357f071fdb13a0175517888c27ed4`. All rules numbers refer to Wizards' [2026-09-25 rules][cr].

Use the existing declared Frog source-Rider repair, literal Vehicle/counter metadata, and Y-Fighter-synchronized Warcaller compensation from `constant-gadgets.md`. The compensation preserves actual register power equal to its value across Alice/Bob turns; without it, the paper's inactive-register power assertion fails. Keep the current X/Y/Z map; X1=Beeble, X3=Drake, X11=Lhurgoyf, XA=Monkey, YA=Metathran. Default unspecified runtime bodies are green; explicit colors below override this. Copiable values are stated separately from subsequent selector/text/color/counter modifications. An engine must reject an unlisted eligible target/blocker or unresolved nonconfluent choice.

## Smallest sum loop to implement first

A sum-only lowering of `n=3; sum=0; while n>0 { sum+=n; n-=1; }` can avoid FIsZero's larger comparison/control machinery. The sequence below performs one harmless extra zero addition, then detects decrement underflow:

| Index | Instruction | Symbols | Physical purpose |
|---:|---|---|---|
| 0 | NumBuild 3 | 10 0 3 | r0=3 from zero |
| 1 | Set r2 r0 | 5 2 0 | n=3 |
| 2 | Add r1 r2 | 0 1 2 | Loop: sum+=n |
| 3 | Sub1Cond r2 | 1 2 2 | n=max(0,n−1); flag=(old n==0) |
| 4 | JumpFwdF 1 | 3 4 1 | Underflow: skip the backward jump to HaltD |
| 5 | JumpBwd 4 | 3 1 4 | Otherwise return to instruction 2 |
| 6 | HaltD | 4 0 0 | r1=6, r2=0; actual drawn game outcome |

The author interpreter advances past the instruction before applying a relative jump: index 4 uses next index 5 + 1 = 6; index 5 uses next index 6 − 4 = 2. The physical program has **21 symbol cards**, P=21. Reuse NumBuild's selected-Y0 Myr and Z3 Kangee; add only Z1 and Z4 Kangees for jump distances. The backward Scarecrow Warcaller needs **P−5=16 +1/+1 counters**. This lowering is valid for this sum body, including initial n=0, not a general permission to execute an arbitrary while-body on zero. Do not silently apply it to bodies with observable zero-iteration effects. [Interpreter][isa], [relative-jump construction B.22–B.24][paper].

The minimum branch subset here is **corrected true-forward Y4**, unconditional backward Y1, immediate distances 1/4, and Sub1Cond with Y=Z. Omit FIsZero, computed-distance Akron, forward-Y0 Dragon-shadow Sliver, true-backward Y5 Angel, and NF gadgets unless compiling those instructions. Constants remain nonconsecutive. This keeps unresolved comparison/control choices out of the first loop profile.

## Flag environment: three battlefield objects and one Bob-owned card

| Object | Controller / exact preparation |
|---|---|
| Inert flag Plains | **Bob-owned**; the only card in Bob's library for false, the only card in Bob's hand for true; no Bob graveyard cards |
| Wheel of Sun and Moon | Alice-controlled Aura enchanting Bob; discard redirects that card to Bob's library bottom |
| Tomorrow, Azami's Familiar | Bob; ordinary green creature, retain draw replacement; no flying/reach/islandwalk |
| Aggressive Mining | Bob; made an artifact with Memnarch, retaining enchantment; forbid land plays; Stony Silence shuts off its sacrifice/draw activation |

Only one flag card is ever in the relevant zones, so Tomorrow's look-at-three has zero or one card and no actual selection/reordering alternative. It replaces empty-library draws too, preventing a draw-loss when true is already held. Do not give the flag card Alice ownership: zone ownership rules would undermine the claimed Bob-library representation. Install the Aura before player shroud or through the existing temporary-lock phasing setup. The prepared flag can be public; these empty/singleton zones contain no hidden alternative. Draw-step Recycle remains in the scaffold. [Full §4.10/live flag][paper], [Tomorrow][tomorrow], [Wheel][wheel], [Mining][mining], CR 400.3, 121.2, 616.1 [rules][cr].

During X1 decrement, Chains is inactive and does not compete with Tomorrow. During X3 jumps, only Alice draws from Lich; Tomorrow applies only to Bob. These are per-player/per-event applicability facts, not permission to pick a preferred effect when two replacements really overlap. Verify them from each event.

## Sub1Cond: nine extra objects

Source B.4–B.5 uses the full subtraction gadget even for a decrement. Restrict this first backend to **Y=Z**; arbitrary SubCond can retain Halfdane's temporary copied P/T and needs broader tests.

| ID / role | Controller | Concrete runtime recipe |
|---|---|---|
| sub-mists | Alice | Animated Necrogen Mists, proposed copiable 1/1 Soldier enchantment creature; add X1 |
| sub-halfdane | Bob | Halfdane, copiable 1/1, retain Shapeshifter; add X1, Reflection, Yeti; green; islandwalk from the next Aura |
| sub-halfdane-oil | Alice | Fishliver Oil enchanting Halfdane; source timestamp before Sun |
| sub-transplanter | **Alice** | Tanuki Transplanter reconfigured onto Halfdane; retain original abilities; while attached it is noncreature Equipment |
| sub-homunculus | Bob | Furtive Homunculus, copiable 0/1; retain Homunculus, add Elf, Dragon, Reflection; add X1; green **and black**; early flying counter; lasting combat-damage-to-player draw ability |
| sub-omnath | Alice | Omnath, Locus of Mana, copiable 0/1; retain Elemental; add X1; green; early reach and indestructible counters |
| sub-wrathful | Bob | Wrathful Red Dragon; retain Dragon; add X1, Reflection, Zubera; green; remove its own flying permanently after final copies |
| sub-brontodon | Bob | Belligerent Brontodon; retain Dinosaur and toughness-damage ability; add X1; green |
| sub-rider | Bob | Riders of Gavony with copiable creature type **X1 only**, proposed Croaking 1/1 X1 token; fresh chosen type Yeti; edit **Human→Rabbit** after final copy; protection-effect timestamp before Sun |

The type-only Rider recipe does not require a specific original P/T in B.5; the explicit Croaking 1/1 variant is a declared preparation choice. Edit Croaking's Frog word to X1 before resolving its copy; final Human→Rabbit affects only the ability, avoiding an unintended type-line Rabbit. Every other size/ability follows B.4/B.5 and live. Use actual lasting modifications, not fabricated additional printed Oracle text. [Halfdane][halfdane], [Tanuki][tanuki], [Homunculus][furtive], [Omnath][omnath], [Wrathful][wrathful], [Brontodon][brontodon], [Mists][mists], [Riders][riders], [Oil][oil], [author recipe][live].

### Real event sequence and sole choices

1. Alice's readers enable X1 and select Y=Z. The early Rabbit protection grant from sub-rider survives on active rZ, while Sun removes it from inactive land registers. The selected Y Rider phases out, but the extra X1 Rider still protects rZ from Yeti. Reflection excludes green/blue targets and the remaining Y protection excludes other red registers: **Halfdane has no legal creature target**. Its mandatory upkeep trigger cannot be placed on the stack with an illegal target (CR 603.3d); baseline power stays 1, rather than copying register power.
2. At Bob upkeep, Alice's Necrogen Mists makes Bob discard the singleton flag if present. Wheel returns it to Bob's library: false. No Tamiyo/Willbreaker family is present, so this does not OR-combine with an old true flag. Empty hand gives no discard choice.
3. Bob's Halfdane attacks by islandwalk. **Alice-controlled** Tanuki triggers and gives **Alice** one green mana, retaining it until end of turn. Alice Omnath has actual P/T 1/2. Tanuki is not controlled by the equipped creature's controller; its trigger is not a granted ability. Current Alice mana must start at zero.
4. Bob's flying Homunculus has power V, toughness V+1 from active register's Elf buff. Omnath can block it iff **1≤V**, because skulk excludes greater-power blockers. All reader gate/Sliver bodies are tapped, phased, suppressed or shadow; inactive Melira's late reach is excluded by its Beast type against green Homunculus; other NumBuild/Set reach/flying bodies are suppressed. Halfdane and Stormtide's islandwalk prevent diverted blocks. Omnath is therefore the sole eligible blocker when V≥1, and no eligible blocker exists when V=0.
5. For V≥1, Omnath deals exactly one damage to Homunculus, triggering Bob's Wrathful. Its Reflection+Zubera source restrictions leave **rZ** as sole target; both players have shroud, green/blue bodies have Reflection protection, inactive registers have Zubera protection. Wrathful makes **Homunculus itself** deal one damage with wither to rZ. One old +1/+1 counter and one new −1/−1 counter cancel as an SBA: V→V−1. Homunculus's combat damage to green Omnath is prevented by protection from Reflection. The trigger source restricting the target is Wrathful; the actual damage source is Homunculus.
6. For V=0, Omnath cannot block. Brontodon makes Homunculus assign its **toughness 1** despite zero power, so combat damage to Alice triggers its added mandatory draw ability. Worship keeps Alice at life 1 but does not erase the damage event. Tomorrow's replacement moves the singleton flag to Bob's hand: true. rZ remains zero.
7. Omnath retains Alice's green mana while active. At Alice's next untap X1 gate returns, making Omnath a land and removing its mana-retention ability; Tanuki's until-end-of-turn retention has expired, so the mana empties at the end of that untap step. Record this actual mana state; do not assign Omnath's power from an ISA value.

With the synchronized register helper, inactive creature P/T remains correctly compensated without restoring its suppressed abilities. All positive blocked damage is to a unique combat counterpart; no split decision occurs. [Card scripts above][wrathful], [Tanuki][tanuki], [Omnath][omnath], [Brontodon][brontodon], CR 509.1, 510.1, 603.3d, 704.5q, 702.118 [rules][cr].

### Preparation details that cannot be skipped

Reconfigure Tanuki onto Alice-controlled Halfdane **before** Donate, Stony and Cursed Totem. It retains Alice control when Halfdane is donated to Bob. Add the draw ability with the author's Blade-of-Shared-Souls / copiably-Angel Ascendant Spirit method, then use Resourceful Defense to remove the **two +1/+1 counters** that Spirit's last activation also places. Remove the Blade/copy and counter-transfer helpers during closure. Counter placement comes after final copies; early flying/reach sources precede final Sun. Fishliver Oil's early source timestamp ensures inactive Halfdane cannot attack on other opcodes. Any counters left on the 0/1 Homunculus change skulk and underflow semantics. [Appendix C][paper], [tooltip draw preparation][explainjs], [Spirit][spirit], [Tanuki][tanuki], [Defense][defense].

## Immediate jump family: exact common board

Sources B.22–B.24; use **PDF Chains islandwalk**, not the live `fly` variant. Give every added selector after the final copy unless the copiable type bundle is explicitly required. Retain specified original printed types/abilities. Proposed animated bodies have copiable type creature through Urza's copy exception; choose its Soldier word as needed before activation. Temporary Memnarch artifact additions on originals are not copied by Urza. [Appendix C][paper], [current jump row][live].

| Role | Controller | Concrete recipe |
|---|---|---|
| jump-emancipation | Bob | Fiery Emancipation; copiable enchantment creature **0/1 Dragon Nomad**; add X3; blue+black; early flying, indestructible, first-strike counters |
| jump-chains | Bob | Chains of Mephistopheles; copiable enchantment creature **0/1 Sliver**; add X3 and lasting artifact; green+black; islandwalk Aura below |
| jump-chains-oil | Alice | Fishliver Oil enchanting Chains; early source timestamp |
| jump-spear | **Alice** | Moonsilver Spear attached to Chains; **Angel→Scarecrow** text edit after final Spear copy; ordinary first strike grant retained |
| jump-reaper | Alice | Reaper King; retain Scarecrow; add X3, Reflection, Beast; **green+blue+red** |
| jump-centaur | Alice | Spellbane Centaur; add X3; edit **blue→red**; green |
| jump-keepers | Alice | Melira's Keepers; copiable **0/1** retaining Human/Warrior; add X3, Sliver, Beast; green; reach, indestructible, lifelink counters placed **while Sun removes its no-counter ability** |
| jump-shadows | Alice | Spiteful Shadows enchanting Melira's Keepers |
| jump-archangel | Alice | Empyrial Archangel; add X3, Dragon; remove flying; early indestructible counter; **BLACK ONLY at T1**, a declared repair below; retain native shroud and redirection |
| jump-lich | Alice | Lich animated as proposed copiable **1/1 Soldier enchantment creature**; add X3; green; runtime Alice life remains 1 |

These are **ten objects**, counting both Chains Oil and Spear/Shadows attachments. Initial literals' Kangees can be shared. Kangee Zn, Bird→Nomad, own flying removed, exactly Z feather counters supplies the immediate Nomad power Z. Its Zn source gate stays directly phased during Bob's turn even though YA returns and suppresses the **gate's** separate +1/+1 ability. For new distances add that indexed Kangee; do not use the suppressed gate itself as the numeric boost. [Emancipation][fiery], [Chains][chains], [Spear][spear], [Reaper][reaper], [Centaur][centaur], [Keepers][keepers], [Shadows][shadows], [Archangel][archangel], [Lich][lich], [Kangee][kangee].

### Forward routing

Add one Alice Shadow Sliver per supported **forward** branch class: X3,Y0 for unconditional forward, or X3,Y4 for corrected true-forward. Edit its printed **Sliver→Dragon**, including its own type line; keep it green. This grant gives Fiery Dragon shadow. Melira is a Sliver and already has shadow from the scaffold's ordinary Shadow Sliver. No backward class gets the Dragon-shadow source. [Author routing B.22/B.25–B.28][paper], [Shadow][shadow].

On Bob's declare attackers, Chains and Fiery attack, with Stormtide also attacking by native islandwalk. Alice's Spear trigger creates one **4/4 white flying Scarecrow**, controlled by Alice. It receives Reaper's +1/+1; its entry triggers Reaper. Reaper's red-source targeting is excluded against every Alice creature by edited Centaur; its blue color excludes Bob's protected Grove, Reflection/Beast exclude colored bodies, and global shroud excludes the other permanents. **The program land is the sole legal target.** Destroy it; Alice's Wheel sends that card to the library bottom. The whole physical program is now cyclic in her library. New Scarecrow can block despite entering this turn, but cannot block the forward shadow attacker. Next Alice turn it is the sole unprotected target for advancing Vaevictis and is sacrificed, permitting the next program card to enter. [Spear][spear], [Reaper][reaper], [Centaur][centaur], [Wheel][wheel], CR 302.6, 509.1, 702.28 [rules][cr].

Melira uniquely blocks shadow Fiery: ordinary Alice gates are already tapped/phased or suppressed; Bloodthirster's Reflection type is excluded by the blue attacker's protection; Empyrial has flying removed; the new Scarecrow has no shadow. Chains' islandwalk rules out every blocker independently. Fiery's power Z is tripled by its own replacement to **3Z**. Wither would put counters on Melira, but Melira cannot receive counters; the damage still happens and triggers Shadows. Melira deals that recorded amount to Alice; Archangel redirects it onto itself. Melira's lifelink gives Alice 3Z life, which Lich replaces with drawing 3Z cards. Active Chains replaces each individual draw: Alice has empty hand, so each becomes mill one. Wheel returns each actual program card to the bottom, producing a forced **3Z single-card rotations**. CR 121.2 prevents treating draw N as one bulk draw; no multi-card bottom-order choice is allowed. [Fiery][fiery], [Keepers][keepers], [Shadows][shadows], [Archangel][archangel], [Lich][lich], [Chains][chains], [Wheel][wheel], [forward narrative][live], CR 616.1, 120.3, 121.2 [rules][cr].

**Protection repair:** both author recipes color Archangel green+black, while Melira is a Beast. Redirection preserves Melira as the damage source (the pinned Shadows explicitly has `DamageSource$ TriggeredTarget`). Green Archangel has protection from Beast through the global edited Gorgon, so **all that redirected damage would be prevented**, destroying the forward jump. Proposed BLACK-ONLY color at T1 removes this protection when active while preserving printed shroud. Keep that Lace **before Life/Limb**: while inactive it becomes green and is protected from Reflection, even though Sun removes its own shroud. A late black override could create extra Wrathful/other legal targets during nonjump instructions. This repair is rules-derived and needs a real damage/lifelink/draw/mill fixture. [Shadows][shadows], [Gorgon][gorgon], [Archangel][archangel], CR 702.16, 614.9 [rules][cr].

### Backward routing: three more Alice objects

| Role | Recipe |
|---|---|
| jump-scarecrow-warcaller | Joraga Warcaller; add X3; **Elf→Scarecrow** after final copy; exactly **P−5 +1/+1 counters**; green. The edit also changes its own Elf type, which is intentional here. |
| jump-justice | Animated Justice, proposed copiable 1/1 Soldier enchantment creature; add X3; **red→white** text edit; early lifelink counter; green. Alice's upkeep trigger is suppressed before X3 is read. |
| jump-rotlung | Rotlung Reanimator; copiable type **X3 only**, proposed Croaking 1/1 X3 copy; edit **Cleric→Scarecrow**, **Zombie→Sand**, **black→white** after final copy; green. Fresh tokens are white **2/2 Sand**. |

With no active forward Dragon-shadow Sliver, the new flying Scarecrow is the sole blocker of Fiery. Melira still has core Sliver shadow, excluding that nonshadow attacker. Reaper gives +1/+1 and Alice's Warcaller adds P−5, so the token is **P/P**. First-strike Fiery damage is **3Z**, leaving it P−3Z in both power and toughness. The PDF's sentence saying Z damage while deriving P−3Z is inconsistent prose; the live page explicitly says 3Z and the actual Fiery replacement settles the amount. In normal damage the surviving Scarecrow hits indestructible Fiery for P−3Z; edited Justice triggers and deals that amount to Alice. Archangel redirects; Justice's lifelink→Lich→Chains→Wheel produces **P−3Z single-card rotations**, equivalent to moving backward Z instructions from the next instruction. [Backward B.24][paper], [live backward][live], [Fiery][fiery], [Justice][justice], [Rotlung][rotlung].

For **3Z≥P**, the token dies to nonpositive toughness after first-strike damage; it does not survive to deal normal damage. Rotlung creates the white Sand replacement so next-turn Vaevictis still has exactly one legal target. The manuscript says “exceeds P”; equality also kills the P/P token. Wheel's text is **card**, so it does not replace this token's death; the token reaches the graveyard, triggers Rotlung, then ceases to exist. Zero rotations gives fallthrough modulo the physical cyclic program. The author ISA wraps arbitrary backward distances without this clamp, another physical/ISA discrepancy: **reject computed backward 3r0>P** in this backend until canonical semantics are chosen; ordinary in-range immediate branches are safe. P is the first multiple of physical program card length ≥6. [Rotlung][rotlung], [Wheel][wheel], [author ISA][isa], CR 111.6, 700.4, 704.5f [rules][cr].

### Conditional F/NF labels are physically inverted

The shared conditional object is **Bob Meishin**, animated copiable 0/3 enchantment creature, add X3 and YA, green, early reach. It is active in Alice combat three only after Y is read and loses its −hand-size power effect at Bob's untap when YA returns. Its static hand count is zero/one and is actual flag observation. An optional conditional skip means no Bob combat, no Spear token or rotation, and ordinary fallthrough. [Meishin][meishin], [Angel][blinding], [author conditional descriptions][paper].

| Author recipe | Flag 0 | Flag 1 | Actual jump behavior |
|---|---|---|---|
| 1/1 Angel labeled NF at Y2/Y3 | Power1, Tetsuko unblocked, hits Bob, skips combat | Power0, no damage/skip | **Jump if true** |
| 2/2 Angel, protection black, labeled F at Y4/Y5 | Meishin uniquely blocks it, no skip | Power1/T2, Tetsuko unblocked, hits Bob, skips combat | **Jump if false** |

Keep ISA `Y4/Y5 = true` and apply **conditional-gates-swap-v1**: 1/1 Angels belong at Y4/Y5; 2/2/protection-black Angels at Y2/Y3. Forward-only Dragon-shadow sources map correspondingly: corrected true-forward at Y4, corrected false-forward at Y2; unconditional Y0 remains Y0. This is a targeted recipe repair, not a global Yn rename or author correction. For the minimal loop instantiate just one **Alice 1/1 Blinding Angel X3,Y4**, Bob Meishin, and the X3,Y4 Dragon-shadow Sliver: **three conditional objects**. For true-backward add a 1/1 Angel X3,Y5, with no associated forward-shadow Sliver. A 1/1 Angel is always Tetsuko-unblockable when attacking, including at zero power; CR 120.8 ensures zero damage gives no skip trigger. The 2/2 variant additionally needs protection black to exclude black flying Fiery/Chains; Meishin is green and can block it. [Header/interpreter semantics][isa], [contradictory physical text][live], [Blinding Angel][blinding], [Tetsuko][tetsuko], [rules][cr].

Immediate minimal-loop jump inventory is **10 shared + 3 backward + 3 true-forward =16 battlefield objects**, plus reused/new Kangees for distances. Support for unconditional-forward Y0 and true-backward Y5 adds one Sliver and one Angel respectively. A full Y0/Y1/Y4/Y5 immediate profile is therefore 18 objects, excluding Kangees/flag/common scaffold. Count attachments as actual objects.

### Computed Z=0: optional extra Akron

Bob Akron Legionnaire: copiable 0/1, retain Giant/Soldier or an explicitly declared copied type bundle; add X3, Z0, Dragon, Elf; blue+black; early flying/indestructible/first strike. Its attack restriction forbids Fiery (nonartifact) but permits itself and artifact Chains. It receives active r0's Elf bonus, giving power r0; active Fiery still triples that damage. It is Dragon for forward shadow routing, so the same blocker/lifelink machinery applies. Its selected Z0 source gate remains phased throughout Bob's turn, and r0 is active. Kraken/Stormtide are nonartifact and are also prevented from attacking while Akron is active; that does not affect library rotation. Reject unsupported computed distances/host bounds. [B.23][paper], [Akron][akron].

### Required jump setup closures

Melira cannot normally receive any counters. Install its X3/Sun land suppression **before** granting its reach/indestructible/lifelink counters. Those counters consequently have late timestamps and can survive on its inactive land body. Its Beast type is essential: it excludes it as a blocker of green Sub1/FIsZero bodies and red NumBuild r0, while blue forward Fiery permits its block. Do not retime Sun later merely to hide this setup requirement. Lich copies have the enter life-loss effect: use the author-noted Platinum Angel precaution and restore Alice life to 1 while Lich is inactive before helper cleanup. Explicitly animate Justice, Lich, Mists and Meishin; their source recipe's shorthand does not supply literal token types/sizes. Reconfigure/equip/enchant every attachment before activation/target locks; spell-copy final edits after final copies. [Keepers][keepers], [Lich][lich], [paper setup note B.22][paper], [rules][cr].

Each drawing/milling chain requires Alice's hand/graveyard empty and the program entirely in her library after Reaper. Log actual card identities in order at each one-card move. Reject bulk milling, ambiguous ordering, extra damage redirection/prevention, an unexpected combat, no legal next-turn token target, or a body that survives/deals damage contrary to the declared profile. A singleton replacement chain is not verified merely because the eventual decoded program counter matches.

## FIsZero: exact family and reasons to defer it

The author's FLess/FIsZero family OR-combines repeated comparisons. FIsZero does **not** unconditionally overwrite a preexisting combining true flag. Bob's Tamiyo control state, not a backend boolean, preserves this OR. The author JS's `setFlag` uses OR while combining and any jump clears the combining mode. [Full B.33/B.34][paper], [live flag-combining][live], [ISA][isa].

| Role | Exact source recipe / preparation choice |
|---|---|
| Alice Mists | Animated Necrogen Mists; X11; green |
| Bob Halfdane | Copiable 1/1; add X11, Reflection, Yeti; green; early flying; exactly one **+1/+0 counter** |
| Bob Okk | Copiable 1/1 **Elf**; X11; green; early flying; lasting mandatory draw-on-combat-damage-to-player ability; zero extra Spirit counters |
| Bob Bishop | Copiable 1/1 **X11 only**; green; text **Vampire→Fractal** after final copy; give early flying as a declared missing-attack repair |
| Alice Tamiyo | Artifact creature **Fractal**, no planeswalker type; proposed copiable 1/1 body; green; retained discard/sacrifice-prevention static; artifact/creature activations locked |
| Bob Willbreaker | Add X11; green; add **Mountain at T4 after Life** using edited Sealock Monster; no flying/reach |
| Alice Shimmer | X3; choose Mountain; **source timestamp after Sun**, declared late grant repair |
| Bob Riders | Copiable type X11 only, proposed 1/1 Croaking token; fresh Yeti; Human→Rabbit after copy; early protection effect |

These eight objects suffice for a comparison+branch-only subset. Full flag readers X2,Y2 and X2,Y3 additionally need their own Mountain Shimmers, totaling ten. There is no Dinosaur word in Bishop's actual card: its attacked target is **Vampire**, and the correct edit is Vampire→Fractal. A proposed Tamiyo route equips an original with Luxior and donates that creature to Bob; Alice's Imposter Mech can then enter copying it. The Mech exception removes planeswalker type and makes its copiable card types artifact Vehicle, retaining Tamiyo's abilities. Urza, edited Soldier→Fractal before activation, then copies this artifact as a copiable 1/1 artifact creature Vehicle Fractal. This role needs no Vizier/no-mana-cost exception. The original, Luxior, Mech and Urza are setup helpers, all requiring cleanup; this route has not been replayed. Choose a zero-power opposing disposable such as Ornithopter for Bishop's final-copy ETB exile, so its attack pump has an explicit zero amount. [Luxior][luxior], [Imposter Mech][mech], [Urza][urza], [Bishop][bishop], [Tamiyo][tamiyo], [Okk][okk], [Halfdane][halfdane], [Willbreaker][willbreaker], [Shimmer][shimmer], [Riders][riders].

**Physical behavior intended:** extra early Rider makes Halfdane's target list empty for Y=Z; its base remains1 and +1/+0 counter makes actual power2. Okk has power V+1 from its own base1 plus rZ's Elf boost, and attacks iff another co-attacker has greater power. With ONLY Halfdane power2 and Bishop power1, this is iff V=0. Okk hits Alice and draws the flag. Bishop attacks and targets the sole Fractal Tamiyo; Bob Willbreaker triggers and takes Tamiyo before Bishop's pump resolves. On later Bob upkeeps Bob's Tamiyo prevents Alice's Mists from making him discard: earlier true OR current zero-test. On any X3 branch, active late Shimmer grants land Willbreaker phasing, it phases at Bob untap, and its “for as long as” control duration ends (CR 702.26f), returning Tamiyo to Alice even if the branch will not be taken. [Cards above][willbreaker], [FIsZero narrative][live], [rules][cr].

**Additional defects to repair before implementation:**

- **Ordinary 8/8 Stormtide is a co-attacker.** Its native islandwalk forces its attack; Okk can attack whenever V+1<8, falsely setting zero for V=1..6. Proposed FIsZero-only port: use a copiable **0/1 Stormtide** (original 8/8 → Belbe X8 gives0/16 → Drana X14 gives0/2 → Saw gives0/1), keeping native islandwalk/environment abilities. Its power0 cannot enable Okk. Check every other possible co-attacker, not just Halfdane/Bishop. This Stormtide resize is unnecessary for the Sub1-based first loop.
- **Bishop lacks flying/islandwalk in both supplied inventories.** Stormtide therefore prevents its claimed attack and the Tamiyo control trigger. An early flying counter is a concrete proposed repair; test sole Fractal target and ETB linked exile.
- **Default-T1 Shimmer phasing is removed by Sun from land Willbreaker.** Active Shimmer's source timestamp must be **after Sun** to grant surviving phasing; verify simultaneous Bob untap and termination of the control effect, not exile/ETB emulation.
- **Persistent comparison state creates broader semantics.** Mixing FIsZero with Sub1 requires physical OR-combining behavior until a flag-read/branch reset; rejecting that combination is safer than clearing the flag in Rust. The initial sum backend omits this entire family.

[Okk][okk], [Stormtide][storm], [Bishop][bishop], [Shimmer][shimmer], [Sun][sun], [rules][cr].

## Implementation gates and remaining uncertainties

1. Validate the nine-object Sub1 family at V=0/1/3, both starting flag states, exact mana cleanup, no Halfdane target, and single Wrathful target. Prove a complete blocker set with all constants/jump bodies present.
2. Validate Reaper's unique program target, Alice-controlled Spear token ownership, native islandwalk, corrected black-only Archangel damage, lifelink and exactly three one-card rotations for immediate1. Reject unresolved replacement/prevention alternatives.
3. Validate backward immediate4 on P21: Scarecrow actual21/21, twelve first-strike wither damage, nine normal damage/Justice/rotations, unique next-turn token sacrifice. Also verify equality/overlong token-death fallback separately, rather than assuming unlimited ISA wrap matches physical behavior.
4. Validate corrected Y4/Y5 Angel truth tables at both flag states with the full board, including Meishin removal at Bob untap, then run the seven-instruction sum program. Do not advertise general `while` or `if zero` physical support from that sum-specific lowering.
5. Defer FIsZero until its resize/flying/timestamp repairs and control-duration state pass. Any choice group lacking a unique option or a proof that all outcomes converge remains an explicit backend rejection.

No loop fixture or setup sequence was executed by this research. Proposed fixes are independently derived from the primary cards/rules; the manuscript/live inconsistencies are not claimed to be author-confirmed errata.

[paper]: https://www.toothycat.net/~hologram/Magic/Magic_Microcontroller_full.pdf
[live]: https://cyh31.neocities.org/amepscimtg/explanation
[explainjs]: https://cyh31.neocities.org/amepscimtg/explanation.js
[isa]: https://www.toothycat.net/~hologram/Magic/MTGProgSimulatorText.js
[cr]: https://media.wizards.com/2026/downloads/MagicCompRules%2020260925.txt
[luxior]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/l/luxior_giadas_gift.txt
[mech]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/i/imposter_mech.txt
[urza]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/u/urza_prince_of_kroog.txt
[tomorrow]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/t/tomorrow_azamis_familiar.txt
[wheel]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/w/wheel_of_sun_and_moon.txt
[mining]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/aggressive_mining.txt
[halfdane]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/h/halfdane.txt
[tanuki]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/t/tanuki_transplanter.txt
[furtive]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/f/furtive_homunculus.txt
[omnath]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/o/omnath_locus_of_mana.txt
[wrathful]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/w/wrathful_red_dragon.txt
[brontodon]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/belligerent_brontodon.txt
[mists]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/n/necrogen_mists.txt
[riders]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/riders_of_gavony.txt
[oil]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/f/fishliver_oil.txt
[spirit]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/ascendant_spirit.txt
[defense]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/resourceful_defense.txt
[fiery]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/f/fiery_emancipation.txt
[chains]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/c/chains_of_mephistopheles.txt
[spear]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/moonsilver_spear.txt
[reaper]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/reaper_king.txt
[centaur]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/spellbane_centaur.txt
[keepers]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/meliras_keepers.txt
[shadows]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/spiteful_shadows.txt
[archangel]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/e/empyrial_archangel.txt
[lich]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/l/lich.txt
[kangee]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/k/kangee_aerie_keeper.txt
[shadow]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/shadow_sliver.txt
[gorgon]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/masked_gorgon.txt
[justice]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/j/justice.txt
[rotlung]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/r/rotlung_reanimator.txt
[meishin]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/m/meishin_the_mind_cage.txt
[blinding]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/blinding_angel.txt
[tetsuko]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/t/tetsuko_umezawa_fugitive.txt
[akron]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/a/akron_legionnaire.txt
[bishop]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/bishop_of_binding.txt
[tamiyo]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/t/tamiyo_collector_of_tales.txt
[okk]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/o/okk.txt
[willbreaker]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/w/willbreaker.txt
[shimmer]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/shimmer.txt
[storm]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/s/stormtide_leviathan.txt
[sun]: https://github.com/Card-Forge/forge/blob/5222f05e569da8e33c85858e6fe1698901548d64/forge-gui/res/cardsfolder/b/blood_sun.txt
