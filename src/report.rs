use crate::construction::PreparedBoard;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write;

fn cell(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', " ")
}

/// A prepared-role inventory is separate from the deck and the legal setup sequence.
pub fn inventory(board: &PreparedBoard) -> String {
    let mut text = String::from(
        "# Prepared card inventory\n\nThis lists the imported roles, including tokens and exiled imprints. It is not a starting deck or a certificate that setup was played legally. See docs/legal-setup.md for the physical preparation research.\n\n",
    );
    let mut names: BTreeMap<&String, [usize; 2]> = BTreeMap::new();
    for card in &board.cards {
        names.entry(&card.name).or_default()[usize::from(card.token)] += 1;
    }
    text.push_str("| Card name | Prepared instances | Token instances |\n| --- | ---: | ---: |\n");
    for (name, [cards, tokens]) in names {
        writeln!(text, "| {} | {} | {tokens} |", cell(name), cards + tokens).unwrap();
    }
    text.push_str("\n## Individual roles\n\nAll owners are recorded in manifest.json; controller and owner can differ. Timestamp values describe the prepared effect ordering.\n\n| Role | Card | Controller | Zone | Recipe | Timestamp | Attached to / imprint | Counters |\n| --- | --- | --- | --- | --- | ---: | --- | --- |\n");
    for card in &board.cards {
        let links = format!(
            "{} / {}",
            card.attached_to.as_deref().unwrap_or(""),
            card.imprint.as_deref().unwrap_or("")
        );
        let counters = serde_json::to_string(&card.counters).unwrap();
        writeln!(
            text,
            "| {} | {} | {:?} | {} | {} | {} | {} | {} |",
            cell(&card.id),
            cell(&card.name),
            card.controller,
            card.zone,
            card.recipe,
            card.timestamp,
            cell(&links),
            cell(&counters)
        )
        .unwrap();
    }
    text
}

/// Print recorded rules events; no program operation or game rule is evaluated here.
pub fn transcript(trace: &str) -> Result<String, String> {
    let rows: Vec<Value> = trace
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()
        .map_err(|error| format!("invalid Forge trace: {error}"))?;
    let manifest = rows
        .first()
        .ok_or("Forge trace is empty")?
        .get("manifest")
        .ok_or("Forge trace has no manifest")?;
    let mut text = String::from(
        "# Recorded Magic execution\n\nEvidence: Forge game from a prepared board. This transcript prints engine observations, including failed or limited runs. The result below determines whether execution completed.\n\n## Source\n\n",
    );
    for line in manifest["source"]
        .as_array()
        .ok_or("manifest has no source")?
    {
        writeln!(text, "    {}", line.as_str().ok_or("invalid source line")?).unwrap();
    }
    text.push_str("\n## Conversion\n\n| Address | Source line | Instruction | Symbols | Program cards |\n| ---: | ---: | --- | --- | --- |\n");
    let instructions: Vec<[u8; 3]> = serde_json::from_value(manifest["instructions"].clone())
        .map_err(|error| error.to_string())?;
    for (pc, symbols) in instructions.into_iter().enumerate() {
        if symbols.iter().any(|n| *n > 11) {
            return Err("invalid instruction symbol".into());
        }
        let cards = symbols.map(|n| crate::SYMBOL_CARDS[n as usize]).join(" / ");
        writeln!(
            text,
            "| {pc} | {} | {} | {:?} | {cards} |",
            manifest["source_lines"][pc],
            crate::machine::describe(symbols),
            symbols
        )
        .unwrap();
    }
    text.push_str("\n## Rules events\n\nCheckpoint state and every decision remain available in trace.jsonl and replay.html. Routine priority passes are omitted from this printed view.\n\n| Event | Turn | Phase | Kind | Recorded facts |\n| ---: | ---: | --- | --- | --- |\n");
    for row in &rows {
        if matches!(
            row["kind"].as_str(),
            Some("header" | "checkpoint" | "result")
        ) || row["decision"] == "priority"
        {
            continue;
        }
        let mut facts = row.as_object().ok_or("invalid trace row")?.clone();
        for key in ["id", "turn", "phase", "kind"] {
            facts.remove(key);
        }
        writeln!(
            text,
            "| {} | {} | {} | {} | {} |",
            row["id"],
            row["turn"],
            cell(row["phase"].as_str().unwrap_or("")),
            cell(row["kind"].as_str().unwrap_or("")),
            cell(&serde_json::to_string(&facts).unwrap())
        )
        .unwrap();
    }
    text.push_str("\n## Result\n\n");
    if let Some(result) = rows.iter().rfind(|row| row["kind"] == "result") {
        writeln!(text, "    {}", serde_json::to_string(result).unwrap()).unwrap();
    } else {
        text.push_str("Recording incomplete; no result was recorded.\n");
    }
    Ok(text)
}
