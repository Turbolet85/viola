# tests extract

## Relevance
relevant — W3d-a is the §6 Path 4 `permission` step the plan names as owed to this entry, and F1 moves a §3 Bootstrap `test-runner-install` bound class.

## Constraints
- test-plan §6 Path 4 (Steps 1 and 3, Verification signal) requires the `permission` step to replay `PermissionRequest.permission-1.json` as prompt 2 of the dialog script, and to answer `{"behavior":"allow"}` and, in a second run, `{"behavior":"deny","message":"no"}`. It must assert four things: the `permission` event is logged exactly once with a `dialog_id`; `wait` returns it at once; the receipt's hook line reads `exit_code:0` and `stderr_len:0`; and the hook stdout matches the per-`<cli-version>` insta snapshot (`.dialog_id` redacted) of PermissionRequest `decision.behavior:"allow"`, and of `"deny"` with `message`. The plan's Surfaces note says the `permission` kind is unit / insta only (`dialog::tests`) and its end-to-end case is owed to "Permission end to end". Whether `tests/cli_answer.rs` and `fixtures/fake-scripts/path4.json` already carry any part of that step is research's question.
- test-plan §11 Test Strategy (first ban) and §2 pyramid keep the PermissionRequest body shapes at unit level, in §4 viola-agent-claude's insta-pinned "Dialog mapping S3/S7/S8": PermissionRequest allow/deny + `message`. The E2E case asserts only cross-process behaviour: logged once, `wait` wakes, `answer` by id, hook stdout and exit. The §4 root-bin `wait` handler case (it wakes on `turn-ended` / `question` / `permission` / `plan` / `session-end`) stays the clock-driven unit witness. The E2E case is the cross-process wake witness that §6 Path 3's Surfaces note says is owed here.
- test-plan §7 Fake agent and §7 Test Data seed rows govern the replay:
  - scenario scripts live in `fixtures/fake-scripts/<scenario>.json`, hold synthetic text only and are validated by `schemas/fake-script.v1.json`;
  - gated steps are released by appending lines to `<home>/fake/<name>.control`, never by timing;
  - hook payload bytes are `<fixtures>/<cli-version>/<Event>.<variant>.json`, chosen by `--cli-version` (`DEFAULT_CLI_VERSION` 2.1.287);
  - on-disk state comes only from real `viola run` / `hook` / verbs in a fresh per-test home, and stamps only from `viola verify` against the fake agent (`stamped_home`).
- test-plan §11 Universal (no blanket approval) and §6 Path 4 Step 6 require two things. A decision body is emitted only when the wheel is `driver` and the CLI version is verify-stamped. No test may pass when a body is emitted without both.
- The `[profile.ci]` rule in test-plan §3 Bootstrap phases `test-runner-install` sets these bounds:
  - `cli_answer` is one of the twelve verify-driven binaries, killed at 20 s (`{ period = "10s", terminate-after = 2 }`);
  - `test(/verify_window_/)` is killed at 45 s;
  - the default kill is 120 s, with `retries = 0`.
  These bounds are justified only as "sized from the measured floor … plus a 3x runner tail". F1's exception adds a planted-hang control on top of that rationale. Whether `.config/nextest.toml` or `tests/support/verify.rs` already holds such a control is research's question.
- test-plan §10 Zero-flakiness budget and §11 CI/Quality forbid nextest `retries` above 0, retry-once policies, and `#[ignore]` / `test.skip` as a parking place. A red stays in the chunk until its root cause is fixed. F1's wording must not read as a licence to raise a bound to clear a red.
- test-plan §11 Test Strategy forbids two things. CI never runs the real `claude` or `viola verify` against it. A green fake-agent run is never proof of real-CLI behaviour: a behaviour change needs a local `viola verify` fixture refresh plus the contract suite. For W3d-b, any PermissionRequest body for a question first raised there is grounded in a measurement of the real CLI (a static bundle read or a named live session). A fake-agent replay alone does not ground it.

## Patterns to follow
- The existing `question` / `plan` cases in `tests/cli_answer.rs` over `fixtures/fake-scripts/path4.json`, named `path4_<slug>` per §2 Test function naming. The new case uses the same driver (assert_cmd 2.2.2 for `answer` / `wait`, and std::process plus a `Drop` guard for the long-lived wrapper, per §6 Drivers and §11 E2E).
- Synchronisation uses `viola wait builder --after C --json` and `events.ndjson` byte offsets (§6 Path 4 Step 2, §2 Agent-runnable invariants). Gated fake-script steps are released through the control file (§7 Fake agent).
- Verdicts come from `events.ndjson`, the fake-agent receipt's `hook` line (`exit_code`, `stderr_len`, `stdout_hex`) and the verb's `--json` document plus exit code (§6 tui driver row, §11 E2E no screen parsing).
- Decision-body checks are insta snapshots in check mode only (`INSTA_UPDATE=no`, §2 Agent-runnable invariants; §4 Fixture pattern `assert_json_snapshot!` with `.dialog_id` redacted).
- The fake-script hygiene walk in `tests/contract_fixture_hygiene.rs` covers every committed script and the recorded `<Event>.<stem>-<n>.json` variants (§7 Fixture hygiene). A new or edited script joins that walk with no new mechanism.

## Anti-patterns to avoid
- No `sleep(N)` or test-owned timer for synchronisation (§11 E2E, §11 Universal). No parsing of the rendered child screen for content (§11 E2E, R7).
- No interactive snapshot acceptance (`cargo insta review`, trycmd overwrite) as a gate step: snapshots change only through the local `viola verify` fixture refresh (§11 Universal; §7 seed row "Decision bodies"). Whether a new permission-body snapshot can be added in this chunk without a refresh, or must reuse the existing `dialog::tests` snapshots, is a P4 question.
- No test that auto-approves a dialog, or that passes on a body emitted without a stamp and a `driver` wheel (§11 Universal).

## Contract bindings
- tests ↔ arch: the case drives the channel `hook.dialog` / `answer` frames and the PermissionRequest decision body (§5 Wrapper channel patterns; architecture §Standard Contracts, which are arch's to cite).
- tests ↔ obs: the `permission` dialog event line on `events.ndjson` (`{"v":1,"ts","instance","kind","source","data"}`, §5 On-disk) is the verdict source. The test home sits under `target/e2e-home/` (§5 Setup / teardown, §7 Test data lifecycle), so obs-plan §9's G2 zero-panics, G4 schema check and secret scan cover the new case.
- tests ↔ security: the no-blanket-approval rule (§11 Universal) binds to the security plan's rule that a non-`null` dialog decision needs a verify stamp. The sixth dated exception (`viola answer`, `hook.dialog`) is exercised, not widened.
- tests ↔ verification matrix: the case is the owed `v1-30` end-to-end wake witness for the `permission` kind (§6 Path 3 Surfaces note). When it lands, the Surfaces notes of §6 Path 3 and Path 4 ("unit / insta only", "owed to Permission end to end") become stale and are a wrap amendment.
- tests ↔ rules: F1's designed-floor exception binds to §3 Bootstrap phases `test-runner-install`'s per-test-kill rationale and to §10 Zero-flakiness. The rule text and the nextest profile must agree.

## Acceptance criteria contributions
- `tests/cli_answer.rs` gains a `permission` case. It replays `PermissionRequest.permission-1.json` through the fake agent on a stamped home with a `driver` wheel and checks four things: the `permission` event is logged exactly once with a `dialog_id`; a parked `viola wait --after C` returns it; `viola answer <id>` with `{"behavior":"allow"}`, and in a second run `{"behavior":"deny","message":"no"}`, exits 0; the hook receipt reads `exit_code:0`, `stderr_len:0`, with stdout matching the PermissionRequest allow / deny-with-`message` insta snapshot (`.dialog_id` redacted). It passes on `windows-2025`, `macos-latest` and `ubuntu-latest` (per test-plan §6 Path 4; §9 Matrix builds).
- `scripts/agent-run.sh run --integration` is green locally. The CI per-OS `test` job's `gate --require coverage,doctest,playwright` holds lines ≥ 85%, functions ≥ 95% and regions ≥ 80% with `retries = 0` (per test-plan §10 Coverage thresholds and Zero-flakiness budget).
- Any new or edited `fixtures/fake-scripts/*.json` validates against `schemas/fake-script.v1.json` and passes the `contract_fixture_hygiene` walk. Its text is synthetic only (per test-plan §7 Fixture hygiene).
- If F1 adds the planted-hang control, a deliberately hanging test under the `[profile.ci]` verify-driven override is killed at that override's bound and reported as a failure, while a control test passes (per test-plan §3 Bootstrap phases `test-runner-install`; §10 Zero-flakiness budget).
