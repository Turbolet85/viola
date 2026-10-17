//! `viola mcp`: the stdio MCP server Claude Code spawns from the plugin's `.mcp.json` inside a
//! wrapped session (run by the plugin, never by a person). The home is checked before anything under
//! it is opened, then the `mcp` role file opens and the five tools are served until stdin closes
//! (architecture §Established Decisions [MCP]). Its `list` tool reads the board CLI `list --json`
//! reads, through this binary's own board read.

use std::io;
use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use serde_json::Value;
use viola_core::obs::ObsProcess;
use viola_mcp::ListProvider;

use super::{client, list};
use crate::{obs, run};

pub(crate) fn mcp(home: &Path) -> anyhow::Result<ExitCode> {
    // Before the log opens: opening it narrows an existing home to its owner, which would erase
    // the very state this check refuses. A refused home prints nothing and opens nothing.
    if !home_passes(home) {
        return Ok(ExitCode::from(1));
    }
    let own_name = client::own_name();
    let (level, rejection) = obs::read_diagnostics_level(home);
    obs::viola_obs_init(home, ObsProcess::Mcp, own_name.clone(), level)?;
    run::log_self_start();
    if let Some(rejection) = rejection {
        obs::log_config_rejection(rejection);
    }
    let board_home = home.to_path_buf();
    // A board that could not be read at all is no document: the tool answers it as its own fault.
    let board: ListProvider =
        Arc::new(move || list::board_document(&board_home).unwrap_or(Value::Null));
    viola_mcp::serve(home.to_path_buf(), own_name, board)?;
    run::log_self_exit(0, None);
    Ok(ExitCode::SUCCESS)
}

/// The home and its `diagnostics/`, each that exists, pass strict-modes; one that cannot be read
/// fails. A home not made yet passes: the log open creates it owner-only.
fn home_passes(home: &Path) -> bool {
    [home.to_path_buf(), home.join("diagnostics")].iter().all(
        |path| match std::fs::symlink_metadata(path) {
            Ok(_) => viola_state::strict::check_path(path).is_ok(),
            Err(error) => error.kind() == io::ErrorKind::NotFound,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_passes_a_new_home_and_refuses_an_unreadable_one() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(home_passes(&tmp.path().join("not-yet")));
        let home = tmp.path().join("home");
        viola_state::fs::create_private_dir(&home).expect("home");
        assert!(home_passes(&home));
    }

    #[cfg(unix)]
    #[test]
    fn home_passes_refuses_a_group_writable_home_or_diagnostics_dir() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        viola_state::fs::create_private_dir(&home.join("diagnostics")).expect("dirs");
        std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o770)).expect("chmod");
        assert!(!home_passes(&home));
        std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o700)).expect("chmod");
        assert!(home_passes(&home));
        let diagnostics = home.join("diagnostics");
        std::fs::set_permissions(&diagnostics, std::fs::Permissions::from_mode(0o720))
            .expect("chmod");
        assert!(!home_passes(&home));
    }
}
