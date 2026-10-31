//! `viola plugin install`: writes the MCP-only plugin for a driver viola does not wrap, in a local
//! marketplace under `<home>/marketplace/`, and prints the CLI's own two commands that add it. viola
//! spawns no `claude` here and writes nothing under `~/.claude`: the human runs the two commands
//! (architecture §Established Decisions [Plugin Scope]). Every directory of the tree is checked
//! before anything under the home is created, written or opened, its own log included.

use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::json;
use tracing::instrument;
use viola_agent_claude::marketplace::{self, PLUGIN_REF};
use viola_core::UnreachableCause;
use viola_state::fs::{create_private_dir, replace_private};

use super::client;
use crate::{human, run};

/// The marketplace dir under the home: written only by this verb.
const MARKETPLACE: &str = "marketplace";
const FILE_MODE: u32 = 0o600;

#[derive(clap::Args)]
pub(crate) struct PluginArgs {
    #[command(subcommand)]
    verb: PluginVerb,
}

#[derive(clap::Subcommand)]
enum PluginVerb {
    /// Write the plugin's marketplace and print the two commands that add it to Claude Code
    Install(InstallArgs),
}

#[derive(clap::Args)]
pub(crate) struct InstallArgs {
    /// Print one JSON document on stdout
    #[arg(long)]
    json: bool,
}

pub(crate) fn plugin(home: &Path, args: &PluginArgs) -> anyhow::Result<ExitCode> {
    match &args.verb {
        PluginVerb::Install(install) => self::install(home, install),
    }
}

fn install(home: &Path, args: &InstallArgs) -> anyhow::Result<ExitCode> {
    let home = std::path::absolute(home)?;
    // Before the log opens: opening it narrows the home to its owner, which would erase the very
    // state this check refuses. A refused check creates nothing and writes no line.
    if !tree_passes(&home) {
        if args.json {
            client::unreachable_document(UnreachableCause::StrictModes);
        } else {
            let (unable, hint) = human::home_state_pair();
            human::refuse(unable, hint);
        }
        return Ok(ExitCode::from(21));
    }
    if let Some(name) = client::own_name() {
        client::start(&home, &name)?;
    }
    let dir = write_tree(&home)?;
    if args.json {
        client::document(&json!({
            "v": 1,
            "ok": {"marketplace": marketplace::NAME, "plugin": PLUGIN_REF},
        }));
    } else {
        let word = human::command_word(&dir.to_string_lossy());
        let _ = human::write_plugin_installed(&mut io::stdout().lock(), &word);
    }
    run::log_self_exit(0, None);
    Ok(ExitCode::SUCCESS)
}

/// The home and each directory of the tree, each that exists, pass strict-modes; one that cannot
/// be read fails. A home not made yet passes: this verb creates it owner-only.
fn tree_passes(home: &Path) -> bool {
    tree_dirs(home)
        .iter()
        .all(|path| match std::fs::symlink_metadata(path) {
            Ok(_) => viola_state::strict::check_path(path).is_ok(),
            Err(error) => error.kind() == io::ErrorKind::NotFound,
        })
}

/// The home, then the tree's four directories, each after its parent.
fn tree_dirs(home: &Path) -> [PathBuf; 5] {
    let root = home.join(MARKETPLACE);
    let plugin = root.join(marketplace::NAME);
    [
        home.to_path_buf(),
        root.clone(),
        root.join(".claude-plugin"),
        plugin.clone(),
        plugin.join(".claude-plugin"),
    ]
}

/// The tree, written whole: every directory 0700, each file replaced 0600; any other entry there
/// is left as found. The `.mcp.json` names the canonical path of this exe and the home, both in
/// forward slashes. Returns the marketplace dir.
#[instrument(skip_all, name = "cli.plugin_install")]
fn write_tree(home: &Path) -> anyhow::Result<PathBuf> {
    let exe = super::canonical_home(&std::env::current_exe()?);
    for dir in &tree_dirs(home)[1..] {
        create_private_dir(dir)?;
    }
    // The home exists now, so this is its one canonical spelling, as `run` spells it.
    let home = super::canonical_home(home);
    let root = home.join(MARKETPLACE);
    for (rel, content) in marketplace::marketplace_files(&fwd(&exe), &fwd(&home)) {
        replace_private(&root.join(rel), content.as_bytes(), FILE_MODE)?;
    }
    Ok(root)
}

fn fwd(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_dirs_are_the_home_and_the_four_marketplace_dirs() {
        let home = Path::new("/h");
        let root = home.join("marketplace");
        assert_eq!(
            tree_dirs(home),
            [
                home.to_path_buf(),
                root.clone(),
                root.join(".claude-plugin"),
                root.join("viola"),
                root.join("viola").join(".claude-plugin"),
            ]
        );
    }

    #[test]
    fn tree_passes_a_new_home_and_its_own_tree() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        assert!(tree_passes(&home));
        for dir in tree_dirs(&home) {
            create_private_dir(&dir).expect("dir");
        }
        assert!(tree_passes(&home));
    }

    #[cfg(unix)]
    #[test]
    fn tree_passes_refuses_any_dir_another_user_could_write() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        for dir in tree_dirs(&home) {
            create_private_dir(&dir).expect("dir");
        }
        for dir in tree_dirs(&home) {
            std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o770)).expect("chmod");
            assert!(!tree_passes(&home), "{dir:?}");
            std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).expect("chmod");
        }
        assert!(tree_passes(&home));
    }

    #[test]
    fn fwd_writes_every_backslash_as_a_slash() {
        assert_eq!(fwd(Path::new(r"C:\h\x")), "C:/h/x");
        assert_eq!(fwd(Path::new("/h/x")), "/h/x");
    }
}
