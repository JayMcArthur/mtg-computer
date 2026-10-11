use mtg_computer::construction::{Modification, Player, add_halt_fixture};
use mtg_computer::{compiler::compile, construction::prepare_program};
use std::collections::BTreeSet;

#[test]
fn complete_fixture_has_distinct_cards_and_valid_cross_controller_attachments() {
    let board = add_halt_fixture();
    let ids: BTreeSet<_> = board.cards.iter().map(|c| c.id.as_str()).collect();
    assert_eq!(ids.len(), board.cards.len());
    let card = |id: &str| board.cards.iter().find(|c| c.id == id).unwrap();
    for object in &board.cards {
        let edits: Vec<_> = object
            .modifications
            .iter()
            .filter_map(|m| match m {
                Modification::TypeWord { timestamp, .. } => Some(*timestamp),
                _ => None,
            })
            .collect();
        assert_eq!(
            edits.iter().copied().collect::<BTreeSet<_>>().len(),
            edits.len(),
            "distinct text edits must not replace one another in the engine"
        );
        for reference in [&object.attached_to, &object.imprint].into_iter().flatten() {
            assert!(ids.contains(reference.as_str()));
        }
    }
    assert_eq!(
        board
            .cards
            .iter()
            .filter(|c| c.id.starts_with("gate-"))
            .count(),
        36
    );
    assert_eq!(
        board
            .cards
            .iter()
            .filter(|c| c.name == "Strata Scythe")
            .count(),
        36
    );
    assert_eq!(board.cards.iter().filter(|c| c.zone == "Exile").count(), 36);
    for stage in ["X", "Y", "Z"] {
        let wildcard = card(&format!("wildcard-{stage}"));
        assert_eq!(wildcard.controller, Player::Alice);
        let fighter = card(wildcard.attached_to.as_ref().unwrap());
        assert_eq!(fighter.controller, Player::Bob);
    }
    for n in 0..12 {
        let rider = card(&format!("target-rider-{n}"));
        assert_eq!(rider.controller, Player::Bob);
        assert_eq!(
            rider.attached_to.as_deref(),
            Some(format!("gate-Y-{n}").as_str())
        );
        assert_eq!(rider.chosen_types, ["Yeti"]);
    }
    assert_eq!(card("r1").counters["P1P1"], 2);
    assert_eq!(card("r2").counters["P1P1"], 3);
    assert_eq!(card("r0").counters["M1M0"], 2);
    assert_eq!(card("halt").counters["OMEN"], 1);
    assert!(card("stormtide").timestamp < card("terrain").timestamp);
    assert!(card("blood-sun").timestamp < card("life-and-limb").timestamp);
    let x2 = card("gate-X-2");
    assert!(x2.modifications.iter().any(|m| matches!(m,
        Modification::TypeWord { from, to, .. } if from == "Goblin" && to == "Capybara"
    )));
}

#[test]
fn initial_program_rotation_reads_add_then_halt() {
    let board = add_halt_fixture();
    let name = |id: &str| {
        board
            .cards
            .iter()
            .find(|c| c.id == id)
            .unwrap()
            .name
            .as_str()
    };
    assert_eq!(name(&board.program_permanent), "Plains");
    let queue: Vec<_> = board.library.iter().map(|id| name(id)).collect();
    assert_eq!(queue, ["Plains", "Island", "Swamp", "Forest", "Plains"]);
    assert_eq!(board.instructions, [[0, 1, 2], [4, 0, 0]]);
    assert_eq!(board.evidence, "prepared-manifest");
}

#[test]
fn readable_program_drives_inputs_queue_and_expected_result() {
    let program = compile("input a = 7\ninput b = 4\na += b\na += b\nhalt").unwrap();
    let board = prepare_program(&program).unwrap();
    assert_eq!(board.instructions, [[0, 1, 2], [0, 1, 2], [4, 0, 0]]);
    assert_eq!(board.source_lines, [3, 4, 5]);
    assert_eq!(board.library.len(), 8);
    assert_eq!(board.program_permanent, "program-8");
    assert_eq!(board.initial_registers[1], "7");
    assert_eq!(board.expected_registers[1], "15");
    assert_eq!(
        board.cards.iter().find(|c| c.id == "r2").unwrap().counters["P1P1"],
        4
    );
    let halt_only = prepare_program(&compile("halt").unwrap()).unwrap();
    assert_eq!(halt_only.instructions, [[4, 0, 0], [4, 0, 0]]);
    for source in [
        "a = 2\na -= 1\nhalt",
        "input a = 2147483648\nhalt",
        "input a = 2147483644\na += a\nhalt",
        "input a = 1\na += a",
    ] {
        assert!(prepare_program(&compile(source).unwrap()).is_err());
    }
}

#[test]
fn assignments_from_zero_export_the_live_constant_gadgets() {
    let board = prepare_program(&compile("a = 2\nb = 3\na += b\nhalt").unwrap()).unwrap();
    assert_eq!(
        board.instructions,
        [
            [10, 0, 2],
            [5, 1, 0],
            [10, 0, 3],
            [5, 2, 0],
            [0, 1, 2],
            [4, 0, 0]
        ]
    );
    assert_eq!(&board.expected_registers[..3], ["3", "5", "3"]);
    assert!(board.initial_registers.iter().all(|r| r == "0"));
    let card = |id: &str| board.cards.iter().find(|c| c.id == id).unwrap();
    assert_eq!(card("num-myr-0").copiable.base_pt, Some([0, 1]));
    assert_eq!(
        card("num-myr-0").copiable.creature_types.as_ref().unwrap(),
        &["Myr", "Nomad"]
    );
    assert_eq!(card("num-kangee-2").counters["FEATHER"], 2);
    assert_eq!(card("num-kangee-3").counters["FEATHER"], 3);
    assert_eq!(card("num-equity").chosen_colors, ["Red"]);
    assert!(card("set-party-Warrior").copiable.blank_rules);
    assert!(card("r0").counter_timestamps["Indestructible"] > card("blood-sun").timestamp);
    assert_eq!(
        board
            .cards
            .iter()
            .filter(|c| c.name == "Fishliver Oil")
            .count(),
        1
    );
    assert!(board.cards.iter().all(|c| c.id != "num-marauders"));
}
