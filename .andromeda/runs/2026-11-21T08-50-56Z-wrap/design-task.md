@task_prose part=session
The session added the identity-floor relied ledger row, the 28th and last relied row, worded: every CLAUDE name the CLI adds to a hook is a known name.
The relied ledger rows moved 27 to 28: viola verify step lines now read [NN/28] and its summary 28 pass 0 fail; a 27-row stamp now reads unverified, a 28-row stamp verified.
The identity floor of CLAUDE names stripped from every viola child grew from 11 to 13 names (adds CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS); a new list of 4 non-identity hook names (CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE, CLAUDE_PLUGIN_DATA) is read only by the verify probe.
The hidden hook capture arm now also writes, on SessionStart alone, an owner-only env-names file (CLAUDE names kept, other names counted, no value); viola verify reads it, stamps an env_probe block, records a names-only Env fixture class, and refuses an env name carrying the user word as an unclean fixture.
The cross-session prefix row and its sender, written by an earlier stopped run, were cut whole and never committed; no IPC method, verb, flag, exit code, env var or config key was added.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger of CLI-behaviour rows and the verify stamp merge, now 28 rows with the identity-floor row, the env-names capture document and reading, and the env fixture scrub
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the Claude agent adapter's identity floor (13 names) stripped from every viola child and the four non-identity hook names
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the ledger's env capture document and env file name
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit tests of the ledger's per-row checks, now the identity-floor pass and fail cases
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit tests of the ledger's verify runs, moved to the 28-row count
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the verify stamp merge, now writing the env_probe block and reading 27-row stamps unverified
@task_file n=7 path=src/cmd/hook.rs
what: the hook verb, whose hidden capture arm now also files the env-names document on SessionStart, owner-only
@task_file n=8 path=src/cmd/verify.rs
what: the viola verify verb: its step lines and summary over 28 relied rows, the identity-floor step line, the env_probe stamp block, and the record scrub refusing a user-word env name
@task_file n=9 path=src/run/version_gate.rs
what: the run-start version gate that reads the verify stamp to decide verified or unverified CLI
@task_file n=10 path=src/run/auto_verify.rs
what: the automatic verify that viola run and revive start for a new CLI version, merging the stamp
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent, now handing the recorded env names to every SessionStart hook child
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: tests of the fake agent's modes, now the SessionStart env replay
@task_file n=13 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: tests of the fake agent's options over the recorded fixture set, its pinned session id moved
@task_file n=14 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig, its kept-stamp labels HF, HF-2, HG and its identity-floor and env-unknown summary lines, floor rows 24
@task_file n=15 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness runner and its expected live verify pass transcript at 28 rows
@task_file n=16 path=crates/viola-e2e/src/harness/secret_scan.rs
what: the harness secret scan and its critical canary list, grown by four env canaries
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests, pass-count moved to 28 of 28
@task_file n=18 path=fixtures/claude/2.1.328/Env.recorded-1.json
what: the new recorded env-names fixture of the claude 2.1.328 set, names only
@task_file n=19 path=fixtures/claude/2.1.328
what: the recorded hook-payload fixture set of claude 2.1.328, re-recorded whole by viola verify
@task_file n=20 path=tests/cli_verify.rs
what: integration tests of the viola verify verb, now the identity-floor pass and unknown-name fail
@task_file n=21 path=tests/cli_verify/runs.rs
what: integration tests of viola verify's runs and their recorded files, at 28 rows
@task_file n=22 path=tests/cli_verify/stamps.rs
what: integration tests of viola verify's stamp, now with the env_probe block
@task_file n=23 path=tests/security_negatives_env_capture.rs
what: security negative tests that the capture arm writes env names and no value, and that kept or user-word names reach no stamp or record
@task_file n=24 path=tests/tui_env_strip.rs
what: the child env strip test and its literal oracle of stripped names, now the thirteen floor names and one unknown
@task_file n=25 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene contract test, now with the env fixture class
@task_file n=26 path=tests/contract_ledger_probes.rs
what: the contract test reading every stamped fixture set at 28 pass 0 fail
@task_file n=27 path=tests/contract_fake_agent_drift.rs
what: the contract test that the fake agent matches the verify fixtures, at 28-row literals
@task_file n=28 path=tests/support/verify.rs
what: the test support for verify runs, its env-names list and env fixture writer
@task_file n=29 path=tests/support/start.rs
what: the test support for starting sessions, its pinned fixture session id moved
@task_file n=30 path=tests/chaos_revive.rs
what: the revive chaos tests, their pinned fixture session id moved
@task_file n=31 path=tests/cli_send.rs
what: the viola send integration tests, the pinned clear session id moved
@task_file n=32 path=Cargo.toml
what: the workspace manifest, its test target entries
@task_file n=33 path=.config/nextest.toml
what: the test runner configuration, its test groups
@task_item n=1 part=touches
The viola verify output step line: the relied rows now number 28, step lines read [NN/28], and a new identity-floor step line joins as the last row; check any row holding a step count or a sample [NN/27] line.
@task_item n=2 part=touches
The viola verify output summary and summary line: now 28 pass 0 fail; a row holding the earlier 27 total is stale.
@task_item n=3 part=touches
The viola verify fail word: the identity-floor row reads fail when the CLI adds an unknown CLAUDE name to a hook, or when the env-names file is absent or off-shape.
@task_item n=4 part=touches
CLI verification state verified and unverified-cli: a stamp of the earlier 27 rows now reads unverified; only a 28-row stamp reads verified.
@task_item n=5 part=touches
The viola run and revive automatic-verify line (no stamp, earlier red, verifying): its verify now measures 28 rows, and a 27-row stamp counts as unverified, through the version gate and the automatic verify.
@task_item n=6 part=touches
The viola verify refusals unclean fixture: the record scrub now also refuses an env name carrying the user word, naming the file and the word username.
@task_item n=7 part=touches
The hook verb (name hook, its exit code and stderr contract): its hidden capture arm now also writes an owner-only env-names file on SessionStart; it still prints nothing and adds no option.
@task_item n=8 part=touches
The verify verb name and its --help: no new flag; --record now also writes the env-names fixture; check any row listing what --record writes.
@task_item n=9 part=touches
Any row stating every relied row or a named relied row's role: the set gains the identity-floor row; a row that counts relied rows is stale.
@task_item n=10 part=touches
Any row stating the identity floor or the CLAUDE env strip of viola children by count (11 names): the floor is now 13 names, named.
@task_item n=11 part=touches
Indirect: rules over the verify transcript's ASCII-only, two-space, step-counter progress pattern govern the new identity-floor step line.
@task_item n=1 part=untouched
Brand identity, the flight-progress strip, the readback box and send readback rows: no strip, dialog, readback or send output changed.
@task_item n=2 part=untouched
Colour palette, typography, spacing, iconography and border radius rows: no colour, glyph, column or font rule moved.
@task_item n=3 part=untouched
The viola list board and list table rows, NAME truncation and width source: list output unchanged.
@task_item n=4 part=untouched
Exit codes, exit 21 causes, strict-modes refusals and wrapper fault detail rows: no exit code, cause or fault detail added.
@task_item n=5 part=untouched
viola run and revive start refusals, revive --json codes and revive output rows: no refusal or code added.
@task_item n=6 part=untouched
viola link, unlink, wait, last, pause, release, answer and plugin install output rows: those verbs unchanged.
@task_item n=7 part=untouched
Refusal hints, peer reply screening and log detail codes: no hint or code word added.
@task_item n=8 part=untouched
Anti-patterns and the self-validation protocol: no rule moved; the cut cross-session row brings no capability, name or output to any row.
