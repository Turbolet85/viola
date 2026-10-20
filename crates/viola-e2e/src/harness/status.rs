//! `status`: the test-plan §3 Status endpoint shape. `viola list --json` runs on the session's
//! `PATH` with no `VIOLA_*` variable, its document is nested as `list`, and `state` is derived from
//! it; `instances` carries pid + start-time liveness beside it.

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

use serde_json::{Value, json};
use viola_core::MAX_FRAME;

use super::boot::{load_supervise_spec, session_path};
use super::{Outcome, Workspace, exe, load_session};

/// The verdict `status` derives from one `viola list` read.
#[derive(Debug, PartialEq, Eq)]
pub struct Derived {
    pub state: &'static str,
    pub last_error: Option<String>,
}

/// `ready` when `list` exited 0 and names every booted instance `wrapped:true, liveness:"live"`;
/// `degraded` when one is `stale` or absent (a gone instance is omitted from `list`); `down` when
/// `list` exited non-zero, never ran, or printed no JSON object. Unwrapped items are ignored.
pub fn derive(code: Option<i32>, list: Option<&Value>, booted: &[String]) -> Derived {
    let down = |code: &str| Derived {
        state: "down",
        last_error: Some(format!("list-exit-{code}")),
    };
    let doc = match (code, list) {
        (None, _) => return down("none"),
        (Some(0), Some(doc)) => doc,
        (Some(c), _) => return down(&c.to_string()),
    };
    let items = doc["ok"]["items"].as_array().map_or(&[][..], Vec::as_slice);
    let live = |name: &String| {
        items.iter().any(|item| {
            item["name"] == name.as_str() && item["wrapped"] == true && item["liveness"] == "live"
        })
    };
    Derived {
        state: if booted.iter().all(live) {
            "ready"
        } else {
            "degraded"
        },
        last_error: None,
    }
}

/// One `viola list --json --home <home>` from the session's bin dir: its exit code and its stdout
/// parsed as one JSON object (`None` when it is not one, or past `MAX_FRAME`).
fn read_list(viola: &Path, home: &Path, session_bin: &Path) -> (Option<i32>, Option<Value>) {
    let spawned = Command::new(viola)
        .arg("--home")
        .arg(home)
        .args(["list", "--json"])
        .env("PATH", session_path(session_bin))
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env_remove("VIOLA_BIN")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn();
    let Ok(mut child) = spawned else {
        return (None, None);
    };
    let doc = child.stdout.take().and_then(capped_document);
    let code = child.wait().ok().and_then(|s| s.code());
    (code, doc)
}

/// `out` read through `take(MAX_FRAME + 1)` and parsed as one JSON object: `None` past `MAX_FRAME`
/// bytes, or when it is not one.
fn capped_document(out: impl Read) -> Option<Value> {
    let mut bytes = Vec::new();
    let _ = out.take(MAX_FRAME + 1).read_to_end(&mut bytes);
    let within = u64::try_from(bytes.len()).is_ok_and(|n| n <= MAX_FRAME);
    within
        .then(|| serde_json::from_slice::<Value>(&bytes).ok())
        .flatten()
        .filter(Value::is_object)
}

pub fn status(ws: &Workspace, session: &str) -> Outcome {
    let record = match load_session(ws, "status", session) {
        Ok(r) => r,
        Err(out) => return out,
    };
    let (code, list) = match load_supervise_spec(&ws.session_dir(session)) {
        Ok(spec) => read_list(
            &exe(&spec.bin_dir, "viola"),
            &record.home,
            &spec.session_bin,
        ),
        Err(_) => (None, None),
    };
    let booted: Vec<String> = record.instances.iter().map(|i| i.name.clone()).collect();
    let derived = derive(code, list.as_ref(), &booted);
    let instances: Vec<Value> = record
        .instances
        .iter()
        .map(
            |i| json!({"name": i.name, "wrapper_pid": i.wrapper_pid, "alive": i.wrapper().alive()}),
        )
        .collect();
    let ready = derived.state == "ready";
    Outcome::new(
        json!({
            "v": 1, "cmd": "status", "ok": ready, "state": derived.state,
            "pid": null, "uptime_ms": null, "last_error": derived.last_error,
            "list": list,
            "instances": instances,
        }),
        ready,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 16 MiB object read whole, and the same object one byte longer refused: the bound's two
    /// sides, each a labelled case.
    #[test]
    fn capped_document_takes_a_document_of_exactly_the_bound_and_refuses_one_byte_more() {
        const BOUND: usize = 16 * 1024 * 1024;
        let mut exact = String::with_capacity(BOUND + 1);
        exact.push_str("{\"a\":\"");
        exact.push_str(&"x".repeat(BOUND - 8));
        exact.push_str("\"}");
        assert_eq!(exact.len(), BOUND);
        let mut over = exact.clone();
        over.push('\n');
        for (label, text, read) in [
            ("exactly the bound", &exact, true),
            ("one byte more", &over, false),
        ] {
            assert_eq!(capped_document(text.as_bytes()).is_some(), read, "{label}");
        }
    }

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|n| (*n).to_owned()).collect()
    }

    fn item(name: &str, wrapped: bool, liveness: &str) -> Value {
        json!({"name": name, "wrapped": wrapped, "liveness": liveness, "status": "idle"})
    }

    fn board(items: &[Value]) -> Value {
        json!({"v": 1, "ok": {"budget": "unknown", "items": items}})
    }

    fn ready() -> Derived {
        Derived {
            state: "ready",
            last_error: None,
        }
    }

    fn degraded() -> Derived {
        Derived {
            state: "degraded",
            last_error: None,
        }
    }

    fn down(code: &str) -> Derived {
        Derived {
            state: "down",
            last_error: Some(code.to_owned()),
        }
    }

    #[test]
    fn status_derives_each_state_from_the_list_read() {
        let pair = names(&["overseer", "builder"]);
        let both_live = board(&[
            item("overseer", true, "live"),
            item("builder", true, "live"),
        ]);
        let one_stale = board(&[
            item("overseer", true, "live"),
            item("builder", true, "stale"),
        ]);
        let one_absent = board(&[item("overseer", true, "live")]);
        let with_unwrapped = board(&[
            item("overseer", true, "live"),
            item("stranger", false, "live"),
            item("builder", true, "live"),
        ]);
        let unwrapped_namesake = board(&[
            item("overseer", true, "live"),
            item("builder", false, "live"),
        ]);
        let no_items = json!({"v": 1, "error": {"code": "state-files"}});
        for (label, code, list, want) in [
            ("every booted live", Some(0), Some(&both_live), ready()),
            (
                "an unwrapped item ignored",
                Some(0),
                Some(&with_unwrapped),
                ready(),
            ),
            ("one stale", Some(0), Some(&one_stale), degraded()),
            ("one absent", Some(0), Some(&one_absent), degraded()),
            (
                "only an unwrapped namesake",
                Some(0),
                Some(&unwrapped_namesake),
                degraded(),
            ),
            ("exit 0 without items", Some(0), Some(&no_items), degraded()),
            ("exit 21", Some(21), Some(&no_items), down("list-exit-21")),
            ("exit 1 with nothing", Some(1), None, down("list-exit-1")),
            (
                "no exit code",
                None,
                Some(&both_live),
                down("list-exit-none"),
            ),
            ("exit 0 with no object", Some(0), None, down("list-exit-0")),
        ] {
            assert_eq!(derive(code, list, &pair), want, "{label}");
        }
    }

    #[test]
    fn status_derives_ready_for_no_booted_instance() {
        assert_eq!(derive(Some(0), Some(&board(&[])), &[]), ready());
    }

    #[test]
    fn status_derives_down_with_no_session_bin_to_run() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (code, doc) = read_list(&tmp.path().join("absent-viola"), tmp.path(), tmp.path());
        assert_eq!((code, doc), (None, None));
    }
}
