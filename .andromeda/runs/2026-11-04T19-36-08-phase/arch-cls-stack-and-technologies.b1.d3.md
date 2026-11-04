@answer n=1 entity=stack-and-technologies#1.1 v=no
why: the Rust 1.98.1 pin and 1.96 floor are a build-wide rule every change owes, and the msrv job is outside the chunk's scope
@answer n=2 entity=stack-and-technologies#1.2 v=yes
why: the pty path and run's feed thread stay on std threads with blocking I/O, and viola-pty stays a sync crate under the Tokio ban
@answer n=3 entity=stack-and-technologies#1.3 v=yes
why: the chunk's tests drive the run and list verbs and the `--` separator through clap's parser
@answer n=4 entity=stack-and-technologies#1.4 v=yes
why: the pty seam's openpty backend on Linux and macOS, with libc's Unix-only tty mode, is what the chunk adds
@answer n=5 entity=stack-and-technologies#1.5 v=yes
why: run's feed thread does pre-send readiness and modal detection on the openpty output, which the parity must keep
@answer n=6 entity=stack-and-technologies#1.6 v=yes
why: the Unix socket endpoint, its 0600 chmod and the socket-dir check before bind and connect are the chunk's IPC endpoint
@answer n=7 entity=stack-and-technologies#1.7 v=yes
why: events.ndjson and the snapshots are the files whose fields must match across OSes
@answer n=8 entity=stack-and-technologies#1.8 v=yes
why: readers skip unknown fields and process-log lines carry no v, so the schema filename versions the one diagnostics schema
@answer n=9 entity=stack-and-technologies#1.9 v=yes
why: the Unix strict-modes check through libc and the mode-before-write rule for snapshots are read per OS by the parity tests
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: content hashing covers the bin/ and plugin/ pinned copies and the ConPTY companions, none of which the chunk reaches
@answer n=11 entity=stack-and-technologies#1.11 v=yes
why: every --json document must carry the same fields, and preserve_order keeps their declared key order on every OS
@answer n=12 entity=stack-and-technologies#1.12 v=yes
why: one-line JSON process logs in diagnostics/ are the schema the chunk keeps the same on every OS
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: the nutype newtypes ViolaName, Percent and DialogHold are not named by the scope
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: RFC 3339 millisecond timestamps in events and snapshots feed the 1 s heartbeat and 5 s stale rule of the v1-40 witness
@answer n=15 entity=stack-and-technologies#1.15 v=yes
why: viola-pty's hand-written PtyError is where the openpty backend's spawn, read, write, resize, wait and kill errors land
@answer n=16 entity=stack-and-technologies#1.16 v=yes
why: per-wrapper local sockets carrying JSON-RPC over ndjson are the endpoint the Unix frame verbs use
@answer n=17 entity=stack-and-technologies#1.17 v=yes
why: pid plus process start time decide that a process is gone, the rule the kill-and-revive test reads child death by on Unix
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the MCP server's tools and rmcp features are not named by the scope, and the chunk adds no MCP surface
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: viola calls no model API and the chunk runs no live claude
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: the mobile framework is a later version, outside the chunk
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: local-only install and the plugin write-out are not named by the scope
@answer n=22 entity=stack-and-technologies#1.22 v=yes
why: the scope names the test, perf and lint CI jobs over ubuntu-latest and macos-latest beside windows-2025
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: the lint job, the G1 to G4 gate tools (ripgrep, jq, jsonschema) and the clippy print bans bind the new Unix code
@answer n=24 entity=stack-and-technologies#1.24 v=no
why: release distribution is out of v1 parity
@answer n=25 entity=stack-and-technologies#2.1 v=yes
why: the server pid and euid that Unix peer_creds gives are the identity the frame-writing clients check before their first frame
