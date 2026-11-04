@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the new Unix code builds on the pinned 1.98.1 toolchain on ubuntu-latest and macos-latest and must stay inside the 1.96 floor
@answer n=2 entity=stack-and-technologies#1.2 v=yes
why: the openpty run and pty path must use std threads and blocking I/O, viola-pty staying a sync crate (need 16)
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: the chunk adds no clap verb, flag or help text, and the parser feature set and plain help output are untouched
@answer n=4 entity=stack-and-technologies#1.4 v=yes
why: the pty seam over portable-pty =0.8.1 and its openpty backend on Linux and macOS is the chunk's core (need 1)
@answer n=5 entity=stack-and-technologies#1.5 v=yes
why: run's feed-thread readiness and modal detection must behave the same on the openpty path the chunk brings to Unix
@answer n=6 entity=stack-and-technologies#1.6 v=yes
why: the Unix socket endpoint, its per-user 0700 directory, the check before bind and connect, and the 0600 chmod are the chunk's IPC (need 8)
@answer n=7 entity=stack-and-technologies#1.7 v=yes
why: events.ndjson appends and atomically replaced snapshots are the stores the one-schema rule covers (task line 4)
@answer n=8 entity=stack-and-technologies#1.8 v=yes
why: event and snapshot records carry v, while process-log lines carry none and are versioned by diag-line.v1.json (needs 5 and 6)
@answer n=9 entity=stack-and-technologies#1.9 v=yes
why: the Unix strict-modes, socket-dir chmod and events.ndjson write rule go through libc and the shared replace_private helper (needs 8 and 14)
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: the chunk touches no pinned copy, bin/ or plugin hash, and the ConPTY companion pin is Windows-only (not_needed 5 and 13)
@answer n=11 entity=stack-and-technologies#1.11 v=yes
why: the --json documents, ndjson event lines and IPC frames the chunk makes uniform across OSes are serde_json documents with declared key order and the default recursion limit (needs 6 and 8)
@answer n=12 entity=stack-and-technologies#1.12 v=yes
why: the scope names the one-line diagnostics discipline (codes only, one line per write, the same on every OS, need 5), so the convention binds here
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: the chunk adds no domain name, percent or dialog-hold value, and its endpoint names are not ViolaName
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: the events, snapshots and heartbeat beat age the chunk makes uniform carry RFC 3339 UTC millisecond timestamps read for the 5 s stale rule (need 11)
@answer n=15 entity=stack-and-technologies#1.15 v=yes
why: the openpty backend's errors are viola-pty's hand-written PtyError, and context chains stay in the owner-only detail file (need 1 and need 5)
@answer n=16 entity=stack-and-technologies#1.16 v=yes
why: the Unix local socket carrying JSON-RPC over ndjson is the per-wrapper IPC whose endpoint rule the chunk governs (need 8)
@answer n=17 entity=stack-and-technologies#1.17 v=yes
why: pid plus process start time decide liveness and the killed-wrapper child check on Unix (needs 11 and 13)
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the chunk adds no MCP tool and changes no MCP surface, and no MCP test is named in its scope (not_needed 1)
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: viola calls no model API and the chunk runs no live claude session (task line 6, not_needed 4)
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: a phone view is a later version and the chunk has no mobile surface (not_needed 12)
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the plugin layout written out by viola run is owned by Version done-check and the chunk does not move it (not_needed 13)
@answer n=22 entity=stack-and-technologies#1.22 v=yes
why: the three-OS matrix against the fake agent is named by the scope, with the ubuntu and macos jobs it re-reads (need 3, task line 5)
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: the lint job, the print_stdout and disallowed-macros bans, the G1 to G4 tools and the per-sync-crate tokio ban bind the chunk's new code (needs 4 and 16, task line 5)
@answer n=24 entity=stack-and-technologies#1.24 v=no
why: dist, signing and self_update are outside v1 parity scope (not_needed 12)
@answer n=25 entity=stack-and-technologies#2.1 v=yes
why: the chunk's IPC pid and euid reads on Linux and macOS, with no pid on macOS, and the Unix euid admission rely on these peer-creds reads (needs 8, 9 and 10)
