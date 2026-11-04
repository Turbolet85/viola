@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the chunk's new Unix code and the ubuntu and macOS jobs build on this pinned toolchain, and the 1.96 floor bounds that code.
@answer n=2 entity=stack-and-technologies#1.2 v=yes
why: std threads and blocking I/O on the run and pty path, with Tokio contained to mcp (task item 16).
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: the chunk adds no verb and changes no parser feature; clap is only used and its plain help is outside the scope.
@answer n=4 entity=stack-and-technologies#1.4 v=yes
why: the openpty backend on Linux and macOS behind the pty seam is what the chunk builds (task item 1).
@answer n=5 entity=stack-and-technologies#1.5 v=yes
why: the run feed thread that the openpty path shares must never read row content, with Screen::rows staying verify-only.
@answer n=6 entity=stack-and-technologies#1.6 v=yes
why: the Unix socket endpoint, its directory check before bind and each connect, and the 0600 mode are the chunk's IPC endpoint (task item 8).
@answer n=7 entity=stack-and-technologies#1.7 v=yes
why: the ndjson event log and atomically replaced snapshots are the shared schema the chunk keeps identical across OSes (task items 4 and 6).
@answer n=8 entity=stack-and-technologies#1.8 v=yes
why: the no-v rule for process-log lines and the version carried in schemas/diag-line.v1.json bind the shared diagnostics schema (task item 5).
@answer n=9 entity=stack-and-technologies#1.9 v=yes
why: replace_private, the strict-modes check and libc private-dir creation are what the per-user 0700 socket dir and the state writes use.
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: the content hash covers the bin/ and plugin/ pinned copies and the Windows ConPTY companions, all outside the chunk (task not_needed items 5 and 13).
@answer n=11 entity=stack-and-technologies#1.11 v=yes
why: the event line, snapshot and --json documents' serde_json shapes and key order bind the one-schema rule, and the default recursion limit binds the openpty path's external-byte parses (task items 6 and 17).
@answer n=12 entity=stack-and-technologies#1.12 v=yes
why: the process-log line format is where the chunk must carry the same fields and closed words on every OS (task items 4 and 5).
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: the nutype newtypes ViolaName, Percent and DialogHold are not reached by the chunk's scope.
@answer n=14 entity=stack-and-technologies#1.14 v=yes
why: RFC 3339 UTC millisecond timestamps are the format every OS's events, snapshots and --json fields must write under the one schema.
@answer n=15 entity=stack-and-technologies#1.15 v=no standing=S5
why: recording context chains only in the owner-only instance detail file is the content-bearing detail convention every new log line owes.
@answer n=16 entity=stack-and-technologies#1.16 v=yes
why: each Unix wrapper's endpoint is the per-wrapper local socket the chunk adds, with no broker in the path.
@answer n=17 entity=stack-and-technologies#1.17 v=yes
why: sysinfo's pid plus process start time decides liveness and the killed-child check on Unix (task items 11 and 13).
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the chunk adds no MCP tool or surface and changes no rmcp feature (task not_needed item 1).
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: the chunk runs no live claude and adds no model call, so the driven-CLI fact reaches no chunk code.
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: a phone view is a later version and the chunk touches no mobile surface (task not_needed item 12).
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: deployment and the plugin written out by run belong to the plugin layout, not the chunk (task not_needed item 13).
@answer n=22 entity=stack-and-technologies#1.22 v=yes
why: the three-OS CI matrix against the fake agent is the CI/CD decision the test, perf and lint jobs run on (task item 3, ci.yml re-read).
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: the lint job, the G1 to G4 gate tools and the jsonschema G4 check are the obs gates and lint job the chunk re-reads (task item 4).
@answer n=24 entity=stack-and-technologies#1.24 v=no
why: dist, signing and self_update are public distribution, outside v1 parity (task not_needed item 12).
@answer n=25 entity=stack-and-technologies#2.1 v=yes
why: the peer pid and euid read from the connected stream, with none on macOS, are the frame-writing client's identity check and the macOS carve-out (task items 9 and 10).
