# tests · cascade read · group 1 of 3

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:123) The chunk edits `run`'s start `pin_bytes` call, so the start sequence must still write the pinned copy and plugin folder before the child spawn and must still exit 1 on a tampered pinned exe or a refused home tree, never overwriting the copy.
  > - **Path:** `run` start sequence. The home-tree strict-modes check before the wrapper log opens (`check_home_tree`: the home under the read rule, the instance check's set, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/` and the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists), the collision check (over a replayed snapshot, liveness is the beat alone), pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn. A second `run` of the same name, or a squatted endpoint, exits 1.
  (.andromeda/test-plan-detail/test-scope-summary.md:127)
  > `snapshot.json` has `pinned_bin`, `pid`, `started_at`, `child_pid` (`endpoint` from "Wrapper channel" on); `plugin/` is rewritten, and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written.
  (.andromeda/test-plan-detail/test-scope-summary.md:129)
  > A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1. A refused home tree (a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy) exits 1 before any log opens, with the home pair (the instance pair for an instance-level refusal), no line and no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`).
  (.andromeda/test-plan.md:135)
  > - **Path:** `run` start sequence — the home-tree strict-modes check before the wrapper log opens, then the collision check, pinned copy and plugin folder, the fail-open ConPTY sideload, version gate, exclusive endpoint bind, first snapshot and heartbeat, and the start `wheel` and `budget-gate` events all precede the child spawn; a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:79) The race witness's rig is the harness `boot` of two instances on an unstamped home, and the race's observable signal is an instance whose `session-start` never lands, so `boot`'s per-instance readiness and its failure document must be what the witness reads, red before the fix and clean after it.
  > Starts one or more `viola run <name> -- <fake agent> <args>` wrappers.
  >     - Readiness, per instance: `instances/<name>/snapshot.json` exists with `pid`, `started_at`, `child_pid` and `endpoint`. The `heartbeat` mtime is under 5 s old. The log has a start `wheel` and a `budget-gate` event, followed by `session-start` once "Verify-stamped test homes and harness" (the `boot` step-4 owner) joins it to harness readiness.
  >     - On readiness failure it exits non-zero with a JSON reason on stdout. A `run` refusal to start (exit 1: a `live` or `stale` name, a tampered pinned copy, or a `.cmd`/`.bat` child) is surfaced verbatim.
  (.andromeda/test-plan.md:344)
  > each harness session writes `target/agent-run/<session>/session.json` with `{v, session, home, instances:[{name, wrapper_pid, started_at, fake_args}], supervisor_pid, supervisor_started_at, agents_mode}`
  (.andromeda/test-plan.md:345)
  > every harness command exits 0 on success, 1 on failure and 2 on usage error. Each prints exactly one JSON document on stdout, starting `{"v":1,"cmd":"<command>","ok":<bool>,…}` (key order held by serde_json `preserve_order`). A usage error carries `reason:"usage"` and a `detail` from `invalid-session-id`, `session-exists`, `invalid-instance`, `arguments`, `no-supervise-spec` and `unknown-suite` (a `gate --require` value outside the eight suites of `gate::SUITES`: `nextest-unit · nextest-integration · doctest · nextest-e2e · mutants · coverage · perf · fuzz-replay`). The grammar grows per chunk: a flag or selector whose surface is not built yet is a usage error until its chunk lands, never a vacuous pass
  (.andromeda/test-plan.md:118)
  > - **5-command requirements:** `boot` builds `viola` and starts the wrappers in a fresh home to readiness;
- (.andromeda/test-plan-detail/test-strategy.md:17) Path 1 is re-asserted and Path 7's return to two instances is a P4 fork, so both stay E2E scenarios over harness-booted wrappers and the fake agent, while a narrower concurrent-pin control the plan may add sits at the lowest layer that can hold it.
  > **Tier-specific depth (Comprehensive, viola):** Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
  (.andromeda/test-plan.md:871)
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:918) The race fails a real hook spawn of the pinned path, so the witness must let the fake agent spawn the real pinned binary through the plugin files `run` wrote, never a stubbed `viola hook`.
  > - NEVER stub `viola hook` in E2E. The fake agent must invoke the real pinned binary through the plugin files `run` wrote.
- (.andromeda/test-plan-detail/test-strategy.md:26) A concurrent-pin control or concurrent-start fault the plan may add must inject its race only through real process and file operations or the existing mockall seams, never a failpoint crate, and must replay deterministically.
  > **Tier-specific depth (Comprehensive, viola):** Required (chaos trigger present). Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
  (.andromeda/test-plan-detail/test-strategy.md:67)
  > | chaos-test | chaos | §6 Chaos suite |
  (.andromeda/test-plan.md:917)
  > NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
  (.andromeda/test-plan.md:952)
  > NEVER run property or chaos tests without deterministic replay: proptest seeds persist in committed `proptest-regressions/`, and chaos faults come only from explicit process / file operations or existing mockall seams (§2 Property-based and Chaos rows).
- (.andromeda/test-plan-detail/test-strategy.md:29) The two-sided witness is a control whose pre-fix arm lets product processes start, so it must spawn through `tests/support/cli.rs` `Running` bounded by `WITHIN` and keep its long-lived wrappers in a `Drop` guard, never an unbounded or blocking spawn.
  > Beside it stands the per-chunk remove-the-guard rule (`.claude/rules/testing.md`): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, since nextest's default profile has no kill line (an unbounded witness hung 27 minutes, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`).
  (.andromeda/test-plan.md:901)
  > NEVER use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. It blocks until exit; use std::process + a `Drop` guard.
- (.andromeda/test-plan-detail/test-strategy.md:75) The race witness is the scope's own flake-prone obligation, so it must be deterministic or red: no retry, no parking, no test-owned timer or sleep, its verdict from logged events, receipts and exit codes, and a witness that could pass vacuously is no witness.
  > | discipline (no screen parsing / no blanket approval / no flakes) | all | §6 Scenario 4 negative; §11 |
  (.andromeda/test-plan.md:925)
  > NEVER ignore flaky tests; quarantine and fix immediately
  (.andromeda/test-plan.md:946)
  > NEVER use real time without injection (use deterministic clocks) in test-side code: no test-owned timer, sleep or elapsed-time verdict.
  (.andromeda/test-plan.md:951)
  > NEVER add retry-once policies or any retry budget for flaky tests (§10 Zero-flakiness budget).
  (.andromeda/test-plan.md:275)
  > Waits must key on logged events and byte offsets, not sleeps.
- (.andromeda/test-plan-detail/test-strategy.md:63) The pinned copy and `plugin/` are the filesystem vector's code-bearing state, so the re-asserted negatives (a widened pinned copy or `bin/` refused before any log, a hash mismatch exiting 1, `plugin/` rewritten on start) must stay green at the integration and chaos layers this row names.
  > | Filesystem (V7) | integration + chaos | §5 Module ↔ disk; §6 Scenario 1, E5 (root `chaos_snapshot_replay.rs`: steps 1–3) + Security control negatives (root `security_negatives_strict_modes.rs`: each of the six frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` refusing a widened instance before any log, exit 21 with the instance pair or the `instance-unreachable` document, no frame and no line; a home another user may read refusing `run`; a start under umask 000 making every home path owner-only (Unix); a Windows `--home` outside the profile taking the protected DACL, read back through `icacls /save`; only the wrapper writing `snapshot.json` and only `verify` writing `stamps.json`, each byte-identical after every other writer-capable entry point) |
  (.andromeda/test-plan.md:211)
  > a widened `ledger/`, `bin/`, `plugin/` or pinned copy → `run` exit 1 before any log; a widened `ledger/`, `bin/` or `plugin/` → `verify` exit 1 before any step
  (.andromeda/test-plan.md:214)
  > - pinned exe hash mismatch → exit 1
  >     - `plugin/` rewritten on start, and on Unix the instance's `settings.json` (none is written on Windows)
- (.andromeda/test-plan-detail/test-strategy.md:70) Whether the exe's absent-copy arm on Windows needs the `ERROR_SHARING_VIOLATION` wait is an OS-only check, so it must be read on the `windows-2025` CI runner and never inferred from a Linux run.
  > | multi-os-compat | CI matrix | §9 Matrix builds |
  (.andromeda/test-plan.md:876)
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/unit-test-strategy.md:28) The chunk edits `pin_bytes` / `pin_exe` in `crates/viola-state/src/pin.rs`, so the pin unit cases must keep holding and a new unit case on the absent-copy arm sits beside them, while the home-tree check's cases witness that the strict-modes check stands unchanged.
  > Stamps and pin: `update_stamps_refuses_a_widened_ledger_and_leaves_it` (`StampsError::StrictModes`, checked before `create_private_dir(ledger/)` and again under the lock, nothing read or written), `stamps_error_messages_are_fixed`, `pin_key_is_the_version_and_the_content_key` and `pin_companions_refuses_a_path_another_user_may_write` (`PinError::StrictModes`, nothing written).
  (.andromeda/test-plan-detail/unit-test-strategy.md:27)
  > The home-tree and diagnostics checks: `check_home_tree_of_a_tree_viola_wrote_passes`, `check_home_tree_refuses_each_widened_path`,
  (.andromeda/test-plan.md:382)
  > and the stamps and pin strict-modes refusals.

## Bindings
- **test-scope-summary Path 1 ↔ test-strategy V7 row ↔ unit-test-strategy pin cases**: the tampered-copy exit 1 and the never-overwrite rule are held at three layers at once, the E2E start (.andromeda/test-plan-detail/test-scope-summary.md:129), the filesystem negatives (.andromeda/test-plan-detail/test-strategy.md:63) and the `viola-state` pin unit cases (.andromeda/test-plan-detail/unit-test-strategy.md:28); the absent-copy fix must leave all three green and turn no mismatch into an overwrite.
- **test-scope-summary `boot` readiness ↔ test-strategy E2E ↔ the body's no-stub rule**: the race signal is a missing `session-start` (.andromeda/test-plan-detail/test-scope-summary.md:80) on a harness-booted wrapper (.andromeda/test-plan-detail/test-strategy.md:17) whose fake agent spawned the real pinned hook (.andromeda/test-plan.md:918); the witness reads it only through that chain.
- **test-strategy remove-the-guard ↔ test-strategy discipline row**: the red-before-the-fix arm is bounded (.andromeda/test-plan-detail/test-strategy.md:29) and the whole witness is deterministic with no retry (.andromeda/test-plan-detail/test-strategy.md:75); a witness that cannot show the race window reachable is red under both.
- **test-strategy chaos rows ↔ the body's mocking ban**: a concurrent-start control's faults come from process and file operations (.andromeda/test-plan-detail/test-strategy.md:26) and never a failpoint crate (.andromeda/test-plan.md:917).
- **test-strategy multi-os row ↔ the body's Linux-only ban**: the Windows exe arm's reading is the CI matrix's (.andromeda/test-plan-detail/test-strategy.md:70), never a Linux run's (.andromeda/test-plan.md:876).
