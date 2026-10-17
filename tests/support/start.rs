//! A `viola run` start as root tests read it (test-plan §6 Path 1): the role file's lines, a second
//! start refused outside any terminal, the endpoint a start computes and the plugin dir it handed
//! the fake agent.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;
use viola_core::ViolaName;

use super::fake::{self, FAKE, of_kind};
use super::home::{VIOLA, Wrapper};
use super::piped::Piped;

/// The complete lines of `builder`'s run role file.
pub fn role_lines(home: &Path) -> Vec<Value> {
    super::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
}

/// How many children `builder`'s starts have spawned.
pub fn child_starts(home: &Path) -> usize {
    role_lines(home)
        .iter()
        .filter(|l| l["event"] == "process-start" && l["subject"] == "claude-child")
        .count()
}

/// A second `viola run builder` outside any terminal. A start that is (wrongly) not refused gets
/// Ctrl-C at once, so the call still ends.
pub fn run_refused(home: &Path) -> Output {
    let piped = Piped::spawn(
        Command::new(VIOLA)
            .arg("--home")
            .arg(home)
            .args(["run", "builder", "--", FAKE]),
    );
    piped.write(b"\x03");
    piped.finish()
}

/// `builder`'s role file holds the refused start's `process-exit` with `detail`.
pub fn refused_with(home: &Path, detail: &str) -> bool {
    role_lines(home).iter().any(|l| {
        l["event"] == "process-exit"
            && l["subject"] == "self"
            && l["level"] == "ERROR"
            && l["exit_code"] == 1
            && l["detail"] == detail
    })
}

/// The endpoint `viola run builder` computes for `home` (its golden vectors are viola-channel's).
pub fn endpoint_of(home: &Path) -> String {
    let home = std::path::absolute(home).expect("absolute home");
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    viola_channel::endpoint_path(&name, &home).expect("endpoint")
}

/// The `--plugin-dir` the wrapper's last start handed the fake agent.
pub fn last_plugin_dir(wrapper: &Wrapper) -> PathBuf {
    let receipt = fake::receipt(&wrapper.receipt());
    let start = of_kind(&receipt, "start")
        .last()
        .copied()
        .cloned()
        .expect("start receipt");
    PathBuf::from(start["plugin_dir"].as_str().expect("plugin_dir"))
}
