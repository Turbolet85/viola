# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the new verification code builds on the pinned 1.98.1 toolchain and must hold the 1.96 MSRV floor the msrv job checks
stack-and-technologies#1.2 · yes · the check on the hook and CLI paths stays std and blocking, and only the viola-mcp tools run through the Tokio client
stack-and-technologies#1.3 · no · the chunk adds no subcommand or flag, so the clap parser and its plain help are untouched
stack-and-technologies#1.4 · no · the chunk reaches no PTY seam, ConPTY sideload or vendored binary (Not needed); its Windows pipe read takes windows-sys through the wrapper-IPC row
stack-and-technologies#1.5 · no · the screen model and wrapper-side send readiness are unchanged (Not needed)
stack-and-technologies#1.6 · yes · the server identity is read on the connected handle, the Windows SQOS Identification pipe open and the Unix per-user socket, plus the Tokio flavour for mcp
stack-and-technologies#1.7 · yes · the check reads pid, started_at and endpoint from snapshot.json, an atomically replaced JSON snapshot; no database is added
stack-and-technologies#1.8 · yes · a snapshot with a newer or unparseable v is read as absent for these clients, the envelope outcome the held item keeps
stack-and-technologies#1.9 · yes · the snapshot read sits under the strict-modes check (libc and windows-sys owner and DACL) that strict.rs re-reads
stack-and-technologies#1.10 · no · the chunk hashes no exe or ConPTY companion; pinned-binary hashing is owned by Home and code-bearing file integrity
stack-and-technologies#1.11 · yes · the refusal document's declared key order (preserve_order) and the tolerant snapshot parse the check reads
stack-and-technologies#1.12 · yes · the refusal line the CLI verbs and mcp write to role files goes through this tracing stack
stack-and-technologies#1.13 · yes · the instance name the check joins into instances/<name>/snapshot.json is a ViolaName, tried before the join
stack-and-technologies#1.14 · yes · the started_at the check compares is an RFC 3339 UTC chrono value; the scope's whole seconds differ from this row's millisecond precision
stack-and-technologies#1.15 · yes · the new refusal is a thiserror variant of viola-channel or viola-state, with anyhow kept to the root bin's edge
stack-and-technologies#1.16 · yes · the check precedes the first frame of each method on the JSON-RPC 2.0 over ndjson channel this row names
stack-and-technologies#1.17 · yes · the server's pid and process start time are read through sysinfo 0.39.6 for the liveness comparison
stack-and-technologies#1.18 · yes · the send, wait, last and answer tools' isError instance-unreachable result, on the rmcp server feature set
stack-and-technologies#1.19 · no · N/A, viola calls no model API
stack-and-technologies#1.20 · no · N/A, a phone view is a later version
stack-and-technologies#1.21 · no · the chunk changes no plugin file or install path; the plugin rewrite stays owned by Home and code-bearing file integrity
stack-and-technologies#1.22 · no · standing S3 · every OS-specific branch compiles and is tested on its CI runner (windows-2025, macos-latest, ubuntu-latest)
stack-and-technologies#1.23 · yes · the new code must pass clippy -D warnings with the print and dbg bans and the cargo-modules orphans gate
stack-and-technologies#1.24 · no · the release line is post-v1 and the chunk touches no release step
