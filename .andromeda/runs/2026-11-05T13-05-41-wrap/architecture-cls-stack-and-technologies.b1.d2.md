@answer n=1 entity=stack-and-technologies#1.1 v=yes mode=stale
why: the Windows target is gone, so the viola.exe binary name this row gives is no longer produced
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: Tokio stays only inside mcp and the sync-crates list is unchanged; the session moves no runtime
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: the clap pins, features and subcommand list are not reached by any session change
@answer n=4 entity=stack-and-technologies#1.4 v=yes mode=contradict
why: the ConPTY sideload, the windows-sys TerminateProcess, console and ReadConsoleW layer and the VT modes are removed, and libc is now a plain dependency rather than Unix-only (F4)
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: vt100 readiness and the verify screen fixtures are not named by any session change
@answer n=6 entity=stack-and-technologies#1.6 v=yes mode=contradict
why: the Windows named pipe, the SQOS CreateFileW client open and the SDDL pipe DACL are retired, leaving a Unix socket only
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: the store is still ndjson plus atomically replaced snapshots, and this row states no lock or retry rationale
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: the schema filenames diag-line.v1.json and diag-detail.v1.json still exist; only the os enum and one property changed
@answer n=9 entity=stack-and-technologies#1.9 v=yes mode=contradict
why: the windows-sys owner and DACL arm and the Windows-only lock rationale are removed, and libc is a plain dependency with Unix modes only (F4)
@answer n=10 entity=stack-and-technologies#1.10 v=yes mode=contradict
why: pin_companions and the Windows x64 ConPTY companion SHA-256 check that this row names are removed
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: the serde pins and the banned unbounded_depth feature stand, and the deny.toml edits were targets and licence exceptions only
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: the tracing features and the obs-owned line format are not named by any session change
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: the nutype newtypes and their bounds are not touched by the session
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: chrono's RFC 3339 millisecond UTC timestamps are unchanged
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: thiserror per crate and the hand-written PtyError still hold, and the row names no removed variant
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: the broker-free per-wrapper local sockets with JSON-RPC over ndjson still hold, and the row names no Windows pipe
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: sysinfo pid plus start time and the claude agents enrichment are unchanged; the liveness change sits in the snapshot check, not here
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the rmcp features, the tool set and the MCP scope are untouched by the session
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: viola calls no model API, and the session does not touch that
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: the mobile row is N/A and the session does not reach it
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: cargo install and the plugin written out by viola run are unchanged, and this row does not name the pinned copy path
@answer n=22 entity=stack-and-technologies#1.22 v=yes mode=contradict
why: the windows-2025 and macos-latest legs and the three-OS native matrix are removed, leaving ubuntu-latest only
@answer n=23 entity=stack-and-technologies#1.23 v=yes mode=stale
why: the per-OS coverage floors name a retired OS set, and deny.toml and deny-sync.toml, reached whole with their targets cut to one triple, can move the row's per-OS reading
@answer n=24 entity=stack-and-technologies#1.24 v=no
why: cargo-dist and self_update are future-scope release tooling with no OS named, and the Cargo.toml edits were the windows-sys removal and the libc move only
@answer n=25 entity=stack-and-technologies#2.1 v=yes mode=contradict
why: the Windows GetNamedPipeServerProcessId peer-pid read and the macOS no-peer-pid carve-out are retired, while the Unix SO_PEERCRED pid and euid read stand
