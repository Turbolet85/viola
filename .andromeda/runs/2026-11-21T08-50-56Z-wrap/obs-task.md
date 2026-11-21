@task_prose part=session
The session landed the identity floor as the 28th and last relied ledger row, identity-floor, worded every CLAUDE name the CLI adds to a hook is a known name; relied rows moved 27 to 28 (step lines [NN/28], summary 28 pass 0 fail).
The identity floor grew from eleven to thirteen names, adding CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS; the strip of every viola child removes all thirteen.
A new list of four non-identity hook names (CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE, CLAUDE_PLUGIN_DATA) is read only by the identity row, never by the strip.
The hidden hook capture arm, on SessionStart alone, now also writes an owner-only (0600) env-names file holding CLAUDE names kept, every other name counted, never a value; viola verify reads it, writes the identity-floor step line and an env_probe block (read, added, unknown, other_count, unshaped) into its stamp, and under --record writes a names-only Env.recorded fixture.
The fake agent replays the recorded env names to every SessionStart hook child; the live rig gained the HF, HF-2, HG labels and identity-floor and env-unknown summary lines; the fixture set for claude 2.1.328 was re-recorded whole.
Retired nothing committed: the cross-session prefix row, its sender and receiver and the verify-sender spawn subject were cut before commit and leave for a later entry; no IPC method, event kind, diag-line field, config key or VIOLA env read was added.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied rows, the stamp merge and its env_probe block, the env-names capture document and reading, the identity-floor check and the env fixture scrub fault; edited
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the CLAUDE env strip plan of every viola child, the identity floor name list and the non-identity hook name list; edited
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit tests for the env capture document and env file name; edited
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's unit tests of each relied row's check, now the identity-floor pass and fail table; edited
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: the ledger's unit tests over verify runs, row counts moved to 28; edited
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's unit tests of the stamp merge and verified reading, now with the env block and 27-row stamps unverified; edited
@task_file n=7 path=src/cmd/hook.rs
what: the viola hook command and its hidden --capture arms, which now file the env-names document beside a SessionStart payload owner-only; edited
@task_file n=8 path=src/cmd/verify.rs
what: viola verify: the probe runs, the step and summary lines, the stamp write, the --record fixture writer and its scrub; edited
@task_file n=9 path=src/run/auto_verify.rs
what: the start's automatic verify of a new CLI version, its stamp merge callers; edited
@task_file n=10 path=src/run/version_gate.rs
what: the run start's version gate reading the ledger stamp for cli_verified; edited
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent that replays recorded hook payloads, now also the recorded env names to SessionStart hook children; edited
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's mode tests, now the SessionStart env replay case; edited
@task_file n=13 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option tests over the recorded fixture set; edited
@task_file n=14 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness runner and its live verify pass literal of step lines; edited
@task_file n=15 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the secret scan's critical canary list over test outputs, four env canaries added; edited
@task_file n=16 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its labels, kept stamps, floor rows and summary lines; edited
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests, pass-count moved to 28/28; edited
@task_file n=18 path=fixtures/claude/2.1.328/Env.recorded-1.json
what: the new names-only recorded env fixture of the claude 2.1.328 set; added
@task_file n=19 path=fixtures/claude/2.1.328
what: the recorded claude 2.1.328 hook payload fixture set, re-recorded whole (new session ids); replaced
@task_file n=20 path=tests/security_negatives_env_capture.rs
what: the security negative tests that the capture arm writes env names and no value, and no kept name or user word reaches a stamp or record; added
@task_file n=21 path=tests/tui_env_strip.rs
what: the strip oracle test listing the stripped CLAUDE names, now thirteen floor names and one unknown; edited
@task_file n=22 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene contract test, now with the env fixture class check; edited
@task_file n=23 path=tests/contract_ledger_probes.rs
what: the contract test that every stamped fixture set passes the ledger probes at 28 rows; edited
@task_file n=24 path=tests/contract_fake_agent_drift.rs
what: the contract test that the fake agent matches the verify fixtures; edited
@task_file n=25 path=tests/cli_verify.rs
what: viola verify integration tests, now the identity-floor pass and fail cases; edited
@task_file n=26 path=tests/cli_verify/runs.rs
what: viola verify integration tests over probe runs and capture files; edited
@task_file n=27 path=tests/cli_verify/stamps.rs
what: viola verify integration tests over the stamp contents; edited
@task_file n=28 path=tests/support/verify.rs
what: the integration support for verify homes, the env names set writer; edited
@task_file n=29 path=tests/support/start.rs
what: the integration support for starts, the pinned fixture session id; edited
@task_file n=30 path=tests/chaos_revive.rs
what: the revive chaos tests, the pinned fixture session id; edited
@task_file n=31 path=tests/cli_send.rs
what: the send integration tests, the pinned clear session id; edited
@task_file n=32 path=Cargo.toml
what: the workspace manifest, test target entries; edited
@task_file n=33 path=.config/nextest.toml
what: the nextest configuration of test groups; edited
@task_item n=1 part=touches
decision/identity-floor-always-stripped: the floor it names is now thirteen names, CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS added to the eleven; any count or list in the row is stale, and the row must name the thirteen by name.
@task_item n=2 part=touches
decision/env-names-only: the capture arm now reads its own process's env names and verify stamps them; names only, never values, holds; the row owes the four non-identity hook names list as built.
@task_item n=3 part=touches
decision/claude-env-strip: the strip of every viola child (run, verify, the agents read) now removes thirteen floor names; a kept name on the floor is stripped all the same.
@task_item n=4 part=touches
contract/claude-child-process-start/env-stripped-known, env-stripped-count, env-kept and env-fields: the known stripped set grew by the two floor names, so what the claude-child process-start line can report as known stripped moved.
@task_item n=5 part=touches
decision/capture-arms-uninstrumented, decision/hidden-capture-arms-uninstrumented and decision/hook-capture-arms-unlogged: the capture arm gained a second file write (env names on SessionStart) and still writes no log line.
@task_item n=6 part=touches
contract/hook-stdout/capture-arms: the capture arm's behaviour grew by the env-names file; its stdout is unchanged.
@task_item n=7 part=touches
decision/r8-strip-logged-as-names-only, decision/stripped-claude-values-never-logged, decision/stripped-claude-env-never-logged, decision/stripped-env-never-logged and decision/claude-env-and-cookie-never-logged: env names now reach a stamp and a fixture, never a value; kept names are removed before stamp and record.
@task_item n=8 part=touches
decision/ledger-contents-never-logged: the ledger stamp now holds a measured env_probe block of CLAUDE names; still never logged.
@task_item n=9 part=touches
decision/unreadable-ledger-is-unverified and decision/unreadable-ledger-means-unverified: a stamp of the 27 earlier rows now reads unverified; only a 28-row stamp reads verified.
@task_item n=10 part=touches
contract/verify-stdout/step-line and contract/verify-stdout/summary-line: step lines read [NN/28] with a new identity-floor line; the summary reads 28 pass 0 fail.
@task_item n=11 part=touches
decision/verify-lines-redacted and decision/verify-lines-carry-no-content: the new identity-floor step line and env_probe carry names only and no value.
@task_item n=12 part=touches
decision/verify-sole-stamps-writer: verify remains the only stamps writer and under --record now also writes the names-only env fixture.
@task_item n=13 part=touches
contract/run-log-parse-rejected/ledger-stamps: the ledger stamp the run parses gained the env_probe block; an off-shape env document reads unread.
@task_item n=14 part=touches
decision/fixtures-not-telemetry, decision/fixtures-not-a-telemetry-source and decision/claude-fixtures-scrub-required: a new env fixture class, names only, passes the record scrub and an env-names fault check refusing a name carrying the user word.
@task_item n=15 part=touches
decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: the fake agent now replays recorded env names so CI verify reads the identity row without the real CLI; the live reading is the rig's HG start only.
@task_item n=16 part=touches
decision/secret-scan-scope and decision/content-canary-never-home-level: the secret scan's critical canary list grew 12 to 16 with the kept, other, notes and launched env canaries.
@task_item n=17 part=touches
decision/no-endpoint-path-or-env-in-spans and decision/no-paths-in-run-spans: the strip and verify read more env names; no env value reaches a span.
@task_fact n=1 slug=4-span-trace-coverage
fact: Four CLAUDE names the CLI adds to a hook are known and not identity, CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE and CLAUDE_PLUGIN_DATA; they are read only by the identity-floor row and never stripped by that list.
@task_fact n=2 slug=4-span-trace-coverage
fact: On SessionStart alone the hidden capture arm writes an owner-only env-names file of its own process, CLAUDE names kept, every other name only counted, no value, and writes no log line for it.
@task_fact n=3 slug=4-span-trace-coverage
fact: viola verify's identity-floor row reads the first SessionStart env-names file and stamps env_probe (read, added, unknown, other_count, unshaped) with names only, kept names removed before the stamp and the record; an unknown CLAUDE name fails the row.
@task_fact n=4 slug=8-pii-scrubbing-compliance
fact: Recorded env fixtures are a names-only fixture class, passed through the record scrub and an env-names fault check; an env name carrying the user word is refused and nothing is recorded.
@task_item n=1 part=untouched
The diagnostic line schema and its event, subject and detail enums: no event kind, spawn subject, field or detail word was added; the cut verify-sender subject was never committed.
@task_item n=2 part=untouched
Channel, send, dialog, wait, wheel and link log contracts, spans and corr propagation: no IPC method or line changed.
@task_item n=3 part=untouched
Panic capture, error reporting and exit detail codes by role.
@task_item n=4 part=untouched
Metrics, SLO budgets, heartbeat liveness and hook performance budgets.
@task_item n=5 part=untouched
Logging stack, log file locations, service identity and config.json keys: no key added, no VIOLA env read added.
@task_item n=6 part=untouched
Absences of exporters, network egress, OTel, receivers and hosted deployment.
