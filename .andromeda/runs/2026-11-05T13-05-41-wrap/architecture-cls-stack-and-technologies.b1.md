@answer n=1 entity=stack-and-technologies#1.1 v=yes mode=contradict
why: the binary is no longer viola.exe on any target, the session made viola Linux-only and removed the Windows build
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: the Tokio containment and std-thread model are unchanged by the session
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: clap and its feature set are not touched by any changed file
@answer n=4 entity=stack-and-technologies#1.4 v=yes mode=contradict
why: windows-sys, the ConPTY sideload, its vendored conpty.dll and OpenConsole.exe, and the DLL search restriction are removed, and libc is now a plain dependency (F4) rather than Unix only
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: vt100 and the run and verify screen uses are not reached by the Windows removals
@answer n=6 entity=stack-and-technologies#1.6 v=yes mode=contradict
why: the Windows named pipe, the SQOS client open and the pipe DACL are removed, so the endpoint is a Unix socket only
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: the state store is still ndjson plus atomically replaced snapshots on Linux, which the session keeps
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: the diag-line schema file path still holds and versioning by filename is unchanged, only its os enum and a property moved
@answer n=9 entity=stack-and-technologies#1.9 v=yes mode=contradict
why: replace_private is now one attempt, the Windows lock rationale and windows-sys owner and DACL arm are removed
@answer n=10 entity=stack-and-technologies#1.10 v=yes mode=contradict
why: the Windows ConPTY companion hash check through pin_companions is removed
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: serde and serde_json are untouched and the unbounded_depth ban in deny.toml still holds, only deny.toml's targets and licence exceptions moved
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: tracing and tracing-subscriber dependents are unchanged by the session
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: nutype newtypes are not touched by any changed file
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: chrono is not touched by any changed file
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: thiserror and anyhow use is unchanged and PtyError stays hand-written, the error census is a separate entity
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: per-wrapper local sockets still hold on Linux and the no-broker claim is unchanged
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: sysinfo and the liveness heartbeat rule are unchanged by the session
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: rmcp features and the MCP server are not reached by the session
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: viola calls no model API, a claim the session does not touch
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: a later-version phone view claim is not touched by the session
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: cargo install and the compiled-in plugin are unchanged, the deployment is still local-only
@answer n=22 entity=stack-and-technologies#1.22 v=yes mode=contradict
why: the windows-2025 and macos-latest legs are removed, CI is ubuntu-latest only and the matrix row no longer holds
@answer n=23 entity=stack-and-technologies#1.23 v=no
why: the lint tools and their gates are unchanged, deny-sync.toml's tokio ban stays with only its targets moved
@answer n=24 entity=stack-and-technologies#1.24 v=no
why: the release tooling claim names no OS and no changed file
@answer n=25 entity=stack-and-technologies#2.1 v=yes mode=contradict
why: the Windows GetNamedPipeServerProcessId pid leg and the macOS no-peer-pid leg are retired, and the Windows DACL and pid controls no longer exist
