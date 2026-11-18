//! The starts ledger (one line before a start, one closed-word summary after), the cap, and the
//! kept frames: each frame's rows in a 0600 file under the gitignored local path, beside one
//! closed-word line in `frames.ndjson` (security-plan · decision/live-session-screens-kept-local).

use std::fs::{self, File};
use std::io::{self, BufRead as _, BufReader, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Map, Value, json};
use viola_core::MAX_FRAME;
use viola_state::fs::{create_private_dir, create_private_new, open_private_append};
use viola_state::pin::content_key;

use super::screen::Model;

/// The UTC clock as `date -u` prints it, read at the moment it is written.
pub fn utc_now() -> String {
    Command::new("/usr/bin/date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .env_clear()
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|| "unread".to_owned())
}

fn append_line(path: &Path, doc: &Value) -> io::Result<()> {
    let mut line = doc.to_string();
    line.push('\n');
    open_private_append(path)?.write_all(line.as_bytes())
}

pub struct Ledger {
    path: PathBuf,
}

impl Ledger {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// The labels of every start line, in order; an absent file holds none.
    pub fn starts(&self) -> Vec<String> {
        let Ok(file) = File::open(&self.path) else {
            return Vec::new();
        };
        BufReader::new(file.take(MAX_FRAME))
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| serde_json::from_str::<Value>(&line).ok())
            .filter(|doc| doc["kind"] == "start")
            .filter_map(|doc| doc["label"].as_str().map(str::to_owned))
            .collect()
    }

    pub fn start(&self, label: &str) -> io::Result<String> {
        let ts = utc_now();
        append_line(
            &self.path,
            &json!({"v": 1, "kind": "start", "label": label, "ts": ts}),
        )?;
        Ok(ts)
    }

    pub fn summary(&self, label: &str, pairs: &[(String, String)]) -> io::Result<()> {
        let reading: Map<String, Value> = pairs
            .iter()
            .map(|(k, v)| (k.clone(), Value::String(v.clone())))
            .collect();
        append_line(
            &self.path,
            &json!({"v": 1, "kind": "summary", "label": label, "ts": utc_now(), "reading": reading}),
        )
    }
}

/// Whether `label` may start: a base label (`R1`, `R2`) while fewer than `cap` base starts are
/// ledgered, a spare (`R1-2`, `S0R-3`) while fewer than `spare` spares are; a label already
/// ledgered is `spent`.
pub fn admit(
    ledgered: &[String],
    label: &str,
    cap: usize,
    spare: usize,
) -> Result<(), &'static str> {
    if ledgered.iter().any(|l| l == label) {
        return Err("spent");
    }
    let is_spare = |l: &str| l.ends_with("-2") || l.ends_with("-3");
    let (used, allowed) = if is_spare(label) {
        (ledgered.iter().filter(|l| is_spare(l)).count(), spare)
    } else {
        (ledgered.iter().filter(|l| !is_spare(l)).count(), cap)
    };
    if used >= allowed { Err("cap") } else { Ok(()) }
}

/// One start's kept frames: `<out>/<run>/screens/<n>-<label>.txt` and `<out>/<run>/frames.ndjson`.
pub struct Frames {
    dir: PathBuf,
    n: usize,
}

impl Frames {
    pub fn open(out: &Path, run: &str) -> io::Result<Self> {
        let dir = out.join(run);
        create_private_dir(&dir.join("screens"))?;
        Ok(Self { dir, n: 0 })
    }

    /// How many frames were kept.
    pub fn count(&self) -> usize {
        self.n
    }

    /// Keeps the frame the model holds now under `label` (a compiled word), with its wait.
    pub fn keep(&mut self, label: &str, model: &Model, wait_ms: u128) {
        self.n += 1;
        let text = model.rows().map(|rows| {
            rows.iter()
                .map(|r| r.trim_end())
                .collect::<Vec<_>>()
                .join("\n")
        });
        let mut line = json!({
            "v": 1,
            "n": self.n,
            "label": label,
            "wait_ms": wait_ms,
            "poisoned": text.is_none(),
            "rows_sha": text.as_deref().map(|t| content_key(t.as_bytes())),
        });
        for (key, word) in model.reading().pairs() {
            line[key] = Value::String(word);
        }
        if let Some(text) = text {
            let file = self
                .dir
                .join("screens")
                .join(format!("{:03}-{label}.txt", self.n));
            if let Ok(mut f) = create_private_new(&file) {
                let _ = f.write_all(text.as_bytes());
            }
        }
        let _ = append_line(&self.dir.join("frames.ndjson"), &line);
    }
}

/// The summary on stdout: one `key: value` line per pair, then the same into the ledger.
pub fn print_summary(pairs: &[(String, String)]) {
    let mut out = io::stdout().lock();
    for (k, v) in pairs {
        let _ = writeln!(out, "{k}: {v}");
    }
    let _ = out.flush();
}

/// Removes an empty run dir left by a start refused before any frame.
pub fn drop_empty(out: &Path, run: &str) {
    let dir = out.join(run);
    let _ = fs::remove_dir(dir.join("screens"));
    let _ = fs::remove_dir(dir);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(l: &[&str]) -> Vec<String> {
        l.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn a_ledgered_label_is_spent() {
        assert_eq!(admit(&labels(&["R1"]), "R1", 2, 1), Err("spent"));
    }

    #[test]
    fn base_starts_stop_at_the_cap_and_spares_at_the_spare() {
        assert_eq!(admit(&labels(&["R1"]), "R2", 2, 1), Ok(()));
        assert_eq!(admit(&labels(&["R1", "R2"]), "R3", 2, 1), Err("cap"));
        assert_eq!(admit(&labels(&["R1", "R2"]), "R1-2", 2, 1), Ok(()));
        assert_eq!(
            admit(&labels(&["R1", "R2", "R1-2"]), "R2-2", 2, 1),
            Err("cap")
        );
        assert_eq!(admit(&labels(&["R1", "R2"]), "R1-2", 2, 0), Err("cap"));
    }

    #[test]
    fn admit_counts_a_dash_three_label_as_a_spare() {
        assert_eq!(admit(&labels(&["S0R", "S0R-2"]), "S0R-3", 1, 2), Ok(()));
        assert_eq!(admit(&labels(&["S0R", "S0R-2"]), "S0R-3", 1, 1), Err("cap"));
        assert_eq!(
            admit(&labels(&["S0R", "S0R-2", "S0R-3"]), "S0R-4", 1, 2),
            Err("cap"),
            "a base label past the cap"
        );
        assert_eq!(
            admit(&labels(&["S0R"]), "R3-326", 1, 2),
            Err("cap"),
            "a -326 label is a base label"
        );
    }

    #[test]
    fn the_ledger_reads_back_its_start_labels() {
        let dir = tempfile::tempdir().expect("tempdir");
        let ledger = Ledger::new(dir.path().join("l.ndjson"));
        assert!(ledger.starts().is_empty());
        let ts = ledger.start("R1").expect("start");
        assert!(ts.ends_with('Z'), "a date -u stamp");
        ledger
            .summary("R1", &[("census".to_owned(), "0".to_owned())])
            .expect("summary");
        assert_eq!(ledger.starts(), labels(&["R1"]));
    }
}
