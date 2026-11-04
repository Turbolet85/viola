@answer n=1 entity=stack-and-technologies#1.1 v=no
why: Cargo.toml, Cargo.lock and rust-toolchain.toml are unchanged, so the Rust 1.98.1 pin, the 1.96 floor and the msrv job stand.
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: No tokio or hot-path threading change, and the reached test files do not touch the hybrid runtime model.
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: No CLI verb or flag was added; the reached test files only call the existing subcommands.
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: The product PTY seam, HostTerminal and the portable-pty, windows-sys and libc dependencies are unchanged; the outer-PTY witness and Frozen are test-side callers.
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: The witness reads the raw SGR byte stream rather than Screen::rows(), so the vt100 role and verify's probe screens still hold.
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: One endpoint per run and the socket and peer-creds rules are unchanged, and the reached test files open no new endpoint.
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: No ndjson log or JSON snapshot store changed, and the test-only Watch report file is not a product store.
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: No record version, snapshot schema or schemas/ file moved, so the no-ORM and versioning claims stand.
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: The replace_private, lock and append primitives are unchanged, and moving Frozen out of cli_instance_state.rs is a test move, not a primitive change.
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: The product sha2 use and the root known-answer test contract_content_hash.rs stay as they were, and the new census binary hashes nothing.
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: No frame, log or snapshot format changed, and the new junit-os-sets.py reader is an operator Python script outside serde.
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: The diagnostics line format and the tracing setup are unchanged, and the Watch report file is a test-only temp file.
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: No nutype domain newtype was added or changed, and no reached file touches ViolaName, Percent or DialogHold.
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: No timestamp format changed, and the witness's back-dated heartbeat is a test-side value.
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: No thiserror derive or anyhow edge changed, and no file under src/ or crates/*/src/ moved.
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: No message broker was introduced, and the per-wrapper local sockets are unchanged.
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: sysinfo remains the pid and start-time reader and the heartbeat stays the primary signal; the stale-row witness exercises the existing rule and Frozen's ps polling is test-only.
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: The rmcp features and the MCP tools are unchanged, and the reached cross_list_rows.rs and other test files do not touch the MCP server.
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: No model API call was introduced, and the session touches only tests.
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: The mobile row is out of scope and nothing in the session touches it.
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: The cargo install and include_str plugin path are unchanged and plugin/ is unmoved; the reached cli_plugin_install.rs edit is gate or freeze wiring, not the compile-in or write-out.
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: .github/ is unchanged and the three-OS matrix, runners and pre-push stage stand; the operator script is not a CI step and the row names no run ids or per-OS counts.
@answer n=23 entity=stack-and-technologies#1.23 v=no
why: Cargo.toml is unchanged, so syn and proc-macro2 remain viola-e2e's alone and the census binary adds no lint, coverage or fuzz tool.
@answer n=24 entity=stack-and-technologies#1.24 v=no
why: No release or distribution change was made, and dist and self_update are untouched.
@answer n=25 entity=stack-and-technologies#2.1 v=no
why: The peer_creds pid and euid reads and the macOS no-peer-pid gap are unchanged, and the session's per-OS counts come from that gap without moving it.
