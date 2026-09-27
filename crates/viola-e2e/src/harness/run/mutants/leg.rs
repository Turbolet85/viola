//! The reduced per-leg verdict a `--leg` run writes for the `gate` union (test-plan §3 `run` step 4).

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::super::{Workspace, write_json};

/// One outcome as the `gate` union reads it; `None` for the baseline and anything else unscored.
fn leg_outcome(summary: &str) -> Option<&'static str> {
    match summary {
        "CaughtMutant" => Some("caught"),
        "MissedMutant" => Some("missed"),
        "Timeout" => Some("timeout"),
        "Unviable" => Some("unviable"),
        _ => None,
    }
}

/// The reduced per-leg verdict: each mutant's name and outcome, never an argv, a log path or test
/// output (`outcomes.json` carries all three).
pub fn leg_verdict(leg: &str, verdict: &str, outcomes: &Value) -> Value {
    let mutants: Vec<Value> = outcomes["outcomes"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|o| {
            let name = o["scenario"]["Mutant"]["name"].as_str()?;
            let outcome = leg_outcome(o["summary"].as_str()?)?;
            Some(json!({"name": name, "outcome": outcome}))
        })
        .collect();
    json!({"v": 1, "leg": leg, "verdict": verdict, "mutants": mutants})
}

pub fn leg_verdict_path(artifacts: &Path, leg: &str) -> PathBuf {
    artifacts.join(format!("mutants-verdict-{leg}.json"))
}

pub(super) fn write_leg_verdict(
    ws: &Workspace,
    leg: Option<&str>,
    verdict: &str,
    outcomes: &Value,
) {
    if let Some(leg) = leg {
        let _ = fs::create_dir_all(ws.artifacts());
        let _ = write_json(
            &leg_verdict_path(&ws.artifacts(), leg),
            &leg_verdict(leg, verdict, outcomes),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::OUTCOMES;
    use super::*;

    #[test]
    fn leg_verdict_keeps_names_and_outcomes_only() {
        let outcomes: Value = serde_json::from_str(OUTCOMES).expect("json");
        let v = leg_verdict("ubuntu-latest", "counted", &outcomes);
        assert_eq!(
            v,
            json!({"v": 1, "leg": "ubuntu-latest", "verdict": "counted", "mutants": [
                {"name": "src/a.rs:1:5: replace a with 0", "outcome": "caught"},
                {"name": "src/a.rs:2:5: replace b with 1", "outcome": "missed"},
                {"name": "src/a.rs:3:5: replace c with 2", "outcome": "timeout"},
                {"name": "src/a.rs:4:5: replace d with 3", "outcome": "unviable"},
            ]})
        );
        assert_eq!(
            leg_verdict("w", "no-rust-delta", &Value::Null)["mutants"],
            json!([])
        );
        assert_eq!(
            leg_verdict_path(Path::new("a"), "w"),
            Path::new("a").join("mutants-verdict-w.json")
        );
    }
}
