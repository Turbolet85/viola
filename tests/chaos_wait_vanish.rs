//! Chaos (test-plan §6 Chaos suite; obs-plan §4 Scenario `wait` / `last` Cleanup): the wrapper is
//! killed from outside while a `viola wait` is parked on it. The caller is `instance-unreachable`
//! with `detail` `endpoint-lost` (exit 21), never a wrapper fault, and its role file says the call
//! was open when it ended.

#[allow(dead_code)]
mod support;

use std::process::{Command, Stdio};
use std::time::Instant;

use serde_json::Value;
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, snapshot_data, workspace_path};
use support::start::kill_wrapper;
use support::watch::{WITHIN, Watch};

fn role(home: &std::path::Path, file: &str) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join(file))
}

#[test]
fn chaos_wait_parked_wrapper_killed_exits_21() {
    let fixtures = workspace_path("fixtures/claude");
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &["--fixtures", fixtures.to_str().expect("utf-8 path")],
    );
    let home = wrapper.home().to_path_buf();
    let pid = snapshot_data(&wrapper.instance_dir()).expect("snapshot")["pid"]
        .as_u64()
        .and_then(|p| u32::try_from(p).ok())
        .expect("pid");
    let mut parked = Command::new(VIOLA)
        .arg("--home")
        .arg(&home)
        .args(["wait", "builder", "--json"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola wait");

    let watch = Watch::start("parked");
    let deadline = Instant::now() + WITHIN;
    while !role(&home, "run-builder.ndjson")
        .iter()
        .any(|l| l["event"] == "channel-request" && l["method"] == "wait")
    {
        if parked.try_wait().expect("try_wait").is_some() {
            panic!("viola wait exited before it parked");
        }
        watch.note("no wait request yet");
        watch.deadline_check(deadline, "the wait never reached the wrapper");
        std::thread::yield_now();
    }
    kill_wrapper(pid);

    let watch = Watch::start("exit");
    let deadline = Instant::now() + WITHIN;
    while parked.try_wait().expect("try_wait").is_none() {
        watch.note("still parked");
        if Instant::now() >= deadline {
            let _ = parked.kill();
        }
        watch.deadline_check(deadline, "viola wait outlived its wrapper");
        std::thread::yield_now();
    }
    let out = parked.wait_with_output().expect("output");
    assert_eq!(out.status.code(), Some(21));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"endpoint-lost\"}\n"
    );
    assert!(out.stderr.is_empty());
    let exits: Vec<Value> = role(&home, "cli-builder.ndjson")
        .into_iter()
        .filter(|l| l["event"] == "process-exit")
        .collect();
    assert_eq!(exits.len(), 1, "{exits:?}");
    assert_eq!(exits[0]["exit_code"], 21);
    assert_eq!(exits[0]["detail"], "endpoint-lost");
    assert_eq!(exits[0]["during"], "call");
    drop(wrapper);
}
