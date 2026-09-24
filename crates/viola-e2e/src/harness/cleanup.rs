//! `cleanup`: stop the session through its supervisor, prove every process gone (pid + start time),
//! remove the home unless `AGENT_RUN_KEEP_HOMES=1`, drop the session record. Idempotent.

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::supervise::remove_stop_files;
use super::{
    Outcome, ProcessId, SessionRecord, Workspace, read_json, session_record_path, valid_session_id,
};

const SUPERVISOR_DEADLINE: Duration = Duration::from_secs(30);
const KILL_DEADLINE: Duration = Duration::from_secs(5);

pub enum Target<'a> {
    Session(&'a str),
    All,
}

struct Report {
    processes_gone: bool,
    home_removed: Value,
    killed: Vec<String>,
}

pub fn cleanup(ws: &Workspace, target: Target<'_>, keep_homes: bool) -> Outcome {
    let sessions = match target {
        Target::Session(id) if !valid_session_id(id) => {
            return Outcome::usage(Some("cleanup"), "invalid-session-id");
        }
        Target::Session(id) => vec![id.to_owned()],
        Target::All => all_sessions(ws),
    };
    let mut cleaned = Vec::new();
    let mut reports = Vec::new();
    for id in sessions {
        let Ok(record) = read_json::<SessionRecord>(&session_record_path(ws, &id)) else {
            continue;
        };
        reports.push(cleanup_one(ws, &id, &record, keep_homes));
        cleaned.push(id);
    }
    summarize(cleaned, reports, keep_homes)
}

fn summarize(cleaned: Vec<String>, reports: Vec<Report>, keep_homes: bool) -> Outcome {
    if reports.is_empty() {
        return Outcome::new(
            json!({"v": 1, "cmd": "cleanup", "ok": true, "cleaned": []}),
            true,
        );
    }
    let processes_gone = reports.iter().all(|r| r.processes_gone);
    let home_removed = if keep_homes {
        json!("kept")
    } else {
        json!(reports.iter().all(|r| r.home_removed == true))
    };
    let killed: Vec<String> = reports.into_iter().flat_map(|r| r.killed).collect();
    let ok = processes_gone && home_removed != false;
    Outcome::new(
        json!({
            "v": 1, "cmd": "cleanup", "ok": ok, "cleaned": cleaned,
            "processes_gone": processes_gone, "endpoint_gone": null, "port_free": null,
            "url_file_removed": null, "home_removed": home_removed, "killed": killed,
        }),
        ok,
    )
}

fn all_sessions(ws: &Workspace) -> Vec<String> {
    let mut ids: Vec<String> = fs::read_dir(ws.agent_run())
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|id| valid_session_id(id) && session_record_path(ws, id).is_file())
        .collect();
    ids.sort();
    ids
}

fn cleanup_one(ws: &Workspace, id: &str, record: &SessionRecord, keep_homes: bool) -> Report {
    let dir = ws.session_dir(id);
    let _ = fs::write(dir.join("stop.request"), b"");
    let supervisor = record.supervisor();
    supervisor.wait_gone(Instant::now() + SUPERVISOR_DEADLINE);

    let mut killed = killed_by_supervisor(&dir);
    let mut targets: Vec<(String, ProcessId)> = vec![("supervisor".to_owned(), supervisor)];
    for inst in &record.instances {
        targets.push((inst.name.clone(), inst.wrapper()));
        targets.push((inst.name.clone(), inst.child()));
    }
    for (name, id) in &targets {
        if id.kill() && !killed.contains(name) {
            killed.push(name.clone());
        }
    }
    let deadline = Instant::now() + KILL_DEADLINE;
    let processes_gone = targets.iter().all(|(_, id)| id.wait_gone(deadline));

    let home_removed = if keep_homes {
        json!("kept")
    } else {
        json!(remove_session_home(ws, &record.home))
    };
    remove_stop_files(&dir);
    let _ = fs::remove_file(session_record_path(ws, id));
    Report {
        processes_gone,
        home_removed,
        killed,
    }
}

fn killed_by_supervisor(dir: &Path) -> Vec<String> {
    read_json::<Value>(&dir.join("supervisor-exit.json"))
        .ok()
        .and_then(|doc| doc["exits"].as_array().cloned())
        .unwrap_or_default()
        .iter()
        .filter(|e| e["killed"] == true)
        .filter_map(|e| e["name"].as_str().map(str::to_owned))
        .collect()
}

/// Removes `<e2e-home>/viola-session-*/` (the parent `boot` created); refuses any other path.
fn remove_session_home(ws: &Workspace, home: &Path) -> bool {
    let Some(parent) = home.parent() else {
        return false;
    };
    let is_session_dir = parent
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with("viola-session-"));
    if !is_session_dir || parent.parent() != Some(ws.e2e_home().as_path()) {
        return false;
    }
    let _ = fs::remove_dir_all(parent);
    !parent.exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleanup_without_a_session_is_ok_and_empty() {
        let out = cleanup(
            &Workspace::from_build(),
            Target::Session("no-such-session-x9"),
            false,
        );
        assert_eq!(out.code, 0);
        assert_eq!(
            out.doc,
            json!({"v": 1, "cmd": "cleanup", "ok": true, "cleaned": []})
        );
    }

    fn report(gone: bool, removed: bool, killed: &[&str]) -> Report {
        Report {
            processes_gone: gone,
            home_removed: json!(removed),
            killed: killed.iter().map(|k| (*k).to_owned()).collect(),
        }
    }

    #[test]
    fn summarize_is_ok_only_when_everything_is_proven() {
        let one = || vec!["s".to_owned()];
        let good = summarize(one(), vec![report(true, true, &[])], false);
        assert_eq!(good.code, 0);
        assert_eq!(good.doc["home_removed"], true);
        assert_eq!(good.doc["cleaned"], json!(["s"]));
        let lingering = summarize(one(), vec![report(false, true, &["a"])], false);
        assert_eq!(lingering.code, 1);
        assert_eq!(lingering.doc["processes_gone"], false);
        assert_eq!(lingering.doc["killed"], json!(["a"]));
        let home_left = summarize(one(), vec![report(true, false, &[])], false);
        assert_eq!(home_left.code, 1);
        assert_eq!(home_left.doc["home_removed"], false);
        let kept = summarize(one(), vec![report(true, false, &[])], true);
        assert_eq!(kept.code, 0);
        assert_eq!(kept.doc["home_removed"], "kept");
        let two = summarize(
            vec!["a".to_owned(), "b".to_owned()],
            vec![report(true, true, &["x"]), report(true, false, &["y"])],
            false,
        );
        assert_eq!(two.doc["home_removed"], false);
        assert_eq!(two.doc["killed"], json!(["x", "y"]));
    }

    #[test]
    fn all_sessions_lists_only_recorded_valid_sessions() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ws = Workspace {
            root: tmp.path().to_path_buf(),
        };
        for (id, recorded) in [("b", true), ("a", true), ("c", false), (".x", true)] {
            let dir = ws.session_dir(id);
            fs::create_dir_all(&dir).expect("mkdir");
            if recorded {
                fs::write(dir.join("session.json"), "{}").expect("write");
            }
        }
        assert_eq!(all_sessions(&ws), vec!["a".to_owned(), "b".to_owned()]);
    }

    #[test]
    fn cleanup_all_stops_recorded_sessions_and_refuses_foreign_homes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ws = Workspace {
            root: tmp.path().to_path_buf(),
        };
        let dead = ProcessId {
            pid: std::process::id(),
            started_at: 1,
        };
        let record = SessionRecord {
            v: 1,
            session: "s".to_owned(),
            home: tmp.path().join("elsewhere").join("home"),
            instances: vec![super::super::InstanceRecord {
                name: "builder".to_owned(),
                wrapper_pid: dead.pid,
                started_at: dead.started_at,
                child_pid: dead.pid,
                child_started_at: dead.started_at,
                fake_args: vec![],
            }],
            ui: None,
            supervisor_pid: dead.pid,
            supervisor_started_at: dead.started_at,
            cookie_file: None,
        };
        fs::create_dir_all(ws.session_dir("s")).expect("mkdir");
        super::super::write_json(&session_record_path(&ws, "s"), &record).expect("write");
        let out = cleanup(&ws, Target::All, false);
        assert_eq!(out.doc["cleaned"], json!(["s"]));
        assert_eq!(out.doc["processes_gone"], true);
        assert_eq!(out.doc["home_removed"], false);
        assert_eq!(out.code, 1);
        assert!(!session_record_path(&ws, "s").exists());
        assert!(!ws.session_dir("s").join("stop.request").exists());
    }

    #[test]
    fn cleanup_rejects_an_invalid_session_id() {
        let out = cleanup(&Workspace::from_build(), Target::Session("../x"), false);
        assert_eq!(out.code, 2);
    }

    #[test]
    fn remove_session_home_refuses_paths_outside_e2e_home() {
        let ws = Workspace::from_build();
        let tmp = tempfile::tempdir().expect("tempdir");
        let outside = tmp.path().join("viola-session-x");
        fs::create_dir_all(outside.join("home")).expect("mkdir");
        assert!(!remove_session_home(&ws, &outside.join("home")));
        assert!(outside.exists());
        let misnamed = ws.e2e_home().join("not-a-session-dir-x9");
        assert!(!remove_session_home(&ws, &misnamed.join("home")));
        assert!(!remove_session_home(&ws, Path::new("home")));
    }

    #[test]
    fn remove_session_home_removes_a_boot_created_parent() {
        let ws = Workspace::from_build();
        fs::create_dir_all(ws.e2e_home()).expect("mkdir");
        let parent = tempfile::Builder::new()
            .prefix("viola-session-")
            .tempdir_in(ws.e2e_home())
            .expect("tempdir")
            .keep();
        fs::create_dir_all(parent.join("home").join("diagnostics")).expect("mkdir");
        assert!(remove_session_home(&ws, &parent.join("home")));
        assert!(!parent.exists());
    }

    #[test]
    fn killed_by_supervisor_reads_the_killed_names() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(killed_by_supervisor(tmp.path()).is_empty());
        let doc = json!({"exits": [
            {"name": "a", "killed": true},
            {"name": "b", "killed": false},
        ]});
        fs::write(tmp.path().join("supervisor-exit.json"), doc.to_string()).expect("write");
        assert_eq!(killed_by_supervisor(tmp.path()), vec!["a".to_owned()]);
    }
}
