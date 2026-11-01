# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the new Unix socket-directory, peer-euid and strict-mode code must build on the pinned 1.98.1 toolchain and stay inside the 1.96 floor the msrv job checks
stack-and-technologies#1.2 · yes · new code on the run, hook, send and channel-server paths stays std threads and blocking I/O; Tokio stays inside viola-mcp
stack-and-technologies#1.3 · no · clap's derive parser and its subcommand list; the chunk adds no subcommand or flag and its exit codes come from the typed set, not the parser
stack-and-technologies#1.4 · no · the PTY seam, portable-pty, the ConPTY sideload and libc's host-tty raw mode are not reached; the chunk's libc uid and owner reads sit under rows 1.6 and 1.9
stack-and-technologies#1.5 · no · vt100 screen model for run's pre-send readiness and verify's probes; the chunk acts at the socket ahead of any typed byte
stack-and-technologies#1.6 · yes · the peer euid check and the 0600 chmod after bind sit on the interprocess local socket, and the Tokio client in viola-channel takes the same euid check
stack-and-technologies#1.7 · no · the no-database choice; the chunk adds no store, and the snapshot and log write forms it touches are governed by rows 1.8 and 1.9
stack-and-technologies#1.8 · yes · a replayed snapshot carries no endpoint, so the instance must read unreachable; the replay rule the chunk's recorded-endpoint read depends on
stack-and-technologies#1.9 · yes · replace_private for the recorded endpoint, the .lock arbiter, create_private_dir and the strict-modes check through libc owner and mode, all directly in the chunk
stack-and-technologies#1.10 · no · sha2 hashing of exe bytes for bin/ and plugin/ keys and pinned-copy re-hash; the chunk does not touch hashing
stack-and-technologies#1.11 · yes · the new refusal documents (instance-unreachable, refused_document, the MCP isError body) are serde_json output with preserve_order's declared key order
stack-and-technologies#1.12 · no · standing S4 · one-line JSON process logs under diagnostics/, the convention S4 names for each new log line: codes only in the role file, inside the default-deny schema
stack-and-technologies#1.13 · no · nutype domain newtypes ViolaName, Percent and DialogHold; the chunk adds no domain value
stack-and-technologies#1.14 · yes · the server check compares the snapshot's started_at, an RFC 3339 UTC millisecond value, against the OS start time beside the new euid check
stack-and-technologies#1.15 · yes · one thiserror enum per crate for the new refusals, closed values extended in viola-core, anyhow only at the root bin
stack-and-technologies#1.16 · yes · the per-wrapper local socket is the endpoint the directory, mode and euid checks guard before the first JSON-RPC frame; the frame form itself is unchanged
stack-and-technologies#1.17 · yes · the server check reads the connected peer's pid and OS start time from sysinfo and compares them with the snapshot
stack-and-technologies#1.18 · yes · the new refusal reaches MCP as the isError result of viola mcp's frame tools; any rmcp test client stays client-only, never transport-child-process
stack-and-technologies#1.19 · no · no model API; the chunk drives no claude behaviour
stack-and-technologies#1.20 · no · a phone view is a later version; not reached
stack-and-technologies#1.21 · no · cargo install and the plugin include_str! placement; the chunk changes no distribution or plugin placement
stack-and-technologies#1.22 · yes · the macOS exit-cause case and the macOS carve-out are OS-only checks, and the matrix names the native runner on which that OS's code is verified
stack-and-technologies#1.23 · yes · clippy's print_stdout and print_stderr bans on the new hint lines, cargo-deny's tokio and C-crate bans on any new dependency, and the schema-check gate on the diagnostics schemas
stack-and-technologies#1.24 · no · dist and self_update release tooling; distribution stays deferred to a later version
stack-and-technologies#2.1 · yes · the euid check on each frame-writing client, and the server's pid read through peer_creds().pid() with no pid on macOS, are the chunk's client-side and server-side checks
