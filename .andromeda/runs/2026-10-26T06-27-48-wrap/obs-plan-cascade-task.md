# Task — obs-plan · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The session
The session moved the locked tokio from =1.53.1 to =1.53.3: the workspace pin in Cargo.toml and the tokio entry's version and checksum in Cargo.lock. That is a 4-line lock delta, and no other crate moved.
It retired the open advisory RUSTSEC-2026-0386 against the locked tokio. cargo deny check (all four families, cargo-deny 0.20.2) exits 0 and no ignore was widened.
CI job supply-chain, red since the 2026-10-20 wrap, read green on ci#40278510123 (sha 948f7f01, checks 15/15). Its steps cargo deny, fuzz lockfile audit, tokio ban per sync crate, ban probes and zizmor each succeeded.
No source, crate manifest, deny.toml, deny-sync.toml, lint, toolchain, workflow, schema, symbol, event, field or gate changed. No crate gained or lost tokio, and the four AsyncFd constructors tokio 1.53.3 deprecates reach no viola call site.

## Reached files
- Cargo.toml
- Cargo.lock

## Touches
- §1 telemetry trigger, security plan Vector 9 (supply chain): obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny. The claim was re-read and is unmoved. cargo deny now exits 0, the four sole-root deny-sync.toml bans exit 0 and channel-tokio-free reads ok.
- §1 scope entities `viola-channel` (Tokio client behind the `tokio` feature, otherwise sync) and `viola-state` (Stack bans Tokio): containment was re-measured by the session and is unmoved.
- §1 scope surface and entity `viola-mcp` (Tokio allowed, rmcp 3.4.1 stdio server): the tokio version under it moved to 1.53.3. The obs claim names no tokio version, and rmcp-features still reads server transport-io.
- §1 harness OTel SDK init and §11 Telemetry Strategy ban: the cargo deny ban on Tokio in sync crates and C-building crates still holds after the bump.
- §7 Error Capture, mcp per-role panic behaviour (a panic inside a spawned Tokio task is contained at the task boundary): it rests on tokio's runtime, which moved a patch version (1.53.1 to 1.53.3, same dependency set). The claim is unmoved.
- §10 build failure condition "a cargo deny (Tokio, C-crate, veil toggle, banned obs crates) failure": the condition is unchanged, and the gate that was red over this window now reads green.
- §10 build failure condition and §9 G3 (no `panic` set to abort in Cargo.toml): Cargo.toml was edited, but only at line 242 (the tokio pin). The profile `panic = "unwind"` lines did not move.
- §1 MSRV floor 1.96 (D-22) in the Section 1 currency note: tokio 1.53.3 declares rust-version 1.71, and the 1.96 `cargo check --workspace` is green. The floor is unmoved.
- Version pins of the obs dependencies held in Cargo.toml's `[workspace.dependencies]` (tracing-subscriber 0.3.23, rmcp 3.4.1, portable-pty =0.8.1, thiserror 2.0.20, anyhow 1.0.104, veil 0.3.0): the file was edited, but none of these pins moved.
- §9 CI Integration, `nightly.yml` weekly `cargo deny check advisories` over the root and fuzz lockfiles: the root lockfile it audits changed (tokio 1.53.3), and the advisory it would flag is closed. The step itself is unchanged.
- §11 Universal ban on proprietary APM, and the §7 sentry inadmissibility (its default reqwest + native-tls + tokio transport): the session added no dependency, so both are unmoved.
- Dev-tool versions named in obs-plan's CI text (zizmor, upload-artifact v7.0.1 SHA-pinned): cargo-deny 0.20.2 and zizmor 1.30.1 were re-read at their CI pins, unchanged.

## New facts
- (none)

## Untouched
- §4 Span / Trace Coverage scenarios, span names and edge flows (E1, E2, E3, E5, torn append, ledger, verify / plugin install): no span, instrument site or code path changed.
- §6 Log Coverage required fields, additive field catalog, detail code catalog, log levels and per-module Targets: no event, field, target or level changed.
- §3 Observability Harness Contract registry keys (logger stack, null encoding, diagnostics root, heartbeat): no logger, sink or contract changed.
- §8 PII Scrubbing data classes, veil Redact layer, secret scan and `schemas/diag-line.v1.json` / `schemas/diag-detail.v1.json`: no schema or scrubbing layer changed.
- §5 Metric Coverage derived metrics, `viola.compat.v_rejected` and cardinality discipline: no metric or grouping key changed.
- §10 performance budgets (hyperfine max, spine deadline, heartbeat flip at >5 s) and the error budget's three zero counts: no hook or gate changed.
- §9 gates G1, G2 and G4 and the CI step order (secret scan, diag / harness / junit uploads, retention-days 7): no workflow or gate script changed.
- §2 Telemetry Strategy signal pyramid, naming conventions and resource attributes: untouched by a lockfile bump.
- §12 Obs Decisions Log and the amendments history: the session made no obs decision.
