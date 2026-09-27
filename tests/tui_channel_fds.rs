//! E2, the Unix half (test-plan §6 E2): the wrapped child holds nothing of viola's. On Linux its
//! live descriptor table is read from `/proc/<pid>/fd`: 0, 1 and 2 are the PTY; no descriptor is a
//! socket, the endpoint or its lock, or any path under the viola home (the instance dir, the event
//! log, the snapshot, the heartbeat, the pinned copy). The one exemption is the pair of fixture
//! files this test hands the fake agent, by exact path: its receipt and its control file, which
//! the fixture keeps under the home. Elsewhere the agent's own `fds` receipt bounds the count: 0, 1,
//! 2, the receipt file and the directory handle its `/dev/fd` listing opens (measured on Linux:
//! `[0, 1, 2, 3, 4]`).
#![cfg(unix)]

#[allow(dead_code)]
mod support;

use rstest::rstest;
use support::fake::{self, of_kind};
use support::home::{Wrapper, booted_wrapper, snapshot_data};

#[rstest]
fn tui_channel_fds_the_child_holds_nothing_of_violas(booted_wrapper: Wrapper) {
    let snapshot = snapshot_data(&booted_wrapper.instance_dir()).expect("snapshot");
    let endpoint = snapshot["endpoint"].as_str().expect("the channel is bound");
    let receipt = fake::wait_for(&booted_wrapper.receipt(), "the fds receipt", |lines| {
        !of_kind(lines, "fds").is_empty()
    });
    let fds: Vec<u64> = of_kind(&receipt, "fds")[0]["fds"]
        .as_array()
        .expect("fds")
        .iter()
        .map(|fd| fd.as_u64().expect("fd number"))
        .collect();
    for fd in [0, 1, 2] {
        assert!(fds.contains(&fd), "fd {fd} in {fds:?}");
    }
    assert!(
        fds.len() <= 5,
        "a viola descriptor reached the child: {fds:?}"
    );

    #[cfg(target_os = "linux")]
    {
        use std::path::{Path, PathBuf};

        let canonical = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
        let home = canonical(booted_wrapper.home());
        let fixtures: Vec<PathBuf> = [
            booted_wrapper.receipt(),
            fake::control_path(booted_wrapper.home(), &booted_wrapper.name),
        ]
        .iter()
        .map(|p| canonical(p))
        .collect();
        let endpoint = Path::new(endpoint);
        let lock = endpoint.with_extension("lock");
        let child = snapshot["child_pid"].as_u64().expect("child_pid");
        let mut table: Vec<(u64, PathBuf)> = std::fs::read_dir(format!("/proc/{child}/fd"))
            .expect("the child's fd table")
            .map(|e| {
                let entry = e.expect("fd");
                let fd = entry
                    .file_name()
                    .to_string_lossy()
                    .parse()
                    .expect("fd number");
                (fd, std::fs::read_link(entry.path()).expect("fd target"))
            })
            .collect();
        table.sort();
        for (fd, target) in &table {
            let shown = target.to_string_lossy();
            if *fd <= 2 {
                assert!(
                    shown.starts_with("/dev/pts/"),
                    "fd {fd} is not the pty: {table:?}"
                );
            }
            assert!(
                !shown.starts_with("socket:"),
                "a socket reached the child: {table:?}"
            );
            assert!(
                target != endpoint && *target != lock,
                "the endpoint reached the child: {table:?}"
            );
            let viola_file = target.starts_with(&home) && !fixtures.contains(target);
            assert!(!viola_file, "fd {fd} holds a viola home file: {table:?}");
        }
    }
    let _ = endpoint;
    assert_eq!(booted_wrapper.stop().code(), Some(0));
}
