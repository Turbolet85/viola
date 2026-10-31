# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · the toolchain pins and the workspace `rust-version = "1.96"` floor stand; Cargo.toml's only change is one `[[test]]` entry, and the task's Untouched keeps the pins as they stood
stack-and-technologies#1.2 · no · Tokio stays inside `mcp` and the hot path stays sync; the session's MCP and CLI edits add error documents, no async code
stack-and-technologies#1.3 · no · the subcommand list is unchanged, no verb was added or renamed; src/main.rs changed only its catch site and the `json_asked` helper
stack-and-technologies#1.4 · no · viola-pty, portable-pty and the ConPTY pair are untouched; the reached-file changes are error documents and cause words, not the PTY seam
stack-and-technologies#1.5 · no · vt100 readers are unchanged: `run` still reads no content and `verify` still reads `Screen::rows()`; verify.rs changed only its log codes and `Ended` returns
stack-and-technologies#1.6 · no · the Windows pipe open is viola-channel's `client::open_handle`, which is not a reached file; the src/cmd/client.rs changes are the Unreachable enum and document writers
stack-and-technologies#1.7 · no · no store, writer or append path changed and `without_endpoint` writes nothing, so the stack still has no database
stack-and-technologies#1.8 · no · process-log lines still carry no `v` and their version still lives in the filename; `schemas/diag-line.v1.json` stays v1 with only the `process-exit` detail enum grown
stack-and-technologies#1.9 · no · replace_private, place_private_new, File::lock and the strict-modes check are unchanged; `without_endpoint` opens nothing and stays behind the strict-modes check
stack-and-technologies#1.10 · no · the session changed no hashing path and the Cargo.toml edit adds no dependency, so the sha2 row stands
stack-and-technologies#1.11 · no · serde_json and `preserve_order` are unchanged, and the exit-21 document keeps its declared key order with `v` first, so the example still holds
stack-and-technologies#1.12 · no · the tracing dependencies and the log line format are unchanged; the new process-exit codes are `detail` words in the existing log, not a logging change
stack-and-technologies#1.13 · no · the nutype newtypes (ViolaName, Percent, DialogHold) are not touched by any session change
stack-and-technologies#1.14 · no · chrono is not touched and no timestamp path changed
stack-and-technologies#1.15 · no · no crate error enum or RefusalReason gained anything; the anyhow internal-error path returns the same error with a fixed body, so the context chain stays in the instance detail file
stack-and-technologies#1.16 · no · no broker was added; the per-wrapper sockets and JSON-RPC framing are unchanged, and JSON-RPC errors are unmoved
stack-and-technologies#1.17 · no · sysinfo and the heartbeat as the primary signal are unchanged; the liveness.rs addition `without_endpoint` reads only directory metadata
stack-and-technologies#1.18 · no · the rmcp version, its features and the five tools (send, wait, last, answer, list) are unchanged; only the error body of an unreachable result changed
stack-and-technologies#1.19 · no · the stack still calls no model API and no session change touches this row
stack-and-technologies#1.20 · no · the mobile row is N/A and the session brings no phone view
stack-and-technologies#1.21 · no · `cargo install` and the plugin compiled in by include_str! and written out by `viola run` are unchanged; plugin.rs changed only its refusal document
stack-and-technologies#1.22 · no · the CI matrix, the pinned actions and the toolchain installs are unchanged; this row names no nextest line
stack-and-technologies#1.23 · no · the lint, deny and schema-check tools and their pins are unchanged; Cargo.toml's new `[[test]]` adds no dependency
stack-and-technologies#1.24 · no · the release row is out of scope for v1 and the session touches no distribution path
stack-and-technologies#2.1 · no · the pid read from the stream and the identity check before a client's first frame are unchanged; the new server-verify-failed word names the failure without moving the check, and it does not arise on macOS, which reports no pid
