//! The wrapper's window step, run at a start `viola open` asked for: the window of the terminal the
//! wrapper runs in, found by that terminal's pid in the compositor's `clients`, moved once to the
//! target workspace, and recorded with a closed placement word. It reads and dispatches only
//! through the Hyprland seam; no reply text, address or pid reaches a line.

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
    let move_to = Request::MoveToWorkspace(seen.address.clone(), WorkspaceId::new(place));
    match read(&move_to) {
        Ok(Reply::Dispatched) => (Some(seen.address), Some(place), "placed"),
        Ok(_) => (Some(seen.address), seen_on, HyprlandError::Malformed.code()),
        Err(error) => (Some(seen.address), seen_on, error.code()),
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
    }
}
