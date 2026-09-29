//! The `run` wrapper's process side: its role lines, the persistent-environment read behind the
//! R8 strip, where the child's program is looked up, and the version gate.

mod env;
pub(crate) mod version_gate;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use viola_agent_claude::{PLUGIN_DIR_FLAG, Refusal, Search, StripPlan};
use viola_core::obs::ObsEvent;
use viola_core::{SERVICE_NAME, VERSION, ViolaName, obs_event};
use viola_pty::Exit;
use viola_state::pin::Pinned;

pub(crate) use env::persistent_names;

pub(crate) fn log_self_start() {
    obs_event!(
        INFO,
        ObsEvent::ProcessStart,
        subject = "self",
        service_name = SERVICE_NAME,
        version = VERSION,
        os = std::env::consts::OS,
        pid = std::process::id(),
        endpoint_kind = viola_channel::ENDPOINT_KIND,
    );
}

/// Names only: the stripped and kept `CLAUDE*` names, never a value (obs-plan D-14); the version
/// gate's reading beside them, and on Windows why the child is on the inbox ConPTY, as a code.
pub(crate) fn log_child_start(
    child_pid: Option<u32>,
    strip: &StripPlan,
    cli_version: Option<&str>,
    cli_verified: bool,
    sideload_fallback: Option<&'static str>,
) {
    let stripped = strip.removed_names();
    let kept = strip.kept_names();
    obs_event!(
        INFO,
        ObsEvent::ProcessStart,
        subject = "claude-child",
        child_pid = child_pid,
        pty_backend = viola_pty::pty_backend(),
        sideload_fallback = sideload_fallback,
        env_stripped_count = strip.count(),
        env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
        env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()),
        cli_version = cli_version,
        cli_verified = cli_verified,
    );
}

pub(crate) fn log_child_exit(exit: Exit) {
    obs_event!(
        INFO,
        ObsEvent::ProcessExit,
        subject = "claude-child",
        child_exit_status = exit.code,
        exit_source = exit.source.as_str(),
    );
}

pub(crate) fn log_self_exit(exit_code: u8, detail: Option<&'static str>) {
    let duration_ms = crate::obs::duration_ms();
    match detail {
        Some(detail) => obs_event!(
            ERROR,
            ObsEvent::ProcessExit,
            subject = "self",
            exit_code = exit_code,
            detail = detail,
            duration_ms = duration_ms,
        ),
        None => obs_event!(
            INFO,
            ObsEvent::ProcessExit,
            subject = "self",
            exit_code = exit_code,
            duration_ms = duration_ms,
        ),
    }
}

/// What the child is started with beyond the inherited environment: the `VIOLA_*` names, the
/// pinned dir first on `PATH`, and the plugin folder ahead of the user's arguments
/// (architecture §Occupied Resources → Environment variables).
pub(crate) struct ChildLaunch {
    pub(crate) env_set: Vec<(OsString, OsString)>,
    pub(crate) args: Vec<OsString>,
}

pub(crate) fn child_launch(
    name: &ViolaName,
    instance_dir: &Path,
    pinned: &Pinned,
    plugin_dir: &Path,
    inherited_path: Option<OsString>,
    user_args: &[OsString],
) -> ChildLaunch {
    let pinned_dir = pinned.path.parent().unwrap_or(&pinned.path).to_path_buf();
    let path = match inherited_path.filter(|p| !p.is_empty()) {
        Some(inherited) => {
            let dirs = std::iter::once(pinned_dir).chain(std::env::split_paths(&inherited));
            std::env::join_paths(dirs).unwrap_or(inherited)
        }
        None => pinned_dir.into_os_string(),
    };
    let env_set = vec![
        ("VIOLA_NAME".into(), OsString::from(name.as_ref())),
        ("VIOLA_DIR".into(), instance_dir.as_os_str().to_owned()),
        ("VIOLA_BIN".into(), OsString::from(&pinned.path_fwd)),
        ("PATH".into(), path),
    ];
    let mut args = vec![
        OsString::from(PLUGIN_DIR_FLAG),
        plugin_dir.as_os_str().to_owned(),
    ];
    args.extend_from_slice(user_args);
    ChildLaunch { env_set, args }
}

/// The program after `--`, looked up by viola itself (never by portable-pty's own search, which
/// takes an extensionless sh shim first on Windows) against this process's `PATH`.
pub(crate) fn resolve_program(program: &OsStr, cwd: &Path) -> Result<PathBuf, Refusal> {
    let path = std::env::var_os("PATH");
    let pathext = std::env::var_os("PATHEXT");
    let search = Search {
        path: path.as_deref(),
        pathext: pathext.as_deref(),
        windows: Search::HOST_IS_WINDOWS,
    };
    viola_agent_claude::resolve_program(program, cwd, search, &|p: &Path| p.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pinned(exe: &Path) -> Pinned {
        Pinned {
            key: "0.1.0-0123456789abcdef".to_owned(),
            path_fwd: exe.to_string_lossy().replace('\\', "/"),
            path: exe.to_path_buf(),
        }
    }

    fn env(launch: &ChildLaunch, key: &str) -> OsString {
        launch
            .env_set
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
            .expect(key)
    }

    #[test]
    fn spawn_env_sets_viola_names_and_prefixes_path() {
        let home = std::env::temp_dir().join("viola-home");
        let instance_dir = home.join("instances").join("builder");
        let pin_dir = home.join("bin").join("0.1.0-0123456789abcdef");
        let exe = pin_dir.join("viola.exe");
        let plugin_dir = home.join("plugin").join("0.1.0-0123456789abcdef");
        let elsewhere = std::env::temp_dir().join("elsewhere");
        let inherited = std::env::join_paths([&elsewhere]).expect("path");
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        let user_args = [OsString::from("--receipt"), OsString::from("r")];

        let launch = child_launch(
            &name,
            &instance_dir,
            &pinned(&exe),
            &plugin_dir,
            Some(inherited),
            &user_args,
        );
        assert_eq!(env(&launch, "VIOLA_NAME"), "builder");
        let dir = PathBuf::from(env(&launch, "VIOLA_DIR"));
        assert_eq!(dir, instance_dir);
        assert_eq!(dir.parent().and_then(Path::parent), Some(home.as_path()));
        let bin = env(&launch, "VIOLA_BIN").into_string().expect("utf-8");
        assert!(!bin.contains('\\'));
        assert!(bin.ends_with("/bin/0.1.0-0123456789abcdef/viola.exe"));
        let path: Vec<PathBuf> = std::env::split_paths(&env(&launch, "PATH")).collect();
        assert_eq!(path, [pin_dir, elsewhere]);
        assert_eq!(launch.env_set.len(), 4);
        assert_eq!(
            launch.args,
            [
                OsString::from("--plugin-dir"),
                plugin_dir.into_os_string(),
                OsString::from("--receipt"),
                OsString::from("r"),
            ]
        );
    }

    #[test]
    fn spawn_env_without_an_inherited_path_puts_only_the_pinned_dir() {
        let pin_dir = std::env::temp_dir().join("bin").join("k");
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        for inherited in [None, Some(OsString::new())] {
            let launch = child_launch(
                &name,
                Path::new("d"),
                &pinned(&pin_dir.join("viola")),
                Path::new("p"),
                inherited,
                &[],
            );
            assert_eq!(env(&launch, "PATH"), pin_dir.as_os_str());
            assert_eq!(
                launch.args,
                [OsString::from("--plugin-dir"), OsString::from("p")]
            );
        }
    }
}
