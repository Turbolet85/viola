# Route archive — viola-0.1.0

<!-- Writer: /andromeda-wrap-session P7 flip-compaction only. Read by NO loop skill. Cold history: each
working-route line whose master record flipped `complete`, copied VERBATIM before its spent annotation
freight was stripped. Superseded at take-up by the chunk's scope.md fold and at flip by the master desc —
never cite this file for current truth. NOT a complete pin chronicle: tail annotations cleared before
their entry froze live only in git + the wrap P5 summary. -->

## 2026-09-24-diagnostics-plane — archived at the 2026-09-24 wrap
[2026-09-24-diagnostics-plane] Diagnostics plane — per-role JSON sinks with process-start service identity, closed event vocabulary and line schemas, owner-only per-instance detail files, diagnostics_level, torn-line-aware logs merge  CARRY: chunk 2026-09-24-three-os-ci-headless-harness-skeleton shipped the first role lines with raw `tracing::info!`/`error!` in `src/run/mod.rs` (event/process/instance passed by hand) — migrate them to `obs_event!`; its panic hook writes the payload nowhere (no `detail-run.ndjson` yet) and `process-exit{subject:"self"}` has no `duration_ms`; the harness `logs` streams only the home-level diag source (events + detail sources and `--kind`/`--after` arrive with their producers)  PREREQ: close rust gate deferral (deferred since 2026-09-24-supply-chain-and-workflow-gates)

## 2026-09-24-log-redaction-and-never-log-floor — archived at the 2026-09-24 wrap
[2026-09-24-log-redaction-and-never-log-floor] Log redaction and never-log floor — skip-all span fields, redacted payload types, fixed error displays, content-bearing records only in instance detail files  CARRY: chunk 2026-09-24-diagnostics-plane shipped the detail sink. `viola::obs::write_detail` / `detail_line` write owner-only `instances/<name>/diagnostics/detail-<process>.ndjson`, and `schemas/diag-detail.v1.json` already admits `chain` and `drift_report`. Anyhow chains and serde drift reports route through that sink; today only the panic hook uses it.

## 2026-09-24-observability-gates — archived at the 2026-09-24 wrap
[2026-09-24-observability-gates] Observability gates — panic hook first, zero-panic, schema, bare-instrument and abort-panic gates, canary secret scan before any upload, print and raw-log lint bans  CARRY: the fake agent is a `[[bin]]` of the root package (lints are per package) — when the print bans land, exempt it with a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs` (obs-plan §3 obs-ci-gate-wire as amended 2026-09-24); the obs G3 gate is written with `rg`, which is absent from the local gate shell's PATH (chunk 1 ran it as `grep -rEn`)
