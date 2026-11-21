@task_prose part=session
The session landed the identity floor as the 28th and last relied ledger row of viola verify, id identity-floor, words: every CLAUDE name the CLI adds to a hook is a known name; relied rows moved 27 to 28, so step lines read [NN/28] and the summary reads 28 pass 0 fail.
The identity floor grew from 11 to 13 names (CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS added); a new list of 4 non-identity hook names (CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE, CLAUDE_PLUGIN_DATA) is read only by the identity check.
The hidden capture arm viola hook <event> --capture now also writes, on SessionStart alone, an owner-only env-names file: CLAUDE names kept, every other name counted, no value; it prints no line.
viola verify reads that file, adds the identity-floor step line and an env_probe block to its stamp; viola verify --record writes a new names-only fixture class Env.recorded-1.json; the record scrub refuses an env name carrying the user word, named as the file and username.
The cross-session prefix row, its sender and the harness prefix were written by a stopped run and cut whole; none was ever committed, so nothing is retired.
Measured live once on claude 2.1.328 under a scratch home: identity-floor pass, 28/28; CI green 7/7. No exit code, verb, flag, IPC method, env var or log format was added.
@task_file n=1 path=src/cmd/verify.rs
what: the viola verify verb: its step lines, summary, stamp, --record and the recorded-fixture scrub refusal
@task_file n=2 path=src/cmd/hook.rs
what: the hidden viola hook verb and its capture arm, which records hook payloads for verify
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows of viola verify, their ids and words, the stamp merge and the env-names reading
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: the claude adapter's identity floor and the env strip of every viola child, and the non-identity hook names list
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the ledger rows and the env capture document
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit tests of each ledger row's check, including the identity-floor pass and fail table
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit tests of the ledger's verify runs
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the verify stamp, its merge and its env block
@task_file n=9 path=src/run/version_gate.rs
what: the CLI version gate of a start that reads the verify stamp
@task_file n=10 path=src/run/auto_verify.rs
what: the start's own automatic verify of a new CLI version
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent that replays recorded hook fixtures, now handing recorded env names to SessionStart hooks
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: tests of the fake agent's replay modes
@task_file n=13 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: tests of the fake agent's options over the recorded fixture set
@task_file n=14 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only standing live rig, its kept stamps, labels and summary lines
@task_file n=15 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness runner and its expected live verify pass lines
@task_file n=16 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the harness secret scan and its critical canary list
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests and their pass-count literals
@task_file n=18 path=fixtures/claude/2.1.328/Env.recorded-1.json
what: the new recorded env-names fixture of the claude 2.1.328 set, names only
@task_file n=19 path=fixtures/claude/2.1.328/SessionStart.default.json
what: a recorded hook payload fixture of the claude 2.1.328 set, re-recorded whole with the set
@task_file n=20 path=tests/cli_verify.rs
what: integration tests of viola verify output, including the identity-floor pass and fail
@task_file n=21 path=tests/cli_verify/runs.rs
what: integration tests of viola verify runs and recorded files
@task_file n=22 path=tests/cli_verify/stamps.rs
what: integration tests of the verify stamp
@task_file n=23 path=tests/contract_fixture_hygiene.rs
what: the recorded-fixture hygiene contract test, now with an env fixture class
@task_file n=24 path=tests/contract_ledger_probes.rs
what: the e2e contract test of the ledger probes over every stamped set
@task_file n=25 path=tests/contract_fake_agent_drift.rs
what: the drift check between the fake agent and the verify fixtures
@task_file n=26 path=tests/security_negatives_env_capture.rs
what: security negative tests of the capture arm's env-names file and the record scrub
@task_file n=27 path=tests/tui_env_strip.rs
what: the strip oracle test of CLAUDE env names removed from a child
@task_file n=28 path=tests/support/verify.rs
what: test support for verify, writing an env-names fixture set
@task_file n=29 path=tests/support/start.rs
what: test support for starts, pinning the recorded fixture session id
@task_file n=30 path=tests/chaos_revive.rs
what: chaos tests of revive, pinning the recorded fixture session id
@task_file n=31 path=tests/cli_send.rs
what: integration tests of viola send, pinning the recorded clear session id
@task_file n=32 path=Cargo.toml
what: the workspace manifest and its test targets
@task_file n=33 path=.config/nextest.toml
what: the nextest test runner configuration
@task_item n=1 part=touches
contract/viola-verify-output/step-line: the step line counter now reads [NN/28], and a new last step line names the identity-floor row.
@task_item n=2 part=touches
contract/viola-verify-output/summary and contract/viola-verify/summary: the all-pass summary now reads 28 pass 0 fail, where a cell holds the relied-row count or the pass line.
@task_item n=3 part=touches
contract/viola-verify-output/failing-row: identity-floor is a new row that can fail, on an unknown CLAUDE name added to a hook or an unread env file.
@task_item n=4 part=touches
contract/viola-verify-output/record: --record now also writes the names-only env fixture Env.recorded-1.json beside the hook payload fixtures.
@task_item n=5 part=touches
contract/viola-verify-refusals/fixture-not-clean: the record scrub now also refuses an env name carrying the user word, naming the file and username.
@task_item n=6 part=touches
contract/viola-verify-output/exit-0 and contract/viola-verify-output/exit-1: the exit depends now on 28 rows, the identity floor among them.
@task_item n=7 part=touches
contract/viola-verify-output/help-paragraph and contract/viola-verify-output/usage: where they list or count the relied rows, the identity-floor row is the 28th; no flag was added.
@task_item n=8 part=touches
contract/viola-verify-output/plain-ascii and contract/viola-verify-output/stop: the new step line follows the same plain form and stop rule.
@task_item n=9 part=touches
decision/mcp-verb-hidden and contract/cli-verbs/help: the hidden viola hook verb gained a write in its capture arm but stays hidden and prints no line; a rule over hidden verbs covers it.
@task_item n=10 part=touches
contract/cli-verbs/run and contract/viola-run/start-output: the env strip of every viola child now removes 13 identity-floor names, where a row names the floor or its count.
@task_item n=11 part=touches
decision/human-output-is-contract: the verify step and summary lines moved, so a rule binding human output as contract rules over them.
@task_fact n=1 slug=surface-cli
fact: viola verify's last relied row is identity-floor, step line [28/28], passing when every CLAUDE name the CLI adds to a hook is on the identity floor or the non-identity hook names.
@task_fact n=2 slug=surface-cli
fact: viola verify --record writes a names-only env fixture beside the hook payloads, and refuses it as not clean when an env name carries the user word, naming the file and username.
@task_fact n=3 slug=surface-cli
fact: the hidden viola hook --capture arm writes an owner-only env-names file on SessionStart alone and prints no line.
@task_item n=1 part=untouched
The viola list board, BAY line, its columns, colour, truncation and --json document: no board output moved.
@task_item n=2 part=untouched
viola send, wait, last and answer output, the readback mirror and its vocabulary: no send or wait line changed.
@task_item n=3 part=untouched
viola revive output, refusals and --json codes, and the run and revive start refusals: no start refusal added.
@task_item n=4 part=untouched
CLI exit codes 10 to 21, the --json form, the refusal line form and the hint rules: no code, cause or hint added.
@task_item n=5 part=untouched
viola link and unlink output, the wheel verbs pause and release, and the frame verb exit-21 hints and details.
@task_item n=6 part=untouched
viola mcp tools and results, viola plugin install output: no tool or plugin output changed.
@task_item n=7 part=untouched
The CLI token rendering rows, styling, spinner, banner, footer and progress-glyph decisions, and the web-spa absence.
