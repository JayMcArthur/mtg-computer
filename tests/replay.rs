use mtg_computer::replay::render;
use serde_json::json;

#[test]
fn replay_preserves_recorded_data_without_allowing_script_termination() {
    let source = "</script><script>throw new Error('injected')</script>&";
    let header = json!({
        "kind": "header", "schema": 1, "evidence": "forge-prepared-game",
        "manifest": {"source": [source]}
    });
    let checkpoint = json!({"kind": "checkpoint", "cards": []});
    let html = render(&format!("{header}\n{checkpoint}\n")).unwrap();
    assert!(!html.contains(source));
    assert!(!html.contains("__TRACE_DATA__"));
    let embedded = html
        .split("<script type=\"application/json\" id=\"trace-data\">")
        .nth(1)
        .unwrap()
        .split("</script>")
        .next()
        .unwrap();
    let decoded: serde_json::Value = serde_json::from_str(embedded).unwrap();
    assert_eq!(decoded["rows"][0]["manifest"]["source"][0], source);
    assert_eq!(decoded["rows"][1], checkpoint);
}

#[test]
fn replay_rejects_missing_or_unrecognized_engine_evidence() {
    assert!(render("").unwrap_err().contains("empty"));
    assert!(render("{}\ninvalid").unwrap_err().contains("trace line 2"));
    for header in [
        json!({"kind":"header","schema":2,"evidence":"forge-prepared-game"}),
        json!({"kind":"header","schema":1,"evidence":"isa-interpreter"}),
    ] {
        assert!(render(&header.to_string()).unwrap_err().contains("header"));
    }
    let header = json!({"kind":"header","schema":1,"evidence":"forge-prepared-game"});
    assert!(
        render(&header.to_string())
            .unwrap_err()
            .contains("checkpoint")
    );
}
