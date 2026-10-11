use super::constants::{colors, remove_flying};
use super::{Copiable, Modification, Player, PreparedBoard, PreparedCard, Selectors};
use std::collections::BTreeSet;

pub(super) fn extend(board: &mut PreparedBoard, s: &Selectors) {
    flag(board);
    if board.instructions.iter().any(|i| i[0] == 1) {
        subtraction(board, s);
    }
    if board.instructions.iter().any(|i| i[0] == 3) {
        jumps(board, s);
    }
}

fn animate(card: &mut PreparedCard, types: &[&str], pt: [i32; 2]) {
    card.token = true;
    card.copiable.card_types = Some(vec!["Enchantment".into(), "Creature".into()]);
    card.copiable.creature_types = Some(types.iter().map(|s| (*s).into()).collect());
    card.copiable.base_pt = Some(pt);
}

fn flag(board: &mut PreparedBoard) {
    use Player::{Alice, Bob};
    let card = board.card("flag-card", "Plains", Bob, "flag", "4.10 singleton flag", 1);
    card.owner = Bob;
    card.zone = "Library";
    card.modifications.clear();
    board
        .card(
            "flag-wheel",
            "Wheel of Sun and Moon",
            Alice,
            "flag",
            "4.10",
            1,
        )
        .enchanting_player = Some(Bob);
    board.card(
        "flag-tomorrow",
        "Tomorrow, Azami's Familiar",
        Bob,
        "flag",
        "4.10",
        1,
    );
    board
        .card("flag-mining", "Aggressive Mining", Bob, "flag", "4.10", 1)
        .add_types(&["Artifact"]);
}

fn subtraction(board: &mut PreparedBoard, s: &Selectors) {
    use Player::{Alice, Bob};
    let card = board.card(
        "sub-mists",
        "Necrogen Mists",
        Alice,
        "branches",
        "B.4 animated",
        1,
    );
    animate(card, &["Soldier"], [1, 1]);
    card.add_types(&[&s.x[1]]);
    let card = board.card(
        "sub-halfdane",
        "Halfdane",
        Bob,
        "branches",
        "B.4 sized copy",
        1,
    );
    card.token = true;
    card.copiable.base_pt = Some([1, 1]);
    card.add_types(&[&s.x[1], "Reflection", "Yeti"]);
    board
        .card(
            "sub-halfdane-oil",
            "Fishliver Oil",
            Alice,
            "branches",
            "B.4",
            1,
        )
        .attached_to = Some("sub-halfdane".into());
    board
        .card(
            "sub-transplanter",
            "Tanuki Transplanter",
            Alice,
            "branches",
            "B.4 reconfigured",
            1,
        )
        .attached_to = Some("sub-halfdane".into());
    let card = board.card(
        "sub-homunculus",
        "Furtive Homunculus",
        Bob,
        "branches",
        "B.4 Spirit draw grant",
        1,
    );
    card.token = true;
    card.copiable.base_pt = Some([0, 1]);
    card.add_types(&[&s.x[1], "Elf", "Dragon", "Reflection"]);
    card.keyword_counter("Flying", card.timestamp + 8);
    card.modifications.push(Modification::CombatDraw {
        timestamp: card.timestamp + 9,
    });
    colors(card, &["Green", "Black"]);
    let card = board.card(
        "sub-omnath",
        "Omnath, Locus of Mana",
        Alice,
        "branches",
        "B.4 sized copy",
        1,
    );
    card.token = true;
    card.copiable.base_pt = Some([0, 1]);
    card.add_types(&[&s.x[1]]);
    for keyword in ["Reach", "Indestructible"] {
        card.keyword_counter(keyword, card.timestamp + 8);
    }
    let card = board.card(
        "sub-wrathful",
        "Wrathful Red Dragon",
        Bob,
        "branches",
        "B.4",
        1,
    );
    card.add_types(&[&s.x[1], "Reflection", "Zubera"]);
    remove_flying(card);
    board
        .card(
            "sub-brontodon",
            "Belligerent Brontodon",
            Bob,
            "branches",
            "B.4",
            1,
        )
        .add_types(&[&s.x[1]]);
    let card = board.card(
        "sub-rider",
        "Riders of Gavony",
        Bob,
        "branches",
        "B.5 X1 copy",
        1,
    );
    card.token = true;
    card.copiable.creature_types = Some(vec![s.x[1].clone()]);
    card.copiable.base_pt = Some([1, 1]);
    card.chosen_types.push("Yeti".into());
    card.edit_type("Human", "Rabbit");
}

fn jumps(board: &mut PreparedBoard, s: &Selectors) {
    use Player::{Alice, Bob};
    let modes: BTreeSet<_> = board
        .instructions
        .iter()
        .filter(|i| i[0] == 3)
        .map(|i| i[1])
        .collect();
    let distances: BTreeSet<_> = board
        .instructions
        .iter()
        .filter(|i| i[0] == 3)
        .map(|i| i[2])
        .collect();
    for z in distances {
        if board
            .cards
            .iter()
            .any(|c| c.id == format!("num-kangee-{z}"))
        {
            continue;
        }
        let card = board.card(
            &format!("num-kangee-{z}"),
            "Kangee, Aerie Keeper",
            Alice,
            "branches",
            "B.22 immediate distance",
            1,
        );
        card.add_types(&[&s.z[z as usize]]);
        card.edit_type("Bird", "Nomad");
        card.counters.insert("FEATHER".into(), u32::from(z));
        remove_flying(card);
    }
    let card = board.card(
        "jump-emancipation",
        "Fiery Emancipation",
        Bob,
        "branches",
        "B.22 animated",
        1,
    );
    animate(card, &["Dragon", "Nomad"], [0, 1]);
    card.add_types(&[&s.x[3]]);
    colors(card, &["Blue", "Black"]);
    for keyword in ["Flying", "Indestructible", "First strike"] {
        card.keyword_counter(keyword, card.timestamp + 8);
    }
    let card = board.card(
        "jump-chains",
        "Chains of Mephistopheles",
        Bob,
        "branches",
        "B.22 animated",
        1,
    );
    animate(card, &["Sliver"], [0, 1]);
    card.add_types(&[&s.x[3], "Artifact"]);
    colors(card, &["Green", "Black"]);
    board
        .card(
            "jump-chains-oil",
            "Fishliver Oil",
            Alice,
            "branches",
            "B.22 PDF islandwalk",
            1,
        )
        .attached_to = Some("jump-chains".into());
    let card = board.card(
        "jump-spear",
        "Moonsilver Spear",
        Alice,
        "branches",
        "B.22",
        1,
    );
    card.edit_type("Angel", "Scarecrow");
    card.attached_to = Some("jump-chains".into());
    let card = board.card("jump-reaper", "Reaper King", Alice, "branches", "B.22", 1);
    card.add_types(&[&s.x[3], "Reflection", "Beast"]);
    colors(card, &["Green", "Blue", "Red"]);
    let card = board.card(
        "jump-centaur",
        "Spellbane Centaur",
        Alice,
        "branches",
        "B.22",
        1,
    );
    card.add_types(&[&s.x[3]]);
    card.modifications.push(Modification::ColorWord {
        from: "Blue".into(),
        to: "Red".into(),
        timestamp: card.timestamp + 5,
    });
    let card = board.card(
        "jump-keepers",
        "Melira's Keepers",
        Alice,
        "branches",
        "B.22 counters while suppressed",
        1,
    );
    card.token = true;
    card.copiable.base_pt = Some([0, 1]);
    card.add_types(&[&s.x[3], "Sliver", "Beast"]);
    for (offset, keyword) in ["Reach", "Indestructible", "Lifelink"]
        .into_iter()
        .enumerate()
    {
        card.keyword_counter(keyword, 50_010 + offset as u64);
    }
    board
        .card(
            "jump-shadows",
            "Spiteful Shadows",
            Alice,
            "branches",
            "B.22",
            1,
        )
        .attached_to = Some("jump-keepers".into());
    let card = board.card(
        "jump-archangel",
        "Empyrial Archangel",
        Alice,
        "branches",
        "B.22 declared black-only repair",
        1,
    );
    card.add_types(&[&s.x[3], "Dragon"]);
    colors(card, &["Black"]);
    remove_flying(card);
    card.keyword_counter("Indestructible", card.timestamp + 9);
    let card = board.card("jump-lich", "Lich", Alice, "branches", "B.22 animated", 1);
    animate(card, &["Soldier"], [1, 1]);
    card.add_types(&[&s.x[3]]);
    for mode in modes.iter().filter(|mode| matches!(mode, 0 | 4)) {
        let card = board.card(
            &format!("jump-shadow-{mode}"),
            "Shadow Sliver",
            Alice,
            "branches",
            "conditional-gates-swap-v1",
            1,
        );
        card.add_types(&[&s.x[3], &s.y[*mode as usize]]);
        card.edit_type("Sliver", "Dragon");
    }
    if modes.iter().any(|mode| matches!(mode, 1 | 5)) {
        let boost = board.instructions.len() as u32 * 3 - 5;
        let card = board.card(
            "jump-warcaller",
            "Joraga Warcaller",
            Alice,
            "branches",
            "B.24",
            1,
        );
        card.add_types(&[&s.x[3]]);
        card.edit_type("Elf", "Scarecrow");
        card.counters.insert("P1P1".into(), boost);
        let card = board.card(
            "jump-justice",
            "Justice",
            Alice,
            "branches",
            "B.24 animated",
            1,
        );
        animate(card, &["Soldier"], [1, 1]);
        card.add_types(&[&s.x[3]]);
        card.modifications.push(Modification::ColorWord {
            from: "Red".into(),
            to: "White".into(),
            timestamp: card.timestamp + 5,
        });
        card.keyword_counter("Lifelink", card.timestamp + 8);
        let card = board.card(
            "jump-rotlung",
            "Rotlung Reanimator",
            Alice,
            "branches",
            "B.24 X3 copy",
            1,
        );
        card.token = true;
        card.copiable = Copiable {
            creature_types: Some(vec![s.x[3].clone()]),
            base_pt: Some([1, 1]),
            ..Copiable::default()
        };
        card.edit_type("Cleric", "Scarecrow");
        card.edit_type("Zombie", "Sand");
        card.modifications.push(Modification::ColorWord {
            from: "Black".into(),
            to: "White".into(),
            timestamp: card.timestamp + 5,
        });
    }
    if modes.iter().any(|mode| *mode >= 4) {
        let card = board.card(
            "jump-meishin",
            "Meishin, the Mind Cage",
            Bob,
            "branches",
            "B.25 animated",
            1,
        );
        animate(card, &["Soldier"], [0, 3]);
        card.add_types(&[&s.x[3], &s.ya]);
        card.keyword_counter("Reach", card.timestamp + 8);
        for mode in modes.iter().filter(|mode| **mode >= 4) {
            let card = board.card(
                &format!("jump-angel-{mode}"),
                "Blinding Angel",
                Alice,
                "branches",
                "conditional-gates-swap-v1",
                1,
            );
            card.token = true;
            card.copiable.base_pt = Some([1, 1]);
            card.add_types(&[&s.x[3], &s.y[*mode as usize]]);
        }
    }
}
