---
paths:
  - "src/**/*.rs"
  - "crates/viola-*/src/**/*.rs"
  - "schemas/**"
  - "clippy.toml"
---

# Observability Rules

Path-scoped rules for product code that logs, spans or handles errors. Authoritative source: `.andromeda/obs-plan.md` (tier Standard with Minimal exporter carve-outs); the log format itself is bound to `.andromeda/test-plan.md` §3.

## Pipeline
- **Library:** tracing 0.1.44 in every instrumentable crate; tracing-subscriber 0.3.23 (`default-features = false`, `fmt,json,registry,std`) in the root bin only. No OTel SDK, no exporter, no reporter (sentry banned), no `tracing-appender`, no `metrics` instruments, no `/metrics` route.
- **Builder:** `fmt().json().flatten_event(true).with_current_span(false).with_span_list(false).with_ansi(false).log_internal_errors(false)`, `MillisUtc` timer (RFC 3339 UTC ms + `Z`), explicit writer — never the stdout default.
- **Levels:** `filter::Targets` from `config.json` `diagnostics_level` (`info` | `debug`); never `RUST_LOG` / `EnvFilter` / the `env-filter` feature. Third-party targets (`rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `notify`, `sysinfo`, `interprocess`) are OFF; capture their failures from returned `Result`s at viola's seam.

## Sinks
- Codes-only role files: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`. Content-bearing records (anyhow chain, drift report, panic payload + backtrace) only in `<home>/instances/<name>/diagnostics/detail-<process>.ndjson`, written by the root-bin detail writer, never through the subscriber.
- 0600 files in 0700 dirs; `OpenOptions::append`; one `write_all` per complete line (concurrent hook processes share a file). A failed log write is swallowed — `hook` still exits 0.
- Init order: panic hook first → clap → home/instance/role → home strict-modes (create first if new) → diagnostics dir/file checks and open → `config.json` → subscriber → `process-start`.

## Lines
- Every line goes through `obs_event!` (defined in `viola_core::obs`): it attaches `event`, `process`, `instance` (from `ProcessCtx`); the caller passes `corr` as a typed field on the events that carry one; `message` is a static literal equal to the event name. Raw `tracing::{event,info,warn,error,debug,trace}!` is banned by clippy `disallowed-macros`.
- `event` is a closed kebab-case enum (`ObsEvent`): a new value needs an obs Decisions Log entry and a `schemas/diag-line.v1.json` update. `a11y-violation` is NOT a product event.
- `corr` per event (JSON-RPC `id` · `dialog_id` · send `cursor`), copied as a number or string — never `?corr` / `%corr`, never renamed. A null `corr` / `instance` is key absence, never a literal `null`.
- Channel lines carry `conn = "<process>-<pid>-<t0>-<n>"`; wrapper `send-*` lines carry `conn` + `rpc_id`; `from` is logged with `from_trust:"self-reported"`.
- Log only fields named in obs-plan §6 (default-deny, enforced by G4). `detail` codes are closed kebab-case lists — never a path or pid.

## Spans
- `#[instrument(skip_all, name = "<area>.<operation>", fields(..))]` only — never bare `#[instrument]` (gate G1), never `err` / `ret` on anything that can carry a serde source or user content. Span names are static; no instance names, ids or paths.
- Spans are not serialized: pass `corr`, `conn`, `instance` as explicit values into threads/tasks. The SSE stream span needs a `Drop` guard emitting `sse-closed`.
- tower-http `TraceLayer` with a custom `make_span_with` recording `uri.path()` only — never `DefaultMakeSpan`, never `.include_headers(true)`.

## Panics and errors
- `panic = "unwind"` in every profile (gate G3). The panic hook writes one line to the role file (no stderr, never the default hook); `catch_unwind` at the main-thread catch site, around `hook` dispatch, the `run` pump and handle-wait threads, and the vt100 feed.
- `hook` → exit 0, empty stdout; `cli` → exit 1 `error: internal error`; `run` → `process-exit{detail:"internal-error"}`; `mcp`/`ui` main-thread → exit 1.
- No per-byte or per-line logging in hot loops (PTY pump, notify tail); no line per heartbeat tick or SSE keep-alive — transitions only.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-09-24: `obs_event!` expands to `::tracing::event!`, so a raw-log census greps `tracing::(info|warn|error|debug|trace)!` — a bare `tracing::` grep reads the sanctioned macro in `viola_core::obs` as a raw call.
