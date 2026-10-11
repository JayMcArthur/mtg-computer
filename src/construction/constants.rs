use super::{Modification, Player, PreparedBoard, PreparedCard, Selectors};
use std::collections::BTreeSet;

/// Explicit live-recipe repairs are described in docs/constant-gadgets.md.
pub(super) fn extend(board: &mut PreparedBoard, s: &Selectors) {
    for card in board
        .cards
        .iter_mut()
        .filter(|c| c.group == "registers" && c.id.starts_with('r'))
    {
        for (offset, keyword) in ["Indestructible", "Vigilance"].into_iter().enumerate() {
            card.keyword_counter(keyword, 50_000 + offset as u64);
        }
    }
    // Replaces the suppressed Z-gate boost while the Y wildcard is present.
    // It phases indirectly with the Fighter, exchanging exactly one boost for the other.
    let card = board.card(
        "inactive-register-boost",
        "Joraga Warcaller",
        Player::Bob,
        "constants",
        "declared Y-synchronized Vehicle repair",
        3,
    );
    card.token = true;
    card.copiable.card_types = Some(vec!["Artifact".into(), "Vehicle".into()]);
    card.copiable.creature_types = Some(Vec::new());
    card.copiable.mana_cost = Some("no cost".into());
    card.copiable.colors = Some(vec!["White".into()]);
    card.copiable.keywords.push("Crew:3".into());
    card.edit_type("Elf", "Saproling");
    card.counters.insert("P1P1".into(), 1);
    card.attached_to = Some("fighter-Y".into());
    if board.instructions.iter().any(|i| i[0] == 10) {
        num_build(board, s);
    }
    if board.instructions.iter().any(|i| i[0] == 5) {
        set(board, s);
    }
}

pub(super) fn colors(card: &mut PreparedCard, names: &[&str]) {
    card.modifications
        .retain(|m| !matches!(m, Modification::Color { .. }));
    card.modifications.push(Modification::Color {
        colors: names.iter().map(|s| (*s).into()).collect(),
        timestamp: 15_000,
    });
}

pub(super) fn remove_flying(card: &mut PreparedCard) {
    card.modifications.push(Modification::RemoveKeyword {
        keyword: "Flying".into(),
        timestamp: card.timestamp + 8,
    });
}

fn num_build(board: &mut PreparedBoard, s: &Selectors) {
    use Player::{Alice, Bob};
    let card = board.card(
        "num-excruciator",
        "Excruciator",
        Alice,
        "constants",
        "B.21 live",
        1,
    );
    card.add_types(&[&s.x[10], &s.xa]);
    for keyword in ["Flying", "Indestructible", "Vigilance"] {
        card.keyword_counter(keyword, card.timestamp + 8);
    }
    let card = board.card(
        "num-fighter",
        "Dream Fighter",
        Alice,
        "constants",
        "B.21 live",
        1,
    );
    card.token = true;
    card.copiable.base_pt = Some([2, 2]);
    card.add_types(&[&s.x[10]]);
    card.keyword_counter("Flying", card.timestamp + 8);
    colors(card, &["Blue"]);
    card.keyword("Protection from red");
    let card = board.card("num-vigor", "Vigor", Bob, "constants", "B.21 live", 1);
    card.add_types(&[&s.x[10], "Beast"]);
    card.keyword_counter("Reach", card.timestamp + 8);
    for (id, name, word) in [
        ("num-khorvath", "Khorvath Brightflame", "Knight"),
        ("num-sylvia", "Sylvia Brightspear", "Dragon"),
    ] {
        let card = board.card(id, name, Bob, "constants", "B.21 late grants", 3);
        card.add_types(&[&s.x[10]]);
        card.edit_type(word, "Rhino");
        if word == "Knight" {
            remove_flying(card);
        }
    }
    let card = board.card(
        "num-equity",
        "Mangara's Equity",
        Alice,
        "constants",
        "B.21 declared animation",
        1,
    );
    card.token = true;
    card.copiable.card_types = Some(vec!["Enchantment".into(), "Creature".into()]);
    card.copiable.creature_types = Some(vec!["Soldier".into()]);
    card.copiable.base_pt = Some([1, 1]);
    card.add_types(&[&s.x[10]]);
    card.modifications.push(Modification::ColorWord {
        from: "White".into(),
        to: "Green".into(),
        timestamp: card.timestamp + 5,
    });
    card.chosen_colors.push("Red".into());
    colors(card, &["Green", "Black"]);
    let ys: BTreeSet<_> = board
        .instructions
        .iter()
        .filter(|i| i[0] == 10)
        .map(|i| i[1])
        .collect();
    let mut zs: BTreeSet<_> = board
        .instructions
        .iter()
        .filter(|i| i[0] == 10 && i[2] != 0)
        .map(|i| i[2])
        .collect();
    // One Kangee supplies Wizard when Set reuses the NumBuild party.
    if zs.is_empty() && board.instructions.iter().any(|i| i[0] == 5) {
        zs.insert(1);
    }
    for y in ys {
        let card = board.card(
            &format!("num-myr-{y}"),
            "Darksteel Myr",
            Alice,
            "constants",
            "B.21 live sized copy",
            1,
        );
        card.token = true;
        card.copiable.base_pt = Some([12 * i32::from(y), 1]);
        card.copiable.creature_types = Some(vec!["Myr".into(), "Nomad".into()]);
        card.add_types(&[&s.x[10], &s.y[y as usize]]);
        card.keyword_counter("Reach", card.timestamp + 8);
    }
    for z in zs {
        let card = board.card(
            &format!("num-kangee-{z}"),
            "Kangee, Aerie Keeper",
            Alice,
            "constants",
            "B.21 live",
            1,
        );
        card.edit_type("Bird", "Nomad");
        card.add_types(&[&s.z[z as usize]]);
        card.counters.insert("FEATHER".into(), u32::from(z));
        remove_flying(card);
    }
    if board
        .instructions
        .windows(2)
        .any(|pair| pair[0][0] == 10 && pair[1][0] == 10)
    {
        let card = board.card(
            "num-marauders",
            "Angrath's Marauders",
            Bob,
            "constants",
            "B.21 live",
            1,
        );
        card.add_types(&[&s.x[10], &s.za]);
    }
    board
        .card(
            "num-oil",
            "Fishliver Oil",
            Alice,
            "constants",
            "B.21 live",
            1,
        )
        .attached_to = Some("num-vigor".into());
}

fn set(board: &mut PreparedBoard, s: &Selectors) {
    use Player::{Alice, Bob};
    let card = board.card(
        "set-copy",
        "Halana and Alena, Partners",
        Bob,
        "constants",
        "B.2",
        1,
    );
    card.token = true;
    card.copiable.base_pt = Some([0, 2]);
    card.add_types(&[&s.x[5], "Elf", "Reflection", "Yeti"]);
    let card = board.card(
        "set-priest",
        "Archpriest of Iona",
        Alice,
        "constants",
        "B.2",
        1,
    );
    card.add_types(&[&s.x[5], &s.ya, "Reflection", "Yeti"]);
    let card = board.card("set-shape", "Shape Stealer", Alice, "constants", "B.2", 1);
    card.token = true;
    card.copiable.base_pt = Some([3, 3]);
    card.add_types(&[&s.x[5], &s.ya]);
    card.counters.insert("M1M0".into(), 1);
    card.keyword_counter("Flying", card.timestamp + 8);
    colors(card, &["Green", "Black"]);
    let has_kangee = board.cards.iter().any(|c| c.id.starts_with("num-kangee-"));
    for role in if has_kangee {
        &["Warrior"][..]
    } else {
        &["Warrior", "Wizard"][..]
    } {
        let card = board.card(
            &format!("set-party-{role}"),
            &format!("{role} Token"),
            Alice,
            "constants",
            "B.2 ability-free token",
            1,
        );
        card.token = true;
        card.copiable.card_types = Some(vec!["Creature".into()]);
        card.copiable.creature_types = Some(vec![(*role).into()]);
        card.copiable.base_pt = Some([0, 2]);
        card.copiable.mana_cost = Some("no cost".into());
        // Rotlung tokens have no rules text. A blank token definition is imported explicitly.
        card.copiable.blank_rules = true;
    }
}
