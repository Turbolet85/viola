//! `viola focus` over the fakes (test-plan · decision/fakes-for-external-services): the fake agent
//! planted as `claude`, as the stand-in `foot` and, where a case names it, as `hyprctl`, in a
//! test-scoped bin dir that is every child's whole `PATH`. A session is opened by `viola open`, or
//! started by hand in the stand-in terminal, or started with no terminal at all; the focus reads
//! the compositor's windows through the fake `hyprctl`, whose `hyprctl-asked` receipt holds each
//! dispatch it was asked. Every case runs under its own home, user home and socket root, and ends
//! the terminal and the wrapper it started; every role-file line is held to
//! `schemas/diag-line.v1.json`.

#[allow(dead_code)]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use serde_json::{Value, json};
use support::cli::{Ran, Running};
use support::events::wait_events;
use support::fake::FAKE;
use support::home::{
    SOCKET_ROOT_VAR, SocketRoot, TestHome, USER_HOME_VAR, VIOLA, grant_read, private_dir,
    process_start, still_widened, unwiden, workspace_path,
};
use support::hygiene::load_schema;
use support::ndjson::read_lines;
use support::tied::{gone_within, recorded_tree, tie};
use support::verify::{seed_user_home, trust_config_of, user_home_of};

/// The dispatches the fake `hyprctl`'s `terminal` mode answers on the stand-in's window: its
/// workspace shown, then the window focused.
const WORKSPACE_LINE: &str = r#"["dispatch","hl.dsp.focus({ workspace = 2 })"]"#;
const WINDOW_LINE: &str = r#"["dispatch","hl.dsp.focus({ window = \"address:0x555aa16bde50\" })"]"#;
const INSTANCE_STATE: &str =
    "hint: viola will not read them; make the viola home and its files owner-only";

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
    fn new(hyprctl: Hyprctl) -> Self {
        let home = TestHome::new();
        seed_user_home(home.path(), Some(&json!({})));
        let root = SocketRoot::new();
        let work = home.scratch().join("work dir");
        fs::create_dir_all(&work).expect("the working folder");
        let bin = home.scratch().join("bin");
        fs::create_dir_all(&bin).expect("the bin dir");
        let mut names = vec!["claude", "foot"];
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

    /// `program` tied, in the working folder, `PATH` the bin dir alone, no inherited instance.
    fn command(&self, program: &Path) -> Command {
        let mut command = Command::new(program);
        tie(&mut command);
        command
            .current_dir(&self.work)
            .env_remove("VIOLA_NAME")
            .env_remove("VIOLA_DIR")
            .env_remove("VIOLA_BIN")
            .env("PATH", &self.bin)
            .env(USER_HOME_VAR, user_home_of(self.home()))
            .env(SOCKET_ROOT_VAR, self.root.path())
            .stdin(Stdio::null());
        command
    }

    /// `viola --home <home> <args…>`, waited.
    fn viola(&self, args: &[&str]) -> Ran {
        let mut command = self.command(Path::new(VIOLA));
        command
            .arg("--home")
            .arg(self.home())
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        Running::over(command.spawn().expect("viola")).finish()
    }

    fn focus(&self, args: &[&str]) -> Ran {
        let mut words = vec!["focus"];
        words.extend_from_slice(args);
        self.viola(&words)
    }

    /// `name` opened by `viola open`, live, with its terminal's guard.
    fn open(&self, name: &str) -> Started {
        let ran = self.viola(&["open", name, "--", "claude"]);
        assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
        Started::terminal(&self.bin)
    }

    /// `name` started by hand in the stand-in terminal, with no `--origin`, once live.
    fn hand_started(&self, name: &str) -> Started {
        let mut command = self.command(&self.bin.join("foot"));
        command
            .arg(VIOLA)
            .arg("--home")
            .arg(self.home())
            .args(["run", name, "--", "claude"])
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let foot = command.spawn().expect("the stand-in terminal");
        let started = Started {
            root: Some(foot.id()),
            child: Some(foot),
        };
        self.wait_live(name);
        started
    }

    /// `name` started with no terminal around it, once live.
    fn bare_started(&self, name: &str) -> Started {
        let mut command = self.command(Path::new(VIOLA));
        command
            .arg("--home")
            .arg(self.home())
            .args(["run", name, "--", "claude"])
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let wrapper = command.spawn().expect("the wrapper");
        let started = Started {
            root: Some(wrapper.id()),
            child: Some(wrapper),
        };
        self.wait_live(name);
        started
    }

    fn wait_live(&self, name: &str) {
        wait_events(
            &self.instance_dir(name),
            "the session-start record",
            |lines| lines.iter().any(|e| e["kind"] == "session-start"),
        );
    }

    /// The `hyprctl-asked` receipt's lines, none when no dispatch was asked.
    fn asked(&self) -> Vec<String> {
        fs::read_to_string(self.bin.join("hyprctl-asked"))
            .map(|text| text.lines().map(str::to_owned).collect())
            .unwrap_or_default()
    }

    fn set_mode(&self, mode: &str) {
        fs::write(self.bin.join("hyprctl-mode"), mode).expect("the hyprctl mode");
    }

    fn cli_lines(&self, name: &str) -> Vec<Value> {
        read_lines(
            &self
                .home()
                .join("diagnostics")
                .join(format!("cli-{name}.ndjson")),
        )
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

/// What a case started — the stand-in terminal (its pid from `foot-pid`) or a bare wrapper — and
/// its whole tree, ended at the case's end, failing or not: each process the test's own, by pid
/// and start time.
struct Started {
    root: Option<u32>,
    child: Option<Child>,
}

impl Started {
    fn terminal(bin: &Path) -> Self {
        let pid = fs::read_to_string(bin.join("foot-pid"))
            .ok()
            .and_then(|text| text.trim().parse::<u32>().ok());
        Self {
            root: pid,
            child: None,
        }
    }

    fn end(&mut self) {
        if let Some(pid) = self.root.take()
            && let Some(started) = process_start(pid)
        {
            let mut tree = recorded_tree(pid);
            tree.push((pid, started));
            for &(member, when) in &tree {
                if process_start(member) == Some(when)
                    && let Ok(member) = libc::pid_t::try_from(member)
                {
                    // SAFETY: a plain signal to a process this case started, its start time read
                    // just above.
                    unsafe {
                        libc::kill(member, libc::SIGKILL);
                    }
                }
            }
            if let Some(mut child) = self.child.take() {
                let _ = child.wait();
            }
            gone_within(&tree, &[("started", pid)]);
        }
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

impl Drop for Started {
    fn drop(&mut self) {
        self.end();
    }
}

fn assert_focused(ran: &Ran, name: &str) {
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr);
    assert_eq!(ran.stdout, format!("{name}  focused\n"));
    assert_eq!(ran.stderr, "");
}

fn assert_refused(ran: &Ran, unable: &str, hint: &str) {
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout);
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr, format!("unable: {unable}\nhint: {hint}\n"));
}

/// One `--json` document, alone on stdout, with no escape, path, address or workspace in either
/// stream.
fn assert_one_clean_document(ran: &Ran, want: &Value) {
    assert_eq!(ran.stderr, "");
    assert_eq!(ran.stdout.lines().count(), 1, "{}", ran.stdout);
    assert!(ran.stdout.ends_with('\n'));
    for stream in [&ran.stdout, &ran.stderr] {
        assert!(!stream.contains('\x1b'), "{stream}");
        assert!(!stream.contains('/'), "{stream}");
        assert!(!stream.contains("0x"), "{stream}");
        assert!(!stream.contains("workspace"), "{stream}");
    }
    let doc: Value = serde_json::from_str(&ran.stdout).expect("one document");
    assert_eq!(&doc, want);
}

fn exit_detail(lines: &[Value]) -> Option<(Value, Value)> {
    lines
        .iter()
        .rev()
        .find(|l| l["event"] == "process-exit" && l["subject"] == "self")
        .map(|l| (l["exit_code"].clone(), l["detail"].clone()))
}

#[test]
fn focus_of_an_opened_session_raises_its_recorded_window() {
    let case = Case::new(Hyprctl::Mode("terminal"));
    let _terminal = case.open("child");
    assert!(case.asked().is_empty(), "{:?}", case.asked());
    assert_focused(&case.focus(&["child"]), "child");
    assert_eq!(case.asked(), [WORKSPACE_LINE, WINDOW_LINE]);
    assert_eq!(
        exit_detail(&case.cli_lines("child")),
        Some((json!(0), Value::Null))
    );
    case.assert_lines_schema_valid();
}

#[test]
fn focus_of_a_hand_started_session_finds_its_window_by_ancestry() {
    let case = Case::new(Hyprctl::Mode("terminal"));
    let _terminal = case.hand_started("hand");
    let snapshot = support::home::snapshot_data(&case.instance_dir("hand")).expect("snapshot");
    assert!(
        snapshot.get("window").is_none_or(Value::is_null),
        "{snapshot}"
    );
    assert_focused(&case.focus(&["hand"]), "hand");
    assert_eq!(case.asked(), [WORKSPACE_LINE, WINDOW_LINE]);
    case.assert_lines_schema_valid();
}

#[test]
fn focus_with_no_window_of_its_session_is_window_not_found() {
    let case = Case::new(Hyprctl::Mode("terminal"));
    let _wrapper = case.bare_started("bare");
    assert_refused(
        &case.focus(&["bare"]),
        "no window of bare was found",
        "start bare in its own terminal window, or open it with viola open",
    );
    assert_one_clean_document(
        &case.focus(&["bare", "--json"]),
        &json!({"v": 1, "error": "window-not-found", "detail": null}),
    );
    assert!(case.asked().is_empty(), "{:?}", case.asked());
    assert_eq!(
        exit_detail(&case.cli_lines("bare")),
        Some((json!(1), json!("window-not-found")))
    );
    case.assert_lines_schema_valid();
}

#[test]
fn focus_off_hyprland_is_unsupported() {
    let case = Case::new(Hyprctl::Absent);
    let _wrapper = case.bare_started("bare");
    assert_refused(
        &case.focus(&["bare"]),
        "no hyprctl on PATH",
        "viola focus moves windows on Hyprland; run it in a Hyprland session",
    );
    let lines = case.cli_lines("bare");
    assert!(
        !lines.iter().any(|l| l["subject"] == "hyprctl-probe"),
        "{lines:?}"
    );
    assert_eq!(exit_detail(&lines), Some((json!(1), json!("unsupported"))));
    case.assert_lines_schema_valid();
}

#[test]
fn focus_with_no_compositor_answering_is_unreachable() {
    let case = Case::new(Hyprctl::Mode("absent"));
    let _wrapper = case.bare_started("bare");
    assert_refused(
        &case.focus(&["bare"]),
        "Hyprland did not answer",
        "run hyprctl version to check the compositor answers",
    );
    assert!(case.asked().is_empty(), "{:?}", case.asked());
    assert_eq!(
        exit_detail(&case.cli_lines("bare")),
        Some((json!(1), json!("unreachable")))
    );
    case.assert_lines_schema_valid();
}

#[test]
fn focus_of_a_dead_or_unknown_name_is_unreachable_before_any_hyprctl_read() {
    let case = Case::new(Hyprctl::Mode("terminal"));
    let ran = case.focus(&["ghost", "--json"]);
    assert_eq!(ran.code, Some(21));
    assert_eq!(
        ran.stdout,
        "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"unwrapped-name\"}\n"
    );
    let mut wrapper = case.bare_started("gone");
    wrapper.end();
    let ran = case.focus(&["gone"]);
    assert_eq!(ran.code, Some(21));
    assert_eq!(ran.stdout, "");
    assert!(
        ran.stderr.starts_with("unable  gone  instance-unreachable"),
        "{}",
        ran.stderr
    );
    let ran = case.focus(&["gone", "--json"]);
    assert_eq!(ran.code, Some(21));
    assert_eq!(
        ran.stdout,
        "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"instance-dead\"}\n"
    );
    for name in ["ghost", "gone"] {
        let lines = case.cli_lines(name);
        assert!(
            !lines.iter().any(|l| l["subject"] == "hyprctl-probe"),
            "{lines:?}"
        );
        assert_eq!(exit_detail(&lines).map(|(code, _)| code), Some(json!(21)));
    }
    assert!(case.asked().is_empty(), "{:?}", case.asked());
    case.assert_lines_schema_valid();
}

#[test]
fn focus_refuses_a_home_another_user_may_read() {
    let case = Case::new(Hyprctl::Mode("terminal"));
    private_dir(case.home(), "the home");
    grant_read(case.home());
    let ran = case.focus(&["builder"]);
    assert_eq!(ran.code, Some(21));
    assert_eq!(ran.stdout, "");
    assert_eq!(ran.stderr.lines().last(), Some(INSTANCE_STATE));
    let ran = case.focus(&["builder", "--json"]);
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
    assert!(still_widened(case.home()), "the home's mode was changed");
    unwiden(case.home());
}

#[test]
fn focus_json_documents_are_one_document_each() {
    let case = Case::new(Hyprctl::Mode("terminal"));
    let _terminal = case.hand_started("hand");
    assert_one_clean_document(
        &case.focus(&["hand", "--json"]),
        &json!({"v": 1, "ok": {"name": "hand"}}),
    );
    for (mode, code) in [
        ("absent", "unreachable"),
        ("oversize", "oversize"),
        ("malformed", "malformed"),
    ] {
        case.set_mode(mode);
        let ran = case.focus(&["hand", "--json"]);
        assert_eq!(ran.code, Some(1), "{mode}");
        assert_one_clean_document(&ran, &json!({"v": 1, "error": code, "detail": null}));
    }
    fs::remove_file(case.bin.join("hyprctl")).expect("hyprctl taken off PATH");
    let ran = case.focus(&["hand", "--json"]);
    assert_eq!(ran.code, Some(1));
    assert_one_clean_document(
        &ran,
        &json!({"v": 1, "error": "unsupported", "detail": null}),
    );
    case.assert_lines_schema_valid();
}

#[test]
fn focus_writes_no_snapshot_or_event_line() {
    let case = Case::new(Hyprctl::Mode("terminal"));
    let _terminal = case.hand_started("hand");
    let dir = case.instance_dir("hand");
    let before = (
        fs::read(dir.join("snapshot.json")).expect("the snapshot"),
        fs::read(dir.join("events.ndjson")).expect("the events log"),
    );
    assert_focused(&case.focus(&["hand"]), "hand");
    let after = (
        fs::read(dir.join("snapshot.json")).expect("the snapshot"),
        fs::read(dir.join("events.ndjson")).expect("the events log"),
    );
    assert!(
        before == after,
        "a focus moved the snapshot or the events log"
    );
}
