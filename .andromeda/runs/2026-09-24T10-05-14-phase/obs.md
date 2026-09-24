# obs extract

## Relevance
Relevant. This chunk builds the obs-plan §3 bootstrap phases `logger-stack-install`, `service-identity-wire` and `log-format-schema-emit`. It also builds part of `pii-scrubbing-wire`: the detail-file routing, limited to the panic payload.

## Constraints
- **Emission goes only through `obs_event!`** (per obs-plan §3 Logging stack; §3 Bootstrap phases → logger-stack-install).
  - The macro lives in `viola_core::obs` as a `macro_rules!` that expands to `::tracing::event!` at the caller. viola-core gets no `tracing` dependency.
  - It always attaches `event` (the kebab `Display` of `ObsEvent`), `corr`, `process` and `instance`. `process` and `instance` come from a process-global `OnceLock<ProcessCtx>`.
  - `message` is a static literal equal to the event name.
  - The inner `::tracing::event!` statement carries `#[allow(clippy::disallowed_macros)]`.
  - `ObsEvent` holds exactly the tests set plus D-01…D-05. `ObsProcess` adds `cli` (D-06). `a11y-violation` gets no variant (§3 Obs extensions).
- **`viola_obs_init` builder settings** (per obs-plan §3 OTel SDK init → Init body sketch; §3 Logging stack → Mandatory builder settings; §6 Per-module log levels).
  - Required: `fmt().json().flatten_event(true).with_current_span(false).with_span_list(false).with_ansi(false).log_internal_errors(false)`, an explicit writer (never stdout, `std::io::sink` if the open fails) and the `MillisUtc` timer. `MillisUtc` must produce RFC 3339 UTC with milliseconds and `Z`, via `chrono` `to_rfc3339_opts(SecondsFormat::Millis, true)`.
  - Levels are set only by a `filter::Targets` built in code. viola crates are at INFO, or DEBUG through `diagnostics_level`. Third-party targets and everything else are OFF.
  - tracing-subscriber features are limited to `fmt,json,registry,std`. `ansi`, `tracing-log` and `chrono` are off (D-11, D-26). The root bin takes a direct `chrono` dependency.
- **Init order** (per obs-plan §3 OTel SDK init → Init order steps 1–6; §7 Panic hooks).
  1. `set_hook(viola_panic_hook)` is the first statement in `main`.
  2. clap parse.
  3. Resolve home, instance and role. For `run`, the instance is its own `ViolaName` argument and an inherited `VIOLA_NAME` never overrides it.
  4. Check the home's strict modes first, then create `diagnostics/`. Unix: 0700 dir, 0600 files, `O_NOFOLLOW`, owner and mode checked with lstat. Windows: owner and DACL.
  5. Parse `config.json` once. `diagnostics_level` falls back to `"info"`, and the parse outcome is held in memory until the subscriber exists.
  6. Emit `process-start`, then `parse-rejected{parser:"config-json", detail, count}` if the parse failed or skipped keys (D-28).
  - For `run`, all of this must finish before `run.collision_check`.
  - Whether the code already runs a home strict-modes check is a question for research.
- **Service identity** (per obs-plan §3 Service identity).
  - `viola_core::SERVICE_NAME = "viola"` and `VERSION = env!("CARGO_PKG_VERSION")` are the only source.
  - `process-start` carries `service_name`, `version`, `os` (from `std::env::consts::OS`) and `pid`.
  - Nothing is read from `OTEL_*` or other env vars.
- **Sinks, one write per line** (per obs-plan §3 Logging stack → Sink; §3 Log file location).
  - One append-mode file per role: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`.
  - Each line is one `write_all` of a complete line. Home-level lines are codes-only.
  - No stdout sink on any role. `mcp` writes stderr JSON only when its file cannot be opened (D-09). No rotation.
- **Detail files** (per obs-plan §3 Logging stack → Sink (D-08, D-30); §3 Init order step 4; §7 Panic hooks).
  - Path: `<home>/instances/<name>/diagnostics/detail-<process>.ndjson`, 0600 files in 0700 dirs, opened lazily.
  - Lines are hand-formatted with the home-level keys: `timestamp`, `level`, `target`, `message`, `event`, `process`, `instance`, plus `corr`/`conn`. They never pass through the subscriber.
  - A failed detail open drops only the detail line and writes nothing to stderr.
  - The panic hook's home-level line holds exactly `timestamp`, `level:"ERROR"`, `target:"viola::panic"`, `message:"panic"`, `event:"panic"`, `process`, `instance` (omitted when not resolved), `panic_location` and `thread`. It never writes `corr`.
  - The detail companion adds `panic_payload` and a `backtrace` array captured with `Backtrace::force_capture()`, which is not gated by `RUST_BACKTRACE`.
  - No `obs_event!` call ever carries `chain`, `drift_report`, `panic_payload` or `backtrace`.
- **Null encoding and schema** (per obs-plan §3 Obs extensions → Null encoding (D-12, D-26); §8 Default-deny posture; §6 `detail` code catalog (D-31)).
  - A null `corr`/`instance` is written as key absence.
  - `schemas/diag-line.v1.json` declares `corr` (number | string) and `instance` (string) as optional and rejects a literal `null`. It uses `additionalProperties: false` per event, with only the §6 Additive field catalog allowed. `parse-rejected.detail` is limited to the closed list.
  - §8 also requires a sibling `schemas/diag-detail.v1.json` for detail lines, which the scope does not list. Planning has to add it or explicitly defer it.
- **CARRY: `run` role lines** (per obs-plan §6 Additive field catalog (`process-exit`: `subject`, `exit_code`, `detail`, `duration_ms`); §6 Log levels mapping).
  - `process-exit{subject:"self"}` needs `duration_ms`. Exit 1 and exit 20 are logged at `error`, exit 21 at `warn`.
  - Exit-1 `detail` codes come from the closed catalog: `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`.
  - Whether the existing raw `tracing::info!`/`error!` lines already carry these fields is a question for research.

## Patterns to follow
- **Catch sites** (obs-plan §7 Panic hooks → Main-thread catch site).
  - One `catch_unwind` around everything after `set_hook`. The role is classified from argv before clap, skipping global flags and their values.
  - `run` exits through `process-exit{subject:"self", exit_code:1, detail:"internal-error"}`.
  - The PTY pump and handle-wait worker threads each run inside `catch_unwind` and report back to `main`.
  - Research should check whether the shipped skeleton already has these catch sites.
- **Panic hook** (obs-plan §3 Init order step 1; §7). It writes through a `OnceLock<Arc<File>>` that stays empty until init step 4, so it never falls back to stderr. It never goes through the subscriber, which avoids re-entrancy.
- **Harness `logs` merge record shapes** (obs-plan §3 Snapshot / paste-to-AI integration).
  - Home-level lines: `{"src":"diag","file","record"}`.
  - Detail lines: `{"src":"diag","file","instance","record"}`. The `instance` field tells apart the same `detail-<process>.ndjson` basename across instances.
  - Torn lines: `{"torn":true,"offset":n}`.
  - Filters: `--instance` and `--process`.
  - The glob must cover `instances/*/diagnostics/detail-*.ndjson`.
- **Illustrative line** (obs-plan §3 Log format JSON schema): the reference for field order and shape (flattened `message`, `corr` as a JSON number).
- **Error-free open budget** (obs-plan §3 Init order per-role anchors; §10 Obs overhead on `hook`). The error-free path does exactly one role-file `open` plus appends. The detail file is not opened at init.

## Anti-patterns to avoid
- **Logs bans** (per obs-plan §11 Logs):
  - no `RUST_LOG`, `EnvFilter` or `env-filter` feature;
  - no default (stdout) writer and no `log_internal_errors(true)`;
  - no raw `tracing::*!` outside `viola_core::obs`;
  - no `?corr`/`%corr`, which stringifies it;
  - no multi-line stack traces;
  - no tracing-appender `non_blocking`;
  - no hand-installed `LogTracer` or `log` bridge.
- **`diagnostics_level` and redaction** (per obs-plan §11 PII Scrubbing): the level must never widen the field allow-list or disable redaction; it changes volume only. No path or pid inside `detail` codes.
- **Panic output** (per obs-plan §11 Error Reporting; §7 Panic hooks → Unwinding is required): never call or let run the default panic hook. Never set `panic = "abort"` in any profile (gate G3).

## Contract bindings
- **obs ↔ tests §3 Log format** (per obs-plan §3 Log format JSON schema, binding verbatim).
  - obs may add fields but must not rename or remove required fields.
  - Absent key = null (tests amendment D-21).
  - The `cli` process value and `cli-<name>.ndjson` come from D-21.
- **obs ↔ tests §3 `logs` command** (per obs-plan §3 Snapshot / paste-to-AI integration; D-08, D-21). obs owns the record shapes and detail-glob coverage. The harness implementation is tests-owned.
- **obs ↔ tests-owned checks** (per obs-plan §3 Bootstrap → log-format-schema-emit; §8 Default-deny posture; §3 Logging stack concurrent-append).
  - A check asserts that the `diag-line.v1.json` `event` enum equals the `ObsEvent` variants. obs owns the schemas; tests own the check bodies and the validator crate.
  - A three-OS concurrent-append check must include a detail line larger than 4 KiB (D-28).
- **obs ↔ security `logging-redaction-wire`** (per obs-plan §3 Bootstrap phases → Merged phase position / Ownership).
  - The 0600/0700 and strict-modes contract for `diagnostics/` and the detail files comes from security's NEVER-log floor and access control.
  - This chunk builds only the detail sink and the panic routing. `#[instrument(skip_all)]`, veil and fixed `Display`s stay in the redaction chunk.
- **obs ↔ arch Occupied Resources** (per obs-plan §3 Log file location, amendment request D-22). The home-level `diagnostics/` root plus the instance `detail-*` files still need to be reflected in arch.

## Acceptance criteria contributions
- (obs) Every line in `<home>/diagnostics/run-<name>.ndjson` from a `run` invocation validates against `schemas/diag-line.v1.json` (per obs-plan §8 Default-deny posture; §3 Obs extensions → Null encoding). This means:
  - `timestamp` matches `…T..:..:..\.\d{3}Z`;
  - `event` is in the closed enum, and `message` equals `event`;
  - `process` and `instance` are present;
  - no literal `null` appears for `corr`/`instance`;
  - there is no key outside the §6 catalog.

  The first line is `process-start` with `service_name:"viola"`, `version`, `os` and `pid`. `process-exit{subject:"self"}` carries `duration_ms`.
- (obs) With `diagnostics_level` absent, `"info"` or `"debug"` in `config.json`, and with `RUST_LOG=trace` set, nothing is written to stdout or stderr. `RUST_LOG` does not change the output. `"debug"` changes volume only, never the key set (per obs-plan §3 Logging stack → Agent-mode flag; §11 Logs).
- (obs) A forced panic in `run` with a resolved instance produces:
  - exactly one single-line `event:"panic"` record in the home-level role file, with no `panic_payload` or `backtrace`;
  - one companion line in `instances/<name>/diagnostics/detail-run.ndjson` with `panic_payload`, a `backtrace` array, and the same `panic_location` and `thread`;
  - nothing on stderr, and on Unix, 0600 files in 0700 dirs.

  (per obs-plan §7 Panic hooks; §10 Zero unlogged panics → Counting rule)
- (obs) `agent-run logs` emits `{"src":"diag","file","record"}` for home-level lines, `{"src":"diag","file","instance","record"}` for detail lines and `{"torn":true,"offset":n}` for a torn line. `--instance` and `--process` filter correctly (per obs-plan §3 Snapshot / paste-to-AI integration).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton — the fake agent's print-ban exemption** (§3 Bootstrap phases → obs-ci-gate-wire; §11 Logs).
  - Only `viola-e2e` omits `[lints] workspace = true`.
  - The fake agent is a root-package `[[bin]]` exempted by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`.
  - Why: lints are set per package, so a separate lint table is impossible for a root bin.
  - Relevance: this chunk adds root-bin `viola::obs` code under the same root `[lints]`. Only the fake-agent bin is exempt, so the new obs modules must pass `print_stdout`/`print_stderr`/`dbg_macro` = deny without local allows.
- **2026-09-24-supply-chain-and-workflow-gates** (§9 Platform, nightly.yml): not relevant to this chunk's area.
