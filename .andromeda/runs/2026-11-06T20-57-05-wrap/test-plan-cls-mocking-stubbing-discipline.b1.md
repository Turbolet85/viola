@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: its clock-seam sites live in src/run/send.rs, src/cmd/run.rs, src/run/links.rs and tests/cli_links.rs, none of which the session reaches
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: the row is HTTP mocking N/A and names no file the session reaches
@answer n=3 entity=mocking-stubbing-discipline#2.2 v=no
why: the dependency move names only portable-pty, nix, the serial family, pin-utils, memoffset and termios, so the claim that mock_instant and tokio test-util sit in no manifest still holds on Cargo.toml
@answer n=4 entity=mocking-stubbing-discipline#2.3 v=yes mode=stale
why: crates/viola-pty/src/lib.rs, which holds the Pty trait this row names as mocked, is reached whole under the portable-pty bump and the Windows remnant retirement, and so is the viola-fake-agent binary file
@answer n=5 entity=mocking-stubbing-discipline#2.4 v=yes mode=stale
why: tests/support/start.rs (kill_wrapper) and crates/viola-e2e/tests/mcp_tools.rs (kill_by_pid) are reached whole by the change, so the kill sites the row names may have moved
@answer n=6 entity=mocking-stubbing-discipline#2.5 v=yes mode=stale
why: tests/hook_fail_open.rs, which holds the Impostor the row names, is reached whole by the change
@answer n=7 entity=mocking-stubbing-discipline#2.6 v=yes mode=stale
why: tests/security_negatives_socket_dir.rs, which holds the Form helper the row names, is reached whole by the change
@answer n=8 entity=mocking-stubbing-discipline#2.7 v=no
why: place_private_new_with lives in crates/viola-state/src/fs.rs, which the session does not reach
@answer n=9 entity=mocking-stubbing-discipline#2.8 v=no
why: peer_euid_admitted and serve_admitting live in crates/viola-channel/src/server.rs, which the session does not reach
