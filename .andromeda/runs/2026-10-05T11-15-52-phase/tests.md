# tests extract

## Relevance
relevant — the chunk grows the capability ledger (the contract suite's literal row set), replaces the relayed dialog-tier fixtures that the contract/replay tests read, and owes test-plan §6 Path 4's `permission` end-to-end case and Path 3's `permission` wake witness.

## Constraints
- Every ledger row needs a `viola verify` probe with a post-condition, judged by exit code plus the `stamped … pass … fail` line (per test-plan §1 Coverage scope, multi-version-compat trigger). A dialog re-probe that fails to raise its dialog must therefore count as a failed row, never as a pass or a skip. The model's nondeterminism is no exception, and §10 Zero-flakiness bans retry budgets.
- The real `claude` CLI never runs in CI. The live re-probe runs only on the operator's host through `agent-run run --local-live`, which must refuse with `live-in-ci` when `CI` is set (per test-plan §11 Test Strategy; per test-plan §3 `run`, `--local-live`). CI covers the new rows only by replaying them against the fake agent, and a green fake-agent run is never proof of real-CLI behaviour. A behaviour change needs a local `viola verify` fixture refresh plus the contract suite (per test-plan §11 Test Strategy; §2 Contract row).
- The contract-suite oracle for the ledger is the row ids as test literals, and the `<n>` in `stamped <ver>  <n> pass  0 fail` must equal the literal row count. Every `fixtures/claude/<ver>/` dir sits on exactly one list, stamped or drift-only (per test-plan §5 `contract_ledger_probes.rs`). The row literals in the plan's text all say ten:
  - `contract_ledger_probes.rs`;
  - `tests/cli_verify.rs`'s `[NN/10]` step lines (§5);
  - the harness's `LEDGER_ROWS` and its `row-missing` code (§3 `run`, `--local-live`).
  Whether they become fourteen here, or how many there are, depends on how many rows P4 lands.
- Dialog fixtures:
  - The recorded dialog payloads replace the relayed 2.1.287 captures. test-plan §2 Contract row, §1 contract-test-against-sandbox trigger and §7 Seed strategies name this entry's re-probe as what supersedes them, so those passages need a wrap amendment.
  - Recorded payloads go under `fixtures/claude/<cli-version>/<Event>.<variant>.json` and must pass the run-time hygiene walk: jsonschema `claude-fixture.v1.json`, no absolute path, username or email, with only the violation class reported (per test-plan §7 Fixture hygiene).
  - The recorder must rewrite home paths to `~` and usernames to `<user>` before it writes (§7 Fixture hygiene). Whether the `--record` scrub already covers dialog `tool_input` (plan text, paths) is research's question. The relayed set redacted `tool_input.plan` (§7 Seed strategies).
- insta check mode (`INSTA_UPDATE=no`) pins the decision bodies, with `.dialog_id` and `.ts` redacted. A snapshot may change only through the local `viola verify` fixture refresh, and is then committed; interactive review is never a gate step (per test-plan §4 insta / Dialog mapping S3/S7/S8; §7 Seed strategies, Decision bodies; §11 Universal).
- `ledger/stamps.json` in every test home comes only from running `viola verify` against the fake agent. No test or harness code writes it (per test-plan §7 Seed strategies; §11 Test Data). The single-writers negative keeps `verify` as its only writer (per test-plan §6 Non-path suites, Security control negatives).
- Coverage is gated per OS: lines ≥ 85 %, functions ≥ 95 %, regions ≥ 80 %. nextest `retries = 0`, and `#[ignore]` may not park a test (per test-plan §10 Coverage thresholds / Zero-flakiness budget). A new `cfg(windows)` path is proven only on its own runner (§11 Test Strategy).

## Patterns to follow
- `contract_ledger_probes.rs` runs `viola verify --home <tmp>` per stamped fixture set against the fake agent, with the row ids as literals (per test-plan §5 Integration Test Strategy). The same check is required under `--local-live`.
- `contract_fake_agent_drift` compares each hook's receipt `stdin_hex` byte for byte with the recorded fixture file and never through an insta snapshot. Its comparator names a planted order swap and a planted byte difference (per test-plan §6 Contract suite). A new dialog fixture set should fall under the same comparison.
- Scenario scripts:
  - They live in `fixtures/fake-scripts/<scenario>.json`, validated by `schemas/fake-script.v1.json`, with synthetic text only.
  - Gated steps are released through the `--control` file, never by sleeping.
  - Groups match on `tool_name` through `hook_commands(…, tool)`.
  (Per test-plan §7 Fake agent.) This is how Path 4's ordinary-tool `PermissionRequest.<variant>.json` step plugs into `path4.json`.
- `tests/cli_verify.rs` pins verify's human lines with literal assert_cmd asserts, covering:
  - the step lines and the last `stamped` line;
  - the named `unable: a recorded fixture is not clean: <file> <code>` refusal;
  - no run dir left behind.
  (Per test-plan §5.) A new re-probe run or refusal joins it the same way.
- The fixture-hygiene walker (`tests/contract_fixture_hygiene.rs` + `tests/support/hygiene.rs`) proves each violation class red with a planted input (per test-plan §7 Fixture hygiene). A new scrub class or fixture shape gets its own planted red.

## Anti-patterns to avoid
- NEVER import the product's ledger rows (`LedgerRow::ALL`) or refusal ordering as the test oracle; write the row ids, the count and the exit codes as literals (per test-plan §11 Unit).
- NEVER write a test that auto-approves a dialog, or one that passes when a decision body is emitted without a verify stamp and a `driver` wheel (per test-plan §11 Universal). Once R2's gap closes, "stamp" means a stamp in which the dialog rows passed.
- NEVER run the real CLI or `viola verify` against it in CI, and never synchronise with `sleep(N)`. Wait on an `events.ndjson` offset, `viola wait --after`, or the fake agent's control file and receipt (per test-plan §11 Test Strategy / E2E).

## Contract bindings
- tests §3 `run` `--local-live` ↔ the ledger's row set:
  - The harness's literal `LEDGER_ROWS` and its `row-missing` / `verify-exit-<n>` codes must track the rows this chunk adds.
  - `local-live`'s live firing itself is owed to "First live test and self-drive" (scope Boundaries). Whether this chunk moves the harness literal is P4's call.
- tests §7 Fixture hygiene ↔ security-plan's `--record` refusal (closed codes `home-path` · `absolute-path` · `username` · `email`, the content never named) and its NEVER-log floor. Recorded dialog `tool_input` can carry plan text and paths.
- tests §6 Non-path suites (Error sanitization and secret scan, canary) ↔ obs-plan §8 / §9:
  - Every dialog answer the tests feed embeds the fixed canary.
  - The canary may appear only in `events.ndjson` and `instances/<name>/diagnostics/detail-*.ndjson`.
  - The Schema conformance check (obs G4) covers any new verify or diagnostic lines.
- tests §6 Path 4 / Path 3 ↔ obs-plan §6 event catalog: each `question` / `permission` / `plan` dialog event is logged exactly once with a `dialog_id`, and `wait` returns it at once.

## Acceptance criteria contributions
- For each stamped fixture set, `viola verify --home <tmp>` against the fake agent lists each literal ledger row id exactly once, the four dialog-tier rows included, each with its post-condition result. It ends `stamped <ver>  <N> pass  0 fail`, where `<N>` is the literal row count. `tests/cli_verify.rs`'s step-counter literals move with it (per test-plan §5 `contract_ledger_probes.rs` / `tests/cli_verify.rs`).
- `tests/cli_answer.rs` gains the `permission` end-to-end case on all three CI OSes. It runs over a recorded ordinary-tool `PermissionRequest.<variant>.json` (`path4.json`, prompt 2):
  - the `permission` event is logged once with a `dialog_id`, and `wait` wakes on it (the Path 3 `permission` wake witness);
  - `{"behavior":"allow"}` and `{"behavior":"deny","message":"no"}` give PermissionRequest bodies pinned by insta per `<cli-version>`, with the hook at `exit_code:0` and `stderr_len:0`.
  (Per test-plan §6 Path 4 / Path 3.)
- The recorded dialog fixtures pass the hygiene walk (schema plus the no-absolute-path, username and email classes) and `contract_fake_agent_drift`'s byte-for-byte replay. Each relayed fixture is retired, or marked superseded in `RELAYED.md`, and every fixture dir stays on exactly one stamped or drift-only list (per test-plan §7 Fixture hygiene; §6 Contract suite; §5 `contract_ledger_probes.rs`).
- Two negatives:
  - No decision body is emitted unless the wheel is `driver` and the child version's stamp holds the passing dialog rows.
  - An `answer` on a home whose stamp lacks them exits 12 `unverified-cli`, and the hook stdout stays empty.
  (Per test-plan §6 Path 4 step 6 / Path 7; §11 Universal.) Whether the gate keys on all four dialog rows or on each kind's own row is P4's choice. The test asserts the chosen rule with literals.
