# Codebase Research — 2026-09-24-diagnostics-plane

## Scope
- **Depth:** moderate · **Reads:** 11 (`src/main.rs`, `src/run/mod.rs`, `src/cmd/{mod,run}.rs`, `crates/viola-core/src/lib.rs`, root + `viola-core` + `viola-e2e` manifests, `crates/viola-e2e/src/harness/logs.rs`, `crates/viola-e2e/src/bin/viola-harness.rs` 40-155, `tests/run_cli.rs`, obs-plan §6–§8) · **Globs/Greps:** 9
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 0 Session Additions · `.claude/rules/testing.md` — read in full, 3 Session Additions applied (observable-effect-per-function under zero-missed mutants; wait on the exact asserted line; no inline format captures inside `proptest!`) · `.claude/rules/observability.md` and `events.md` auto-loaded in full (0 additions)
- **Platform issues consulted:** none needed. There is no runner-only bullet, and the plan's one CI-reading entry (the operator's `check-runs` read of the pushed sha) follows a green predecessor: `4d8be52` has 9/9 check-runs `success`, read via `gh api repos/Turbolet85/viola/commits/4d8be52…/check-runs` at Setup. So there is no failure signature to search a tracker for. (This slot was corrected at P5 check 9, because the first wording overlooked the CI-reading entry.)

## Files inspected
- `crates/viola-core/src/lib.rs` (full) — `SERVICE_NAME` / `VERSION` already at :3-4; `ViolaName` (nutype, `try_new`) at :9-25. No `obs` module. Deps: `nutype` only (`crates/viola-core/Cargo.toml`), so `obs_event!` must expand to `::tracing::…` resolved at the caller.
- `src/main.rs` (full) — `PANIC_SINK: OnceLock<PanicSink{file, process, instance}>` at :14-28; `main` installs the hook first (:31) and wraps `cmd::dispatch(Cli::parse())` in one `catch_unwind` (:32-36) whose `Err` becomes `ExitCode::from(1)` with **no** role line; `viola_panic_hook` (:41-57) returns silently before the sink is set, never calls the default hook, never writes stderr, writes one `panic_line` (:59-82) with `panic_location` + `thread`; the payload is dropped (the CARRY gap). Tests :107-197 pin the line shape and payload absence.
- `src/run/mod.rs` (full) — `ensure_private_dir` (:19-30, 0700 + re-narrow on Unix), `open_role_file` (:33-50, `run-<name>.ndjson`, append, 0600 on Unix), `timestamp`/`MillisUtc` (:52-62), `init_role_logger` (:65-80, the full mandatory builder chain already, `Targets::new().with_target("viola", INFO)` only), and the six raw `tracing::info!/error!` call sites (:83-148) passing `event`/`process`/`instance` by hand; `log_self_exit` has no `duration_ms`.
- `src/cmd/run.rs` (full) — the run sequence: open role file :24 → `init_role_logger` :25 → `log_self_start` :26 → spawn → `log_child_start` :34 → `child.wait()` in `main` → `log_child_exit` :36 → `log_self_exit` :37 / :41 (missing-program `internal-error`). No worker threads.
- `src/cmd/mod.rs` (full) — `enum Command { Run(..) }` is the only verb; home = `--home` or `home_dir()/.viola` (no `VIOLA_DIR` step yet — Home/CLI chunks own it).
- `crates/viola-e2e/src/harness/logs.rs` (full) — `diag_lines` reads `<home>/diagnostics/*.ndjson` sorted, `take(64 MiB)` per file; `wrap_lines` emits `{"src":"diag","file","record"}` / `{"src":"diag","file","torn":true,"offset":n}` (torn = no trailing `\n` OR non-object JSON), CRLF-tolerant; `matches` treats absent = null. No detail source.
- `crates/viola-e2e/src/bin/viola-harness.rs` (:40-155) — `Logs { session, instance, process }`; clap parse failure → `Outcome::usage(cmd, "arguments")` exit 2 (:77-85; `harness/mod.rs:43-47`), so `--kind`/`--after` are already usage errors.
- `tests/run_cli.rs` (full) — pins the four run lines (`process-start` self/child, `process-exit` child/self), `level INFO`, `target` prefix `viola`, `message == event`, `process run`, `instance builder`, `corr` absent, identity fields on line 0, `child_exit_status`/`exit_source` on line 2, `exit_code` on line 3, canary/argv/version absence; missing program → 2 lines, `ERROR`, `detail:"internal-error"`; append → 8 lines; Unix 0700/0600.
- `crates/viola-e2e/src/harness/boot.rs` (:204-215 via grep) — interim readiness keys on `(event, subject)` + `pid` / `child_pid` / `exit_code`.
- Root `Cargo.toml` — `tracing =0.1.44` (`std`), `tracing-subscriber =0.3.23` (`fmt,json,registry,std`), `chrono =0.4.45`, `serde_json` `preserve_order`, `jsonschema =0.57.0` dev-dep (default features off); `[workspace.lints]` holds only `rust.unused_must_use`; no `clippy.toml`, no `.cargo/`.
- `crates/viola-e2e/Cargo.toml` — depends on `viola-core` (so `ViolaName::try_new` is available to the logs walk), serde_json, no tracing.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-09-24T10-05-14-phase/tree-query-2026-09-24-diagnostics-plane.json`, rust plane, 31 rows; lines below are editor lines = SCIP +1)
- **open_role_file / init_role_logger / log_self_start / log_child_start / log_child_exit / log_self_exit** — each has exactly one production caller, `run` @ `src/cmd/run.rs:24/25/26/34/36/37+41`. The migration is contained to `src/cmd/run.rs` + `src/run/mod.rs`.
- **set_panic_sink** — `init_role_logger` @ `src/run/mod.rs:66` + the test @ `src/main.rs:181`. A signature change (e.g. carrying the home for the detail path) touches both.
- **panic_line** — `viola_panic_hook` @ `src/main.rs:49` + tests @ `src/main.rs:143, 167`.
- **timestamp** — `viola_panic_hook` @ `src/main.rs:50`, `format_time` @ `src/run/mod.rs:60`, test @ `src/run/mod.rs:168`. Moving it into `viola::obs` re-points both callers.
- **diag_lines / wrap_lines** — `logs` @ `crates/viola-e2e/src/harness/logs.rs:23`, `diag_lines` @ :45, plus 7 inline test sites (:93, :111, :122, :127, :132, :133, :142, :151, :155). **logs** — `main` @ `crates/viola-e2e/src/bin/viola-harness.rs:143`, `harness_session_boots_reports_logs_and_tears_down` @ `crates/viola-e2e/tests/harness_lifecycle.rs:65`, and the unknown-session test @ `logs.rs:160`.
- Companion sweep: `grep -rn -H -E 'process-start|process-exit|duration_ms|claude-child|subject|panic|diagnostics' tests/run_cli.rs crates/viola-e2e/src/harness/boot.rs crates/viola-e2e/tests/*.rs` → 30 hits · 0 changed · 30 no-change. The migration keeps every pinned key and value (`event`, `subject`, `pid`, `child_pid`, `child_exit_status`, `exit_source`, `exit_code`, `detail`, `level`, `target` prefix `viola`), and `duration_ms` is additive. `tests/run_cli.rs` gains assertions and loses none.

## Measured facts
- **Write guard (scope item 8).** The probe was run as `python -X utf8 D:/dev/projects/additional/viola-overseer/guard_probe.py "C:/Program Files/Git/bin/bash.exe"` (GNU bash 5.2.37, x86_64-pc-msys). The probe reads `.claude/settings.json` `hooks.PreToolUse[0].hooks[0].command` and asserts that the shipped substring is present.
  - SHIPPED form: exit 0 on all six paths (`D:\…\target\x.rs`, `D:/…/target/x.rs`, `target\x.rs`, `target/x.rs`, `src\x.rs`, `src/x.rs`).
  - FIXED form: exit 2, 2, 2, 2, 0, 0 on the same six paths.
  - Mechanism: in the same bash, `path="a/b\\c/d"; ${path//\\//}` gives `ab\cd`, so forward slashes are deleted. The fixed `tr` form gives `a/b/c/d`.
  - The claim is re-derived at HEAD. The edit is `.claude/settings.json:14`, in a JSON string, so the `tr "\\\\" "/"` bytes must be written JSON-escaped.
- **`filter::Targets` + `MillisUtc` compile under `fmt,json,registry,std`:** they are already in `src/run/mod.rs:12,56-62,77`, and HEAD `4d8be52` is CI-green on 3 OSes. No new tracing-subscriber feature is needed, so no Stack-row amendment.
- `Cargo.lock` tracing-subscriber = `0.3.23`, which meets security's `>=0.3.20` (`grep -n 'tracing-subscriber' -A3 Cargo.lock`).
- `a11y-violation` is not in obs-plan's accepted event list (obs-plan §3 Obs extensions: "Not a product `event` value").
- **Readers of the role line shape:** harness readiness (`boot.rs:204-207`) and `tests/run_cli.rs`. Neither keys on key ORDER. `serde_json` `preserve_order` only affects harness re-serialization.

## Patterns detected
- **Explicit-mode private dirs/files** (`src/run/mod.rs:19-50`): `DirBuilderExt::mode(0o700)` plus a re-narrowing `set_permissions`, and `OpenOptionsExt::mode(0o600)` plus `set_permissions`, under `#[cfg(unix)]`. The detail dir and file reuse these helpers; move them into `viola::obs`.
- **Hand-formatted JSON line for out-of-subscriber writes** (`src/main.rs:59-82`): `serde_json::json!` with a conditional `instance` key (absence = null), `to_string()` + `'\n'`, one `write_all`. The detail writer follows the same shape.
- **Panic-hook unit test with a temp sink** (`src/main.rs:172-197`): `set_panic_sink` → `set_hook` → `catch_unwind(panic!(…))` → `take_hook`, then assert the line. This is the lean for proving the payload routing. A product-side panic trigger in the real binary would need an env/flag knob, and arch §Config management forbids that, so an in-process test is the only admissible forced panic.
- **Harness line wrapping** (`logs.rs:50-71`): the detail source reuses `wrap_lines` and adds the `instance` key.

## Conventions to follow
- **Test naming** `<subject>_<condition>_<expected>`, inline `#[cfg(test)] mod tests`, rstest `#[case::label]` tables (testing.md; `tests/run_cli.rs:53-153`).
- **Literal oracles**: the `event` enum equality test pins the obs-plan §3 values as literals, never `ObsEvent`'s own list (testing.md "What to assert").
- **No `set_var`**: level and `RUST_LOG` behaviour are tested with `Command::env` on the spawned `viola` (`tests/run_cli.rs:16-30` pattern).
- **Fresh home per test** under `target/e2e-home` via `support::home` (`tests/run_cli.rs:13`).
- **Every new function needs an observable effect** under the zero-missed mutation gate (testing.md Session Additions). A role-file path helper for roles without a producer (`hook`/`mcp`/`ui`/`cli`) must be exercised by a unit test of its name mapping, or it is an unkillable mutant.

## New files to create
- `crates/viola-core/src/obs.rs`: `ObsEvent` (closed, kebab `Display`), `ObsProcess`, `ProcessCtx` + its `OnceLock` setter/getter, and `obs_event!` (`#[macro_export]`, inner `#[allow(clippy::disallowed_macros)]`).
- `src/obs.rs` (root bin `viola::obs`): `MillisUtc`/`timestamp`, the private-dir/file helpers, the role-file path per `ObsProcess`, `viola_obs_init` (writer + `Targets` from the level), the detail writer, and the `config.json` `diagnostics_level` read.
- `schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`.
- `tests/contract_diag_schema.rs` (root, feature `fake-agent` like its siblings): schema conformance of real run lines, literal-enum equality, and literal-`null` rejection.

## Files to modify
- `crates/viola-core/src/lib.rs`: `pub mod obs;`.
- `src/main.rs`: `mod obs;`; the panic hook writes the detail line through `viola::obs` when an instance resolves; the catch site writes the run `process-exit{internal-error}` line; `timestamp` is re-pointed (callers :50).
- `src/run/mod.rs`: drop `init_role_logger`/`MillisUtc`/helpers (moved), and migrate the six `log_*` sites to `obs_event!` (+ `duration_ms` on self-exit).
- `src/cmd/run.rs`: call `viola_obs_init` at :24-25, and thread a start `Instant` for `duration_ms` to :37/:41.
- `crates/viola-e2e/src/harness/logs.rs`: add the `instances/*/diagnostics/detail-*.ndjson` walk (names through `ViolaName::try_new`, `symlink_metadata` skip), wrapped with `instance`.
- `tests/run_cli.rs`: add `duration_ms`, `config.json` level, `RUST_LOG`-inert, stdout/stderr-silent and detail-path assertions. Existing assertions stay.
- Root `Cargo.toml`: `[[test]] contract_diag_schema` entry (`required-features = ["fake-agent"]`).
- `.claude/settings.json:14`: the guard's `path=` rewrite.

## Open questions
- Three-OS concurrent-append check (test-plan §5 Module ↔ DB row, obs D-28) → blocks: plan-decision. No product path can have two writers on one file yet: `run`'s `already-live` collision does not exist, and `detail-run` has one writer. Lean: leave it to the first shared-writer producer (hook). The only test available now would be a synthetic harness-side check with no product writer behind it.
