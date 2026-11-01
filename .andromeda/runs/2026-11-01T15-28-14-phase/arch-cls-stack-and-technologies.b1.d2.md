# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the new peer and directory code must stay inside the 1.96 MSRV floor and the rust-toolchain pin, which the scope names as the stack's pins
stack-and-technologies#1.2 · yes · viola-channel's Tokio client gets the client-side euid check, so Tokio stays inside mcp and the std-thread paths stay blocking
stack-and-technologies#1.3 · no · clap's subcommand set and parser are unchanged; the chunk adds no verb or flag
stack-and-technologies#1.4 · no · the PTY seam is not reached; the checks act at the socket ahead of any typed byte
stack-and-technologies#1.5 · no · the vt100 screen model is not reached; no screen read changes
stack-and-technologies#1.6 · yes · the Unix socket is chmod 0600 after the bind and the uid comes from libc for the per-user directory, both on the interprocess local socket
stack-and-technologies#1.7 · yes · the instance snapshot carries the recorded endpoint, and a replayed snapshot with no endpoint must read as unreachable
stack-and-technologies#1.8 · yes · the replay rebuild of an unsupported-v snapshot binds through the endpoint-less replay reading unreachable; the process-log record rule is standing S4 and is not carried
stack-and-technologies#1.9 · yes · the strict-modes check and private-dir creation through libc (Unix modes and owner) are the directory and strict-modes checks this chunk adds
stack-and-technologies#1.10 · no · content hashing is not touched by any check of this chunk
stack-and-technologies#1.11 · yes · the new refusal --json documents are printed through serde_json and must keep the declared key order (v, cmd, ok); MCP's isError document is the same less v
stack-and-technologies#1.12 · no · standing S4 · each new log line is codes only in its role file under diagnostics/, one line per write, inside the default-deny diag-line schema
stack-and-technologies#1.13 · no · the nutype domain newtypes are not reached
stack-and-technologies#1.14 · no · chrono timestamps are not reached by any refusal or check
stack-and-technologies#1.15 · yes · the new refusals need one thiserror enum per crate with closed values in viola-core, and anyhow only at the root bin
stack-and-technologies#1.16 · yes · each per-wrapper local socket gains the peer euid check before its first JSON-RPC frame
stack-and-technologies#1.17 · yes · the server check compares the connected peer's pid and OS start time (sysinfo process start time) against the snapshot
stack-and-technologies#1.18 · yes · the new refusal reaches MCP clients as the isError result of the frame tools
stack-and-technologies#1.19 · no · no model API is called and the chunk does not reach it
stack-and-technologies#1.20 · no · the phone view is out of scope
stack-and-technologies#1.21 · no · the install path and the plugin write are not changed by any check of this chunk
stack-and-technologies#1.22 · yes · the macOS exit-cause case and the macOS carve-out are OS-only checks, so the rule on where macOS code is verified (the macos-latest runner) binds
stack-and-technologies#1.23 · yes · any dependency added for the peer euid check must pass the deny.toml allow list, the C-crate and sole-root tokio bans, and scripts/channel-tokio-free.sh
stack-and-technologies#1.24 · no · release and self_update distribution are deferred to a later version
stack-and-technologies#2.1 · yes · the peer pid read through peer_creds is the identity the server check compares, with the macOS carve-out where no pid is reported
