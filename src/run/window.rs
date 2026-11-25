//! The wrapper's window step, run at a start `viola open` asked for: the window of the terminal the
//! wrapper runs in, found by that terminal's pid in the compositor's `clients`, moved once to the
//! target workspace, and recorded with a closed placement word. Around that move it reads the
//! focused monitor's active workspace before and after, and focuses the operator's prior workspace
//! back once when the move took it (the operator's own answer to fork M, M1), so a placed window
//! leaves the view where it was. It reads and dispatches only through the Hyprland seam; no reply
//! text, address, pid or workspace reaches a line.

use std::time::{Duration, Instant};

use tracing::instrument;
use viola_hyprland::{Address, HyprlandError, Reply, Request, WorkspaceId};
use viola_state::snapshot::Window;

/// How long the terminal's window may take to appear in `clients`.
pub(crate) const WINDOW_WAIT: Duration = Duration::from_secs(5);
/// How often `clients` is read while it is awaited.
pub(crate) const WINDOW_POLL: Duration = Duration::from_millis(100);

/// The terminal's window found through `read` within `wait`, moved to `place` when one is given,
/// as the record the start keeps: its address and workspace (`None` where unread) with the
/// placement word and the `origin` handed in. A read that fails ends the step at once with the
/// seam's own word.
#[instrument(skip_all, name = "run.window", fields(outcome = tracing::field::Empty))]
pub(crate) fn window_step(
    read: &mut dyn FnMut(&Request) -> Result<Reply, HyprlandError>,
    terminal_pid: u32,
    place: Option<i32>,
    origin: &str,
    wait: Duration,
) -> Window {
    let (address, workspace, placement) = find_and_place(read, terminal_pid, place, wait);
    tracing::Span::current().record("outcome", placement);
    Window {
        address: address.map(|address| address.as_ref().to_owned()),
        workspace,
        placement: placement.to_owned(),
        origin: origin.to_owned(),
    }
}

fn find_and_place(
    read: &mut dyn FnMut(&Request) -> Result<Reply, HyprlandError>,
    terminal_pid: u32,
    place: Option<i32>,
    wait: Duration,
) -> (Option<Address>, Option<i32>, &'static str) {
    let started = Instant::now();
    let seen = loop {
        let windows = match read(&Request::Clients) {
            Ok(Reply::Clients(windows)) => windows,
            Ok(_) => return (None, None, HyprlandError::Malformed.code()),
            Err(error) => return (None, None, error.code()),
        };
        if let Some(window) = windows
            .into_iter()
            .find(|window| window.pid == Some(terminal_pid))
        {
            break window;
        }
        if started.elapsed() >= wait {
            return (None, None, "window-unseen");
        }
        std::thread::sleep(WINDOW_POLL.min(wait));
    };
    let seen_on = seen.workspace.map(WorkspaceId::into_inner);
    let Some(place) = place else {
        return (Some(seen.address), seen_on, "unplaced");
    };
    let prev = active_workspace(read);
    let move_to = Request::MoveToWorkspace(seen.address.clone(), WorkspaceId::new(place));
    match read(&move_to) {
        Ok(Reply::Dispatched) => {
            if let Some(prev) = prev {
                keep_view(read, prev);
            }
            (Some(seen.address), Some(place), "placed")
        }
        Ok(_) => (Some(seen.address), seen_on, HyprlandError::Malformed.code()),
        Err(error) => (Some(seen.address), seen_on, error.code()),
    }
}

/// The focused monitor's active workspace, through one `monitors` read; `None` when the read
/// fails, answers another reply or names no focused monitor with an active workspace.
fn active_workspace(
    read: &mut dyn FnMut(&Request) -> Result<Reply, HyprlandError>,
) -> Option<WorkspaceId> {
    match read(&Request::Monitors) {
        Ok(Reply::Monitors(monitors)) => monitors
            .into_iter()
            .find(|monitor| monitor.focused == Some(true))
            .and_then(|monitor| monitor.active_workspace),
        _ => None,
    }
}

/// The move takes the active workspace with the moved window on Hyprland 0.56.2, which offers no
/// measured silent move: the operator's view is focused back to `prev` once, only when the move
/// took it. A failed read or restore leaves the placement as the move set it.
fn keep_view(read: &mut dyn FnMut(&Request) -> Result<Reply, HyprlandError>, prev: WorkspaceId) {
    if active_workspace(read).is_some_and(|now| now != prev) {
        let _restored = read(&Request::Workspace(prev));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TERMINAL: u32 = 4242;

    fn client(address: &str, pid: u32, workspace: i32) -> viola_hyprland::Window {
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

    /// A read that answers `clients` with `windows` and a dispatch with `moved`, keeping every
    /// request it was asked.
    fn seam(
        windows: Result<Vec<viola_hyprland::Window>, HyprlandError>,
        moved: Result<Reply, HyprlandError>,
        asked: &mut Vec<Request>,
    ) -> impl FnMut(&Request) -> Result<Reply, HyprlandError> + '_ {
        move |request| {
            asked.push(request.clone());
            match request {
                Request::Clients => windows.clone().map(Reply::Clients),
                _ => moved.clone(),
            }
        }
    }

    /// A read that answers `clients` with `windows`, each `monitors` with the next of `views`, the
    /// move with `moved` and a workspace focus with `restored`, keeping every request it was asked.
    fn view_seam(
        windows: Vec<viola_hyprland::Window>,
        mut views: std::collections::VecDeque<Result<Reply, HyprlandError>>,
        moved: Result<Reply, HyprlandError>,
        restored: Result<Reply, HyprlandError>,
        asked: &mut Vec<Request>,
    ) -> impl FnMut(&Request) -> Result<Reply, HyprlandError> + '_ {
        move |request| {
            asked.push(request.clone());
            match request {
                Request::Clients => Ok(Reply::Clients(windows.clone())),
                Request::Monitors => views.pop_front().unwrap_or(Err(HyprlandError::Unreachable)),
                Request::Workspace(_) => restored.clone(),
                _ => moved.clone(),
            }
        }
    }

    /// A `monitors` reply of two monitors: the focused one showing `active`, the other 9.
    fn view(active: i32) -> Result<Reply, HyprlandError> {
        let monitor = |id: i64, focused: bool, workspace: i32| viola_hyprland::Monitor {
            id,
            width: None,
            height: None,
            x: None,
            y: None,
            focused: Some(focused),
            active_workspace: Some(WorkspaceId::new(workspace)),
        };
        Ok(Reply::Monitors(vec![
            monitor(0, false, 9),
            monitor(1, true, active),
        ]))
    }

    fn moved_to(workspace: i32) -> Request {
        let address = Address::try_new("0xbbb".to_owned()).expect("an address");
        Request::MoveToWorkspace(address, WorkspaceId::new(workspace))
    }

    #[test]
    fn window_step_restores_the_active_workspace_the_move_took() {
        let mut asked = Vec::new();
        let mut read = view_seam(
            vec![client("0xbbb", TERMINAL, 2)],
            [view(1), view(5)].into(),
            Ok(Reply::Dispatched),
            Ok(Reply::Dispatched),
            &mut asked,
        );
        let window = window_step(&mut read, TERMINAL, Some(5), "agent", WINDOW_WAIT);
        drop(read);
        assert_eq!(window, record(Some("0xbbb"), Some(5), "placed", "agent"));
        assert_eq!(
            asked,
            [
                Request::Clients,
                Request::Monitors,
                moved_to(5),
                Request::Monitors,
                Request::Workspace(WorkspaceId::new(1)),
            ]
        );
    }

    #[test]
    fn window_step_restores_nothing_when_the_view_stayed() {
        let mut asked = Vec::new();
        let mut read = view_seam(
            vec![client("0xbbb", TERMINAL, 2)],
            [view(1), view(1)].into(),
            Ok(Reply::Dispatched),
            Ok(Reply::Dispatched),
            &mut asked,
        );
        let window = window_step(&mut read, TERMINAL, Some(5), "cli", WINDOW_WAIT);
        drop(read);
        assert_eq!(window, record(Some("0xbbb"), Some(5), "placed", "cli"));
        assert_eq!(
            asked,
            [
                Request::Clients,
                Request::Monitors,
                moved_to(5),
                Request::Monitors
            ]
        );
    }

    #[test]
    fn window_step_moves_on_when_the_monitors_read_fails() {
        let mut asked = Vec::new();
        let mut read = view_seam(
            vec![client("0xbbb", TERMINAL, 2)],
            [Err(HyprlandError::Unreachable), view(5)].into(),
            Ok(Reply::Dispatched),
            Ok(Reply::Dispatched),
            &mut asked,
        );
        let window = window_step(&mut read, TERMINAL, Some(5), "cli", WINDOW_WAIT);
        drop(read);
        assert_eq!(window, record(Some("0xbbb"), Some(5), "placed", "cli"));
        assert_eq!(asked, [Request::Clients, Request::Monitors, moved_to(5)]);
    }

    #[test]
    fn window_step_keeps_placed_when_the_restore_fails() {
        let mut asked = Vec::new();
        let mut read = view_seam(
            vec![client("0xbbb", TERMINAL, 2)],
            [view(1), view(5)].into(),
            Ok(Reply::Dispatched),
            Err(HyprlandError::Refused),
            &mut asked,
        );
        let window = window_step(&mut read, TERMINAL, Some(5), "cli", WINDOW_WAIT);
        drop(read);
        assert_eq!(window, record(Some("0xbbb"), Some(5), "placed", "cli"));
        assert_eq!(asked.last(), Some(&Request::Workspace(WorkspaceId::new(1))));
    }

    fn record(
        address: Option<&str>,
        workspace: Option<i32>,
        placement: &str,
        origin: &str,
    ) -> Window {
        Window {
            address: address.map(str::to_owned),
            workspace,
            placement: placement.to_owned(),
            origin: origin.to_owned(),
        }
    }

    #[test]
    fn window_step_moves_the_window_of_its_terminal_to_the_place() {
        let mut asked = Vec::new();
        let windows = vec![client("0xaaa", 7, 3), client("0xbbb", TERMINAL, 2)];
        let mut read = seam(Ok(windows), Ok(Reply::Dispatched), &mut asked);
        let window = window_step(&mut read, TERMINAL, Some(1), "agent", WINDOW_WAIT);
        drop(read);
        assert_eq!(window, record(Some("0xbbb"), Some(1), "placed", "agent"));
        let address = Address::try_new("0xbbb".to_owned()).expect("an address");
        assert_eq!(
            asked,
            [
                Request::Clients,
                Request::Monitors,
                Request::MoveToWorkspace(address, WorkspaceId::new(1))
            ]
        );
    }

    #[test]
    fn window_step_with_no_place_is_unplaced() {
        let mut asked = Vec::new();
        let windows = vec![client("0xbbb", TERMINAL, 2)];
        let mut read = seam(Ok(windows), Ok(Reply::Dispatched), &mut asked);
        let window = window_step(&mut read, TERMINAL, None, "cli", WINDOW_WAIT);
        drop(read);
        assert_eq!(window, record(Some("0xbbb"), Some(2), "unplaced", "cli"));
        assert_eq!(asked, [Request::Clients]);
    }

    /// Other windows stand, none of them the terminal's: the step polls until its bound, then
    /// reads `window-unseen` and moves nothing.
    #[test]
    fn window_step_with_no_window_is_window_unseen() {
        let mut asked = Vec::new();
        let windows = vec![client("0xaaa", 7, 3), client("0xccc", 8, 1)];
        let mut read = seam(Ok(windows), Ok(Reply::Dispatched), &mut asked);
        let started = Instant::now();
        let window = window_step(
            &mut read,
            TERMINAL,
            Some(1),
            "cli",
            Duration::from_millis(250),
        );
        assert!(started.elapsed() >= Duration::from_millis(250));
        drop(read);
        assert_eq!(window, record(None, None, "window-unseen", "cli"));
        assert!(asked.len() >= 2, "{asked:?}");
        assert!(asked.iter().all(|request| *request == Request::Clients));
    }

    #[rstest]
    #[case::unsupported(HyprlandError::Unsupported, "unsupported")]
    #[case::unreachable(HyprlandError::Unreachable, "unreachable")]
    #[case::oversize(HyprlandError::Oversize, "oversize")]
    #[case::malformed(HyprlandError::Malformed, "malformed")]
    fn window_step_off_hyprland_is_unsupported_at_once(
        #[case] error: HyprlandError,
        #[case] word: &str,
    ) {
        let mut asked = Vec::new();
        let mut read = seam(Err(error), Ok(Reply::Dispatched), &mut asked);
        let started = Instant::now();
        let window = window_step(&mut read, TERMINAL, Some(1), "cli", WINDOW_WAIT);
        assert!(started.elapsed() < WINDOW_POLL);
        drop(read);
        assert_eq!(window, record(None, None, word, "cli"));
        assert_eq!(asked, [Request::Clients]);
    }

    #[test]
    fn window_step_with_a_refused_move_is_refused() {
        let mut asked = Vec::new();
        let windows = vec![client("0xbbb", TERMINAL, 2)];
        let mut read = seam(Ok(windows), Err(HyprlandError::Refused), &mut asked);
        let window = window_step(&mut read, TERMINAL, Some(1), "cli", WINDOW_WAIT);
        drop(read);
        assert_eq!(window, record(Some("0xbbb"), Some(2), "refused", "cli"));
        assert_eq!(asked, [Request::Clients, Request::Monitors, moved_to(1)]);
    }
}
