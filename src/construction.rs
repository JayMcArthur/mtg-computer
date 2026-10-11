use crate::SYMBOL_CARDS;
use crate::compiler::{Program, compile};
use crate::machine::Machine;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod constants;
mod loops;

pub const FORGE_REVISION: &str = "5222f05e569da8e33c85858e6fe1698901548d64";
pub const RULES_SHA256: &str = "8d860e451f20f38865b725b42d82feb714c725373dd8f3b32b8652b3eeb070ca";

#[derive(Deserialize)]
struct Selectors {
    #[serde(rename = "X")]
    x: [String; 12],
    #[serde(rename = "XA")]
    xa: String,
    #[serde(rename = "Y")]
    y: [String; 12],
    #[serde(rename = "YA")]
    ya: String,
    #[serde(rename = "Z")]
    z: [String; 12],
    #[serde(rename = "ZA")]
    za: String,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub enum Player {
    Alice,
    Bob,
}

#[derive(Debug, Serialize)]
pub struct PreparedBoard {
    pub schema: u32,
    pub profile: &'static str,
    pub evidence: &'static str,
    pub rules_sha256: &'static str,
    pub engine_revision: &'static str,
    pub life: [u32; 2],
    pub cards: Vec<PreparedCard>,
    pub library: Vec<String>,
    pub program_permanent: String,
    pub instructions: Vec<[u8; 3]>,
    pub source: Vec<String>,
    pub variables: BTreeMap<String, usize>,
    pub source_lines: Vec<usize>,
    pub initial_registers: [String; 12],
    pub expected_registers: [String; 12],
    pub expected_flag: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PreparedCard {
    pub id: String,
    pub name: String,
    pub owner: Player,
    pub controller: Player,
    pub zone: &'static str,
    pub group: &'static str,
    pub recipe: &'static str,
    pub timestamp: u64,
    pub token: bool,
    pub copiable: Copiable,
    pub modifications: Vec<Modification>,
    pub counters: BTreeMap<String, u32>,
    pub counter_timestamps: BTreeMap<String, u64>,
    pub chosen_types: Vec<String>,
    pub chosen_colors: Vec<String>,
    pub attached_to: Option<String>,
    pub enchanting_player: Option<Player>,
    pub imprint: Option<String>,
    pub tapped: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct Copiable {
    pub blank_rules: bool,
    pub card_types: Option<Vec<String>>,
    pub creature_types: Option<Vec<String>>,
    pub base_pt: Option<[i32; 2]>,
    pub mana_cost: Option<String>,
    pub colors: Option<Vec<String>>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum Modification {
    TypeWord {
        from: String,
        to: String,
        timestamp: u64,
    },
    ColorWord {
        from: String,
        to: String,
        timestamp: u64,
    },
    AddTypes {
        types: Vec<String>,
        timestamp: u64,
    },
    Color {
        colors: Vec<String>,
        timestamp: u64,
    },
    BasePt {
        power: i32,
        toughness: i32,
        timestamp: u64,
    },
    Keyword {
        keyword: String,
        timestamp: u64,
    },
    RemoveKeyword {
        keyword: String,
        timestamp: u64,
    },
    CombatDraw {
        timestamp: u64,
    },
}

impl PreparedCard {
    fn new(
        id: String,
        name: &str,
        controller: Player,
        group: &'static str,
        recipe: &'static str,
        band: u64,
        index: usize,
    ) -> Self {
        let timestamp = band * 10_000 + index as u64 * 10;
        Self {
            id,
            name: name.into(),
            owner: Player::Alice,
            controller,
            zone: "Battlefield",
            group,
            recipe,
            timestamp,
            token: false,
            copiable: Copiable::default(),
            modifications: vec![Modification::Color {
                colors: vec!["Green".into()],
                timestamp: 10_000 + index as u64 * 10 + 1,
            }],
            counters: BTreeMap::new(),
            counter_timestamps: BTreeMap::new(),
            chosen_types: Vec::new(),
            chosen_colors: Vec::new(),
            attached_to: None,
            enchanting_player: None,
            imprint: None,
            tapped: false,
        }
    }

    fn edit_type(&mut self, from: &str, to: &str) {
        self.modifications.push(Modification::TypeWord {
            from: from.into(),
            to: to.into(),
            timestamp: self.timestamp + self.modifications.len() as u64 + 1,
        });
    }

    fn add_types(&mut self, types: &[&str]) {
        self.modifications.push(Modification::AddTypes {
            types: types.iter().map(|s| (*s).into()).collect(),
            timestamp: self.timestamp + self.modifications.len() as u64 + 1,
        });
    }

    fn keyword(&mut self, keyword: &str) {
        self.modifications.push(Modification::Keyword {
            keyword: keyword.into(),
            timestamp: self.timestamp + self.modifications.len() as u64 + 1,
        });
    }

    fn keyword_counter(&mut self, keyword: &str, timestamp: u64) {
        self.counters.insert(keyword.into(), 1);
        self.counter_timestamps.insert(keyword.into(), timestamp);
    }
}

/// A complete named-role prepared fixture, not a legal-setup certificate.
pub fn add_halt_fixture() -> PreparedBoard {
    prepare_program(
        &compile(include_str!("../examples/add-halt.mtg")).expect("bundled program must compile"),
    )
    .expect("bundled program must fit the prepared profile")
}

/// Only validated gadget families can be included in this prepared profile.
pub fn prepare_program(program: &Program) -> Result<PreparedBoard, String> {
    let mut reference = Machine::default();
    reference.registers = program.initial_registers.clone();
    let bound = BigUint::from(i32::MAX as u32 - 2);
    if reference.registers.iter().any(|n| n > &bound) {
        return Err("prepared register exceeds Forge's signed counter/power bound".into());
    }
    for (pc, instruction) in program.instructions.iter().enumerate() {
        let [x, y, z] = instruction.symbols;
        if !((matches!(x, 0 | 5 | 10)
            || x == 1 && y == z
            || x == 3 && matches!(y, 0 | 1 | 4 | 5) && z != 0)
            && pc + 1 < program.instructions.len()
            || instruction.symbols == [4, 0, 0] && pc + 1 == program.instructions.len())
        {
            return Err(format!(
                "line {}: prepared backend supports Add, Set, NumBuild, checked decrement, immediate jumps and final HaltD; {} needs an unverified gadget",
                instruction.source_line,
                crate::machine::describe(instruction.symbols)
            ));
        }
    }
    for _ in 0..10_000 {
        let frame = reference.step(&program.instructions)?;
        if reference.registers.iter().any(|n| n > &bound) {
            return Err(format!(
                "line {}: result exceeds Forge's signed counter/power bound",
                frame.source_line
            ));
        }
        if reference.halted {
            break;
        }
    }
    if !reference.halted {
        return Err("prepared preflight reached 10000 ISA instructions without halting".into());
    }
    let mut instructions: Vec<_> = program.instructions.iter().map(|i| i.symbols).collect();
    let mut source_lines: Vec<_> = program.instructions.iter().map(|i| i.source_line).collect();
    // The physical cyclic queue needs at least six cards.
    if instructions.len() == 1 {
        instructions.push(instructions[0]);
        source_lines.push(source_lines[0]);
    }
    let selectors: Selectors =
        serde_json::from_str(include_str!("../data/construction/types.json"))
            .expect("bundled construction type mapping must be valid");
    let loops = instructions.iter().any(|i| matches!(i[0], 1 | 3));
    let constants = loops || instructions.iter().any(|i| matches!(i[0], 5 | 10));
    let mut board = PreparedBoard {
        schema: 3,
        profile: if loops {
            "loop-prepared-v1"
        } else if constants {
            "constants-prepared-v2"
        } else {
            "add-halt-prepared-v2"
        },
        evidence: "prepared-manifest",
        rules_sha256: RULES_SHA256,
        engine_revision: FORGE_REVISION,
        life: [1, 1],
        cards: Vec::new(),
        library: Vec::new(),
        program_permanent: format!("program-{}", instructions.len() * 3 - 1),
        instructions,
        source: program.source.clone(),
        variables: program.variables.clone(),
        source_lines,
        initial_registers: program
            .initial_registers
            .each_ref()
            .map(ToString::to_string),
        expected_registers: reference.registers.each_ref().map(ToString::to_string),
        expected_flag: loops.then_some(reference.flag),
    };
    globals(&mut board, &selectors);
    reader(&mut board, &selectors);
    registers(&mut board, &selectors, &program.initial_registers);
    arithmetic(&mut board, &selectors);
    if constants {
        constants::extend(&mut board, &selectors);
    }
    if loops {
        loops::extend(&mut board, &selectors);
    }
    program_cards(&mut board);
    Ok(board)
}

impl PreparedBoard {
    fn card(
        &mut self,
        id: &str,
        name: &str,
        controller: Player,
        group: &'static str,
        recipe: &'static str,
        band: u64,
    ) -> &mut PreparedCard {
        let card = PreparedCard::new(
            id.into(),
            name,
            controller,
            group,
            recipe,
            band,
            self.cards.len(),
        );
        self.cards.push(card);
        self.cards.last_mut().unwrap()
    }
}

fn globals(board: &mut PreparedBoard, s: &Selectors) {
    use Player::{Alice, Bob};
    for (id, name, band) in [
        ("grand-melee", "Grand Melee", 1),
        ("life-and-limb", "Life and Limb", 3),
        ("blood-sun", "Blood Sun", 2),
        ("bludgeon", "Bludgeon Brawl", 3),
        ("shadow", "Shadow Sliver", 1),
        ("tetsuko", "Tetsuko Umezawa, Fugitive", 1),
        ("guardian", "Spectral Guardian", 3),
        ("root-maze", "Root Maze", 1),
        ("choke", "Choke", 1),
        ("stony-silence", "Stony Silence", 1),
        ("cursed-totem", "Cursed Totem", 1),
        ("mirror", "Mirror Gallery", 1),
    ] {
        board.card(id, name, Alice, "environment", "4.2-4.11", band);
    }
    board
        .card(
            "stormtide",
            "Stormtide Leviathan",
            Bob,
            "environment",
            "4.2",
            1,
        )
        .timestamp = 10_000;
    board.card(
        "preserver",
        "Tajuru Preserver",
        Bob,
        "environment",
        "4.3",
        1,
    );
    for player in [Alice, Bob] {
        for (id, name, band) in [
            ("ivory-mask", "Ivory Mask", 3),
            ("recycle", "Recycle", 1),
            ("worship", "Worship", 1),
            ("mentor", "Corrosive Mentor", 1),
            ("grove", "Sterling Grove", 3),
        ] {
            board.card(
                &format!("{player:?}-{id}"),
                name,
                player,
                "environment",
                "4.6-4.11",
                band,
            );
        }
    }
    let grove = board
        .cards
        .iter_mut()
        .find(|c| c.id == "Alice-grove")
        .unwrap();
    grove.add_types(&["Artifact"]);
    let grove = board
        .cards
        .iter_mut()
        .find(|c| c.id == "Bob-grove")
        .unwrap();
    grove.keyword("Protection from blue");
    let wheel = board.card(
        "program-wheel",
        "Wheel of Sun and Moon",
        Alice,
        "environment",
        "4.3",
        1,
    );
    wheel.enchanting_player = Some(Alice);
    board
        .card(
            "grove-bonds",
            "Suppression Bonds",
            Alice,
            "environment",
            "4.11",
            1,
        )
        .attached_to = Some("Bob-grove".into());
    for (id, color, protection) in [
        ("gorgon-reflection", "Blue", "Reflection"),
        ("gorgon-beast", "Red", "Beast"),
    ] {
        let card = board.card(id, "Masked Gorgon", Alice, "environment", "4.6", 3);
        card.edit_type("Gorgon", protection);
        card.modifications.push(Modification::ColorWord {
            from: "White".into(),
            to: color.into(),
            timestamp: card.timestamp + 5,
        });
    }
    let terrain = board.card(
        "terrain",
        "Illusionary Terrain",
        Alice,
        "environment",
        "4.7",
        1,
    );
    terrain.token = true;
    terrain.copiable.card_types = Some(vec!["Enchantment".into(), "Creature".into()]);
    terrain.copiable.creature_types = Some(vec![s.za.clone()]);
    terrain.copiable.base_pt = Some([1, 1]);
    terrain.chosen_types = vec!["Island".into(), "Island".into()];
}

fn reader(board: &mut PreparedBoard, s: &Selectors) {
    use Player::{Alice, Bob};
    for (stage, types, wildcard, previous) in [
        ("X", &s.x, &s.xa, None),
        ("Y", &s.y, &s.ya, Some(&s.xa)),
        ("Z", &s.z, &s.za, Some(&s.ya)),
    ] {
        for (n, name) in SYMBOL_CARDS.iter().enumerate() {
            let id = format!("gate-{stage}-{n}");
            let gate = board.card(&id, "Dralnu's Crusade", Alice, "reader", "4.4-4.5", 2);
            gate.token = true;
            gate.copiable.card_types = Some(vec!["Enchantment".into(), "Creature".into()]);
            gate.copiable.creature_types = Some(vec!["Sliver".into()]);
            gate.copiable.base_pt = Some([2, 1]);
            gate.copiable.keywords.push("Flying".into());
            gate.edit_type("Goblin", &types[n]);
            gate.edit_type("Zombie", "Saproling");
            if let Some(previous) = previous {
                gate.add_types(&[previous]);
            }
            let imprint_id = format!("imprint-{stage}-{n}");
            let scythe = board.card(
                &format!("scythe-{stage}-{n}"),
                "Strata Scythe",
                Alice,
                "reader",
                "4.4",
                1,
            );
            scythe.token = true;
            scythe.attached_to = Some(id);
            scythe.imprint = Some(imprint_id.clone());
            let imprint = board.card(&imprint_id, name, Alice, "imprints", "4.4", 1);
            imprint.zone = "Exile";
            imprint.modifications.clear();
        }
        let fighter_id = format!("fighter-{stage}");
        let fighter = board.card(&fighter_id, "Dream Fighter", Bob, "reader", "4.4-4.5", 1);
        fighter.add_types(&["Sliver"]);
        fighter.counters.insert("Reach".into(), 1);
        fighter
            .counter_timestamps
            .insert("Reach".into(), fighter.timestamp + 8);
        if let Some(previous) = previous {
            fighter.add_types(&[previous]);
        }
        let gate = board.card(
            &format!("wildcard-{stage}"),
            "Dralnu's Crusade",
            Alice,
            "reader",
            "4.4-4.5",
            2,
        );
        gate.add_types(&["Artifact"]);
        gate.edit_type("Goblin", wildcard);
        gate.edit_type("Zombie", "Saproling");
        gate.attached_to = Some(fighter_id);
        let vaevictis = board.card(
            &format!("vaevictis-{stage}"),
            "Vaevictis Asmadi, the Dire",
            Alice,
            "reader",
            "4.3-4.5",
            1,
        );
        vaevictis.token = true;
        vaevictis.copiable.creature_types =
            Some(vec!["Sliver".into(), "Beast".into(), "Reflection".into()]);
        vaevictis.copiable.base_pt = Some([1, 1]);
        vaevictis.copiable.colors = Some(vec!["Green".into()]);
        if let Some(previous) = previous {
            vaevictis.add_types(&[previous]);
        }
    }
    let thirster = board.card("bloodthirster", "Bloodthirster", Alice, "reader", "4.5", 1);
    thirster.token = true;
    thirster.copiable.creature_types =
        Some(vec!["Sliver".into(), "Beast".into(), "Reflection".into()]);
    thirster.copiable.base_pt = Some([1, 1]);
    thirster.copiable.colors = Some(vec!["Green".into()]);
    thirster.counters.insert("Double Strike".into(), 1);
    thirster
        .counter_timestamps
        .insert("Double Strike".into(), thirster.timestamp + 8);
}

fn registers(board: &mut PreparedBoard, s: &Selectors, initial: &[BigUint; 12]) {
    for (n, ty) in s.z.iter().enumerate() {
        let register = board.card(
            &format!("r{n}"),
            "Joraga Warcaller",
            Player::Bob,
            "registers",
            "4.8",
            1,
        );
        register.token = true;
        register.copiable.creature_types = Some(vec![ty.clone()]);
        register.add_types(&["Rabbit"]);
        if n == 0 {
            register.add_types(&["Rhino"]);
        }
        for (i, keyword) in ["Indestructible", "Vigilance"].into_iter().enumerate() {
            register.counters.insert(keyword.into(), 1);
            register
                .counter_timestamps
                .insert(keyword.into(), register.timestamp + 7 + i as u64);
        }
        register.modifications.push(Modification::BasePt {
            power: 2,
            toughness: 2,
            timestamp: register.timestamp + 6,
        });
        register.modifications.push(Modification::Color {
            colors: vec!["Red".into()],
            timestamp: 40_000 + n as u64,
        });
        register.counters.insert("M1M0".into(), 2);
        register.counters.insert(
            "P1P1".into(),
            initial[n]
                .to_string()
                .parse()
                .expect("validated register bound"),
        );
        let rider = board.card(
            &format!("target-rider-{n}"),
            "Riders of Gavony",
            Player::Bob,
            "registers",
            "4.8",
            3,
        );
        rider.token = true;
        rider.copiable.card_types = Some(vec!["Artifact".into(), "Vehicle".into()]);
        rider.copiable.creature_types = Some(Vec::new());
        rider.copiable.mana_cost = Some("no cost".into());
        rider.copiable.colors = Some(vec!["White".into()]);
        rider.copiable.keywords.push("Crew:3".into());
        rider.edit_type("Human", ty);
        rider.chosen_types.push("Yeti".into());
        rider.attached_to = Some(format!("gate-Y-{n}"));
    }
    let rider = board.card(
        "source-rider",
        "Riders of Gavony",
        Player::Bob,
        "registers",
        "4.8",
        3,
    );
    // A Frog copy avoids turning the Rider itself into an intrinsic Saproling
    // when Human is rewritten in its rules text.
    rider.token = true;
    rider.copiable.creature_types = Some(vec!["Frog".into()]);
    rider.copiable.base_pt = Some([1, 1]);
    rider.copiable.colors = Some(vec!["Green".into()]);
    rider.edit_type("Human", "Saproling");
    rider.chosen_types.push("Zubera".into());
}

fn arithmetic(board: &mut PreparedBoard, s: &Selectors) {
    let add = board.card(
        "add",
        "Halana and Alena, Partners",
        Player::Bob,
        "arithmetic",
        "B.1",
        1,
    );
    add.token = true;
    add.copiable.base_pt = Some([0, 2]);
    add.add_types(&[&s.x[0], "Elf", "Reflection", "Yeti"]);
    let halt = board.card(
        "halt",
        "Celestial Convergence",
        Player::Bob,
        "arithmetic",
        "B.16",
        1,
    );
    halt.token = true;
    halt.copiable.card_types = Some(vec!["Enchantment".into(), "Creature".into()]);
    halt.copiable.creature_types = Some(vec![s.x[4].clone(), s.y[0].clone()]);
    halt.copiable.base_pt = Some([1, 1]);
    halt.counters.insert("OMEN".into(), 1);
}

fn program_cards(board: &mut PreparedBoard) {
    let symbols: Vec<_> = board.instructions.iter().flatten().copied().collect();
    let last = symbols.len() - 1;
    for (i, symbol) in symbols.into_iter().enumerate() {
        let id = format!("program-{i}");
        let card = board.card(
            &id,
            SYMBOL_CARDS[symbol as usize],
            Player::Alice,
            "program",
            "4.3",
            1,
        );
        card.modifications.clear();
        if i == last {
            card.tapped = true;
        } else {
            card.zone = "Library";
            board.library.push(id);
        }
    }
}
