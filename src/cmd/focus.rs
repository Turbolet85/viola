//! `viola focus <name> [--json]`: the named live session's terminal window brought forward and its
//! workspace shown on Hyprland. The window is the one the live wrapper recorded, trusted only while
//! its pid is one of the wrapper's ancestors, else the window of the wrapper's nearest ancestor
//! pid. The verb reads and dispatches only through the Hyprland seam, at most one `clients` read,
//! one workspace dispatch and one window dispatch; it prints and logs no address, pid or workspace,
//! and writes no snapshot and no event line.

use std::ffi::OsStr;
use std::path::Path;
use std::process::ExitCode;

use serde_json::json;
use tracing::instrument;
use viola_core::ViolaName;
use viola_hyprland::{Address, HyprlandError, Reply, Request, Window};
use viola_state::liveness::{ancestors, live_endpoint, same_process};
use viola_state::replay::{Recovered, read_snapshot_or_replay};

use super::client::{self, Unreachable};
use crate::{human, run};

/// How many of the wrapper's ancestors are matched against the compositor's windows.
const ANCESTRY: usize = 8;
/// The refusal of a live session none of whose windows the compositor lists.
pub(crate) const WINDOW_NOT_FOUND: &str = "window-not-found";

#[derive(clap::Args)]
pub(crate) struct FocusArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// Print one JSON document on stdout instead of the result line
    #[arg(long)]
    json: bool,
}

pub(crate) fn focus(home: &Path, args: &FocusArgs) -> anyhow::Result<ExitCode> {
    if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
        return Ok(ExitCode::from(code));
    }
    // The log's open created the home: from here on its one canonical spelling, as `open`'s.
    let home = super::canonical_home(&client::start(home, &args.name)?);
    focused(&home, args)
}

#[instrument(skip_all, name = "cli.focus")]
fn focused(home: &Path, args: &FocusArgs) -> anyhow::Result<ExitCode> {
    let name: &str = args.name.as_ref();
    let instance_dir = home.join("instances").join(name);
    let not_live = || {
        let cause = Unreachable::without_endpoint(&instance_dir);
        let code = client::unreachable(args.json, cause, || {
            client::unable_unreachable(name, cause);
        });
        Ok(ExitCode::from(code))
    };
    if live_endpoint(&instance_dir).is_none() {
        return not_live();
    }
    let snapshot = match read_snapshot_or_replay(&instance_dir) {
        Ok(Recovered::Snapshot(snapshot)) if same_process(&snapshot) => snapshot,
        _ => return not_live(),
    };
    let here = std::env::current_dir()?;
    let (persistent, _) = run::persistent_names(home);
    let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), &persistent);
    let hyprctl = run::resolve_program(OsStr::new(viola_hyprland::PROGRAM), &here).ok();
    let mut read =
        |request: &Request| viola_hyprland::read(hyprctl.as_deref(), request, &strip.remove).reply;
    let recorded = snapshot
        .window
        .and_then(|window| window.address)
        .and_then(|address| Address::try_new(address).ok());
    let chain = ancestors(snapshot.pid, ANCESTRY);
    if let Err(code) = focus_with(&mut read, recorded.as_ref(), &chain) {
        if args.json {
            client::refused_document(code);
        } else if let Some((unable, hint)) = human::focus_pair(code, name) {
            human::refuse(&unable, &hint);
        }
        run::log_self_exit(1, Some(code));
        return Ok(ExitCode::from(1));
    }
    if args.json {
        client::document(&json!({"v": 1, "ok": {"name": name}}));
    } else {
        let _ = human::write_result(&mut std::io::stdout().lock(), &format!("{name}  focused"));
    }
    run::log_self_exit(0, None);
    Ok(ExitCode::SUCCESS)
}

/// One `clients` read through `read`, then the found window's workspace shown when it is a
/// numbered one, then the window focused. Each failure ends the focus with its closed word; no
/// window found is [`WINDOW_NOT_FOUND`] with nothing dispatched.
fn focus_with(
    read: &mut dyn FnMut(&Request) -> Result<Reply, HyprlandError>,
    recorded: Option<&Address>,
    chain: &[u32],
) -> Result<(), &'static str> {
    let windows = match read(&Request::Clients) {
        Ok(Reply::Clients(windows)) => windows,
        Ok(_) => return Err(HyprlandError::Malformed.code()),
        Err(error) => return Err(error.code()),
    };
    let found = client_of(&windows, recorded, chain);
    let Some(client) = found else {
        return Err(WINDOW_NOT_FOUND);
    };
    // A special workspace has a negative id, and no workspace dispatch shows it.
    if let Some(id) = client.workspace.filter(|id| id.into_inner() >= 1) {
        dispatched(read(&Request::Workspace(id)))?;
    }
    dispatched(read(&Request::FocusWindow(client.address.clone())))
}

fn dispatched(reply: Result<Reply, HyprlandError>) -> Result<(), &'static str> {
    match reply {
        Ok(Reply::Dispatched) => Ok(()),
        Ok(_) => Err(HyprlandError::Malformed.code()),
        Err(error) => Err(error.code()),
    }
}

/// The window at `recorded` while its pid is one of `chain` (an address a gone terminal left can
/// name another program's window), else the window of the first pid of `chain` that holds one.
fn client_of<'a>(
    windows: &'a [Window],
    recorded: Option<&Address>,
    chain: &[u32],
) -> Option<&'a Window> {
    let owned = |window: &&Window| window.pid.is_some_and(|pid| chain.contains(&pid));
    if let Some(recorded) = recorded
        && let Some(window) = windows
            .iter()
            .filter(owned)
            .find(|window| &window.address == recorded)
    {
        return Some(window);
    }
    chain
        .iter()
        .find_map(|pid| windows.iter().find(|window| window.pid == Some(*pid)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::Value;
    use viola_hyprland::WorkspaceId;

    /// The recorded focus fixtures' address and workspace.
    const RECORDED: &str = "0x555a9c521790";
    const RECORDED_ON: i32 = 6;

    fn address(text: &str) -> Address {
        Address::try_new(text.to_owned()).expect("an address")
    }

    fn client(at: &str, pid: u32, workspace: Option<i32>) -> Window {
        Window {
            address: address(at),
            pid: Some(pid),
            workspace: workspace.map(WorkspaceId::new),
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

    /// Where a seam read fails, if anywhere.
    #[derive(Clone, Copy, Debug)]
    enum Fails {
        Nowhere,
        Clients(HyprlandError),
        Workspace(HyprlandError),
        Window(HyprlandError),
    }

    /// A read that answers `clients` with `windows` and each dispatch `ok`, save where `fails`
    /// names, keeping every request it was asked.
    fn seam(
        windows: Vec<Window>,
        fails: Fails,
        asked: &mut Vec<Request>,
    ) -> impl FnMut(&Request) -> Result<Reply, HyprlandError> + '_ {
        move |request| {
            asked.push(request.clone());
            match (request, fails) {
                (Request::Clients, Fails::Clients(error))
                | (Request::Workspace(_), Fails::Workspace(error))
                | (Request::FocusWindow(_), Fails::Window(error)) => Err(error),
                (Request::Clients, _) => Ok(Reply::Clients(windows.clone())),
                _ => Ok(Reply::Dispatched),
            }
        }
    }

    fn focus_over(
        windows: Vec<Window>,
        fails: Fails,
        recorded: Option<&str>,
        chain: &[u32],
    ) -> (Result<(), &'static str>, Vec<Request>) {
        let mut asked = Vec::new();
        let recorded = recorded.map(address);
        let outcome = focus_with(
            &mut seam(windows, fails, &mut asked),
            recorded.as_ref(),
            chain,
        );
        (outcome, asked)
    }

    fn fixture_args(file: &str) -> Value {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures/hyprland/0.56.2")
            .join(file);
        let doc: Value =
            serde_json::from_slice(&std::fs::read(path).expect("the fixture")).expect("json");
        doc["args"].clone()
    }

    #[test]
    fn focus_dispatches_the_workspace_then_the_window() {
        let windows = vec![
            client("0x1", 70, Some(1)),
            client(RECORDED, 40, Some(RECORDED_ON)),
        ];
        let (outcome, asked) = focus_over(windows, Fails::Nowhere, Some(RECORDED), &[40, 30]);
        assert_eq!(outcome, Ok(()));
        let workspace = Request::Workspace(WorkspaceId::new(RECORDED_ON));
        let window = Request::FocusWindow(address(RECORDED));
        assert_eq!(asked, [Request::Clients, workspace.clone(), window.clone()]);
        assert_eq!(
            serde_json::to_value(
                workspace
                    .args()
                    .iter()
                    .map(|a| a.to_str())
                    .collect::<Vec<_>>()
            )
            .expect("args"),
            fixture_args("Workspace.recorded-1.json")
        );
        assert_eq!(
            serde_json::to_value(window.args().iter().map(|a| a.to_str()).collect::<Vec<_>>())
                .expect("args"),
            fixture_args("FocusWindow.recorded-1.json")
        );
    }

    #[test]
    fn focus_client_skips_a_recorded_address_outside_the_ancestry() {
        let windows = vec![client("0xa", 99, Some(4)), client("0xb", 30, Some(5))];
        let (outcome, asked) = focus_over(windows.clone(), Fails::Nowhere, Some("0xa"), &[40, 30]);
        assert_eq!(outcome, Ok(()));
        assert_eq!(
            asked,
            [
                Request::Clients,
                Request::Workspace(WorkspaceId::new(5)),
                Request::FocusWindow(address("0xb")),
            ]
        );
        let (outcome, asked) = focus_over(windows, Fails::Nowhere, Some("0xa"), &[40]);
        assert_eq!(outcome, Err(WINDOW_NOT_FOUND));
        assert_eq!(asked, [Request::Clients]);
    }

    #[test]
    fn focus_client_falls_back_to_the_nearest_ancestor_window() {
        let windows = vec![client("0xa", 10, Some(4)), client("0xb", 11, Some(6))];
        let (outcome, asked) = focus_over(windows.clone(), Fails::Nowhere, None, &[99, 11, 10]);
        assert_eq!(outcome, Ok(()));
        assert_eq!(asked[2], Request::FocusWindow(address("0xb")));
        let (_, asked) = focus_over(windows, Fails::Nowhere, Some("0xc"), &[10, 11]);
        assert_eq!(asked[2], Request::FocusWindow(address("0xa")));
    }

    #[test]
    fn focus_with_no_client_is_window_not_found_and_dispatches_nothing() {
        let windows = vec![client("0xa", 10, Some(4))];
        for (windows, recorded) in [(windows, Some("0xa")), (Vec::new(), None)] {
            let (outcome, asked) = focus_over(windows, Fails::Nowhere, recorded, &[20, 30]);
            assert_eq!(outcome, Err(WINDOW_NOT_FOUND));
            assert_eq!(asked, [Request::Clients]);
        }
    }

    #[rstest]
    #[case::unread(None)]
    #[case::special(Some(-98))]
    #[case::zero(Some(0))]
    fn focus_skips_the_workspace_dispatch_for_an_unread_or_special_workspace(
        #[case] workspace: Option<i32>,
    ) {
        let windows = vec![client("0xa", 10, workspace)];
        let (outcome, asked) = focus_over(windows, Fails::Nowhere, Some("0xa"), &[10]);
        assert_eq!(outcome, Ok(()));
        assert_eq!(
            asked,
            [Request::Clients, Request::FocusWindow(address("0xa"))]
        );
    }

    #[rstest]
    #[case::clients_unsupported(Fails::Clients(HyprlandError::Unsupported), "unsupported", 1)]
    #[case::clients_unreachable(Fails::Clients(HyprlandError::Unreachable), "unreachable", 1)]
    #[case::clients_oversize(Fails::Clients(HyprlandError::Oversize), "oversize", 1)]
    #[case::clients_malformed(Fails::Clients(HyprlandError::Malformed), "malformed", 1)]
    #[case::workspace_unreachable(Fails::Workspace(HyprlandError::Unreachable), "unreachable", 2)]
    #[case::workspace_oversize(Fails::Workspace(HyprlandError::Oversize), "oversize", 2)]
    #[case::workspace_malformed(Fails::Workspace(HyprlandError::Malformed), "malformed", 2)]
    #[case::workspace_refused(Fails::Workspace(HyprlandError::Refused), "refused", 2)]
    #[case::window_unreachable(Fails::Window(HyprlandError::Unreachable), "unreachable", 3)]
    #[case::window_oversize(Fails::Window(HyprlandError::Oversize), "oversize", 3)]
    #[case::window_malformed(Fails::Window(HyprlandError::Malformed), "malformed", 3)]
    #[case::window_refused(Fails::Window(HyprlandError::Refused), "refused", 3)]
    fn focus_maps_each_seam_word_to_its_code(
        #[case] fails: Fails,
        #[case] code: &str,
        #[case] asked_count: usize,
    ) {
        let windows = vec![client("0xa", 10, Some(3))];
        let (outcome, asked) = focus_over(windows, fails, Some("0xa"), &[10]);
        assert_eq!(outcome, Err(code));
        assert_eq!(asked.len(), asked_count, "{asked:?}");
    }

    #[test]
    fn focus_maps_a_reply_of_another_kind_to_malformed() {
        let mut asked = 0;
        let mut read = |_: &Request| {
            asked += 1;
            Ok(Reply::Dispatched)
        };
        assert_eq!(focus_with(&mut read, None, &[10]), Err("malformed"));
        assert_eq!(asked, 1);
    }

    #[test]
    fn focus_of_an_unparseable_recorded_address_uses_the_ancestry() {
        assert!(Address::try_new("0xZZ".to_owned()).is_err());
        let windows = [client("0xa", 10, Some(4)), client("0xb", 11, Some(6))];
        let recorded = Address::try_new("0xZZ".to_owned()).ok();
        assert_eq!(
            client_of(&windows, recorded.as_ref(), &[11]).map(|w| w.address.clone()),
            Some(address("0xb"))
        );
        assert_eq!(client_of(&windows, recorded.as_ref(), &[]), None);
    }

    #[derive(clap::Parser)]
    struct Line {
        #[command(flatten)]
        args: FocusArgs,
    }

    fn parse(words: &[&str]) -> Result<FocusArgs, clap::error::ErrorKind> {
        use clap::Parser as _;
        Line::try_parse_from(std::iter::once("focus").chain(words.iter().copied()))
            .map(|line| line.args)
            .map_err(|error| error.kind())
    }

    #[test]
    fn focus_args_take_a_name_and_json_only() {
        let args = parse(&["builder", "--json"]).expect("args");
        assert_eq!(args.name.as_ref() as &str, "builder");
        assert!(args.json);
        assert!(!parse(&["builder"]).expect("args").json);
        for words in [
            &["builder", "other"][..],
            &["builder", "--workspace", "2"],
            &["builder", "--", "x"],
            &["Builder"],
        ] {
            assert!(parse(words).is_err(), "{words:?}");
        }
        assert_eq!(
            parse(&["builder", "--workspace", "2"]).err(),
            Some(clap::error::ErrorKind::UnknownArgument)
        );
    }
}
