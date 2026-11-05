//! The strict-modes check (security-plan §Authentication & Authorization, `~/.viola/` access control),
//! first wired on `run`'s `ledger/stamps.json` read: a trusted file, or a folder holding one, that
//! another user could write is refused before it is read (ssh StrictModes style); a folder whose
//! content another user must not read (the home, an instance's directory and its `diagnostics/`) is
//! held to the read rule as well. It reads the owner and the mode; the decisions are plain
//! functions, apart from their reader.

use std::io;
use std::path::{Path, PathBuf};

use viola_core::ViolaName;

use crate::events::EVENTS;
use crate::snapshot::SNAPSHOT;
use crate::stamps::{STAMPS, ledger_dir};

/// Why a path failed the check: fixed messages only (security-plan §Error Handling).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Refused {
    #[error("a trusted state path is not owned by this user")]
    NotOwner,
    #[error("a trusted state path is writable by another user")]
    Writable,
    #[error("a trusted state path is readable by another user")]
    Readable,
    #[error("a trusted state path is a symbolic link")]
    Symlink,
    #[error("a trusted state path is not a directory")]
    NotADirectory,
    #[error("a trusted state path's permissions could not be read")]
    Unreadable,
}

/// The rule a path is held to: no write by another user, or no access by another user at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule {
    Write,
    Read,
}

/// The instance's `diagnostics/` folder.
pub const DIAGNOSTICS: &str = "diagnostics";

/// `events.ndjson` takes the write rule: its folders' `0o077` keeps it from other users.
const EVENTS_RULE: Rule = Rule::Write;

/// A diagnostics file takes the read rule (it is created 0600).
const DIAGNOSTICS_FILE_RULE: Rule = Rule::Read;

/// `ledger/` and `ledger/stamps.json`, each that exists, in that order: the folder first, since an
/// entry written by another user between the checks would land through it.
pub fn check_stamps(home: &Path) -> Result<(), Refused> {
    let dir = ledger_dir(home);
    check_existing(&[(dir.clone(), Rule::Write), (dir.join(STAMPS), Rule::Write)])
}

/// The home, `instances/`, the instance directory, its `diagnostics/`, its `snapshot.json` and its
/// `events.ndjson`, each that exists, a folder before what it holds: the check a process runs
/// before it uses a value read from either file or opens anything under the instance. The home, the
/// instance directory and `diagnostics/` take the read rule, the rest the write rule.
pub fn check_instance(home: &Path, instance_dir: &Path) -> Result<(), Refused> {
    check_existing(&[
        (home.to_path_buf(), Rule::Read),
        (home.join("instances"), Rule::Write),
        (instance_dir.to_path_buf(), Rule::Read),
        (instance_dir.join(DIAGNOSTICS), Rule::Read),
        (instance_dir.join(SNAPSHOT), Rule::Write),
        (instance_dir.join(EVENTS), EVENTS_RULE),
    ])
}

/// Which side of the home tree [`check_home_tree`] refused: the caller prints the home's pair or
/// the instance's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeRefused {
    HomeLevel(Refused),
    InstanceLevel(Refused),
}

/// The tree a starting process trusts, each path that exists: the home (the read rule); with a
/// name, the instance's set as [`check_instance`] reads it; then `ledger/`, `bin/` and `plugin/`
/// and, with a pinned copy's key, `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/` and its
/// `hooks/` and `.claude-plugin/` (the write rule).
pub fn check_home_tree(
    home: &Path,
    name: Option<&ViolaName>,
    key: Option<&str>,
) -> Result<(), TreeRefused> {
    check_existing(&[(home.to_path_buf(), Rule::Read)]).map_err(TreeRefused::HomeLevel)?;
    if let Some(name) = name {
        let instance_dir = home.join("instances").join(AsRef::<str>::as_ref(name));
        check_instance(home, &instance_dir).map_err(TreeRefused::InstanceLevel)?;
    }
    let mut code = vec![
        (ledger_dir(home), Rule::Write),
        (home.join("bin"), Rule::Write),
        (home.join("plugin"), Rule::Write),
    ];
    if let Some(key) = key {
        let bin = home.join("bin").join(key);
        let plugin = home.join("plugin").join(key);
        code.extend([
            (bin.clone(), Rule::Write),
            (
                bin.join(format!("viola{}", std::env::consts::EXE_SUFFIX)),
                Rule::Write,
            ),
            (plugin.clone(), Rule::Write),
            (plugin.join("hooks"), Rule::Write),
            (plugin.join(".claude-plugin"), Rule::Write),
        ]);
    }
    check_existing(&code).map_err(TreeRefused::HomeLevel)
}

/// A diagnostics folder or file, before it is created or opened: a symlink is refused (an open
/// through it lands wherever it points), then a folder takes the read rule and a file the write
/// rule, and on Unix no group or other bit. An absent path passes.
pub fn check_diagnostics(path: &Path) -> Result<(), Refused> {
    let meta = match std::fs::symlink_metadata(path) {
        Ok(meta) => meta,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(Refused::Unreadable),
    };
    if meta.file_type().is_symlink() {
        return Err(Refused::Symlink);
    }
    let rule = if meta.is_dir() {
        Rule::Read
    } else {
        DIAGNOSTICS_FILE_RULE
    };
    check_rule(path, rule)
}

/// The per-user socket directory, by one `lstat` before a wrapper binds or a client connects
/// (security-plan §Authentication & Authorization, IPC access control (Unix)): a symlink, a
/// non-directory, another owner and any group or other bit are refused and left as found. An absent
/// path passes: a start then makes it, and a client's connect fails on its own.
pub fn check_socket_dir(dir: &Path) -> Result<(), Refused> {
    use std::os::unix::fs::MetadataExt as _;
    let meta = match std::fs::symlink_metadata(dir) {
        Ok(meta) => meta,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(Refused::Unreadable),
    };
    socket_dir_verdict(
        meta.file_type().is_symlink(),
        meta.is_dir(),
        meta.mode(),
        meta.uid(),
        unix::euid(),
    )
}

/// [`check_socket_dir`] on the directory a recorded endpoint lies in.
pub fn check_endpoint_dir(endpoint: &str) -> Result<(), Refused> {
    Path::new(endpoint)
        .parent()
        .map_or(Ok(()), check_socket_dir)
}

/// A symlink, then a non-directory, then the read rule.
pub fn socket_dir_verdict(
    is_symlink: bool,
    is_dir: bool,
    mode: u32,
    uid: u32,
    euid: u32,
) -> Result<(), Refused> {
    if is_symlink {
        return Err(Refused::Symlink);
    }
    if !is_dir {
        return Err(Refused::NotADirectory);
    }
    unix_read_verdict(mode, uid, euid)
}

/// Each path that exists, in order, under its rule; an absent one is skipped, and one that cannot
/// be statted is refused.
fn check_existing(paths: &[(PathBuf, Rule)]) -> Result<(), Refused> {
    for (path, rule) in paths {
        match std::fs::symlink_metadata(path) {
            Ok(_) => check_rule(path, *rule)?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(_) => return Err(Refused::Unreadable),
        }
    }
    Ok(())
}

/// The write rule on `path` when it exists; one that cannot be statted is refused.
pub fn check_present(path: &Path) -> Result<(), Refused> {
    check_existing(&[(path.to_path_buf(), Rule::Write)])
}

fn check_rule(path: &Path, rule: Rule) -> Result<(), Refused> {
    match rule {
        Rule::Write => check_path(path),
        Rule::Read => check_read(path),
    }
}

/// One path through the reader and the write rule.
pub fn check_path(path: &Path) -> Result<(), Refused> {
    let (mode, uid) = unix::reading(path).map_err(|_| Refused::Unreadable)?;
    unix_verdict(mode, uid, unix::euid())
}

/// One path through the reader and the read rule.
pub fn check_read(path: &Path) -> Result<(), Refused> {
    let (mode, uid) = unix::reading(path).map_err(|_| Refused::Unreadable)?;
    unix_read_verdict(mode, uid, unix::euid())
}

/// Owned by the effective user, and no group or other write bit.
pub fn unix_verdict(mode: u32, uid: u32, euid: u32) -> Result<(), Refused> {
    if uid != euid {
        return Err(Refused::NotOwner);
    }
    if mode & 0o022 != 0 {
        return Err(Refused::Writable);
    }
    Ok(())
}

/// The write rule, then no group or other bit at all (security-plan detail
/// authentication-authorization.md, the read rule).
pub fn unix_read_verdict(mode: u32, uid: u32, euid: u32) -> Result<(), Refused> {
    unix_verdict(mode, uid, euid)?;
    if mode & 0o077 != 0 {
        return Err(Refused::Readable);
    }
    Ok(())
}

mod unix {
    use std::io;
    use std::os::unix::fs::MetadataExt as _;
    use std::path::Path;

    /// The mode bits and the owner uid; a symlink is followed, as `stat` does.
    pub(super) fn reading(path: &Path) -> io::Result<(u32, u32)> {
        let meta = std::fs::metadata(path)?;
        Ok((meta.mode(), meta.uid()))
    }

    pub(super) fn euid() -> u32 {
        // SAFETY: `geteuid` takes no arguments, cannot fail and touches no memory.
        unsafe { libc::geteuid() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::owner_0600(0o100_600, 1000, 1000, Ok(()))]
    #[case::owner_0700_dir(0o040_700, 1000, 1000, Ok(()))]
    #[case::group_read_only(0o100_640, 1000, 1000, Ok(()))]
    #[case::group_write(0o100_620, 1000, 1000, Err(Refused::Writable))]
    #[case::other_write(0o040_702, 1000, 1000, Err(Refused::Writable))]
    #[case::other_owner(0o100_600, 0, 1000, Err(Refused::NotOwner))]
    #[case::other_owner_and_writable(0o100_666, 1001, 1000, Err(Refused::NotOwner))]
    fn unix_verdict_needs_the_owner_and_no_foreign_write(
        #[case] mode: u32,
        #[case] uid: u32,
        #[case] euid: u32,
        #[case] verdict: Result<(), Refused>,
    ) {
        assert_eq!(unix_verdict(mode, uid, euid), verdict);
    }

    #[test]
    fn refused_messages_are_fixed() {
        let messages: Vec<String> = [
            Refused::NotOwner,
            Refused::Writable,
            Refused::Readable,
            Refused::Symlink,
            Refused::NotADirectory,
            Refused::Unreadable,
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        assert_eq!(
            messages,
            [
                "a trusted state path is not owned by this user",
                "a trusted state path is writable by another user",
                "a trusted state path is readable by another user",
                "a trusted state path is a symbolic link",
                "a trusted state path is not a directory",
                "a trusted state path's permissions could not be read",
            ]
        );
    }

    /// A foreign owner and every kind the one account of a runner cannot make, by injected values.
    #[rstest]
    #[case::owner_0700(false, true, 0o040_700, 1000, 1000, Ok(()))]
    #[case::symlink_to_0700(true, false, 0o120_700, 1000, 1000, Err(Refused::Symlink))]
    #[case::symlink_as_a_directory(true, true, 0o040_700, 1000, 1000, Err(Refused::Symlink))]
    #[case::regular_file(false, false, 0o100_700, 1000, 1000, Err(Refused::NotADirectory))]
    #[case::foreign_owner(false, true, 0o040_700, 0, 1000, Err(Refused::NotOwner))]
    #[case::world_readable(false, true, 0o040_755, 1000, 1000, Err(Refused::Readable))]
    #[case::group_readable(false, true, 0o040_750, 1000, 1000, Err(Refused::Readable))]
    #[case::group_writable(false, true, 0o040_770, 1000, 1000, Err(Refused::Writable))]
    fn socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit(
        #[case] is_symlink: bool,
        #[case] is_dir: bool,
        #[case] mode: u32,
        #[case] uid: u32,
        #[case] euid: u32,
        #[case] verdict: Result<(), Refused>,
    ) {
        assert_eq!(
            socket_dir_verdict(is_symlink, is_dir, mode, uid, euid),
            verdict
        );
    }

    /// Real paths, each left as found: a symlink to a 0700 directory reads as the link, where the
    /// read rule's own check follows it and passes.
    #[test]
    fn check_socket_dir_reads_the_path_itself_and_leaves_it_as_found() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let private = tmp.path().join("private");
        crate::fs::create_private_dir(&private).expect("0700");
        assert_eq!(check_socket_dir(&private), Ok(()));

        let open = tmp.path().join("open");
        crate::fs::create_private_dir(&open).expect("made");
        std::fs::set_permissions(&open, std::fs::Permissions::from_mode(0o755)).expect("chmod");
        assert_eq!(check_socket_dir(&open), Err(Refused::Readable));
        let mode = std::fs::metadata(&open).expect("meta").permissions().mode();
        assert_eq!(mode & 0o777, 0o755);

        let link = tmp.path().join("link");
        std::os::unix::fs::symlink(&private, &link).expect("symlink");
        assert_eq!(check_read(&link), Ok(()), "the check that follows the link");
        assert_eq!(check_socket_dir(&link), Err(Refused::Symlink));
        assert!(
            std::fs::symlink_metadata(&link)
                .expect("meta")
                .file_type()
                .is_symlink()
        );

        let file = tmp.path().join("file");
        std::fs::write(&file, b"x").expect("a file");
        assert_eq!(check_socket_dir(&file), Err(Refused::NotADirectory));

        assert_eq!(check_socket_dir(&tmp.path().join("absent")), Ok(()));
        assert_eq!(
            check_socket_dir(Path::new("dir\0name")),
            Err(Refused::Unreadable)
        );
    }

    /// The endpoint's own parent is what is read.
    #[test]
    fn check_endpoint_dir_reads_the_endpoints_directory() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("sockets");
        crate::fs::create_private_dir(&dir).expect("0700");
        let endpoint = dir.join("viola-x.sock");
        let endpoint = endpoint.to_str().expect("utf-8");
        assert_eq!(check_endpoint_dir(endpoint), Ok(()));
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o755)).expect("chmod");
        assert_eq!(check_endpoint_dir(endpoint), Err(Refused::Readable));
    }

    #[test]
    fn check_stamps_of_a_home_without_a_ledger_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(check_stamps(&tmp.path().join("home")), Ok(()));
    }

    /// A path holding a NUL byte fails the stat with a kind other than `NotFound`, before any
    /// filesystem call: only an absent path is skipped, and a stat that fails is a refusal.
    #[test]
    fn check_stamps_of_a_path_that_cannot_be_statted_is_unreadable() {
        assert_eq!(
            check_stamps(Path::new("home\0dir")),
            Err(Refused::Unreadable)
        );
    }

    /// A ledger viola created, and the stamps file it wrote, pass on the OS the test runs on.
    #[test]
    fn check_stamps_of_a_home_viola_wrote_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        crate::stamps::update_stamps(&home, |_| b"{}".to_vec()).expect("written");
        assert_eq!(check_stamps(&home), Ok(()));
    }

    #[cfg(unix)]
    #[rstest]
    #[case::stamps_group_writable("stamps", 0o620, Refused::Writable)]
    #[case::stamps_world_writable("stamps", 0o606, Refused::Writable)]
    #[case::ledger_group_writable("ledger", 0o770, Refused::Writable)]
    #[case::ledger_world_writable("ledger", 0o707, Refused::Writable)]
    fn check_stamps_refuses_a_widened_mode(
        #[case] which: &str,
        #[case] mode: u32,
        #[case] refused: Refused,
    ) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        crate::stamps::update_stamps(&home, |_| b"{}".to_vec()).expect("written");
        let path = match which {
            "stamps" => ledger_dir(&home).join(STAMPS),
            _ => ledger_dir(&home),
        };
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(check_stamps(&home), Err(refused));
    }

    /// A home and an instance laid down by the crate's own writers: the directory, a snapshot and
    /// one event line.
    fn instance_viola_wrote(root: &Path) -> (PathBuf, PathBuf) {
        use crate::events::{EventLine, Source, append_event};
        use crate::snapshot::{InstanceSnapshot, Wheel, write_snapshot};
        let home = root.join("home");
        let instance_dir = home.join("instances").join("builder");
        crate::fs::create_private_dir(&instance_dir).expect("the instance dir");
        let snapshot = InstanceSnapshot {
            endpoint: None,
            pid: 1,
            started_at: "s".to_owned(),
            pinned_bin: "b".to_owned(),
            cli_verified: false,
            cli_version: None,
            wheel: Wheel::Driver,
            budget_paused: false,
            budget_override_until: None,
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
            cwd: None,
            statusline_command: None,
            agent_session_id: None,
        };
        write_snapshot(&instance_dir, &snapshot).expect("the snapshot");
        let name = viola_core::ViolaName::try_new("builder".to_owned()).expect("valid");
        let line = EventLine::new(
            &name,
            viola_core::EventKind::BudgetGate,
            Source::Wrapper,
            serde_json::json!({"paused": false}),
            chrono::Utc::now(),
        );
        append_event(&instance_dir, &line).expect("the log");
        (home, instance_dir)
    }

    #[test]
    fn strict_instance_of_a_tree_viola_wrote_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, instance_dir) = instance_viola_wrote(tmp.path());
        assert!(instance_dir.join(SNAPSHOT).is_file() && instance_dir.join(EVENTS).is_file());
        assert_eq!(check_instance(&home, &instance_dir), Ok(()));
    }

    #[test]
    fn strict_instance_of_a_home_with_no_instance_directory_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        let instance_dir = home.join("instances").join("builder");
        assert_eq!(check_instance(&home, &instance_dir), Ok(()));
        crate::fs::create_private_dir(&home).expect("the home");
        assert_eq!(check_instance(&home, &instance_dir), Ok(()));
    }

    /// Only an absent path is skipped: a home that cannot be statted refuses, and so does an
    /// instance directory that cannot be under a home that passes.
    #[test]
    fn strict_instance_of_a_path_that_cannot_be_statted_is_unreadable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, _) = instance_viola_wrote(tmp.path());
        assert_eq!(
            check_instance(Path::new("home\0dir"), Path::new("absent")),
            Err(Refused::Unreadable)
        );
        assert_eq!(
            check_instance(&home, Path::new("instance\0dir")),
            Err(Refused::Unreadable)
        );
    }

    #[cfg(unix)]
    #[rstest]
    #[case::home_group_writable("home", 0o770)]
    #[case::instances_world_writable("instances", 0o707)]
    #[case::instance_dir_group_writable("instance", 0o770)]
    #[case::snapshot_group_writable("snapshot", 0o620)]
    #[case::events_world_writable("events", 0o606)]
    fn strict_instance_refuses_a_widened_mode(#[case] which: &str, #[case] mode: u32) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, instance_dir) = instance_viola_wrote(tmp.path());
        let path = match which {
            "home" => home.clone(),
            "instances" => home.join("instances"),
            "instance" => instance_dir.clone(),
            "snapshot" => instance_dir.join(SNAPSHOT),
            _ => instance_dir.join(EVENTS),
        };
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(check_instance(&home, &instance_dir), Err(Refused::Writable));
    }

    #[rstest]
    #[case::owner_dir_0700(0o040_700, 1000, 1000, Ok(()))]
    #[case::owner_file_0600(0o100_600, 1000, 1000, Ok(()))]
    #[case::group_read_dir(0o040_740, 1000, 1000, Err(Refused::Readable))]
    #[case::other_read_dir(0o040_704, 1000, 1000, Err(Refused::Readable))]
    #[case::group_exec_dir(0o040_710, 1000, 1000, Err(Refused::Readable))]
    #[case::other_exec_dir(0o040_701, 1000, 1000, Err(Refused::Readable))]
    #[case::group_read_file(0o100_640, 1000, 1000, Err(Refused::Readable))]
    #[case::group_write_dir(0o040_770, 1000, 1000, Err(Refused::Writable))]
    #[case::other_owner(0o040_700, 0, 1000, Err(Refused::NotOwner))]
    fn unix_read_verdict_needs_the_write_rule_and_no_foreign_bit(
        #[case] mode: u32,
        #[case] uid: u32,
        #[case] euid: u32,
        #[case] verdict: Result<(), Refused>,
    ) {
        assert_eq!(unix_read_verdict(mode, uid, euid), verdict);
    }

    /// A home with one instance, a ledger, a pinned copy and a plugin folder, laid down by the
    /// crate's own writers; its key.
    fn home_tree_viola_wrote(root: &Path) -> (PathBuf, PathBuf, String) {
        let (home, instance_dir) = instance_viola_wrote(root);
        crate::fs::create_private_dir(&instance_dir.join(DIAGNOSTICS)).expect("diagnostics");
        crate::stamps::update_stamps(&home, |_| b"{}".to_vec()).expect("stamps");
        let exe = root.join("source-exe");
        std::fs::write(&exe, b"abc").expect("source");
        let pinned = crate::pin::pin_exe(&home, &exe).expect("pinned");
        let plugin = home.join("plugin").join(&pinned.key);
        for dir in ["hooks", ".claude-plugin"] {
            crate::fs::create_private_dir(&plugin.join(dir)).expect("plugin");
        }
        (home, instance_dir, pinned.key)
    }

    fn builder() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    #[test]
    fn check_home_tree_of_a_tree_viola_wrote_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, _, key) = home_tree_viola_wrote(tmp.path());
        assert_eq!(check_home_tree(&home, Some(&builder()), Some(&key)), Ok(()));
        assert_eq!(check_home_tree(&home, None, None), Ok(()));
        assert_eq!(
            check_home_tree(&tmp.path().join("absent"), Some(&builder()), Some(&key)),
            Ok(())
        );
    }

    /// Each path of the tree widened in turn is refused on its own side; with no name the
    /// instance's set is not read, with no key the key's folders are not.
    #[cfg(unix)]
    #[rstest]
    #[case::home_group_readable("", 0o750, Some(TreeRefused::HomeLevel(Refused::Readable)))]
    #[case::home_group_writable("", 0o770, Some(TreeRefused::HomeLevel(Refused::Writable)))]
    #[case::instances_world_writable(
        "instances",
        0o707,
        Some(TreeRefused::InstanceLevel(Refused::Writable))
    )]
    #[case::instance_group_readable(
        "instances/builder",
        0o740,
        Some(TreeRefused::InstanceLevel(Refused::Readable))
    )]
    #[case::instance_diagnostics_other_readable(
        "instances/builder/diagnostics",
        0o704,
        Some(TreeRefused::InstanceLevel(Refused::Readable))
    )]
    #[case::ledger_group_writable("ledger", 0o770, Some(TreeRefused::HomeLevel(Refused::Writable)))]
    #[case::bin_group_writable("bin", 0o770, Some(TreeRefused::HomeLevel(Refused::Writable)))]
    #[case::plugin_world_writable("plugin", 0o707, Some(TreeRefused::HomeLevel(Refused::Writable)))]
    #[case::bin_key_group_writable(
        "bin/KEY",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::pinned_copy_group_writable(
        "bin/KEY/viola",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::plugin_key_group_writable(
        "plugin/KEY",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::plugin_hooks_group_writable(
        "plugin/KEY/hooks",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::plugin_manifest_dir_group_writable(
        "plugin/KEY/.claude-plugin",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::ledger_group_readable_passes("ledger", 0o750, None)]
    fn check_home_tree_refuses_each_widened_path(
        #[case] rel: &str,
        #[case] mode: u32,
        #[case] refused: Option<TreeRefused>,
    ) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, _, key) = home_tree_viola_wrote(tmp.path());
        let path = rel
            .split('/')
            .filter(|part| !part.is_empty())
            .fold(home.clone(), |path, part| {
                path.join(if part == "KEY" { key.as_str() } else { part })
            });
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(
            check_home_tree(&home, Some(&builder()), Some(&key)),
            refused.map_or(Ok(()), Err)
        );
        let instance_side = matches!(refused, Some(TreeRefused::InstanceLevel(_)));
        let key_side = rel.contains("KEY");
        if instance_side || key_side {
            assert_eq!(check_home_tree(&home, None, None), Ok(()), "{rel}");
        }
    }

    #[test]
    fn check_diagnostics_of_what_viola_wrote_or_of_nothing_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("home").join(DIAGNOSTICS);
        assert_eq!(check_diagnostics(&dir), Ok(()));
        crate::fs::create_private_dir(&dir).expect("dir");
        let file = dir.join("run-builder.ndjson");
        crate::fs::open_private_append(&file).expect("file");
        assert_eq!(check_diagnostics(&dir), Ok(()));
        assert_eq!(check_diagnostics(&file), Ok(()));
        assert_eq!(
            check_diagnostics(Path::new("diag\0dir")),
            Err(Refused::Unreadable)
        );
    }

    #[cfg(unix)]
    #[rstest]
    #[case::dir_group_readable(true, 0o750, Refused::Readable)]
    #[case::dir_group_writable(true, 0o770, Refused::Writable)]
    #[case::file_group_readable(false, 0o640, Refused::Readable)]
    #[case::file_other_writable(false, 0o602, Refused::Writable)]
    fn check_diagnostics_refuses_a_widened_mode(
        #[case] dir: bool,
        #[case] mode: u32,
        #[case] refused: Refused,
    ) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let diagnostics = tmp.path().join(DIAGNOSTICS);
        crate::fs::create_private_dir(&diagnostics).expect("dir");
        let file = diagnostics.join("run-builder.ndjson");
        crate::fs::open_private_append(&file).expect("file");
        let path = if dir { &diagnostics } else { &file };
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(check_diagnostics(path), Err(refused));
    }

    /// A role file reached through a symlink is refused, wherever the link points, and so is a
    /// symlinked `diagnostics/` folder.
    #[cfg(unix)]
    #[test]
    fn check_diagnostics_refuses_a_symlinked_role_file_and_folder() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let diagnostics = tmp.path().join(DIAGNOSTICS);
        crate::fs::create_private_dir(&diagnostics).expect("dir");
        let target = tmp.path().join("elsewhere.ndjson");
        crate::fs::open_private_append(&target).expect("target");
        let file = diagnostics.join("run-builder.ndjson");
        std::os::unix::fs::symlink(&target, &file).expect("symlink");
        assert_eq!(check_diagnostics(&target), Ok(()));
        assert_eq!(check_diagnostics(&file), Err(Refused::Symlink));
        let folder = tmp.path().join("linked-diagnostics");
        std::os::unix::fs::symlink(&diagnostics, &folder).expect("symlink");
        assert_eq!(check_diagnostics(&folder), Err(Refused::Symlink));
    }

    /// The read rule reaches the home, the instance directory and its `diagnostics/`; a
    /// group-readable `instances/` and snapshot stay under the write rule alone.
    #[cfg(unix)]
    #[rstest]
    #[case::home_group_readable("home", 0o750, Err(Refused::Readable))]
    #[case::instance_other_readable("instance", 0o705, Err(Refused::Readable))]
    #[case::diagnostics_group_readable("diagnostics", 0o740, Err(Refused::Readable))]
    #[case::instances_group_readable("instances", 0o750, Ok(()))]
    #[case::snapshot_group_readable("snapshot", 0o640, Ok(()))]
    #[case::events_group_readable("events", 0o640, Ok(()))]
    fn strict_instance_holds_the_content_folders_to_the_read_rule(
        #[case] which: &str,
        #[case] mode: u32,
        #[case] verdict: Result<(), Refused>,
    ) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, instance_dir) = instance_viola_wrote(tmp.path());
        crate::fs::create_private_dir(&instance_dir.join(DIAGNOSTICS)).expect("diagnostics");
        let path = match which {
            "home" => home.clone(),
            "instances" => home.join("instances"),
            "instance" => instance_dir.clone(),
            "diagnostics" => instance_dir.join(DIAGNOSTICS),
            "snapshot" => instance_dir.join(SNAPSHOT),
            _ => instance_dir.join(EVENTS),
        };
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(check_instance(&home, &instance_dir), verdict);
    }
}
