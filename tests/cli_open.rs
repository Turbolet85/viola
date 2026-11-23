//! `viola open` over the fakes (test-plan · decision/fakes-for-external-services): the fake agent
//! planted as `claude`, as the stand-in `foot` and, where a case names it, as `hyprctl`, in a
//! test-scoped bin dir that is the open's whole `PATH`. The open spawns the stand-in with its closed
//! argv, the stand-in runs the pinned `viola run` as its direct child, and that wrapper finds its
//! window by the stand-in's pid, moves it and records it. Every case runs under its own home, user
//! home and socket root, and ends the stand-in terminal and the wrapper it started; every role-file
//! line the case's processes wrote is held to `schemas/diag-line.v1.json`.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use serde_json::{Value, json};
use support::cli::{Running, viola};
use support::fake::FAKE;
use support::home::{
    SOCKET_ROOT_VAR, SocketRoot, TestHome, USER_HOME_VAR, private_dir, process_start,
    snapshot_data, unwiden, widen, workspace_path,
};
use support::hygiene::load_schema;
use support::ndjson::read_lines;
use support::tied::{gone_within, recorded_tree};
use support::verify::{seed_user_home, trust_config_of, user_home_of};

/// The address the fake `hyprctl`'s `terminal` mode lists the stand-in's window at.
const ADDRESS: &str = "0x555aa16bde50";
const HOME_STATE: &str =
    "the viola home must be readable only by you; viola does not change its permissions";

/// How the case's bin dir is planted.
#[derive(Clone, Copy)]
enum Hyprctl {
    /// No `hyprctl`: off Hyprland.
    Absent,
    /// The fake `hyprctl` in the given mode over the recorded set.
    Mode(&'static str),
}

/// One case's world: its home, socket root, working folder and bin dir.
struct Case {
    home: TestHome,
    root: SocketRoot,
    work: PathBuf,
    bin: PathBuf,
}

impl Case {
    fn new(hyprctl: Hyprctl, foot: bool) -> Self {
        let home = TestHome::new();
        seed_user_home(home.path(), Some(&json!({})));
        let root = SocketRoot::new();
        let work = home.scratch().join("work dir");
        fs::create_dir_all(&work).expect("the working folder");
        let bin = home.scratch().join("bin");
        fs::create_dir_all(&bin).expect("the bin dir");
        let mut names = vec!["claude"];
        if foot {
            names.push("foot");
        }
        if let Hyprctl::Mode(mode) = hyprctl {
            let set = bin.join("hyprctl-set");
            fs::create_dir_all(&set).expect("the hyprctl set");
            for fixture in fs::read_dir(workspace_path("fixtures/hyprland/0.56.2")).expect("set") {
                let fixture = fixture.expect("a fixture").path();
                fs::copy(&fixture, set.join(fixture.file_name().expect("a name")))
                    .expect("the fixture copy");
            }
            fs::write(bin.join("hyprctl-mode"), mode).expect("the hyprctl mode");
            names.push("hyprctl");
        }
        let fixtures = workspace_path("fixtures/claude");
        let fixtures = fixtures.to_str().expect("utf-8 path");
        let config = trust_config_of(home.path());
        let config = config.to_str().expect("utf-8 path");
        fs::write(
            bin.join("fake-args"),
            format!("--fixtures\n{fixtures}\n--trust-config\n{config}\n"),
        )
        .expect("fake-args");
        for name in names {
            let planted = bin.join(name);
            if fs::hard_link(FAKE, &planted).is_err() {
                fs::copy(FAKE, &planted).expect("the fake copy");
            }
        }
        Self {
            home,
            root,
            work,
            bin,
        }
    }

    fn home(&self) -> &Path {
        self.home.path()
    }

    fn instance_dir(&self, name: &str) -> PathBuf {
        self.home().join("instances").join(name)
    }

    /// `viola --home <home> open <args…>` typed in the working folder, `PATH` the bin dir alone,
    /// `env` set beside it; waited, with the guard that ends what it started.
    fn open(&self, args: &[&str], env: &[(&str, &Path)]) -> (support::cli::Ran, Terminal) {
        let mut command = support::tied::viola();
        command
            .arg("--home")
            .arg(self.home())
            .arg("open")
            .args(args)
            .current_dir(&self.work)
            .env_remove("VIOLA_NAME")
            .env_remove("VIOLA_DIR")
            .env_remove("VIOLA_BIN")
            .env("PATH", &self.bin)
            .env(USER_HOME_VAR, user_home_of(self.home()))
            .env(SOCKET_ROOT_VAR, self.root.path())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (name, value) in env {
            command.env(name, value);
        }
        let ran = Running::over(command.spawn().expect("viola")).finish();
        (ran, Terminal(self.bin.clone()))
    }

    fn foot_argv(&self) -> Option<Vec<String>> {
        let text = fs::read_to_string(self.bin.join("foot-argv")).ok()?;
        Some(serde_json::from_str(&text).expect("the argv receipt"))
    }

    fn events(&self, name: &str) -> Vec<Value> {
        read_lines(&self.instance_dir(name).join("events.ndjson"))
    }

    /// Every line of every role file under the home's `diagnostics/`, each valid by the schema.
    fn assert_lines_schema_valid(&self) {
        let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
        let validator = jsonschema::validator_for(&schema).expect("valid schema");
        let Ok(files) = fs::read_dir(self.home().join("diagnostics")) else {
            return;
        };
        for file in files {
            for line in read_lines(&file.expect("a role file").path()) {
                assert!(validator.is_valid(&line), "{line}");
            }
        }
    }
}

/// The stand-in terminal a case's open started and its whole tree (the wrapper, its child), ended
/// at the case's end, failing or not: each process the test's own, by pid and start time.
struct Terminal(PathBuf);

impl Drop for Terminal {
    fn drop(&mut self) {
        let Some(pid) = fs::read_to_string(self.0.join("foot-pid"))
            .ok()
            .and_then(|text| text.trim().parse::<u32>().ok())
        else {
            return;
        };
        let Some(started) = process_start(pid) else {
            return;
        };
        let mut tree = recorded_tree(pid);
        tree.push((pid, started));
        for &(member, when) in &tree {
            if process_start(member) == Some(when)
                && let Ok(member) = libc::pid_t::try_from(member)
            {
                // SAFETY: a plain signal to a process this case's own open started, its start time
                // read just above.
                unsafe {
                    libc::kill(member, libc::SIGKILL);
                }
            }
        }
        gone_within(&tree, &[("foot", pid)]);
    }
}

/// The open's one `window` line, the line right after its start's `launch` line.
fn window_line(events: &[Value]) -> Value {
    let at = events
        .iter()
        .position(|e| e["kind"] == "launch")
        .expect("a launch line");
    let line = &events[at + 1];
    assert_eq!(line["kind"], "window", "{events:?}");
    assert_eq!(line["source"], "wrapper");
    line["data"].clone()
}

fn assert_live_with(ran: &support::cli::Ran, placement: &str) {
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, format!("child  live  {placement}\n"));
    assert_eq!(ran.stderr, "");
}

#[test]
fn open_spawns_the_terminal_with_the_closed_argv() {
    let case = Case::new(Hyprctl::Mode("terminal"), true);
    let (ran, _terminal) = case.open(
        &[
            "child",
            "--role",
            "worker",
            "--model",
            "haiku",
            "--effort",
            "low",
            "--workspace",
            "1",
            "--",
            "claude",
        ],
        &[],
    );
    assert_live_with(&ran, "placed");
    let snapshot = snapshot_data(&case.instance_dir("child")).expect("the snapshot");
    let home = fs::canonicalize(case.home()).expect("the home");
    let work = fs::canonicalize(&case.work).expect("the working folder");
    let want: Vec<String> = [
        "--app-id=viola.child",
        "--title=child",
        &format!("--working-directory={}", work.display()),
        snapshot["pinned_bin"].as_str().expect("the pinned copy"),
        "--home",
        home.to_str().expect("utf-8"),
        "run",
        "child",
        "--role",
        "worker",
        "--model",
        "haiku",
        "--effort",
        "low",
        "--origin",
        "cli",
        "--place",
        "1",
        "--",
        "claude",
    ]
    .map(str::to_owned)
    .into();
    assert_eq!(case.foot_argv(), Some(want));
    case.assert_lines_schema_valid();
}

#[test]
fn open_records_the_window_found_by_the_terminal_pid() {
    let case = Case::new(Hyprctl::Mode("terminal"), true);
    let (ran, _terminal) = case.open(&["child", "--workspace", "1", "--", "claude"], &[]);
    assert_live_with(&ran, "placed");
    let recorded = json!({"address": ADDRESS, "workspace": 1, "placement": "placed",
        "origin": "cli"});
    assert_eq!(window_line(&case.events("child")), recorded);
    let snapshot = snapshot_data(&case.instance_dir("child")).expect("the snapshot");
    assert_eq!(snapshot["window"], recorded);
    let listed = viola(case.home(), &["list", "--json"], None, None);
    assert_eq!(listed.code, Some(0), "{}", listed.stderr);
    let doc: Value = serde_json::from_str(&listed.stdout).expect("one document");
    let item = &doc["ok"]["items"][0];
    assert_eq!(item["name"], "child");
    assert_eq!(item["window"], json!({"address": ADDRESS, "workspace": 1}));
    assert_eq!(item["origin"], "cli");
    case.assert_lines_schema_valid();
}

#[test]
fn open_from_an_agent_shell_names_the_caller_as_parent() {
    let case = Case::new(Hyprctl::Mode("terminal"), true);
    let parent_dir = case.instance_dir("overseer");
    let name = PathBuf::from("overseer");
    let (ran, _terminal) = case.open(
        &["child", "--", "claude"],
        &[("VIOLA_NAME", &name), ("VIOLA_DIR", &parent_dir)],
    );
    assert_live_with(&ran, "unplaced");
    let events = case.events("child");
    let lineage = events
        .iter()
        .find(|e| e["kind"] == "lineage")
        .expect("the lineage line");
    assert_eq!(lineage["data"]["parent"], "overseer");
    assert_eq!(window_line(&events)["origin"], "agent");
    assert!(
        case.foot_argv()
            .expect("the argv")
            .windows(2)
            .any(|pair| pair == ["--parent", "overseer"])
    );
    case.assert_lines_schema_valid();
}

#[test]
fn open_with_no_target_is_unplaced() {
    let case = Case::new(Hyprctl::Mode("terminal"), true);
    let (ran, _terminal) = case.open(&["child", "--", "claude"], &[]);
    assert_live_with(&ran, "unplaced");
    assert_eq!(
        window_line(&case.events("child")),
        json!({"address": ADDRESS, "workspace": 2, "placement": "unplaced", "origin": "cli"})
    );
    case.assert_lines_schema_valid();
}

#[test]
fn open_off_hyprland_is_unsupported_and_live() {
    let case = Case::new(Hyprctl::Absent, true);
    let (ran, _terminal) = case.open(&["child", "--workspace", "1", "--", "claude"], &[]);
    assert_live_with(&ran, "unsupported");
    assert_eq!(
        window_line(&case.events("child")),
        json!({"address": null, "workspace": null, "placement": "unsupported", "origin": "cli"})
    );
    case.assert_lines_schema_valid();
}

/// The recorded `clients` lists windows, none of them the stand-in's: the wrapper polls its whole
/// window bound, records `window-unseen`, and still comes up within the open's live bound.
#[test]
fn open_with_no_window_of_its_terminal_is_window_unseen() {
    let case = Case::new(Hyprctl::Mode("recorded"), true);
    let (ran, _terminal) = case.open(&["child", "--workspace", "1", "--", "claude"], &[]);
    assert_live_with(&ran, "window-unseen");
    assert_eq!(
        window_line(&case.events("child")),
        json!({"address": null, "workspace": null, "placement": "window-unseen", "origin": "cli"})
    );
    case.assert_lines_schema_valid();
}

#[test]
fn open_json_document_is_the_one_ok_document() {
    let case = Case::new(Hyprctl::Mode("terminal"), true);
    let (ran, _terminal) = case.open(
        &["child", "--workspace", "1", "--json", "--", "claude"],
        &[],
    );
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(ran.stderr, "");
    assert_eq!(ran.stdout.lines().count(), 1, "{}", ran.stdout);
    assert!(ran.stdout.ends_with('\n'));
    let doc: Value = serde_json::from_str(&ran.stdout).expect("one document");
    assert_eq!(
        doc,
        json!({"v": 1, "ok": {"name": "child", "parent": null,
            "window": {"address": ADDRESS, "workspace": 1}, "placement": "placed"}})
    );
    case.assert_lines_schema_valid();
}

#[test]
fn open_without_foot_refuses_terminal_not_found_and_spawns_nothing() {
    let case = Case::new(Hyprctl::Mode("terminal"), false);
    let (ran, _terminal) = case.open(&["child", "--", "claude"], &[]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(
        ran.stderr,
        "unable: no foot terminal on PATH\nhint: install foot, the terminal viola open starts\n"
    );
    let (ran, _terminal) = case.open(&["child", "--json", "--", "claude"], &[]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(
        ran.stdout,
        "{\"v\":1,\"error\":\"terminal-not-found\",\"detail\":null}\n"
    );
    assert_eq!(ran.stderr, "");
    assert!(snapshot_data(&case.instance_dir("child")).is_none());
    assert!(case.foot_argv().is_none());
    let lines = read_lines(&case.home().join("diagnostics").join("cli-child.ndjson"));
    assert!(
        lines.iter().any(|l| l["event"] == "process-exit"
            && l["detail"] == "terminal-not-found"
            && l["exit_code"] == 1),
        "{lines:?}"
    );
    assert!(
        !lines.iter().any(|l| l["subject"] == "terminal"),
        "{lines:?}"
    );
    case.assert_lines_schema_valid();
}

#[test]
fn open_refuses_a_live_name_before_any_spawn() {
    let case = Case::new(Hyprctl::Mode("terminal"), true);
    let (ran, _terminal) = case.open(&["child", "--", "claude"], &[]);
    assert_live_with(&ran, "unplaced");
    let first = fs::read(case.bin.join("foot-argv")).expect("the first receipt");
    fs::remove_file(case.bin.join("foot-argv")).expect("the receipt cleared");
    let (again, second) = case.open(&["child", "--json", "--", "claude"], &[]);
    // The second open spawned nothing: its guard must not end the first open's terminal early.
    std::mem::forget(second);
    assert_eq!(again.code, Some(1));
    assert_eq!(
        again.stdout,
        "{\"v\":1,\"error\":\"already-live\",\"detail\":null}\n"
    );
    assert!(case.foot_argv().is_none(), "a second terminal was spawned");
    assert!(!first.is_empty());
    case.assert_lines_schema_valid();
}

#[test]
fn open_refuses_a_word_after_dashes_and_reads_nothing() {
    let case = Case::new(Hyprctl::Mode("terminal"), true);
    let (ran, _terminal) = case.open(&["child", "--", "claude", "--x; sh"], &[]);
    assert_eq!(ran.code, Some(2));
    assert_eq!(ran.stdout, "");
    assert!(
        ran.stderr.contains(
            "viola open starts claude alone: put 'claude' after '--' and give viola's own flags before it"
        ),
        "{}",
        ran.stderr
    );
    assert!(!ran.stderr.contains("--x"), "{}", ran.stderr);
    assert!(case.foot_argv().is_none());
    assert!(!case.home().exists(), "the home was touched");
}

#[test]
fn open_refuses_a_home_another_user_may_read() {
    let case = Case::new(Hyprctl::Mode("terminal"), true);
    private_dir(case.home(), "the home");
    widen(case.home());
    let (ran, _terminal) = case.open(&["child", "--", "claude"], &[]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(
        ran.stderr.lines().last(),
        Some(format!("hint: {HOME_STATE}").as_str())
    );
    let (ran, _terminal) = case.open(&["child", "--json", "--", "claude"], &[]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(
        ran.stdout,
        "{\"v\":1,\"error\":\"strict-modes-failed\",\"detail\":null}\n"
    );
    assert_eq!(ran.stderr, "");
    assert!(case.foot_argv().is_none());
    assert!(
        !case.home().join("diagnostics").exists(),
        "a log was opened"
    );
    unwiden(case.home());
}
