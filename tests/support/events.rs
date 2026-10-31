//! The one `events.ndjson` reader and wait for root tests, and the wrapper boot that waits for the
//! session-start record (test-plan §3 `run` step 2). The wait is the root-watch loop on the one
//! bound `WITHIN`; the boot takes its home from the caller and makes none.

use std::path::Path;
use std::time::Instant;

use serde_json::Value;

use super::home::{StampedHome, Wrapper, resolved, workspace_path};
use super::hygiene::has_absolute_path;
use super::ndjson;
use super::watch::{WITHIN, Watch};

/// The class of the first thing in `body` a machine reader's error may not hold: `absolute-path` (a
/// drive letter form, `/home/`, `/Users/`, either spelling of `home`), `caused-by`, `serde-path`
/// (`at line ` … ` column `) or `planted`. Every key and string is read.
pub fn error_body_hit(body: &Value, home: &Path, planted: &[&str]) -> Option<&'static str> {
    let mut strings = Vec::new();
    strings_of(body, &mut strings);
    let spellings = [home.to_path_buf(), resolved(home)].map(|p| p.display().to_string());
    strings.into_iter().find_map(|text| {
        if planted.iter().any(|value| text.contains(value)) {
            Some("planted")
        } else if has_absolute_path(text)
            || has_drive_letter(text)
            || spellings.iter().any(|home| text.contains(home.as_str()))
        {
            Some("absolute-path")
        } else if text.contains("Caused by") {
            Some("caused-by")
        } else if text
            .split_once("at line ")
            .is_some_and(|(_, rest)| rest.contains(" column "))
        {
            Some("serde-path")
        } else {
            None
        }
    })
}

/// Panics on a hit, naming `place` and the class and never the bytes: a failing witness prints no
/// canary into JUnit, which the scan reads (security-plan §Bootstrap phases, `logging-redaction-wire`).
pub fn assert_error_body_clean(place: &str, body: &Value, home: &Path, planted: &[&str]) {
    if let Some(class) = error_body_hit(body, home, planted) {
        panic!("{place}: an error body holds {class}");
    }
}

fn strings_of<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
    match value {
        Value::String(text) => out.push(text),
        Value::Array(items) => items.iter().for_each(|item| strings_of(item, out)),
        Value::Object(members) => members.iter().for_each(|(key, item)| {
            out.push(key);
            strings_of(item, out);
        }),
        _ => {}
    }
}

/// `C:\` or `C:/` anywhere, the letter standing alone.
fn has_drive_letter(text: &str) -> bool {
    text.as_bytes().windows(3).enumerate().any(|(at, window)| {
        window[0].is_ascii_alphabetic()
            && window[1] == b':'
            && matches!(window[2], b'\\' | b'/')
            && (at == 0 || !text.as_bytes()[at - 1].is_ascii_alphanumeric())
    })
}

/// The complete lines of `<instance_dir>/events.ndjson`.
pub fn events(instance_dir: &Path) -> Vec<Value> {
    ndjson::read_lines(&instance_dir.join("events.ndjson"))
}

/// The complete lines once `pred` holds over them; past `WITHIN` it panics naming `what`.
pub fn wait_events(instance_dir: &Path, what: &str, pred: impl Fn(&[Value]) -> bool) -> Vec<Value> {
    let watch = Watch::start("events");
    let deadline = Instant::now() + WITHIN;
    loop {
        let lines = events(instance_dir);
        if pred(&lines) {
            return lines;
        }
        watch.note(&format!("events {}", lines.len()));
        watch.deadline_check(deadline, &format!("timed out waiting for {what}"));
        std::thread::yield_now();
    }
}

/// Raises the next gated step and waits for its dialog line: its `dialog_id`.
pub fn raise(wrapper: &Wrapper, kind: &str, nth: usize) -> u64 {
    wrapper.release();
    let of_kind = |lines: &[Value]| -> Vec<Value> {
        lines
            .iter()
            .filter(|l| l["kind"] == kind)
            .cloned()
            .collect()
    };
    let lines = wait_events(&wrapper.instance_dir(), "the dialog line", |l| {
        of_kind(l).len() >= nth
    });
    of_kind(&lines)[nth - 1]["data"]["dialog_id"]
        .as_u64()
        .expect("a dialog_id")
}

/// A `builder` wrapper on `stamped` over the committed hook fixtures (then `extra`, and `script`),
/// its SessionStart record landed.
pub fn boot(stamped: StampedHome, script: Option<&str>, extra: &[&str]) -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let mut args = vec!["--fixtures", fixtures.to_str().expect("utf-8 path")];
    args.extend_from_slice(extra);
    let wrapper = Wrapper::boot(stamped, "builder", script, &args);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wrapper
}
