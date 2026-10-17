//! `harness_session`: a booted session for `viola-e2e` scenarios (test-plan §3 `run` step 2). It
//! boots through the library `boot` from the test executable's bin dir, so the spawned
//! `supervise` process owns the outer PTYs and `cleanup`'s pid + start-time proof stays exact. It
//! never shells out to `agent-run`.

use std::path::{Path, PathBuf};

use super::boot::{AgentsMode, BootOptions, DEFAULT_CLI_VERSION, InstanceSpec, boot, instance_dir};
use super::cleanup::{Target, cleanup};
use super::{Outcome, Workspace, bin_dir_from_exe, read_json, session_record_path};

/// What a scenario boots: the instances (default `overseer` + `builder`) and the two boot flags.
#[derive(Debug, Clone)]
pub struct SessionOptions {
    pub instances: Vec<String>,
    pub statusline_echo: bool,
    pub agents_mode: AgentsMode,
}

impl Default for SessionOptions {
    fn default() -> Self {
        Self {
            instances: vec!["overseer".to_owned(), "builder".to_owned()],
            statusline_echo: false,
            agents_mode: AgentsMode::Recorded,
        }
    }
}

/// Keeps the home for a post-mortem: under `AGENT_RUN_KEEP_HOMES=1`, or for a panicking test under
/// `AGENT_RUN_KEEP_FAILED=1`.
pub fn keep_home(keep_homes: bool, keep_failed: bool, panicking: bool) -> bool {
    keep_homes || (keep_failed && panicking)
}

/// A booted, stamped session, stopped when dropped even after a failed assertion (a leaked
/// supervisor locks its `.exe` on Windows); its home goes then too unless `keep_home` says otherwise.
pub struct HarnessSession {
    ws: Workspace,
    session: String,
    home: PathBuf,
    bin_dir: PathBuf,
    cleaned: bool,
}

/// Boots `<test>-<label>-<pid>` and panics with the boot document when it does not reach readiness.
pub fn harness_session(test: &str, label: &str, opts: SessionOptions) -> HarnessSession {
    let ws = Workspace::from_build();
    let bin_dir = bin_dir_from_exe(&std::env::current_exe().expect("test exe"));
    let session = format!("{test}-{label}-{}", std::process::id());
    let instances = opts
        .instances
        .iter()
        .map(|name| InstanceSpec::parse(name).expect("valid instance name"))
        .collect();
    let booted = boot(&BootOptions {
        ws: ws.clone(),
        bin_dir: bin_dir.clone(),
        session: session.clone(),
        instances,
        cli_version: DEFAULT_CLI_VERSION.to_owned(),
        build: false,
        stamp: true,
        statusline_echo: opts.statusline_echo,
        agents_mode: opts.agents_mode,
    });
    assert_eq!(booted.code, 0, "harness_session boot: {}", booted.doc);
    let record: super::SessionRecord =
        read_json(&session_record_path(&ws, &session)).expect("session record");
    HarnessSession {
        ws,
        session,
        home: record.home,
        bin_dir,
        cleaned: false,
    }
}

impl HarnessSession {
    pub fn session(&self) -> &str {
        &self.session
    }

    pub fn workspace(&self) -> &Workspace {
        &self.ws
    }

    pub fn home(&self) -> &Path {
        &self.home
    }

    pub fn instance_dir(&self, name: &str) -> PathBuf {
        instance_dir(&self.home, name)
    }

    pub fn bin_dir(&self) -> &Path {
        &self.bin_dir
    }

    /// The harness `cleanup` report for this session, the home kept for the drop's decision.
    pub fn cleanup(mut self) -> Outcome {
        self.cleaned = true;
        cleanup(&self.ws, Target::Session(&self.session), true)
    }
}

impl Drop for HarnessSession {
    fn drop(&mut self) {
        if !self.cleaned {
            let _ = cleanup(&self.ws, Target::Session(&self.session), true);
        }
        let flag = |name: &str| std::env::var(name).is_ok_and(|v| v == "1");
        let keep = keep_home(
            flag("AGENT_RUN_KEEP_HOMES"),
            flag("AGENT_RUN_KEEP_FAILED"),
            std::thread::panicking(),
        );
        if !keep && let Some(parent) = self.home.parent() {
            let _ = std::fs::remove_dir_all(parent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_home_is_keep_homes_or_a_failed_test_under_keep_failed() {
        for (label, keep_homes, keep_failed, panicking, want) in [
            ("nothing set", false, false, false, false),
            ("keep homes", true, false, false, true),
            ("keep failed, passing", false, true, false, false),
            ("keep failed, panicking", false, true, true, true),
            ("panicking alone", false, false, true, false),
        ] {
            assert_eq!(
                keep_home(keep_homes, keep_failed, panicking),
                want,
                "{label}"
            );
        }
    }

    #[test]
    fn session_options_default_to_the_pair_recorded_without_the_echo() {
        let opts = SessionOptions::default();
        assert_eq!(opts.instances, ["overseer", "builder"]);
        assert!(!opts.statusline_echo);
        assert_eq!(opts.agents_mode, AgentsMode::Recorded);
    }
}
