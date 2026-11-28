//! `viola console`: the terminal fleet console, the human's, in the terminal it is started in. It
//! refuses a stdin or stdout that is not a terminal before any home read, then a home another
//! user may read before any log opens; then it logs under its own `console` role, with no
//! instance, and holds the terminal until `q` or Ctrl-C.

use std::io::{self, IsTerminal as _};
use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, SystemTime};

use anyhow::Context as _;
use chrono::Utc;
use viola_core::obs::{ObsEvent, ObsProcess};
use viola_core::{SERVICE_NAME, VERSION, obs_event};
use viola_state::board::Observation;

use crate::{console, human, obs, run};

/// The refusal off a terminal.
pub(crate) const NO_TERMINAL: (&str, &str) = (
    "viola console needs a terminal",
    "run viola console in a terminal window",
);

/// The refusal of a home another user may read (or that is not the user's).
pub(crate) const HOME_REFUSED: (&str, &str) = (
    "viola console could not use its home",
    "make the viola home readable by you alone, then run viola console again",
);

pub(crate) fn console(home: &Path) -> anyhow::Result<ExitCode> {
    if !(io::stdin().is_terminal() && io::stdout().is_terminal()) {
        human::refuse(NO_TERMINAL.0, NO_TERMINAL.1);
        return Ok(ExitCode::from(1));
    }
    let home = std::path::absolute(home)?;
    // Before the log opens: opening it narrows the home to its owner, which would erase the very
    // state this check refuses. A refused check opens nothing under the home and writes no line.
    if home_refused(&home) {
        human::refuse(HOME_REFUSED.0, HOME_REFUSED.1);
        return Ok(ExitCode::from(1));
    }
    let (level, rejection) = obs::read_diagnostics_level(&home);
    obs::viola_obs_init(&home, ObsProcess::Console, None, level)?;
    log_self_start();
    if let Some(rejection) = rejection {
        obs::log_config_rejection(rejection);
    }
    // The log has made the home, so this is its one canonical spelling, the one every action's
    // child is handed, beside this exe's own canonical path.
    let runner = console::action::Runner::new(
        super::canonical_home(&std::env::current_exe()?),
        super::canonical_home(&home),
    );
    let mode = viola_pty::HostTerminal::enter().context("raw mode")?;
    let agents = console::fleet::spawn_agents(agents_read(&home), AGENTS_EVERY);
    let mut fleet = console::fleet::FleetSource::new(home, agents);
    let sources = console::Sources {
        size: viola_pty::host_size,
        board: move || fleet.read(SystemTime::now()),
        clock: Utc::now,
        read_every: console::READ_EVERY,
        act: move |action| runner.spawn(action),
    };
    let end = console::run(viola_pty::host_stdin(), io::stdout(), mode, sources)?;
    // The run has restored the terminal before it returned, so the pair lands on the user's own
    // screen.
    if end == console::End::Refused {
        run::log_self_exit(1, Some("strict-modes-failed"));
        human::refuse(HOME_REFUSED.0, HOME_REFUSED.1);
        return Ok(ExitCode::from(1));
    }
    run::log_self_exit(0, None);
    Ok(ExitCode::SUCCESS)
}

/// The least time between two `claude agents --json` reads.
const AGENTS_EVERY: Duration = Duration::from_secs(5);

/// The agents read `list` makes, from the console's current directory; a read that fails, or a
/// directory that cannot be read, gives no list.
fn agents_read(home: &Path) -> impl FnMut() -> Option<Vec<Observation>> + Send + 'static {
    let home = home.to_owned();
    move || {
        let cwd = std::env::current_dir().ok()?;
        run::agents::read_here(&home, &cwd)
            .ok()
            .map(super::list::observations)
    }
}

/// A home that exists and fails the read rule; an absent one is made by the log, owner-only.
fn home_refused(home: &Path) -> bool {
    match std::fs::symlink_metadata(home) {
        Ok(_) => viola_state::strict::check_read(home).is_err(),
        Err(e) => e.kind() != io::ErrorKind::NotFound,
    }
}

fn log_self_start() {
    obs_event!(
        INFO,
        ObsEvent::ProcessStart,
        subject = "self",
        service_name = SERVICE_NAME,
        version = VERSION,
        os = std::env::consts::OS,
        pid = std::process::id(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_refused_passes_an_absent_home_and_refuses_a_widened_one() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        assert!(!home_refused(&home), "an absent home is made owner-only");
        viola_state::fs::create_private_dir(&home).expect("home");
        assert!(!home_refused(&home));
        std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o750)).expect("chmod");
        assert!(home_refused(&home));
    }
}
