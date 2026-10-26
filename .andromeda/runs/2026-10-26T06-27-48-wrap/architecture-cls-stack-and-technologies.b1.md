# Classified — architecture · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · no · the MSRV floor 1.96 and the 1.98.1 toolchain pin read unmoved on the changed Cargo.toml; tokio 1.53.3's rust-version 1.71 sits under the floor
stack-and-technologies#1.2 · yes · stale · the tokio pin and lock moved 1.53.1 to 1.53.3; F1 (AsyncFd deprecation) and F2 (the RUSTSEC-2026-0386 patched floor) join this row
stack-and-technologies#1.3 · no · clap 4.6.7 and its feature set re-read unmoved; only the tokio pin line and its lock entry moved
stack-and-technologies#1.4 · no · portable-pty =0.8.1 re-read unmoved and the lock delta is tokio alone, so windows-sys 0.61.2 and libc =0.2.189 stand
stack-and-technologies#1.5 · no · vt100 0.16.2 is a locked crate the tokio-only lock delta leaves unmoved
stack-and-technologies#1.6 · no · interprocess 2.4.4 re-read unmoved; the row names no tokio version and the lock delta is tokio alone
stack-and-technologies#1.7 · no · no database exists and no store, snapshot or record changed
stack-and-technologies#1.8 · no · N/A, and no record format, reader or snapshot rule changed
stack-and-technologies#1.9 · no · tempfile 3.27.0 re-read unmoved and the lock delta is tokio alone
stack-and-technologies#1.10 · no · sha2 =0.11.0 and its profile line stand; the lock delta is tokio alone
stack-and-technologies#1.11 · no · serde_json 1.0.151 re-read unmoved; serde and serde_path_to_error stand in the tokio-only lock delta
stack-and-technologies#1.12 · no · tracing, tracing-subscriber and veil =0.3.0 stand; the lock delta is tokio alone
stack-and-technologies#1.13 · no · nutype 0.8.0 re-read unmoved
stack-and-technologies#1.14 · no · chrono 0.4.45 re-read unmoved
stack-and-technologies#1.15 · no · thiserror 2.0.20 and anyhow 1.0.104 re-read unmoved
stack-and-technologies#1.16 · no · no broker exists and no IPC method or frame changed
stack-and-technologies#1.17 · no · sysinfo 0.39.6 is a locked crate the tokio-only lock delta leaves unmoved; no liveness code changed
stack-and-technologies#1.18 · no · rmcp 3.4.1, schemars 1.2.2 and the server + transport-io feature set re-read unmoved
stack-and-technologies#1.19 · no · N/A, and viola still calls no model API
stack-and-technologies#1.20 · no · N/A, and the phone view is a later version with nothing changed here
stack-and-technologies#1.21 · no · cargo install and the include_str! plugin are unchanged; no workflow, release check or gate moved
stack-and-technologies#1.22 · no · no workflow changed; the matrix, SHA-pinned actions and pre-push gate the row names are unmoved
stack-and-technologies#1.23 · no · cargo-deny 0.20.2, clippy, rustfmt and the deny-sync.toml tokio ban re-read unmoved; the four sole-root bans exit 0 and deny.toml is untouched
stack-and-technologies#1.24 · no · release tooling (dist 0.33.0, cargo-auditable 0.7.6, self_update 1.3.0) is outside this change and unmoved
