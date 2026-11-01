# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · the toolchain pins, rust-version 1.96 and the msrv path stand; Cargo.toml gains only a `[[test]]` table and no dependency is added or bumped
stack-and-technologies#1.2 · no · Tokio stays feature-gated in viola-channel's tokio_client and built only through viola-mcp; the new TokioClient::server_euid adds no runtime to the hot hook path
stack-and-technologies#1.3 · no · no clap feature or subcommand changes; the session adds no CLI verb
stack-and-technologies#1.4 · no · viola-pty and src/conpty.rs are not reached; no portable-pty, windows-sys or libc dependency changes
stack-and-technologies#1.5 · no · vt100 use in run and verify is untouched; no screen-model change
stack-and-technologies#1.6 · no · the endpoint's shape, name, per-user directory and 0600 mode stand; the Windows pipe open is unchanged and the Unix endpoint form the row states still holds
stack-and-technologies#1.7 · no · no database or persisted store is added
stack-and-technologies#1.8 · no · diag-line.v1.json keeps its filename and version, only its enum members grow; the no-v rule stands
stack-and-technologies#1.9 · no · viola_state::fs is not edited and create_private_dir gains test callers only; the new directory check reads metadata through the existing read rule, so the primitives row holds
stack-and-technologies#1.10 · no · sha2 and the content hash path are not reached
stack-and-technologies#1.11 · no · the new cause word has no serde path and RefusalReason gains nothing; serde and serde_json use is unchanged
stack-and-technologies#1.12 · no · the new WARN lines use the existing obs line format; tracing dependencies and levels, owned by the obs plan, do not move
stack-and-technologies#1.13 · no · the nutype newtypes ViolaName, Percent and DialogHold are not changed
stack-and-technologies#1.14 · no · chrono use is untouched
stack-and-technologies#1.15 · no · thiserror per-crate derives keep their count; Refused gains a variant, not a derive, and the row names no count
stack-and-technologies#1.16 · no · per-wrapper local sockets with JSON-RPC over ndjson stand; no broker is added
stack-and-technologies#1.17 · no · sysinfo and liveness rules are unchanged; a peer's euid decides whom to talk to, never liveness
stack-and-technologies#1.18 · no · rmcp features and the tool list send, wait, last, answer, list are unchanged; frame gains only the unreachable result path
stack-and-technologies#1.19 · no · no model API is touched
stack-and-technologies#1.20 · no · no phone view work in the session
stack-and-technologies#1.21 · no · cargo install and the include_str! plugin written by viola run stand; the new check adds no deployment step
stack-and-technologies#1.22 · no · no CI workflow step or action pin changed; the nextest ci profile term is a test config edit, not a CI step
stack-and-technologies#1.23 · no · lint and quality tools and their gates are unchanged; no new script or tool version
stack-and-technologies#1.24 · no · release tooling is not reached
stack-and-technologies#2.1 · yes · stale · the row names the identity each frame-writing client checks before its first frame as the pid from peer_creds; the session adds a peer euid check on Linux and macOS on the same client path (F3, F4), so that reading has moved
