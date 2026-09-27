# tests extract

## Relevance
Relevant. The chunk lands most of test-plan §6 Path 1 (the endpoint-bind and squat halves are out), the exit-1 rows of the §6 Exit-cause matrix, the E2 `VIOLA_*` presence half, and the `viola-state` unit cases in §4.

## Constraints
- **Path 1 order and fields (test-plan §1 Critical paths "`run` start sequence", §6 Path 1).**
  - The first three `events.ndjson` records must be `wheel{cause:"start"}` → `budget-gate` → `session-start{source:"hook"}`.
  - The receipt's `started_at` must be later than the `budget-gate` `ts`. That is how the plan proves the spawn came last.
  - The snapshot must carry `endpoint`, `pinned_bin`, `pid`, `started_at` and `child_pid`.
  - If this chunk leaves the start events to "The wheel" or "Budget governor", this ordering signal cannot be asserted yet. Research must close that premise against the specs.
  - `endpoint` has no value until "Wrapper channel" lands. Whether the snapshot may carry a placeholder or `null` there is research's question.
- **Liveness terms: stale is not the same as gone (test-plan §4 viola-state, §6 Exit-cause matrix, §6 Chaos suite).**
  - The classifier has three outcomes. `live` = heartbeat ≤5 s. `stale` = pid + start time still alive but heartbeat >5 s. `gone` = pid dead.
  - The plan requires `run` to exit 1 on `stale` (the matrix row "live pid, heartbeat older than 5 s", and the chaos SIGSTOP case). A takeover is allowed only for a dead wrapper (E5: "not 1").
  - The scope's line "a stale instance (its wrapper gone, liveness stale) is taken over" uses "stale" for what the plan calls `gone`. Reconcile this at planning; do not build a takeover of a `stale` instance.
- **Exit-1 rows each get their own hint (test-plan §6 Exit-cause matrix, `cross_exit_causes.rs`).**
  - Rows in scope here: name already live, heartbeat `stale`, pinned exe fails its SHA-256 re-hash, and `--home` fails strict-modes at `run` (if strict-modes lands in this chunk).
  - Each row asserts the exit code, the `--json` document, and a last stderr `hint:` line naming that cause. The live-name hint is `viola list`.
  - A hint shared across causes fails the table.
- **`viola-state` unit cases (test-plan §4 viola-state, §10 Performance budgets heartbeat row).**
  - The ndjson writer does one `write` per line.
  - The liveness classifier is tested with mock_instant `MockClock::advance` behind `viola_core::Clock`, at the exact boundary: 4.9 s → live, 5.1 s → gone or stale.
  - Unix strict-modes checks `mode & 0o077`, if it lands here.
  - The heartbeat target is a `heartbeat` file under `instances/<name>/`, touched every 1 s (test-plan §3 PID file, §3 boot Readiness). Arch still owns the final carrier.
- **Readiness checks move from interim to real (test-plan §3 `boot` Readiness signal, §3 `run` step 2).**
  - The interim `process-start` readiness is replaced check by check "once its surface lands": `snapshot.json` parses with the four fields, `heartbeat` mtime is under 5 s old, and `events.ndjson` lines 1–3 appear in order.
  - The root `booted_wrapper` wait keeps its 10 s exit-aware bound.
  - A `run` refusal surfaces as `reason:"run-exited"` with the wrapper's `exit_code` verbatim.
- **Sync crate and gates (test-plan §9 Lint and Supply-chain rows, §10).**
  - `viola-state` joins `scripts/sync-crates.txt`, which puts it in the tokio-free `cargo check` and the sole-root `deny-sync.toml` tokio ban.
  - Coverage: lines ≥85, functions ≥95, regions ≥80, per OS.
  - Mutation: `missed == 0`, `timeout == 0`, `unviable <= caught`. `#[cfg(unix)]` mode code is judged only on the leg that compiles it.
- **Where the E2E witnesses live (test-plan §3 `run` step 2).** Paths 1 and E2 use the chain `home → fake_agent_path → stamped_home → booted_wrapper` from `viola_e2e::fixtures`, and that copy "lands with its first consumer". Whether this chunk's E2 or Path 1 witness is that first Tokio-side consumer, or stays in root `tests/`, is research's question (the scope's CARRY 2).

## Patterns to follow
- **Root sync fixture chain:** `tests/support/home.rs` (`Wrapper::boot/stop`, homes under `target/e2e-home/viola-test-*`), `tests/support/outer_pty.rs`, `tests/run_cli.rs::run_viola`. Long-lived wrappers are driven with std::process plus a `Drop` guard (test-plan §3 `run` step 2, §5 Setup/teardown).
- **E2 receipt env assertion:** `tests/tui_env_strip.rs`. The fake agent's `env {names}` receipt kind (names only) is the oracle for `VIOLA_NAME` / `VIOLA_DIR` presence (test-plan §6 E2, §7 Fake agent).
- **Exit-1 refusal shape:** `tests/cli_program_resolution.rs` is the model. It shows exit 1 with no child spawned, `process-exit{subject:"self", exit_code:1, detail:…}`, and exactly two fixed `unable:` / `hint:` stderr lines (test-plan §6 Chaos suite, `.cmd` row).
- **Content-hash oracle:** `tests/contract_content_hash.rs` already pins sha2 and the 16-hex truncation (test-plan §6 Contract suite). The re-hash refusal builds on that library rather than restating it.
- **Plugin-folder oracle:** the fake agent reads hooks from `<plugin-dir>/hooks/hooks.json`, which `run` passes as `--plugin-dir`. It runs only an absolute exec-form `command`. A non-absolute one gets receipt `command_absolute:false, ran:false` (test-plan §7 Fake agent). That receipt is the M6 absolute-path witness. The written layout must put `hooks.json` under `hooks/`.

## Anti-patterns to avoid
- Never hand-write `snapshot.json`. On-disk state comes only from a real `viola run` in a fresh home. The only exception is chaos corruption of files the product already wrote, such as the one-byte tamper of `pinned_bin` (test-plan §11 Test Data, §7 seed table).
- Never synchronise on sleeps or real time in test code. The heartbeat staleness verdict is clock-driven in a §4 unit case. E2E may block on the product's staleness deadline only when the verdict is the product's own exit code or status field (test-plan §11 Universal, §11 E2E).
- Never use `std::env::set_var` or `assert_cmd .assert()` on the long-lived `viola run`; use `Command::env` per child and std::process plus a `Drop` guard. Never import product constants (for example the exit codes) as the test oracle (test-plan §11 Integration, §11 E2E, §11 Unit).

## Contract bindings
- **obs, event line:** the log format `{"v":1,"ts","instance","kind","source","data"}` (test-plan §3 Log format; obs §Log format). Every event kind must round-trip, with `ts` as RFC 3339 UTC with ms and `Z` (test-plan §5 On-disk).
- **obs, readiness and status:** the harness readiness reads obs `process-start` / `process-exit` lines in `diagnostics/run-<name>.ndjson` until the snapshot, heartbeat and events surfaces replace them. The status shape's `instances[]` interim-`null` fields (`pid`, `uptime_ms`, …) may be filled now that the snapshot exists (test-plan §3 Status endpoint shape; obs §Status endpoint shape).
- **obs, G4 schema gate and secret scan:** these run over `target/e2e-home/**` diagnostics after every E2E test (test-plan §6 Schema conformance, §6 Error sanitization canary).
- **security Vector 7:**
  - 0600 files and 0700 dirs under `umask 000`, and a pinned exe that is never group- or world-writable (test-plan §6 Security control negatives, Umask-independent modes).
  - `snapshot.json` has a single writer (test-plan §6 Single writers).
  - `plugin/` and `settings.json` are rewritten on every start (test-plan §1 Coverage triggers, filesystem).
- **arch:** the heartbeat carrier and period, and the plugin layout under `--plugin-dir` (test-plan §3 boot step 5 defers to arch §Occupied Resources).

## Acceptance criteria contributions
- A Path 1 subset witness (per test-plan §6 Path 1):
  - the snapshot has `pinned_bin`, `pid`, `started_at` and `child_pid`, and `endpoint` as far as this chunk defines it;
  - a duplicate `run` of a live name exits 1;
  - after a one-byte tamper of the file at `pinned_bin`, `run` exits 1;
  - after a restart, the sentinel written into `hooks.json` / `settings.json` is gone, and every hook command is the absolute `pinned_bin` path.
- A `viola-state` liveness classifier unit test on the exact boundary, driven by mock_instant: 4.9 s → live; 5.1 s → `gone` with a dead pid and `stale` with a live pid + start time (per test-plan §4 viola-state, §10 Performance budgets).
- `#[cfg(unix)]` under `umask 000`: `events.ndjson` and `snapshot.json` are 0600, dirs are 0700, and `bin/<version>-<hash>/viola` is 0700 (per test-plan §6 Security control negatives, Umask-independent modes).
- The fake-agent `env` receipt contains `VIOLA_NAME` and `VIOLA_DIR` (per test-plan §6 E2).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton, interim readiness/status:** interim readiness (`process-start` lines for `self` and `claude-child`) and `null` status fields were introduced because `snapshot.json`, `heartbeat` and `events.ndjson` were unbuilt. Ruling: "the grammar grows per chunk". This chunk is where those checks start to be replaced.
- **2026-09-24-fake-agent-and-test-data-fixtures, fake-agent contract "consumer-first":**
  - hooks are read from `<plugin-dir>/hooks/hooks.json` and only absolute exec-form commands run;
  - the `viola_e2e::fixtures` copy lands with its first consumer;
  - the root `stamped_home` is an interim no-stamp seam.
  - Why: the operator ruled "no shapes invented before a recorded fixture". Plan sites :229 and :349 (`viola run` rewriting its plugin folder) were kept as target state.
- **2026-09-24-fake-agent-and-test-data-fixtures, fixture naming:** `--plugin-dir` comes from `viola run`, not the harness (architecture §Occupied Resources).
- **2026-09-24-observability-gates:** root `booted_wrapper` readiness was cut to 10 s and made exit-aware. Measured cause: waits spinning to 20 s under cargo-mutants' 20 s floor produced Timeouts. Any new readiness wait for snapshot or heartbeat must keep this bound.
- **2026-09-25-pty-wrapper-on-windows:** the E2 `VIOLA_*` presence half was re-pinned to "Instance state and start order" as its route CARRY. The strip half landed in `tests/tui_env_strip.rs`.
- **2026-09-25-security-prerequisites:** the pinned-exe content hash was pinned by `tests/contract_content_hash.rs` (sha2, FIPS 180-2 vectors plus the 16-hex truncation) ahead of its product consumer, which is this chunk's re-hash (CARRY 1).
- **2026-09-24-supply-chain-and-workflow-gates:** the Lint `cargo check` and the sole-root tokio ban read `scripts/sync-crates.txt`, and a crate joins the list "with its crate". Why: the report disproved the old wrappers-list mechanism. `viola-state` is the next entrant.
- **2026-09-26-ci-chunk-base-and-union-verdict:** the union rule judges a mutant only on the leg that compiles its line. This is relevant to the `#[cfg(unix)]` / `#[cfg(windows)]` mode and strict-modes bodies this chunk adds.
