# tests extract

## Relevance
relevant. The chunk builds the process-log format that the harness greps, plus the harness `logs` command. Both are tests-owned contracts in test-plan §3. It also carries a `.rs` delta, so the unit, mutation and coverage gates all apply. Item 8, the `.claude/settings.json` write guard, is out of the tests domain.

## Constraints
- test-plan §3 Log format (Required fields) requires every process-log line to carry `timestamp` (RFC 3339 UTC, ms, `Z`), `level`, `target`, `message`, `event`, `process` and `instance`. `corr` is also required and is never renamed. `obs_event!` / `viola_obs_init` must not rename or remove any of these; obs may only add fields (§3 Bootstrap `log-format-bind-with-obs`). A null `corr` or `instance` is written as key absence, never a literal `null` (§3 Log format, Null encoding).
- test-plan §3 Log format says `event` is a closed kebab enum with a fixed `corr` meaning per value. Any value beyond the listed set needs a §12 Decisions Log entry first. `a11y-violation` is explicitly not a product value and gets no `ObsEvent` variant (§3 Log format, Harness-side a11y rows; §12 Z7). `process` is `run|hook|mcp|ui|cli` (§3 Log format; §12 O1–O3).
- test-plan §3 `logs` fixes the diag wrappers, which must be exact:
  - `{"src":"diag","file","record"}` for home-level role files.
  - `{"src":"diag","file","instance","record"}` for `instances/*/diagnostics/detail-*.ndjson`.
  - Torn or unparseable lines are emitted as `{"src":…,"torn":true,"offset":n}` and never dropped. Panic counts use role files only (`.file|startswith("detail-")|not`).
- test-plan §3 preamble (Exit codes) says a flag whose surface is not built yet is a usage error (exit 2, `reason:"usage"`), never a vacuous pass. So `logs --kind` / `--after` and the `events` source must be rejected as usage errors until their producer lands, not silently ignored. Whether the current `logs` already rejects them is research's question.
- test-plan §3 `boot` Readiness signal (interim) reads `process-start` lines for `subject:"self"` and `subject:"claude-child"` from `diagnostics/run-<name>.ndjson`, and treats a first `process-exit{subject:"self"}` as `run-exited`. The `obs_event!` migration of `src/run/mod.rs` must keep those lines matchable, and the §3 `run` step 2 `booted_wrapper` interim wait depends on the same lines.
- test-plan §3 Log format (Format, Constraints) requires:
  - files 0600 and dirs 0700, with one `write` per line
  - no multi-line stack traces
  - a panic logged as one `level:"ERROR", event:"panic"` line
  - content-bearing detail (panic payload and backtrace) only in `detail-<process>.ndjson`
- test-plan §10 Mutation gate + §3 `run` step 4: a diff naming any `.rs` path is `counted` and must show `missed == 0 && timeout == 0`. Coverage is ≥85 lines / ≥95 functions / ≥80 regions per OS (§10 Coverage thresholds). There are no retries (§10 Zero-flakiness budget).

## Patterns to follow
- Unit tests go inline in `#[cfg(test)] mod tests`, named `<subject>_<condition>_<expected>`, with rstest `#[case::label]` tables (test-plan §4 Conventions). This fits the `ObsEvent` kebab `Display` table, the `diagnostics_level` default/absent/`debug` cases and the torn-line merge cases.
- Real tempfile homes under `target/e2e-home/`, passed as a path that does not exist yet, so viola creates the 0700 home itself (test-plan §3 `boot` step 2; §5 Setup / teardown lifecycle). Do not mock the filesystem (§8). Mode checks read back `std::fs::metadata` (`& 0o777`) on Unix.
- Schema conformance: tests own the check body; obs owns the schema. Validate with jsonschema 0.57.0 against `schemas/diag-line.v1.json`, skip and count non-JSON lines, and print file, line and keyword but never the content (test-plan §6 Schema conformance). The `[inferred]` "schema `event` enum == `ObsEvent` variants" check belongs to this tests-owned family.
- Concurrent-append check: separate processes append to a shared diagnostics file, including one detail line over 4 KiB, and every line must parse as one JSON object (test-plan §5 Boundary table, Module ↔ DB row). Only the `run` role has a producer today, so the reachable cases are the colliding second `run` on a live `run-<name>` file and the `detail-run` file. Whether a second `run` writes before it refuses is research's question.
- Assertions pipe harness `logs` output into `jq -e` / jaq 3.1.1 (test-plan §3 `logs`; §3 Bootstrap `log-format-bind-with-obs`: "done when `agent-run logs` output passes the `jq -e` / jaq assertions").

## Anti-patterns to avoid
- NEVER use the product's own list as the test oracle (test-plan §11 Unit). The enum-equality check should pin the §3 Log format `event` values as literals in the test. Diffing the schema only against `ObsEvent` would let both drift together.
- NEVER use `std::env::set_var`, including to exercise `RUST_LOG` or level behaviour; use `Command::env` per child (test-plan §11 Integration). NEVER sleep to wait for log lines; key on file state or offsets (§11 E2E; §11 Universal).
- NEVER ignore or skip a test to park a failure, and NEVER retry (test-plan §10 Zero-flakiness budget; §11 CI).

## Contract bindings
- **tests §3 Log format ↔ obs-plan §3 Log format / `schemas/diag-line.v1.json`.** Tests owns the harness-grepped field names; obs owns the schema file. test-plan §6 Schema conformance also validates detail lines against `schemas/diag-detail.v1.json`, but the scope names only `diag-line.v1.json`. Whether this chunk must also ship `diag-detail.v1.json`, now that detail files get a producer, is an obs/research question to close at P3.
- **tests §3 `logs` ↔ obs-plan D-08 (detail files)** and obs D-12 (null as key absence).
- **tests §6 Error sanitization and secret scan (canary) ↔ obs-plan §8 / security-plan `logging-redaction-wire`.** The canary may appear in `instances/*/diagnostics/detail-*.ndjson` but never in home-level `diagnostics/*.ndjson`. This binds the home-level panic line (payload-free) against the `detail-run.ndjson` payload routing.
- **tests §5 `tests/cli_controls_not_disableable.rs` ↔ arch `config.json`.** Its completeness case fails if a config key is missing from its literal table, so the new `diagnostics_level` key has to be accounted for. Whether that test exists yet is research's question.
- **tests §9 Lint row (`cargo check` of `scripts/sync-crates.txt` crates without tokio) ↔ arch.** `viola-core` is a listed sync crate, and the scope's "viola-core gains no `tracing` dependency" is checked by that same Lint stage and by `cargo modules`.

## Acceptance criteria contributions
- `scripts/agent-run.sh run --unit` exits 0 with the new `.rs` delta. This closes the Rust gate deferral carried from 2026-09-24-supply-chain-and-workflow-gates. `run --mutants` reports `"verdict":"counted"` with `missed == 0 && timeout == 0`, and coverage holds at 85/95/80 per OS (per test-plan §10 Mutation gate and Coverage thresholds; §3 `run` step 4).
- In a booted session, `agent-run logs --process run | jq -e` shows:
  - every home-level line wrapped `{"src":"diag","file","record"}` with `timestamp`/`level`/`target`/`message`/`event`/`process`/`instance` present
  - no literal `"corr":null` or `"instance":null`
  - `process-start` carrying `service_name`, `version`, `os` and `pid`
  - `process-exit{subject:"self"}` carrying `duration_ms`

  A truncated last line appears as `{"torn":true,"offset":n}`, and `--instance` / `--process` filter correctly (per test-plan §3 `logs` and §3 Log format).
- After a forced panic in `run` with a resolved instance, the role file holds exactly one single-line `level:"ERROR", event:"panic"` with no payload. The payload and backtrace appear only in `instances/<name>/diagnostics/detail-run.ndjson`, surfaced by `logs` with an `instance` key. On Unix both files are 0600 in 0700 dirs, and the canary never appears in home-level `diagnostics/*.ndjson` (per test-plan §3 Log format; §6 Error sanitization and secret scan).
- `logs --kind` / `logs --after` exit 2 with `reason:"usage"` until the events producer lands. Every home-level line validates against `schemas/diag-line.v1.json` under the tests-owned conformance body, and a literal-list test asserts that the schema's `event` enum equals the §3 Log format values (per test-plan §3 preamble Exit codes; §6 Schema conformance; §11 Unit).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton, interim supervisor, readiness, status and cleanup.** This amendment defined interim readiness as `process-start` lines for `self` and `claude-child` in the role file, plus the per-chunk grammar-growth rule (an unbuilt selector is a usage error). Why: the report's Harness surface and the operator decision "the grammar grows per chunk". This chunk's `obs_event!` migration must keep those readiness lines intact, and `logs --kind` / `--after` stay usage errors under that rule.
- **2026-09-24-three-os-ci-headless-harness-skeleton, harness builds in its own target dir.** Harness cargo work uses `CARGO_TARGET_DIR=target/harness` and `--features viola/fake-agent`. Why: a Windows `os error 5` when relinking the running harness exe. This applies to any `viola-e2e` `logs` changes and their tests.
- **2026-09-24-three-os-ci-headless-harness-skeleton, mutation gate prebuilds the root bins.** `run --mutants` prebuilds the root `viola` and fake agent and passes `--copy-target=true`. Why: a diff touching only `viola-e2e` failed its baseline. This chunk's diff spans `viola-core`, the root bin and `viola-e2e`, so the path applies.
- **2026-09-24-fake-agent-and-test-data-fixtures, mutation verdict for Rust-free diffs.** A diff naming a `.rs` path is `counted`, and a missing fresh `outcomes.json` is red. Why: cargo-mutants exits 0 on a Rust-free diff. This chunk is the first `counted` run since the supply-chain chunk's `no-rust-delta`.
- **2026-09-24-supply-chain-and-workflow-gates, Lint row reads sync-crates.txt.** The Lint `cargo check` takes one `-p` per crate in `scripts/sync-crates.txt`. Why: the wrappers mechanism was falsified. This is the gate that holds the "viola-core has no tracing/tokio dependency" constraint.
