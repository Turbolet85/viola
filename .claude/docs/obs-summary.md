# Observability Summary — viola

_Distilled from `.andromeda/obs-plan.md` by `/andromeda-setup-project`. wrap-session does not modify._

## Obs tier

**Tier:** Standard (1), with Minimal-tier exporter carve-outs

**Justification (one sentence):** four process roles, seven instrumentable crates and seven must-trace paths make it Standard-class, but upstream bans egress, new listeners, stdout on three roles and Tokio/C crates in the sync graph — so there is no exporter and no reporter, only local JSON files.

## Harness contract (§3)

The obs harness is **bound** to the test harness: tests owns the grepped fields and the `logs` wrapper; obs may add fields, never rename or remove them. See `.claude/rules/observability.md` and `.claude/rules/verification-harness.md`.

- **Logger / library:** tracing 0.1.44 facade + tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)` (+ `with_span_list(false)`, `with_ansi(false)`, `log_internal_errors(false)`, `MillisUtc`, explicit writer)
- **OTel SDK:** none in v1 (opentelemetry 0.33 / tracing-opentelemetry 0.34 deferred; any future Resource uses `builder_empty()`)
- **Log sink path:** codes-only role files `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`; content-bearing detail `<home>/instances/<name>/diagnostics/detail-<process>.ndjson`; 0600 in 0700
- **Log format:** JSON Lines — required `timestamp` (RFC 3339 ms Z), `level`, `target`, `message` (= event name), `event` (closed kebab enum), `corr`, `process` (`run|hook|mcp|ui|cli`), `instance`; null = key absence
- **Heartbeat:** the `heartbeat` file touched every 1 s (not logged per tick); `liveness-changed` on live↔stale↔gone transitions (flip at > 5 s); SSE keep-alive every 15 s
- **Integrity check:** schema conformance gate G4 against `schemas/diag-line.v1.json` / `diag-detail.v1.json`
- **Status endpoint:** `agent-run status` (tests-owned) over `viola list --json` + `/ready` + `/api/sessions`
- **Consumption:** `scripts/agent-run.* logs` merges `events.ndjson`, role files and detail files, filterable, torn lines marked

## SLO invariants (§10)

| Metric | SLO | Source |
|---|---|---|
| Unlogged panics | 0 — every panic after init step 4 = exactly one `event:"panic"` line; `hook` still exits 0 | G2 over home-level role files |
| Secret-scan hits | 0 (token, `Cookie`, `?t=`, `CLAUDE*`, canary in home-level files) | `secret-scan` step before any upload |
| Schema-conformance failures | 0 | G4 |
| Hook latency | hyperfine `max` < 1.0 s (SessionEnd; spine provisional) — logger init = 1 open + appends | perf job, with `VIOLA_NAME` set |
| Heartbeat flip | 4.9 s live / 5.1 s stale-or-gone | mock_instant unit test |

## Service identity tagging

- `service.name="viola"` (`viola_core::SERVICE_NAME`, compile-time) → `service_name` on `process-start`
- `service.version=env!("CARGO_PKG_VERSION")` (`viola_core::VERSION`) — same value as `sender`, `writer`, `/health.version`
- `deployment.environment` N/A (local-only); `os` + `pid` on `process-start`; never read from env vars

## Event vocabulary (closed)

`channel-request` · `channel-response` (corr = JSON-RPC id) · `dialog-raised` · `dialog-answered` (corr = dialog_id) · `hook-invoked` · `hook-decision` · `send-issued` · `send-confirmed` · `send-refused` (corr = send cursor; wrapper lines add `conn` + `rpc_id`) · `release-from-driver` · `process-start` · `process-exit` · `http-request` · `panic` · `liveness-changed` · `state-recovered` · `sse-opened` · `sse-closed` · `parse-rejected`. `a11y-violation` is a harness-only row, not in this enum.

## PII scrubbing wire

- `#[instrument(skip_all, fields(..))]` everywhere (gate G1); no `err` / `ret`
- Default-deny: only fields in the obs-plan §6 catalog; veil `#[derive(Redact)]` on payload types (`toggle` banned)
- HTTP spans record `uri.path()` only; no headers
- Content-bearing records only in instance detail files; boundary-only logging — never in the PTY pump or notify tail loops
- Merged phase with security's `logging-redaction-wire`

## Universal anti-patterns

- NEVER write telemetry to stdout, or use print/`dbg!` macros in product crates (clippy `print_stdout`/`print_stderr`/`dbg_macro` = deny).
- NEVER call raw `tracing::*!` outside `viola_core::obs` — only `obs_event!`.
- NEVER read the level from `RUST_LOG` / `EnvFilter`; `diagnostics_level` changes volume, never redaction.
- NEVER use tracing-appender `non_blocking`, sentry, an OTLP exporter or a `/metrics` route.
- NEVER log per heartbeat tick or SSE keep-alive; NEVER put a path or pid inside a `detail` code.

## Critical decisions

- D-08: home-level role files are codes-only; content goes only to per-instance detail files (reconciles tests' paths with security's "only instance diagnostics hold content").
- D-10/D-32: additive `conn = "<process>-<pid>-<t0>-<n>"`; the channel join key is `(conn, corr)`; wrapper send lines carry `rpc_id`.
- D-11/D-23: third-party log targets OFF, `tracing-log` off; their failures are captured from returned `Result`s at viola's seam.
- D-12: no `trace_id`/`span_id`; null encoded as key absence.
- D-20: closed exit-cause `detail` codes (exit 1 `already-live|squatted-name|pinned-hash-mismatch|batch-script-child|internal-error`; exit 21 `instance-dead|strict-modes-failed|server-verify-failed`).
- D-22: arch amendment — diagnostics roots + workspace `rust-version` 1.96.

---

**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.claude/rules/observability.md`.
