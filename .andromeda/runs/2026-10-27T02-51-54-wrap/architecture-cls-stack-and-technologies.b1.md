# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · no toolchain pin, rust-version floor or dependency moved, and the change adds no crate
stack-and-technologies#1.2 · no · the std-thread and Tokio split lives in the crate graph and src/cmd, neither reached, and viola-state stays sync
stack-and-technologies#1.3 · no · clap dispatch lives in src/cmd, which the session did not reach and whose subcommand set is unchanged
stack-and-technologies#1.4 · no · the portable-pty seam lives in viola-pty, not reached, and the ConPTY companion write in pin.rs is unchanged
stack-and-technologies#1.5 · no · the vt100 screen model lives in viola-agent-claude and run's feed thread, neither reached
stack-and-technologies#1.6 · no · the interprocess transport, the pipe open and the Unix socket bind live in viola-channel, which the session did not reach
stack-and-technologies#1.7 · no · snapshots and logs still go through the unchanged replace helper and append path, since the change only placed the pinned copy beside them
stack-and-technologies#1.8 · no · no viola record, schema or snapshot field changed, and the fake agent's test-only receipt is not a viola format
stack-and-technologies#1.9 · yes · contradict · fs.rs gained place_private_new over persist_noclobber beside replace_private, so "one shared helper" no longer holds
stack-and-technologies#1.10 · no · the lost placement re-hashes the standing copy before reusing it, so re-hash before every reuse still holds in pin.rs, and the hash function and key are unchanged
stack-and-technologies#1.11 · no · serde and serde_json serve channel frames, logs and snapshots, none of which the session changed
stack-and-technologies#1.12 · no · the tracing setup, veil and diagnostics line format are unchanged, and the lost placement logs only the existing run.pin_copy ok
stack-and-technologies#1.13 · no · the nutype newtypes live in viola-core, which the session did not reach
stack-and-technologies#1.14 · no · chrono timestamps are unchanged and the session adds no timestamp
stack-and-technologies#1.15 · no · the per-crate thiserror and bin-edge anyhow rules are unchanged, since place_private_new returns the existing StateError and reuses PinError::HashMismatch with no new error type
stack-and-technologies#1.16 · no · no message broker exists and the session adds no IPC
stack-and-technologies#1.17 · no · the sysinfo liveness and the claude agents join live in viola-state liveness and viola-agent-claude, neither reached, and no liveness reading changed
stack-and-technologies#1.18 · yes · stale · the reached mcp_tools.rs changed Path 7's MCP leg (default pair booted unstamped, race workaround retired), and the report does not show that its reading, that the E2E owns the viola mcp child and awaits its exit, still holds in that file
stack-and-technologies#1.19 · no · viola calls no model API and the session adds none
stack-and-technologies#1.20 · no · the mobile framework is N/A and the session adds no phone surface
stack-and-technologies#1.21 · no · cargo install and the include_str! plugin embedding are untouched, and the plugin still goes out through the unchanged replace_private_shared in pin_and_plugin
stack-and-technologies#1.22 · no · the workflows, matrix and toolchain installs are CI files the session did not reach, and the changed tests run in the same suites against the same fake agent
stack-and-technologies#1.23 · no · the lint, format and gate configuration lives in files the session did not reach
stack-and-technologies#1.24 · no · the dist and self_update release tooling is not reached and no release artifact changed
stack-and-technologies#2.1 · no · the peer-pid read on both clients lives in viola-channel's client code, which the session did not reach
