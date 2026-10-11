use crate::{
    compiler::Program,
    construction::{PreparedBoard, prepare_program},
    replay, report,
};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Compile a supported program into a prepared board, run Forge, and package its recorded facts.
pub fn run(program: &Program, output: &Path, limit: usize) -> Result<PathBuf, String> {
    if limit == 0 {
        return Err("MAX_BOUNDARIES must be positive".into());
    }
    let board = prepare_program(program)?;
    fs::create_dir_all(output).map_err(|e| format!("{}: {e}", output.display()))?;
    let output = output.canonicalize().map_err(|e| e.to_string())?;
    let manifest = output.join("manifest.json");
    let trace = output.join("trace.jsonl");
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&board).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{}: {e}", manifest.display()))?;
    fs::write(output.join("inventory.md"), report::inventory(&board))
        .map_err(|e| format!("write prepared inventory: {e}"))?;
    let html = output.join("replay.html");
    let transcript = output.join("transcript.md");
    // A failed new launch must never reuse older successful artifacts.
    for path in [&trace, &html, &transcript] {
        if path.exists() {
            fs::remove_file(path).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    let status = Command::new("bash")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tools/forge/run-board.sh"))
        .arg(&manifest)
        .arg(&trace)
        .arg(limit.to_string())
        .status()
        .map_err(|e| format!("launch Forge: {e}"))?;
    if trace.exists() {
        let recorded =
            fs::read_to_string(&trace).map_err(|e| format!("{}: {e}", trace.display()))?;
        fs::write(&html, replay::render(&recorded)?)
            .map_err(|e| format!("{}: {e}", html.display()))?;
        fs::write(&transcript, report::transcript(&recorded)?)
            .map_err(|e| format!("{}: {e}", transcript.display()))?;
    }
    if !status.success() {
        return Err(format!(
            "Forge stopped with {status}; recorded artifacts are in {}",
            output.display()
        ));
    }
    let recorded = fs::read_to_string(&trace).map_err(|e| format!("{}: {e}", trace.display()))?;
    verify_result(&board, &recorded)?;
    Ok(html)
}

fn verify_result(board: &PreparedBoard, trace: &str) -> Result<(), String> {
    let rows: Vec<Value> = trace
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()
        .map_err(|e| format!("invalid Forge trace: {e}"))?;
    let header = rows.first().ok_or("Forge trace is empty")?;
    if header["manifest"] != serde_json::to_value(board).map_err(|e| e.to_string())? {
        return Err("Forge trace manifest differs from the compiled prepared board".into());
    }
    let result = rows.last().ok_or("Forge trace has no result")?;
    if result["kind"] != "result" || result["status"] != "verified" || result["draw"] != true {
        return Err("Forge trace does not record a verified Magic draw".into());
    }
    let cards = rows
        .iter()
        .rev()
        .find(|r| r["kind"] == "checkpoint")
        .and_then(|r| r["cards"].as_array())
        .ok_or("Forge trace has no final board")?;
    for (r, expected) in board.expected_registers.iter().enumerate() {
        let id = format!("r{r}");
        let card = cards
            .iter()
            .find(|c| c["id"] == id)
            .ok_or_else(|| format!("final board lacks {id}"))?;
        let counters = card["counters"]
            .as_object()
            .ok_or_else(|| format!("invalid {id} counters"))?;
        let actual = match counters.get("P1P1") {
            None => 0,
            Some(n) => n
                .as_u64()
                .ok_or_else(|| format!("invalid {id} counter value"))?,
        };
        if actual.to_string() != *expected {
            return Err(format!(
                "{id}: ISA expected {expected}, Forge recorded {actual}"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn success_marker_without_matching_counters_cannot_pass_backend_validation() {
        let board = crate::construction::add_halt_fixture();
        let header = json!({"kind":"header","manifest":board});
        let cards: Vec<_> = board.expected_registers.iter().enumerate().map(|(r,n)|
            json!({"id":format!("r{r}"),"counters":{"P1P1":n.parse::<u64>().unwrap()}})).collect();
        let mut checkpoint = json!({"kind":"checkpoint","cards":cards});
        let result = json!({"kind":"result","status":"verified","draw":true});
        let trace = format!("{header}\n{checkpoint}\n{result}\n");
        verify_result(&board, &trace).unwrap();
        assert!(verify_result(&board, &format!("{header}\n{checkpoint}\n")).is_err());
        checkpoint["cards"][1]["counters"]["P1P1"] = json!(2);
        assert!(
            verify_result(&board, &format!("{header}\n{checkpoint}\n{result}"))
                .unwrap_err()
                .contains("r1")
        );
    }
}
