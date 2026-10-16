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
- **Library:** tracing 0.1.44 in every instrumentable crate; tracing-subscriber 0.3.23 (`default-features = false`, `fmt,json,registry,std`) as a product dependency in the root bin only (`viola-channel` and `viola-state` take it as a dev-dependency for their unit tests' line capture). No OTel SDK, no exporter, no reporter (sentry banned), no `tracing-appender`, no `metrics` instruments, no `/metrics` route.
- **Builder:** `fmt().json().flatten_event(true).with_current_span(false).with_span_list(false).with_ansi(false).log_internal_errors(false)`, `MillisUtc` timer (RFC 3339 UTC ms + `Z`), explicit writer — never the stdout default.
- **Levels:** `filter::Targets` from `config.json` `diagnostics_level` (`info` | `debug`); never `RUST_LOG` / `EnvFilter` / the `env-filter` feature. Third-party targets (`rmcp`, `portable_pty`, `sysinfo`, `interprocess`) are OFF; capture their failures from returned `Result`s at viola's seam.

## Sinks
- Codes-only role files: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`. Content-bearing records (anyhow chain, drift report, panic payload + backtrace) only in `<home>/instances/<name>/diagnostics/detail-<process>.ndjson`, written by the root-bin detail writer, never through the subscriber.
- 0600 files in 0700 dirs; `OpenOptions::append`; one `write_all` per complete line (concurrent hook processes share a file). A failed log write is swallowed — `hook` still exits 0.
- Init order: panic hook first → clap → home/instance/role → home strict-modes (create first if new) → diagnostics dir/file checks and open → `config.json` → subscriber → `process-start`. `viola hook statusline` runs its instance check BEFORE this init (the role-file open sets an existing home to 0700, so a check made after it passes a widened home) and a refused check writes no line and opens no role file (exit 0 at once); its stdout carries the user's status-line output, never telemetry. `viola revive`'s start arm runs its instance check before the log init as well: a refused check is the stderr pair and exit 1 with no log opened.

## Lines
- Every line goes through `obs_event!` (defined in `viola_core::obs`): it attaches `event`, `process`, `instance` (from `ProcessCtx`); the caller passes `corr` as a typed field on the events that carry one; `message` is a static literal equal to the event name. Raw `tracing::{event,info,warn,error,debug,trace}!` is banned in two halves. Clippy `disallowed-macros` (`clippy.toml`) bans the level macros by path. Raw `event!` is caught by the fail-closed grep in `scripts/lint-probes.sh`, because on clippy 1.98.1 no allow inside `obs_event!` can exempt its inner `::tracing::event!` (obs-plan §3 logger-stack-install, D-33).
- `event` is a closed kebab-case enum (`ObsEvent`): a new value needs an obs Decisions Log entry and a `schemas/diag-line.v1.json` update.
- `corr` per event (JSON-RPC `id` · `dialog_id` · send `cursor`), copied as a number or string — never `?corr` / `%corr`, never renamed. A null `corr` / `instance` is key absence, never a literal `null`.
- Channel lines carry `conn = "<process>-<pid>-<t0>-<n>"`; wrapper `send-*` lines carry `conn` + `rpc_id`; `from` is logged with `from_trust:"self-reported"`.
- Log only fields named in obs-plan §6 (default-deny, enforced by G4). `detail` codes are closed kebab-case lists — never a path or pid.

## Spans
- `#[instrument(skip_all, name = "<area>.<operation>", fields(..))]` only — never bare `#[instrument]` (gate G1), never `err` / `ret` on anything that can carry a serde source or user content. Span names are static; no instance names, ids or paths.
- Spans are not serialized: pass `corr`, `conn`, `instance` as explicit values into threads/tasks.

## Panics and errors
- `panic = "unwind"` in every profile (gate G3). The panic hook writes one line to the role file (no stderr, never the default hook); `catch_unwind` at the main-thread catch site, around `hook` dispatch, the `run` pump and handle-wait threads, and the vt100 feed.
- `hook` → exit 0, empty stdout; `cli` → `process-exit{detail:"internal-error"}` when an instance resolves, then exit 1 with exactly `error: internal error` (no hint); `run` → `process-exit{detail:"internal-error"}`; `mcp` main-thread → `process-exit{detail:"internal-error"}`, then exit 1.
- The detail-line `backtrace` is raw, never-symbolised frames (`src/panic_frames.rs`: `0x<ip> <module path> base=0x<base> +0x<offset>`, up to 62) — symbolising cost 351 of a 403 ms hook run on the Windows runner; never call `Backtrace::force_capture().to_string()` in a panic path.
- The hidden capture arms, `hook <event> --capture` and `hook statusline --capture` (the latter runs no instance check, no shell-out and no `budget.json` write, opens no role file and prints nothing), do no obs init and write no line (a bounded panic-line exemption, obs-plan §10). A `viola verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) starts no further spawn, kills and reaps the child it is running, closes every spawn pair it opened (a child of an interactive PTY run killed at the stop closes with `duration_ms` and a `child_exit_status`, the PTY pump's exit code; a print-mode or `--version` probe child killed at the stop is not measured) and ends `process-exit{subject:"self", exit_code:1, detail:"interrupted"}` (a twelfth `detail` value in `schemas/diag-line.v1.json`); `run`'s version gate observes no stop.
- `viola list` (span `cli.list`) runs its strict-modes scan before its log; a refused scan (exit 21) opens no log and writes no line (obs-plan §10 exemption 7; the founder's own answer, 2026-10-13, superseding the operator's). Under a `VIOLA_NAME` it writes the self pair, one `parse-rejected{parser:"state-entry"}` per nonzero scan count, one `state-recovered` per replayed snapshot and the `config-json` threshold rejections into `cli-<name>.ndjson`; with no name it opens no log.
- `viola revive`'s start arm logs as process `run` once its instance check has passed (`run-<name>.ndjson`, each preflight refusal — `already-live`, `no-session`, `cwd-missing` — one `process-exit{exit_code:1, detail}`); `viola revive --list` opens no process log (the sixth bounded exemption, standing only until it gains `--json` on the route entry "CLI machine contract"). A refused instance check on the start arm or on `viola hook statusline` opens no role file and writes no line (the seventh exemption; on `viola hook statusline` the founder's own answer, 2026-10-13; on the start arm the operator's answer, the founder not having ruled).
- `run.budget_eval` (INTERNAL; `trigger` `start|send|turn-ended`, `paused`, `window` only when paused) is declared once and opened once per evaluation; at start it sits between `channel.bind` and `state.snapshot_write`. `read_budget` and `release --budget`'s lift carry no span and no line of their own; `budget-gate` is appended only when (paused, window, override end) changes. The implicit link record on a `send` or an `answer`, the link set's restore at start and the replay's link rule carry no span and no process-log line either; `driver`, `driven` and `since` reach no home-level line. A `budget-paused` refusal's `send-refused` line carries `window`, `used_percentage`, `resets_at` and `paused:true`.
- No per-byte or per-line logging in hot loops (PTY pump); no line per heartbeat tick — transitions only.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-09-24: `obs_event!` expands to `::tracing::event!`, so a raw-log census greps `tracing::(info|warn|error|debug|trace)!` — a bare `tracing::` grep reads the sanctioned macro in `viola_core::obs` as a raw call.
