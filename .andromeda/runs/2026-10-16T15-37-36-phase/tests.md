# tests extract

## Relevance
relevant — the chunk removes a whole test leg (the `run --browser` arm, the `playwright` suite and gate value, the pre-push browser stage and Node pin, the CI browser and npm steps) and narrows several harness closed enums and documents that test-plan §3 and §9 define.

## Constraints
- Every harness closed enum this chunk narrows is a contract change, not a cleanup: the `run` `suite` set (`playwright` leaves, so `gate --require` takes the remaining values other than `local-live`), the `run` reason `browser-missing`, the `pre-push` `detail` `node` and its `linux{run,browser,gate}` member, and the `secret-scan` classes `token-query` / `gui-token`. A closed-enum change goes through the test-plan Decisions Log (the wrap's amendment, named in the plan as Expected amendments, never written by implement) (per test-plan §3 5-command implementation, Closed enums).
- `gate` must be run with `--require` set to exactly the suites the job ran, and a required suite absent from `run-summary.json` is a breach. Once the browser leg goes, the per-OS `test` job's list and the pre-push's `linux-tests` gate both drop `playwright`. A list still naming it would fail every run, and a list naming fewer suites than the job ran would let a suite pass ungated (per test-plan §3 `gate`).
- `pre-push` stays the native Linux gate: every child runs under `/usr/bin/env -i` with only constant PATH components, stages fail fast in order (`host` · `tools` · `linux-tests`), and the printed document holds codes, counts and repo-relative names only. Removing the Node tool check, the `run --browser` stage and the `<home>/.local/viola-node/bin` PATH element must keep that discipline, so the result is a narrowing, never a new inherited value (per test-plan §3 `pre-push`).
- CI workflow hygiene still holds on every edited workflow (`ci.yml`, `nightly.yml`, `windows-mutants.yml`): `permissions: {}` at the top, `contents: read` per job, every `uses:` SHA-pinned, zizmor as the assertion, and obs-plan §9's G2 → G4 → capture → scan → uploads → gate step order unchanged in the per-OS `test` job (per test-plan §9 CI Integration).
- Every Supply-chain stage pass condition is an exact last line. If the `axum-http2` ban and its probe go, `scripts/deny-probes.sh`'s count changes, and the CI step, the script's own last line and the master text must agree on the new N (per test-plan §9 Pipeline structure, Supply-chain row and Build failure conditions).
- The per-OS coverage floors (`--fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`) still gate every OS. The removed product code in `src/obs.rs` and `crates/viola-core/src/obs.rs` is covered code, and `crates/viola-e2e` is excluded by `COVERAGE_IGNORE`. Whether removing those obs paths and their tests moves any floor is research's question (per test-plan §10 Coverage thresholds).
- The `fixed-port` nextest group is the one sanctioned serialisation. Removing it must also remove the plan's references to it as the only shared resource and as the Integration row's `max-threads = 1` exception. Whether any test is still a member of the group is research's question (per test-plan §5 Setup / teardown lifecycle).

## Patterns to follow
- Harness tool calls go through the `Runner` seam (`run_with`), with fixed argv and stand-in runners in the harness's own tests. The narrowed `run` and `pre-push` keep their unit tests on that seam and never spawn real Node or cargo tools (per test-plan §3 5-command implementation, `run` step 3 and `--local-live`).
- A usage refusal is a typed document plus an exit code: `gate --require` with a value outside the closed set is usage `unknown-suite` (exit 2). After this chunk the same path refuses `playwright` (per test-plan §3 `gate`).
- Contract tests that pin a list to its single source keep it in step. `tests/contract_windows_mutation_scope.rs` pins `windows-mutants.yml`'s tool pins against the `test` job, and `crates/viola-e2e/tests/scan_patterns.rs` keeps the secret-scan canary table single-sourced. Both are the checks to keep green when the mutants file list and the scan classes narrow (per test-plan §9 CI Integration, tool installs paragraph; §3 `secret-scan`).
- Unit oracles are literals. A narrowed vocabulary assertion (for example the obs process-name list without `ui`) is written as the expected literal in the test and never imported from the product enum (per test-plan §11 Unit).

## Anti-patterns to avoid
- NEVER park a removed or broken test behind `#[ignore]` or `test.skip`. A browser- or `ui`-bound test leaves with its code (per test-plan §10 Zero-flakiness budget).
- NEVER widen the coverage `--ignore-filename-regex` to absorb a coverage drop from removed tests (per test-plan §11 CI).
- NEVER skip a quality gate to reach "no Node on the host". The gates must be green with the leg removed, not with steps skipped or made conditional (per test-plan §11 Quality).

## Contract bindings
- tests §3 Log format ↔ obs-plan §3 Log format / the obs event catalogue and `schemas/diag-line.v1.json` / `diag-detail.v1.json`: the reserved `ui` process value (`ui-<port>` file) and the `http-request`, `liveness-changed`, `sse-opened` and `sse-closed` events are marked "removed with the code by 'Remove the browser leg'". The obs side and the harness `logs --process run|hook|mcp|ui|cli` filter narrow in one pass. The Log format's harness-side a11y rows (`e2e-web/test-results/a11y/`, `e2e-web/schemas/a11y-row.v1.json`) lose their home with `e2e-web/`.
- tests §3 `secret-scan` ↔ obs-plan §9 (step 3) and security-plan Secret Management: the `token-query` and `gui-token` classes.
- tests §3 Status endpoint shape ↔ the harness `status` document: the `null` `ui` and `api_sessions_equal_list` members. Also tests §3 `cleanup` ↔ its report: the `null` `port_free` and `url_file_removed` keys.
- tests §9 Supply-chain / pre-push ↔ security-plan Dependencies and CI (`.claude/rules/security.md`): the npm audit, the Node pin, `deny.toml`'s `axum` `http2` ban and the pre-push `env -i` PATH are stated on both sides.
- tests §9 Lint row ↔ a11y-plan D-A11Y-14: the a11y lint that was to run in the ubuntu leg after `npm ci --prefix e2e-web` loses its toolchain with `e2e-web/`.

## Acceptance criteria contributions
- `viola-harness gate --require playwright` exits 2 with usage `unknown-suite`, and the per-OS `test` job's last step and the pre-push's `linux-tests` stage gate exactly `coverage,doctest` (per test-plan §3 `gate`).
- `scripts/agent-run.sh pre-push` on the Linux dev host prints `stage:"linux-tests"`, `ok:true`, exit 0. Its `tools` stage runs no `node --version`, its document has no `browser` member, and no child's PATH holds a `.local/viola-node/bin` element (per test-plan §3 `pre-push`).
- `scripts/agent-run.sh run --coverage` stays green on all three CI OSes at the unchanged floors (85 lines / 95 functions / 80 regions), with `COVERAGE_IGNORE` unchanged (per test-plan §10 Coverage thresholds).
- The CI `supply-chain` job passes with no Node or npm step. `bash scripts/deny-probes.sh` ends `deny-probes: N/N banned, control clean`, where N is the count of the remaining bans (per test-plan §9 Pipeline structure, Supply-chain row).
