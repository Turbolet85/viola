# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · no toolchain, rust-version floor or dependency version moved; Cargo.toml's reach is the new `[[test]]` table only
stack-and-technologies#1.2 · no · Tokio stays built only inside mcp (TokioClient alone), and the new wrapper filter sits on the existing std accept thread
stack-and-technologies#1.3 · no · no subcommand or clap feature added; the session adds no verb
stack-and-technologies#1.4 · no · viola-pty and src/conpty.rs are not reached; portable-pty and the other PTY crates are unchanged
stack-and-technologies#1.5 · no · vt100's use on run and verify is untouched; no screen code is reached
stack-and-technologies#1.6 · no · interprocess's sync API, the Tokio flavour's placement and the Unix uid and 0600 socket stand as stated; no dependency moved
stack-and-technologies#1.7 · no · no database; no store is reached
stack-and-technologies#1.8 · no · diag-line.v1.json gains enum members at v1 with no version change, and process-log lines still carry no v
stack-and-technologies#1.9 · no · viola_state::fs is not edited and the strict-modes check still reads modes and owner through libc; the new socket-dir check adds a metadata read under the same read rule
stack-and-technologies#1.10 · no · sha2 and the content-hash use are not reached
stack-and-technologies#1.11 · no · serde and serde_json unchanged; the new word has no serde path and the schema's version holds
stack-and-technologies#1.12 · no · the log line format and levels are owned by the obs plan and unchanged; the session adds only detail values
stack-and-technologies#1.13 · no · nutype newtypes are not reached
stack-and-technologies#1.14 · no · chrono is not reached
stack-and-technologies#1.15 · no · Refused gains a variant, not a derive; the row names no count, and thiserror per crate stands
stack-and-technologies#1.16 · no · per-wrapper local sockets for request/response stand; the directory check adds no broker
stack-and-technologies#1.17 · no · sysinfo and liveness are not reached; a peer euid never decides liveness
stack-and-technologies#1.18 · no · rmcp features and the five tools are unchanged; the new isError cause is not a stack claim
stack-and-technologies#1.19 · no · no model API; not reached
stack-and-technologies#1.20 · no · no phone view; not reached
stack-and-technologies#1.21 · no · cargo install and the plugin's include_str! are unchanged; the session adds no install or deployment step
stack-and-technologies#1.22 · no · no CI step changed; ci#41329861618 measures the run but adds no job or matrix entry
stack-and-technologies#1.23 · no · lint, deny and hyperfine tooling unchanged; no new lib or bin target for the orphans gate
stack-and-technologies#1.24 · no · release tooling is not reached
stack-and-technologies#2.1 · yes · stale · the row names the pid as the one identity each frame-writing client checks before its first frame; the session adds a peer euid check on Linux and macOS (client.rs, F3), so the identity checked now includes euid
