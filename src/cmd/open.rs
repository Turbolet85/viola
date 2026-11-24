//! `viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>]
//! [--workspace N] [--model M] [--effort E] [--json] -- claude`: a `foot` terminal window started
//! directly, no shell, running the pinned `viola run <name> … -- claude`, then a wait until the
//! instance is live. The flags are a closed allowlist and the words after `--` are `claude` alone,
//! each checked before the home is read. The open computes the target workspace and hands it with
//! its origin to the started wrapper, which finds its own window, moves it and records it: the
//! open writes no snapshot and no event line.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use tracing::instrument;
use viola_core::obs::ObsEvent;
use viola_core::{ViolaName, obs_event};
use viola_hyprland::{HyprlandError, Reply, Request};
use viola_state::liveness::{ancestors, live_endpoint};
use viola_state::pin::{PinError, pin_bytes};
use viola_state::replay::{Recovered, read_snapshot_or_replay};
use viola_state::snapshot::Window;
use viola_state::strict::check_instance;

use super::client::{self, own_name};
use super::run::{Started, checked_exe, collision_check, refuse_tampered_pin, refuse_tree};
use crate::{human, run};

/// How long the open waits for the instance to be live.
pub(crate) const OPEN_LIVE_BOUND: Duration = Duration::from_secs(30);
const LIVE_POLL: Duration = Duration::from_millis(100);
/// How many of the parent wrapper's ancestors are matched against the compositor's windows.
const ANCESTRY: usize = 8;
pub(super) const TERMINAL: &str = "foot";
const PROGRAM: &str = "claude";

/// The usage error of any word after `--` but `claude` alone: one fixed line, none of the words.
pub(crate) const OPEN_PROGRAM: &str = "viola open starts claude alone: put 'claude' after '--' and give viola's own flags before it\n";

#[derive(clap::Args)]
pub(crate) struct OpenArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// The session this one is opened for, a name; else the opening session's own
    #[arg(long, value_name = "NAME", value_parser = super::send::parse_name)]
    parent: Option<ViolaName>,
    /// This session's role
    #[arg(long, value_name = "ROLE", value_parser = ["overseer", "builder", "worker"])]
    role: Option<String>,
    /// The folder the session starts in; else this one
    #[arg(long, value_name = "DIR")]
    cwd: Option<PathBuf>,
    /// The workspace the window goes to; else the parent window's
    #[arg(long, value_name = "N", allow_negative_numbers = true)]
    workspace: Option<i32>,
    /// The model the session starts on: an alias or a full name; else the configured default
    #[arg(long, value_name = "MODEL", value_parser = super::run::parse_model)]
    model: Option<String>,
    /// The effort the session starts on: low, medium, high, xhigh or max; else the configured default
    #[arg(long, value_name = "EFFORT", value_parser = super::run::parse_effort)]
    effort: Option<String>,
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    json: bool,
    /// claude, after `--`
    #[arg(last = true, required = true)]
    program: Vec<OsString>,
}

/// The arguments clap cannot check alone: a word after `--` other than `claude` alone, and a
/// `--parent` naming the session itself, are usage errors.
fn checked(args: OpenArgs) -> Result<OpenArgs, clap::Error> {
    if args.program != [OsStr::new(PROGRAM)] {
        return Err(clap::Error::raw(
            clap::error::ErrorKind::ArgumentConflict,
            OPEN_PROGRAM,
        ));
    }
    if args.parent.as_ref() == Some(&args.name) {
        return Err(clap::Error::raw(
            clap::error::ErrorKind::ArgumentConflict,
            "the argument '--parent <NAME>' cannot name the session itself\n",
        ));
    }
    Ok(args)
}

pub(crate) fn open(home: &Path, args: OpenArgs) -> anyhow::Result<ExitCode> {
    let args = checked(args).unwrap_or_else(|usage| usage.exit());
    let exe = match checked_exe(home, &args.name)? {
        Ok(exe) => exe,
        Err(refused) => {
            if args.json {
                client::refused_document("strict-modes-failed");
            } else {
                refuse_tree(&args.name, refused);
            }
            return Ok(ExitCode::from(1));
        }
    };
    // The log's open created the home: from here on its one canonical spelling, as `run`'s.
    let home = super::canonical_home(&client::start(home, &args.name)?);
    opened(&home, &args, &exe)
}

#[instrument(skip_all, name = "cli.open")]
fn opened(home: &Path, args: &OpenArgs, exe: &[u8]) -> anyhow::Result<ExitCode> {
    let name: &str = args.name.as_ref();
    let instance_dir = home.join("instances").join(name);
    let recovered = read_snapshot_or_replay(&instance_dir).ok();
    if let Some(Started::Refused(code)) =
        collision_check(&args.name, &instance_dir, recovered.as_ref(), args.json)
    {
        return Ok(code);
    }
    let pinned = match pin_bytes(home, exe) {
        Ok(pinned) => pinned,
        Err(PinError::HashMismatch) => {
            return Ok(refused(args.json, "pinned-hash-mismatch", || {
                refuse_tampered_pin();
            }));
        }
        Err(error) => return Err(error.into()),
    };
    let caller = own_name();
    let origin = if caller.is_some() { "agent" } else { "cli" };
    let parent = args
        .parent
        .clone()
        .or_else(|| caller.filter(|own| own != &args.name));
    let here = std::env::current_dir()?;
    let (persistent, _) = run::persistent_names(home);
    let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), &persistent);
    let target = args.workspace.or_else(|| {
        let parent = parent.as_ref()?;
        let hyprctl = run::resolve_program(OsStr::new(viola_hyprland::PROGRAM), &here).ok();
        let mut read = |request: &Request| {
            viola_hyprland::read(hyprctl.as_deref(), request, &strip.remove).reply
        };
        parent_target(parent, home, &mut read)
    });
    let Ok(foot) = run::resolve_program(OsStr::new(TERMINAL), &here) else {
        return Ok(refused(args.json, "terminal-not-found", || {
            let (unable, hint) = human::terminal_not_found_pair();
            human::refuse(unable, hint);
        }));
    };
    let cwd = match &args.cwd {
        Some(cwd) => std::path::absolute(cwd)?,
        None => here,
    };
    let argv = open_argv(&Opened {
        name: &args.name,
        cwd: &cwd,
        pinned: &pinned.path,
        home,
        parent: parent.as_ref(),
        role: args.role.as_deref(),
        model: args.model.as_deref(),
        effort: args.effort.as_deref(),
        origin,
        place: target,
    });
    let terminal = spawn_terminal(&foot, &argv)?;
    if !came_up(&instance_dir, terminal) {
        return Ok(refused(args.json, "not-live", || {
            let (unable, hint) = human::not_live_pair(name);
            human::refuse(&unable, &hint);
        }));
    }
    let window = recorded_window(&instance_dir);
    let placement = window
        .as_ref()
        .map_or("unknown", |window| window.placement.as_str());
    if args.json {
        client::document(&json!({"v": 1, "ok": {
            "name": name,
            "parent": parent.as_ref().map(|p| p.as_ref().to_owned()),
            "window": window_document(window.as_ref()),
            "placement": placement,
        }}));
    } else {
        let _ = human::write_result(
            &mut std::io::stdout().lock(),
            &format!("{name}  live  {placement}"),
        );
    }
    run::log_self_exit(0, None);
    Ok(ExitCode::SUCCESS)
}

/// A refusal of the open, before or after its spawn: the stderr pair, or with `json` the code's
/// one document; one `process-exit` either way.
fn refused(json: bool, detail: &'static str, human: impl FnOnce()) -> ExitCode {
    if json {
        client::refused_document(detail);
    } else {
        human();
    }
    run::log_self_exit(1, Some(detail));
    ExitCode::from(1)
}

fn window_document(window: Option<&Window>) -> Value {
    window
        .and_then(|window| {
            let address = window.address.as_ref()?;
            Some(json!({"address": address, "workspace": window.workspace}))
        })
        .unwrap_or(Value::Null)
}

/// The workspace of the parent's window, read through `read`: the address its snapshot records,
/// else a window whose pid is one of the parent wrapper's ancestors. A refused check, an absent
/// snapshot or a failed read gives none.
fn parent_target(
    parent: &ViolaName,
    home: &Path,
    read: &mut dyn FnMut(&Request) -> Result<Reply, HyprlandError>,
) -> Option<i32> {
    let parent_dir = home.join("instances").join(parent.as_ref() as &str);
    check_instance(home, &parent_dir).ok()?;
    let (recorded, pid) = match read_snapshot_or_replay(&parent_dir).ok()? {
        Recovered::Snapshot(snapshot) => (
            snapshot.window.and_then(|window| window.address),
            Some(snapshot.pid),
        ),
        Recovered::Replayed { state, .. } => (state.window.and_then(|w| w.address), None),
        Recovered::Absent => return None,
    };
    let Ok(Reply::Clients(windows)) = read(&Request::Clients) else {
        return None;
    };
    let chain = pid.map(|pid| ancestors(pid, ANCESTRY)).unwrap_or_default();
    target_of(&windows, recorded.as_deref(), &chain)
}

/// The workspace of the window at `recorded`, else of the window of the nearest pid of
/// `ancestors`; none when neither stands.
fn target_of(
    windows: &[viola_hyprland::Window],
    recorded: Option<&str>,
    ancestors: &[u32],
) -> Option<i32> {
    let workspace = |window: &viola_hyprland::Window| window.workspace.map(|id| id.into_inner());
    if let Some(recorded) = recorded
        && let Some(window) = windows.iter().find(|w| w.address.as_ref() == recorded)
    {
        return workspace(window);
    }
    ancestors.iter().find_map(|pid| {
        windows
            .iter()
            .find(|window| window.pid == Some(*pid))
            .and_then(workspace)
    })
}

/// What the terminal's argv is built from, each value one element.
struct Opened<'a> {
    name: &'a ViolaName,
    cwd: &'a Path,
    pinned: &'a Path,
    home: &'a Path,
    parent: Option<&'a ViolaName>,
    role: Option<&'a str>,
    model: Option<&'a str>,
    effort: Option<&'a str>,
    origin: &'a str,
    place: Option<i32>,
}

/// `foot`'s argv, closed: its three options, then the pinned `viola run` with only the open's own
/// checked values, each one element whatever characters it holds, and `-- claude`.
fn open_argv(opened: &Opened<'_>) -> Vec<OsString> {
    let name: &str = opened.name.as_ref();
    let mut working = OsString::from("--working-directory=");
    working.push(opened.cwd.as_os_str());
    let mut argv: Vec<OsString> = vec![
        format!("--app-id=viola.{name}").into(),
        format!("--title={name}").into(),
        working,
        opened.pinned.into(),
        "--home".into(),
        opened.home.into(),
        "run".into(),
        name.into(),
    ];
    let mut pair = |flag: &str, value: Option<&str>| {
        if let Some(value) = value {
            argv.push(flag.into());
            argv.push(value.into());
        }
    };
    pair("--parent", opened.parent.map(|p| p.as_ref() as &str));
    pair("--role", opened.role);
    pair("--model", opened.model);
    pair("--effort", opened.effort);
    pair("--origin", Some(opened.origin));
    let place = opened.place.map(|n| n.to_string());
    pair("--place", place.as_deref());
    argv.push("--".into());
    argv.push(PROGRAM.into());
    argv
}

/// The window the instance's real snapshot records now; none from a replayed or absent one.
pub(super) fn recorded_window(instance_dir: &Path) -> Option<Window> {
    match read_snapshot_or_replay(instance_dir) {
        Ok(Recovered::Snapshot(snapshot)) => snapshot.window,
        _ => None,
    }
}

/// Whether the instance in the spawned `terminal` came up live within `OPEN_LIVE_BOUND`; a
/// terminal that ended first reads not live. The terminal outlives the caller by design: its
/// handle is dropped, never waited on.
pub(super) fn came_up(instance_dir: &Path, mut terminal: Child) -> bool {
    wait_live(
        || {
            if live_endpoint(instance_dir).is_some() {
                Probe::Live
            } else if matches!(terminal.try_wait(), Ok(Some(_)) | Err(_)) {
                Probe::Ended
            } else {
                Probe::Pending
            }
        },
        OPEN_LIVE_BOUND,
        LIVE_POLL,
    )
}

/// The terminal, spawned directly with no shell, its streams null and in a process group of its
/// own, so a Ctrl-C to the open ends no window; one `process-start{subject:"terminal"}`.
pub(super) fn spawn_terminal(foot: &Path, argv: &[OsString]) -> anyhow::Result<Child> {
    use std::os::unix::process::CommandExt as _;
    let child = Command::new(foot)
        .args(argv)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn()?;
    obs_event!(
        INFO,
        ObsEvent::ProcessStart,
        subject = "terminal",
        child_pid = child.id()
    );
    Ok(child)
}

/// What one look at the opened instance read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Probe {
    Live,
    /// The terminal ended before the instance was live.
    Ended,
    Pending,
}

/// Whether `probe` read live within `bound`, looking every `poll`; a terminal that ended reads
/// not live at once.
fn wait_live(mut probe: impl FnMut() -> Probe, bound: Duration, poll: Duration) -> bool {
    let started = Instant::now();
    loop {
        match probe() {
            Probe::Live => return true,
            Probe::Ended => return false,
            Probe::Pending if started.elapsed() >= bound => return false,
            Probe::Pending => std::thread::sleep(poll),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use viola_hyprland::{Address, WorkspaceId};

    #[derive(clap::Parser)]
    struct Line {
        #[command(flatten)]
        args: OpenArgs,
    }

    fn parse(words: &[&str]) -> Result<OpenArgs, clap::error::ErrorKind> {
        use clap::Parser as _;
        Line::try_parse_from(std::iter::once("open").chain(words.iter().copied()))
            .map_err(|error| error.kind())
            .and_then(|line| checked(line.args).map_err(|error| error.kind()))
    }

    #[test]
    fn open_args_take_the_allowlist() {
        let args = parse(&[
            "child",
            "--parent",
            "overseer",
            "--role",
            "worker",
            "--cwd",
            "/tmp/a b",
            "--workspace",
            "3",
            "--model",
            "haiku",
            "--effort",
            "low",
            "--json",
            "--",
            "claude",
        ])
        .expect("args");
        assert_eq!(args.name.as_ref() as &str, "child");
        assert_eq!(
            args.parent.as_ref().map(|p| p.as_ref() as &str),
            Some("overseer")
        );
        assert_eq!(args.role.as_deref(), Some("worker"));
        assert_eq!(args.cwd, Some(PathBuf::from("/tmp/a b")));
        assert_eq!(args.workspace, Some(3));
        assert_eq!(
            (args.model.as_deref(), args.effort.as_deref()),
            (Some("haiku"), Some("low"))
        );
        assert!(args.json);
        assert!(parse(&["child", "--", "claude"]).is_ok());
        assert_eq!(
            parse(&["child", "--home-dir", "x", "--", "claude"]).err(),
            Some(clap::error::ErrorKind::UnknownArgument)
        );
        assert_eq!(
            parse(&["child", "--origin", "cli", "--", "claude"]).err(),
            Some(clap::error::ErrorKind::UnknownArgument)
        );
    }

    #[rstest]
    #[case::a_claude_flag(&["claude", "--x"])]
    #[case::a_shell(&["sh"])]
    #[case::a_shell_line(&["claude; x"])]
    #[case::a_substitution(&["$(x)"])]
    #[case::a_newline(&["claude\nx"])]
    #[case::claude_twice(&["claude", "claude"])]
    fn open_args_refuse_a_word_after_dashes_other_than_claude(#[case] program: &[&str]) {
        let words: Vec<&str> = ["child", "--"]
            .into_iter()
            .chain(program.iter().copied())
            .collect();
        assert_eq!(
            parse(&words).err(),
            Some(clap::error::ErrorKind::ArgumentConflict),
            "{program:?}"
        );
        let line =
            clap::Error::raw(clap::error::ErrorKind::ArgumentConflict, OPEN_PROGRAM).to_string();
        for word in program.iter().filter(|word| **word != PROGRAM) {
            assert!(!line.contains(word), "{line}");
        }
        assert_eq!(
            clap::Error::new(clap::error::ErrorKind::ArgumentConflict).exit_code(),
            2
        );
    }

    #[rstest]
    #[case::a_driver(&["child", "--role", "driver", "--", "claude"])]
    #[case::upper(&["child", "--role", "Builder", "--", "claude"])]
    fn open_args_refuse_a_role_outside_the_three(#[case] words: &[&str]) {
        assert_eq!(
            parse(words).err(),
            Some(clap::error::ErrorKind::InvalidValue)
        );
    }

    #[test]
    fn open_args_refuse_a_self_parent() {
        assert_eq!(
            parse(&["child", "--parent", "child", "--", "claude"]).err(),
            Some(clap::error::ErrorKind::ArgumentConflict)
        );
    }

    fn window(address: &str, pid: u32, workspace: i32) -> viola_hyprland::Window {
        viola_hyprland::Window {
            address: Address::try_new(address.to_owned()).expect("an address"),
            pid: Some(pid),
            workspace: Some(WorkspaceId::new(workspace)),
            monitor: None,
            at: None,
            size: None,
            floating: None,
            mapped: None,
            hidden: None,
            focus_history_id: None,
            swallowing: None,
            grouped: Vec::new(),
        }
    }

    #[test]
    fn target_of_takes_the_recorded_window_first() {
        let windows = [window("0xa", 10, 4), window("0xb", 11, 6)];
        assert_eq!(target_of(&windows, Some("0xb"), &[10]), Some(6));
    }

    #[test]
    fn target_of_falls_back_to_an_ancestor_window() {
        let windows = [window("0xa", 10, 4), window("0xb", 11, 6)];
        assert_eq!(target_of(&windows, Some("0xc"), &[99, 11, 10]), Some(6));
        assert_eq!(target_of(&windows, None, &[10]), Some(4));
    }

    #[test]
    fn target_of_with_neither_is_none() {
        let windows = [window("0xa", 10, 4)];
        assert_eq!(target_of(&windows, Some("0xc"), &[99]), None);
        assert_eq!(target_of(&windows, None, &[]), None);
        assert_eq!(target_of(&[], Some("0xa"), &[10]), None);
    }

    #[test]
    fn open_argv_is_the_closed_argv_one_element_each() {
        let name = ViolaName::try_new("child".to_owned()).expect("valid");
        let parent = ViolaName::try_new("overseer".to_owned()).expect("valid");
        let argv = open_argv(&Opened {
            name: &name,
            cwd: Path::new("/w/a dir 'q' \"d\""),
            pinned: Path::new("/h/bin/k/viola"),
            home: Path::new("/h ome"),
            parent: Some(&parent),
            role: Some("worker"),
            model: Some("haiku"),
            effort: None,
            origin: "agent",
            place: Some(2),
        });
        let want: Vec<OsString> = [
            "--app-id=viola.child",
            "--title=child",
            "--working-directory=/w/a dir 'q' \"d\"",
            "/h/bin/k/viola",
            "--home",
            "/h ome",
            "run",
            "child",
            "--parent",
            "overseer",
            "--role",
            "worker",
            "--model",
            "haiku",
            "--origin",
            "agent",
            "--place",
            "2",
            "--",
            "claude",
        ]
        .map(OsString::from)
        .into();
        assert_eq!(argv, want);
        let bare = open_argv(&Opened {
            name: &name,
            cwd: Path::new("/w"),
            pinned: Path::new("/p"),
            home: Path::new("/h"),
            parent: None,
            role: None,
            model: None,
            effort: None,
            origin: "cli",
            place: None,
        });
        assert_eq!(
            bare[8..],
            ["--origin", "cli", "--", "claude"].map(OsString::from)
        );
    }

    #[test]
    fn wait_live_past_its_bound_is_not_live() {
        let started = Instant::now();
        let bound = Duration::from_millis(200);
        assert!(!wait_live(
            || Probe::Pending,
            bound,
            Duration::from_millis(20)
        ));
        assert!(started.elapsed() >= bound);
        let mut looks = 0;
        assert!(wait_live(
            || {
                looks += 1;
                if looks < 3 {
                    Probe::Pending
                } else {
                    Probe::Live
                }
            },
            bound,
            Duration::from_millis(1)
        ));
        let started = Instant::now();
        assert!(!wait_live(
            || Probe::Ended,
            bound,
            Duration::from_millis(20)
        ));
        assert!(started.elapsed() < bound);
        assert_eq!(OPEN_LIVE_BOUND, Duration::from_secs(30));
    }
}
