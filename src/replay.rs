use serde_json::{Value, json};

/// Package recorded engine facts into an offline viewer. No rules are simulated here.
pub fn render(trace: &str) -> Result<String, String> {
    let rows: Vec<Value> = trace
        .lines()
        .enumerate()
        .map(|(line, row)| {
            serde_json::from_str(row).map_err(|e| format!("trace line {}: {e}", line + 1))
        })
        .collect::<Result<_, _>>()?;
    let header = rows.first().ok_or("trace is empty")?;
    if header["kind"] != "header"
        || header["schema"] != 1
        || header["evidence"] != "forge-prepared-game"
    {
        return Err("replay requires a version 1 Forge prepared-game trace header".into());
    }
    if !rows.iter().any(|row| row["kind"] == "checkpoint") {
        return Err("trace has no board checkpoint".into());
    }
    let texts: Value = serde_json::from_str(include_str!(
        "../data/cards/forge-20261010/oracle-text.json"
    ))
    .expect("bundled card text must be valid");
    let data = serde_json::to_string(
        &json!({"rows": rows, "texts": texts, "symbols": crate::SYMBOL_CARDS}),
    )
    .map_err(|e| e.to_string())?
    .replace('<', "\\u003c")
    .replace('&', "\\u0026");
    Ok(include_str!("../web/replay.html").replace("__TRACE_DATA__", &data))
}
