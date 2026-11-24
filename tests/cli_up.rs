//! `viola up` over the fakes (test-plan · decision/fakes-for-external-services): the fake agent
//! planted as `claude`, as the stand-in `foot` and as `hyprctl` in its `terminal` mode, in a
//! test-scoped bin dir that is every viola process's whole `PATH`. The pre-reboot fleet is made by
//! running viola: each instance opened by `viola open` (a recorded window) or started by `viola run`
//! in a stand-in terminal the case spawns itself (no window record), its wrapper then killed by the
//! pid its own snapshot records; the one hand-made change is each heartbeat's mtime, set before this
//! host's boot time as `/proc/stat` reads it. The `claude` copy answers `agents --json` malformed
//! (its existing mode) unless a case plants rows, so a revive reads no other revived instance's
//! shared fixture session as live. Every case runs under its own home, user home and socket root,
//! and ends every stand-in terminal `foot-spawns.ndjson` records and the tree under it.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime};

use serde_json::{Value, json};
use support::cli::{Ran, Running};
use support::events::wait_events;
use support::fake::FAKE;
use support::frozen::Frozen;
use support::home::{
    SOCKET_ROOT_VAR, SocketRoot, TestHome, USER_HOME_VAR, VIOLA, private_dir, process_start,
    snapshot_data, still_widened, unwiden, widen, workspace_path,
};
use support::hygiene::load_schema;
use support::ndjson::read_lines;
use support::start::{FIXTURE_SESSION, kill_wrapper, listed};
use support::terminal::end_trees;
use support::verify::{seed_user_home, trust_config_of, user_home_of};
use support::watch::{WITHIN, Watch};

const HOME_STATE: &str =
    "the viola home must be readable only by you; viola does not change its permissions";
/// The address the fake `hyprctl`'s `terminal` mode lists the stand-in's window at.
const ADDRESS: &str = "0x555aa16bde50";

/// One case's world: its home, socket root, working folder and bin dir.
struct Case {
    home: TestHome,
    root: SocketRoot,
    work: PathBuf,
    bin: PathBuf,
}

impl Case {
    fn new() -> Self {
        let home = TestHome::new();
        seed_user_home(home.path(), Some(&json!({})));
        let root = SocketRoot::new();
        let work = home.scratch().join("work dir");
        fs::create_dir_all(&work).expect("the working folder");
        let bin = home.scratch().join("bin");
        fs::create_dir_all(&bin).expect("the bin dir");
        let set = bin.join("hyprctl-set");
        fs::create_dir_all(&set).expect("the hyprctl set");
        for fixture in fs::read_dir(workspace_path("fixtures/hyprland/0.56.2")).expect("set") {
            let fixture = fixture.expect("a fixture").path();
            fs::copy(&fixture, set.join(fixture.file_name().expect("a name")))
                .expect("the fixture copy");
        }
        // The recorded move to workspace 1, its argv re-pointed at workspace 3: the same reply.
        let moved = fs::read_to_string(set.join("MoveToWorkspace.recorded-1.json"))
            .expect("the move fixture");
        fs::write(
            set.join("MoveToWorkspace.synthetic-3.json"),
            moved.replace("workspace = 1,", "workspace = 3,"),
        )
        .expect("the workspace-3 move");
        fs::write(bin.join("hyprctl-mode"), "terminal").expect("the hyprctl mode");
        fs::write(bin.join("agents-mode"), "malformed").expect("the agents mode");
        let fixtures = workspace_path("fixtures/claude");
        let config = trust_config_of(home.path());
        fs::write(
            bin.join("fake-args"),
            format!(
                "--fixtures\n{}\n--trust-config\n{}\n",
                fixtures.to_str().expect("utf-8 path"),
                config.to_str().expect("utf-8 path")
            ),
        )
        .expect("fake-args");
        for name in ["claude", "foot", "hyprctl"] {
            let planted = bin.join(name);
            fs::copy(FAKE, &planted).expect("the fake copy");
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

    /// A viola process typed in the working folder, `PATH` the bin dir alone, its streams piped.
    fn viola(&self, args: &[&str]) -> Command {
        let mut command = support::tied::viola();
        command
            .arg("--home")
            .arg(self.home())
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
        command
    }

    fn ran(&self, args: &[&str]) -> Ran {
        Running::over(self.viola(args).spawn().expect("viola")).finish()
    }

    /// `viola open <name> <flags…> -- claude`: live, in a window of its own, its session logged (the
    /// open returns at the live endpoint, before the agent's `session-start` line).
    fn open(&self, name: &str, flags: &[&str]) {
        let mut args = vec!["open", name];
        args.extend(flags);
        args.extend(["--", "claude"]);
        let ran = self.ran(&args);
        assert_eq!(ran.code, Some(0), "open {name}: {}", ran.stderr);
        wait_events(&self.instance_dir(name), "the session start", |lines| {
            lines.iter().any(|l| l["kind"] == "session-start")
        });
    }

    /// `viola run <name> -- claude` in a stand-in terminal the case spawns itself: no window step.
    fn hand_start(&self, name: &str) {
        let mut working = std::ffi::OsString::from("--working-directory=");
        working.push(&self.work);
        let mut foot = Command::new(self.bin.join("foot"));
        foot.arg(format!("--app-id=viola.{name}"))
            .arg(format!("--title={name}"))
            .arg(working)
            .arg(VIOLA)
            .arg("--home")
            .arg(self.home())
            .args(["run", name, "--", "claude"])
            .env_remove("VIOLA_NAME")
            .env_remove("VIOLA_DIR")
            .env_remove("VIOLA_BIN")
            .env("PATH", &self.bin)
            .env(USER_HOME_VAR, user_home_of(self.home()))
            .env(SOCKET_ROOT_VAR, self.root.path())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        // The stand-in outlives this handle; the case's `Terminals` guard ends it.
        drop(foot.spawn().expect("the stand-in terminal"));
        wait_events(&self.instance_dir(name), "the session start", |lines| {
            lines.iter().any(|l| l["kind"] == "session-start")
        });
    }

    /// `viola run <name> -- whoami` typed outside any terminal: a program that ends at once, so the
    /// instance logs no session.
    fn sessionless(&self, name: &str) {
        let ran = self.ran(&["run", name, "--", "/usr/bin/whoami"]);
        assert!(ran.code.is_some(), "the run ended");
        assert!(snapshot_data(&self.instance_dir(name)).is_some());
    }

    /// The wrapper of `name` killed by the pid its snapshot records, then its child gone.
    fn kill(&self, name: &str) {
        let data = snapshot_data(&self.instance_dir(name)).expect("the snapshot");
        let pid = u32::try_from(data["pid"].as_u64().expect("pid")).expect("u32");
        let child = data["child_pid"]
            .as_u64()
            .and_then(|pid| u32::try_from(pid).ok());
        let started = process_start(pid).expect("the wrapper runs");
        kill_wrapper(pid);
        let watch = Watch::start("killed");
        let deadline = Instant::now() + WITHIN;
        while process_start(pid) == Some(started)
            || child.is_some_and(|child| process_start(child).is_some())
        {
            watch.note("the wrapper or its child runs");
            watch.deadline_check(
                deadline,
                "the killed wrapper or its child outlived the kill",
            );
            std::thread::yield_now();
        }
    }

    /// The heartbeat of `name` set a minute before this host's boot.
    fn age_before_boot(&self, name: &str) {
        fs::OpenOptions::new()
            .append(true)
            .open(self.instance_dir(name).join("heartbeat"))
            .expect("the heartbeat")
            .set_modified(boot_time() - Duration::from_secs(60))
            .expect("mtime");
    }

    /// The fleet of `names`, opened or started, then each killed and aged: a reboot's leavings.
    fn rebooted(&self, names: &[&str]) {
        for name in names {
            self.kill(name);
            self.age_before_boot(name);
        }
    }

    /// `viola hook session-end` as `name`'s own hook, its wrapper gone: the line it appends.
    fn session_end(&self, name: &str) {
        let dir = fs::canonicalize(self.instance_dir(name)).expect("the instance dir");
        let mut child = support::tied::viola()
            .args(["hook", "session-end"])
            .env("VIOLA_NAME", name)
            .env("VIOLA_DIR", &dir)
            .env("PATH", &self.bin)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("viola hook");
        {
            use std::io::Write as _;
            let mut input = child.stdin.take().expect("stdin");
            input
                .write_all(
                    json!({"hook_event_name": "SessionEnd", "reason": "other"})
                        .to_string()
                        .as_bytes(),
                )
                .expect("payload");
        }
        let out = Running::over(child).finish();
        assert_eq!(out.code, Some(0));
        let lines = read_lines(&dir.join("events.ndjson"));
        assert_eq!(lines.last().expect("a line")["kind"], "session-end");
    }

    fn events(&self, name: &str) -> Vec<Value> {
        read_lines(&self.instance_dir(name).join("events.ndjson"))
    }

    /// Every stand-in spawn, in spawn order.
    fn spawns(&self) -> Vec<Value> {
        read_lines(&self.bin.join("foot-spawns.ndjson"))
    }

    fn argv_of(&self, spawn: &Value) -> Vec<String> {
        spawn["argv"]
            .as_array()
            .expect("an argv")
            .iter()
            .map(|word| word.as_str().expect("a word").to_owned())
            .collect()
    }

    /// The move dispatches the fake `hyprctl` was asked, in order.
    fn moves(&self) -> Vec<Vec<String>> {
        let Ok(text) = fs::read_to_string(self.bin.join("hyprctl-asked")) else {
            return Vec::new();
        };
        text.lines()
            .map(|line| serde_json::from_str(line).expect("one argv a line"))
            .collect()
    }

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

/// Every stand-in terminal the case's processes spawned and its whole tree, ended at the case's
/// end, failing or not: each process by pid and start time.
struct Terminals(PathBuf);

impl Drop for Terminals {
    fn drop(&mut self) {
        let mut roots = Vec::new();
        for spawn in read_lines(&self.0.join("foot-spawns.ndjson")) {
            let Some(pid) = spawn["pid"].as_u64().and_then(|p| u32::try_from(p).ok()) else {
                continue;
            };
            if let Some(started) = process_start(pid) {
                roots.push((pid, started));
            }
        }
        end_trees(&roots, &[]);
    }
}

/// This host's boot time from `/proc/stat`'s `btime` line.
fn boot_time() -> SystemTime {
    let stat = fs::read_to_string("/proc/stat").expect("/proc/stat");
    let secs: u64 = stat
        .lines()
        .find_map(|line| line.strip_prefix("btime "))
        .expect("a btime line")
        .trim()
        .parse()
        .expect("whole seconds");
    SystemTime::UNIX_EPOCH + Duration::from_secs(secs)
}

/// The lines of `name`'s log past the first `from`.
fn since(lines: Vec<Value>, from: usize) -> Vec<Value> {
    lines.into_iter().skip(from).collect()
}

#[test]
fn up_reopens_each_pre_boot_instance_parents_first() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    case.open("b-parent", &["--workspace", "1"]);
    case.open("a-child", &["--parent", "b-parent", "--workspace", "3"]);
    case.rebooted(&["b-parent", "a-child"]);
    let before = case.spawns().len();
    let logged: Vec<usize> = ["b-parent", "a-child"]
        .map(|name| case.events(name).len())
        .into();
    let ran = case.ran(&["up"]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "b-parent  live  placed\na-child  live  placed\n"
    );
    assert_eq!(ran.stderr, "");
    let spawns = since(case.spawns(), before);
    let home = fs::canonicalize(case.home()).expect("the home");
    let work = fs::canonicalize(&case.work).expect("the working folder");
    let order: Vec<Vec<String>> = spawns.iter().map(|s| case.argv_of(s)).collect();
    assert_eq!(order.len(), 2, "{order:?}");
    for (argv, name) in order.iter().zip(["b-parent", "a-child"]) {
        let pinned = snapshot_data(&case.instance_dir(name)).expect("the snapshot")["pinned_bin"]
            .as_str()
            .expect("the pinned copy")
            .to_owned();
        let want: Vec<String> = [
            format!("--app-id=viola.{name}"),
            format!("--title={name}"),
            format!("--working-directory={}", work.display()),
            pinned,
            "--home".to_owned(),
            home.to_str().expect("utf-8").to_owned(),
            "revive".to_owned(),
            name.to_owned(),
            "--origin".to_owned(),
            "cli".to_owned(),
        ]
        .into();
        assert_eq!(*argv, want);
    }
    for (name, from) in ["b-parent", "a-child"].into_iter().zip(logged) {
        let lines = wait_events(&case.instance_dir(name), "the resumed session", |l| {
            l[from..].iter().any(|e| e["kind"] == "session-start")
        });
        let kinds: Vec<&str> = lines[from..]
            .iter()
            .map(|e| e["kind"].as_str().expect("a kind"))
            .collect();
        assert_eq!(
            kinds[..5],
            ["wheel", "lineage", "launch", "window", "session-start"],
            "{name}"
        );
        assert_eq!(
            lines[from]["data"],
            json!({"holder": "driver", "cause": "start"})
        );
        let resumed = &lines[from + 4]["data"];
        assert_eq!(resumed["cause"], "resume", "{name}");
        assert_eq!(resumed["agent_session_id"], FIXTURE_SESSION, "{name}");
    }
    let child_lineage = &case.events("a-child")[logged_lineage(&case, "a-child")];
    assert_eq!(child_lineage["data"]["parent"], "b-parent");
    case.assert_lines_schema_valid();
}

/// The index of the last `lineage` line in `name`'s log.
fn logged_lineage(case: &Case, name: &str) -> usize {
    case.events(name)
        .iter()
        .rposition(|e| e["kind"] == "lineage")
        .expect("a lineage line")
}

#[test]
fn up_moves_each_window_to_its_recorded_workspace() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    case.open("b-parent", &["--workspace", "1"]);
    case.open("a-child", &["--parent", "b-parent", "--workspace", "3"]);
    case.rebooted(&["b-parent", "a-child"]);
    let asked = case.moves().len();
    let ran = case.ran(&["up"]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    let move_to = |n: i32| {
        vec![
            "dispatch".to_owned(),
            format!("hl.dsp.window.move({{ workspace = {n}, window = \"address:{ADDRESS}\" }})"),
        ]
    };
    assert_eq!(case.moves()[asked..], [move_to(1), move_to(3)]);
    for (name, workspace) in [("b-parent", 1), ("a-child", 3)] {
        let window = &snapshot_data(&case.instance_dir(name)).expect("the snapshot")["window"];
        assert_eq!(
            *window,
            json!({"address": ADDRESS, "workspace": workspace, "placement": "placed",
                "origin": "cli"}),
            "{name}"
        );
    }
    case.assert_lines_schema_valid();
}

#[test]
fn up_opens_a_hand_started_instance_unplaced() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    case.hand_start("hand");
    assert!(
        snapshot_data(&case.instance_dir("hand")).expect("the snapshot")["window"].is_null(),
        "a hand start records no window"
    );
    case.rebooted(&["hand"]);
    let asked = case.moves().len();
    let ran = case.ran(&["up"]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, "hand  live  unplaced\n");
    assert_eq!(case.moves().len(), asked, "a hand-started window was moved");
    let window = &snapshot_data(&case.instance_dir("hand")).expect("the snapshot")["window"];
    assert_eq!(window["placement"], "unplaced");
    assert_eq!(window["origin"], "cli");
    case.assert_lines_schema_valid();
}

#[test]
fn up_skips_a_live_and_a_stale_instance() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    case.open("live-one", &[]);
    case.open("stale-one", &[]);
    let pid = snapshot_data(&case.instance_dir("stale-one")).expect("the snapshot")["pid"]
        .as_u64()
        .and_then(|pid| u32::try_from(pid).ok())
        .expect("the pid");
    let frozen = Frozen::attach(pid);
    fs::OpenOptions::new()
        .append(true)
        .open(case.instance_dir("stale-one").join("heartbeat"))
        .expect("the heartbeat")
        .set_modified(SystemTime::now() - Duration::from_secs(10))
        .expect("mtime");
    let before = case.spawns().len();
    let ran = case.ran(&["up"]);
    drop(frozen);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "live-one  already-live\nstale-one  stale-heartbeat\n"
    );
    assert_eq!(ran.stderr, "");
    assert_eq!(
        case.spawns().len(),
        before,
        "a skipped instance was spawned"
    );
    case.assert_lines_schema_valid();
}

#[test]
fn up_leaves_out_an_instance_ended_on_purpose_or_this_boot() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    case.open("e-ended", &["--workspace", "1"]);
    case.open("t-this-boot", &[]);
    case.kill("e-ended");
    case.session_end("e-ended");
    case.age_before_boot("e-ended");
    case.kill("t-this-boot");
    let before = case.spawns().len();
    let ran = case.ran(&["up"]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, "");
    assert_eq!(
        case.spawns().len(),
        before,
        "a left-out instance was spawned"
    );
    let ran = case.ran(&["up", "e-ended", "t-this-boot"]);
    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "t-this-boot  not-taken\ne-ended  live  placed\n"
    );
    let spawned = since(case.spawns(), before);
    assert_eq!(spawned.len(), 1);
    assert!(case.argv_of(&spawned[0]).contains(&"e-ended".to_owned()));
    case.assert_lines_schema_valid();
}

#[test]
fn up_names_no_session_cwd_missing_and_session_live() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    let gone = case.home.scratch().join("gone dir");
    fs::create_dir(&gone).expect("the recorded dir");
    case.open("a-ok", &["--workspace", "1"]);
    case.open("c-cwd", &["--cwd", gone.to_str().expect("utf-8")]);
    case.sessionless("n-session");
    case.rebooted(&["a-ok", "c-cwd"]);
    case.age_before_boot("n-session");
    fs::remove_dir(&gone).expect("the recorded dir removed");
    let before = case.spawns().len();
    let ran = case.ran(&["up"]);
    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "c-cwd  cwd-missing\nn-session  no-session\na-ok  live  placed\n"
    );
    assert_eq!(ran.stderr, "");
    assert_eq!(since(case.spawns(), before).len(), 1);

    let other = Case::new();
    let _others = Terminals(other.bin.clone());
    other.open("s-live", &[]);
    other.rebooted(&["s-live"]);
    fs::write(other.bin.join("agents-mode"), "recorded").expect("the agents mode");
    fs::write(other.bin.join("agents-rows.json"), listed(FIXTURE_SESSION)).expect("the rows");
    let before = other.spawns().len();
    let ran = other.ran(&["up", "--json"]);
    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr);
    assert_eq!(
        ran.stdout,
        "{\"v\":1,\"ok\":{\"instances\":[{\"name\":\"s-live\",\"outcome\":\"session-live\",\"placement\":null}]}}\n"
    );
    assert_eq!(other.spawns().len(), before);
    case.assert_lines_schema_valid();
}

#[test]
fn up_dry_run_prints_the_plan_and_spawns_nothing() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    case.open("b-parent", &["--workspace", "1"]);
    case.open("a-child", &["--parent", "b-parent"]);
    case.rebooted(&["b-parent", "a-child"]);
    let before = case.spawns().len();
    let snapshot = fs::read(case.instance_dir("a-child").join("snapshot.json")).expect("snapshot");
    let ran = case.ran(&["up", "--dry-run"]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, "b-parent  would-open\na-child  would-open\n");
    assert_eq!(case.spawns().len(), before, "a dry run spawned");
    assert_eq!(
        fs::read(case.instance_dir("a-child").join("snapshot.json")).expect("snapshot"),
        snapshot
    );
}

#[test]
fn up_json_is_one_document_of_closed_words() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    case.open("l-live", &[]);
    case.open("c-gone", &["--workspace", "1"]);
    case.rebooted(&["c-gone"]);
    let ran = case.ran(&["up", "--json"]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(ran.stderr, "");
    assert_eq!(ran.stdout.lines().count(), 1, "{}", ran.stdout);
    let doc: Value = serde_json::from_str(&ran.stdout).expect("one document");
    assert_eq!(
        doc,
        json!({"v": 1, "ok": {"instances": [
            {"name": "l-live", "outcome": "already-live", "placement": null},
            {"name": "c-gone", "outcome": "live", "placement": "placed"},
        ]}})
    );
    assert!(!ran.stdout.contains(ADDRESS), "an address on the document");
}

#[test]
fn up_refuses_a_home_another_user_may_read_with_no_line() {
    let case = Case::new();
    private_dir(case.home(), "the home");
    widen(case.home());
    let ran = case.ran(&["up"]);
    assert_eq!(ran.code, Some(21));
    assert_eq!(ran.stdout, "");
    assert_eq!(
        ran.stderr.lines().last(),
        Some(format!("hint: {HOME_STATE}").as_str())
    );
    let ran = case.ran(&["up", "--json"]);
    assert_eq!(ran.code, Some(21));
    assert_eq!(
        ran.stdout,
        "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"strict-modes-failed\"}\n"
    );
    assert_eq!(ran.stderr, "");
    assert!(
        !case.home().join("diagnostics").exists(),
        "a log was opened"
    );
    assert!(still_widened(case.home()), "the refusal changed the mode");
    unwiden(case.home());
}

#[test]
fn up_without_foot_refuses_terminal_not_found_and_spawns_nothing() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    case.open("gone", &[]);
    case.rebooted(&["gone"]);
    fs::remove_file(case.bin.join("foot")).expect("foot removed");
    let before = case.spawns().len();
    let ran = case.ran(&["up"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stdout, "");
    assert_eq!(
        ran.stderr,
        "unable: no foot terminal on PATH\nhint: install foot, the terminal viola open starts\n"
    );
    let ran = case.ran(&["up", "--json"]);
    assert_eq!(ran.code, Some(1));
    assert_eq!(
        ran.stdout,
        "{\"v\":1,\"error\":\"terminal-not-found\",\"detail\":null}\n"
    );
    assert_eq!(ran.stderr, "");
    assert_eq!(case.spawns().len(), before);
}

#[test]
fn up_writes_neither_snapshot_nor_stamps() {
    let case = Case::new();
    let _terminals = Terminals(case.bin.clone());
    case.open("live-one", &["--workspace", "1"]);
    let snapshot = case.instance_dir("live-one").join("snapshot.json");
    let stamps = case.home().join("ledger").join("stamps.json");
    let snapshot_before = fs::read(&snapshot).expect("the snapshot");
    let stamps_before = fs::read(&stamps).ok();
    let events_before = fs::read(case.instance_dir("live-one").join("events.ndjson")).expect("log");
    let ran = case.ran(&["up"]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, "live-one  already-live\n");
    assert_eq!(fs::read(&snapshot).expect("the snapshot"), snapshot_before);
    assert_eq!(fs::read(&stamps).ok(), stamps_before);
    assert_eq!(
        fs::read(case.instance_dir("live-one").join("events.ndjson")).expect("log"),
        events_before
    );
}
