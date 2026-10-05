# tests extract

## Relevance
relevant — the chunk grows the ledger and the verify output that every stamped test home, the contract suite and the harness `local-live` suite key on. It also lands two owed §6 witnesses (Path 2 `local`, Path 4 / Path 3 `permission`) and adds a new fixture class (recorded screen text).

## Constraints
- Tier Comprehensive (per test-plan §1 Test Scope Summary). The multi-version-compat trigger requires every ledger row to have a `viola verify` probe with a post-condition, checked by a local run's exit code and its `stamped … pass … fail` line. The contract-test-against-sandbox trigger says the relayed dialog captures stay relayed until `:84`'s own re-probe supersedes them (per §1, trigger rows multi-version-compat / contract-test-against-sandbox). This chunk is that re-probe: the `fixtures/claude/<ver>/` dialog set and its `RELAYED.md` provenance move to verify-recorded (per §2 Contract row; §7 Recorded hook payloads).
- Only `viola verify` may produce stamps. In CI it runs against the fake agent (`stamped_home`, harness `boot` step 4). Boot requires exit 0 and a last line matching `^stamped \S+  \d+ pass  0 fail$` (per §7 Seed strategies `ledger/stamps.json`; §3 `boot`). Every new row (input-box signature, modal-absent, quiet-period / max-wait, local-command, S3/S7/S8/dialog-concurrency, long-paste / tag-escape / harness-prefix, R8 identity floor) must therefore pass against the fake agent's replay of the recorded fixtures. If it does not, every stamped home in CI breaks. Whether the fake agent today has a mode that verify's new PTY probe can drive, and that replays a recorded screen, is research's question. The fake-agent rule "no shape is invented before a recorded fixture" applies (per §7 Fake agent, Modes).
- Several suites hold literal row counts that grow with `LedgerRow::ALL`, and the expected values stay literals in each test (per §11 Unit):
  - `tests/cli_verify.rs` pins the six `[NN/06]` step lines (per §5 CLI);
  - `contract_ledger_probes.rs` lists every expected row id as a literal, with `<n>` equal to the literal count (per §5 CLI);
  - the harness `local-live` suite checks "each of the six literal row ids" (per §3 `run`, `--local-live`).
- The real CLI runs only through `agent-run run --local-live`. It refuses `live-in-ci` (exit 2) when `CI` is set, runs one `viola verify` per invocation, and goes through the `run_with` runner seam, so the harness's own tests use a stand-in runner (per §3 `run`; §6 skipped-as-untestable list). `gate --require` never takes `local-live` (per §3 `gate`). The live 2.1.288 stamp and any `--record` refresh are local operator runs, never a CI step.
- Fixture hygiene (per §7 Fixture hygiene):
  - the recorder rewrites home paths to `~` and usernames to `<user>` before it writes;
  - fixture prompts are synthetic probe text only;
  - the run-time walk in `tests/contract_fixture_hygiene.rs` fails on absolute paths and non-placeholder usernames, and checks the file against its schema;
  - a planted input proves each violation class red.

  Recorded screen text is a new fixture class, so it needs the same walk and a planted-red case. Whether the walk and `schemas/claude-fixture.v1.json` cover a non-JSON or new-variant fixture today is research's question.
- No timer may decide a test verdict:
  - zero flakes, `retries = 0` (per §10 Zero-flakiness budget);
  - no test-owned sleep or elapsed-time verdict (per §11 Universal).

  The probe's quiet-period / max-wait waits are product deadlines. An E2E may block on one only when two things hold: the verdict is the product's exit code or status field, and the same deadline is clock-driven in a §4 unit case through `viola_core::Clock` / mock_instant (per §8 Time mocking).
- Coverage is ≥ 85 % lines, ≥ 95 % functions and ≥ 80 % regions, per OS (per §10 Coverage thresholds). CI never runs the real CLI, so the PTY probe path has to be reached against the fake agent to count. Mutation runs at the epoch boundary, not per chunk (per §10 Mutation gate).

## Patterns to follow
- The fixture chain `home → fake_agent_path → stamped_home → booted_wrapper`, in which `stamped_home` runs `viola verify` against the fake agent (per §3 `run`, Cleanup / fixture chain; §7 Seed strategies).
- Gated fake-agent scripts: `fixtures/fake-scripts/<scenario>.json` with `--control <home>/fake/<name>.control` lines. The permission end-to-end case extends `path4.json` step 2 with a recorded ordinary-tool `PermissionRequest.<variant>.json` (per §7 Fake agent; §6 Path 4 Steps 1).
- Drift guard: `contract_fake_agent_drift` compares each hook's receipt `stdin_hex` byte for byte with the recorded `<Event>.<variant>.json`. New recorded variants (SessionStart `clear`, the dialog bodies, harness-prefix prompts) join this comparison rather than becoming snapshots (per §2 Contract row; §6 Non-path suites, Contract suite).
- Screen signatures are tested only as the readiness verdict (ready / `input-not-ready`) over fed vt100 bytes, never by content (per §4, the screen-signature bullet). The R8 identity floor is checked against a literal 11-name copy written in the test (per §4, R8 strip bullet; §11 Unit). The harness-prefix normalisation keeps its M2 prefix table (per §4, `prompt-submitted` normalisation bullet).
- Decision bodies are insta check-mode snapshots per `<cli-version>`, regenerated only by the local verify fixture refresh and then committed (per §6 Path 4 Verification; §7 Decision bodies).

## Anti-patterns to avoid
- Never run the real `claude`, or `viola verify` against it, in CI. Never treat a green fake-agent run as proof of real-CLI behaviour. Never treat a Linux-only local run as proof for Windows or macOS (per §11 Test Strategy). The Windows-only live items belong to `:86`.
- Never import `LedgerRow::ALL`, `IDENTITY_FLOOR` or the refusal order as a test oracle. Never hand-write `ledger/stamps.json` (per §11 Unit; §11 Test Data).
- Never parse the rendered child screen for a verdict, and never synchronise with `sleep(N)` (per §11 E2E). Never accept interactive snapshot review as a gate step (per §11 Universal). Never write a test that passes when a decision body is emitted without a verify stamp and a `driver` wheel (per §11 Universal; §6 Path 4 step 6).

## Contract bindings
- tests §3 `run` `--local-live` / §3 `boot` step 4 ↔ arch CLI contract for `viola verify`'s human lines: the `[NN/<total>]` step lines and `stamped <ver>  <n> pass  <m> fail`. The harness parses these lines, so a format or row-count change moves the harness literal list, `cli_verify.rs` and `contract_ledger_probes.rs` together.
- tests §6 Non-path suites, the canary and secret scan ↔ obs-plan §8 and the security NEVER-log floor. Recorded screen text, and any verify-probe diagnostics, must hold no path, username, token or stripped `CLAUDE*` value. Content is allowed only in `instances/<name>/diagnostics/detail-*`.
- tests §3 `schema-check` (`diag-line.v1.json`) ↔ obs-plan §6 subjects. A new verify-probe `subject`, or a redefined `verify-probe`, changes the schema that the harness `schema-check` validates. That change needs the obs Decisions Log entry the scope names.

## Acceptance criteria contributions
- `contract_ledger_probes.rs` lists every ledger row id, new ones included, as a test literal exactly once per `fixtures/claude/<ver>/` set against the fake agent, with `<n>` in `stamped <ver>  <n> pass  0 fail` equal to the literal count. `tests/cli_verify.rs`'s step-counter literals move to the new total. `stamped_home` / `boot` still end `0 fail` on all three CI OSes (per test-plan §5 CLI; §3 `boot`; §11 Unit).
- On the dev host, `scripts/agent-run.sh run --local-live` reports suite `local-live` passed 1. Every literal row id, new ones included, names exactly one step line ending `  pass`, and the last line reads `stamped 2.1.288  <n> pass  0 fail`. The same command with `CI` set exits 2 `live-in-ci` before any spawn (per §3 `run`, `--local-live`; §11 Test Strategy).
- Path 2 `local`: a ledger-listed local command now exits 0 with `{confirmed:false, detail:"unconfirmable", cursor}`. The `send_window_local_command_is_not_presumed_delivered` pin is rewritten to the new outcome, never deleted, and `mute` stays exit 13 `no-prompt-submitted` (per §6 Path 2 Verification signal).
- Path 4 `permission` lands end to end in `tests/cli_answer.rs` from a recorded ordinary-tool PermissionRequest. It covers `allow` and `deny` + `message`, with the bodies insta-pinned per `<cli-version>`. Path 3's `permission` wake witness lands end to end too, and the step 6 human-wheel negative (empty hook stdout, `answer` exit 10) still holds (per §6 Path 4; §6 Path 3 Surfaces; §11 Universal).
