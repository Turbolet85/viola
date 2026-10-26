# Task — architecture · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The session
The workspace's exact tokio pin moved from =1.53.1 to =1.53.3 (default-features = false kept), and the lock's tokio entry moved with it: version and checksum, a 4-line lock delta, tokio alone.
It retired the open advisory RUSTSEC-2026-0386 against the locked tokio: cargo deny check (all four families, cargo-deny 0.20.2) exits 0 with no ignore widened, deny.toml untouched.
CI job supply-chain, red on every run since the 2026-10-20 wrap, read green on ci#40278510123 (checks 15/15), with the fuzz lockfile audit, the sole-root tokio bans, the ban probes and zizmor each success.
No source, crate manifest, crate, module, symbol, IPC method, env var, schema, config, toolchain, lint or workflow changed; tokio-macros 2.7.2, tokio-stream 0.1.19, tokio-util 0.7.20 and every other locked crate are unmoved.
Measured: tokio 1.53.3 declares rust-version 1.71 and the same dependency set as 1.53.1; the MSRV-1.96 cargo check, fmt, clippy (host and x86_64-pc-windows-msvc) and pre-push read green.
Measured: tokio 1.53.3 deprecates AsyncFd::{new, with_interest, try_new, try_with_interest}, and viola calls no AsyncFd constructor (clippy -D warnings shows no deprecated line).

## Reached files
- Cargo.toml
- Cargo.lock

## Touches
- The Stack and Technologies detail's Backend framework row: "Tokio 1.53.1 built only inside `mcp`" now states a stale version; the locked and pinned tokio is 1.53.3.
- The body's Inherited Defaults Framework line: "Tokio 1.53.1 only in `mcp`" now states a stale version; it is 1.53.3.
- Any other standing statement of the pinned or locked tokio version (the `[workspace.dependencies]` exact pin, the lock resolution) in any architecture group: 1.53.1 becomes 1.53.3, `default-features = false` kept.
- The stack group's other dependency versions (rmcp 3.4.1, interprocess 2.4.4, clap 4.6.7, tempfile 3.27.0, serde_json 1.0.151, chrono 0.4.45, thiserror 2.0.20, anyhow 1.0.104, nutype 0.8.0, portable-pty =0.8.1) and any tokio-macros, tokio-stream or tokio-util version: re-read, unmoved; only tokio moved.
- The Concurrency / Backend Framework keystone (Tokio only in `mcp`, the sole-root cargo deny tokio ban, the sync-crate check): re-read and unmoved; the four sole-root bans over scripts/sync-crates.txt exit 0 after the bump.
- The Module Boundaries decision's claim that among the product crates only `viola-mcp` enables tokio: re-read and unmoved (tokio stays built only in `viola-mcp` (rt), `viola-channel`'s optional `tokio` feature (io-util, dev rt) and the test-only `viola-e2e` (rt + process)).
- The Cross-cutting Tokio containment pattern: re-read and unmoved; no new code path builds a runtime.
- The Occupied Resources repository groups naming scripts/sync-crates.txt (four crates) and scripts/channel-tokio-free.sh: re-read, unmoved; channel-tokio-free ends ok and deny-probes ends 13/13 banned, control clean.
- The Occupied Resources filesystem group's `target/supply-chain/` artifacts (deny.json, deny-fuzz.json, zizmor.json): paths unmoved; the job that writes them now reads green.
- Any standing architecture text stating that CI supply-chain is red, or that an advisory is open against the locked tokio: that fact is now closed (ci#40278510123 green; RUSTSEC-2026-0386 absent from cargo deny output).
- Any standing mention of a route entry titled «tokio RUSTSEC-2026-0386 lock bump» as an owner of open work: that entry is this chunk, now done.
- The MSRV floor 1.96 and the Rust 1.98.1 toolchain pin: re-read, unmoved; tokio 1.53.3's rust-version 1.71 sits under the floor and the floor's cargo check is green.
- The Crate dependency direction key file (infra-crate-dependency-direction): re-read, unmoved; tokio's own dependency set is identical across the bump and no crate edge moved.
- The rmcp feature set (exactly `server` + `transport-io` in the release graph): re-read, unmoved; rmcp-features reads `server transport-io`.
- The Deployment / Distribution and CI decisions (three-OS GitHub Actions, the release build carrying `viola` only): re-read, unmoved; no workflow, release check or gate changed.

## New facts
- F1 · stack-and-technologies · tokio 1.53.3 deprecates the four AsyncFd constructors (`AsyncFd::new`, `with_interest`, `try_new`, `try_with_interest`); viola calls none, so a future AsyncFd use in `viola-mcp` or `viola-channel`'s Tokio client would trip clippy -D warnings.
- F2 · stack-and-technologies · The tokio pin =1.53.3 is held at or above RUSTSEC-2026-0386's patched floor (patched >= 1.51.6, < 1.52.0 and >= 1.53.3): a pin below 1.53.3 on the 1.53 line re-opens the advisory.

## Untouched
- Conventions (refusal reasons, exit codes, MCP result shapes): no refusal, code or result changed.
- Standard Contracts (the list payload, snapshot envelope, ledger stamps, instance snapshot, channel methods, event data, hook statusline, session liveness): no wire format or method changed.
- The capability ledger and the CLI version compatibility decision (the twenty-four rows, the relied set, the stamped CLI versions): no ledger row, probe or stamp moved.
- Occupied Resources IPC endpoints, binaries, Claude integration and env-var groups: no endpoint, binary, plugin file or env var added or changed.
- The Occupied Resources home layout (instances/, budget.json, ledger/stamps.json, diagnostics/, schemas/): no on-disk path or mode changed.
- The PTY, screen model, delivery confirmation, hook transport, hook contract, wheel, links and budget governor decisions: no runtime behaviour changed.
- The State Store and Snapshot writer decisions (ndjson logs, replace_private, the deny.toml licence allow list rationale): the allow list and the licence policy are unchanged.
- The Config management, Identity by instance, Fail open, Mixed-version tolerance and Diagnostic output channels patterns: no code path changed.
- Project Intent, Design Philosophy and Existing Scopes: no product scope moved.
