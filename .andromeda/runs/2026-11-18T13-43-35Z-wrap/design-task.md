@task_prose part=session
viola run and viola revive now write the Claude CLI's trust record for an untrusted spawn folder at a wrapped start, so the CLI's trust dialog no longer stands on the first screen and no key is typed into it.
The write happens after the version gate and before the endpoint is bound, only when the CLI is verified; it sets the one trust flag in the folder's entry and leaves every other key untouched.
No record is written for the user's home folder, a folder already trusted, a folder under a trusted parent, or an unverified CLI; revive keys the record by the session's recorded cwd.
The start's outcome is one of 16 fixed start words, logged as one new trust-record log line per start with no path, key or value; it never refuses the start and moves no exit code, hint, --json code or terminal byte.
The closed log event set grew from 15 to 16; the rule that run and revive write no trust record is retired, and verify's own trust record stands as before.
Proven on the fake agent and live on claude 2.1.325 only.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the Claude-shape trust-record reading: the CLI's folder walk, the flag-only edit, the exact read-back and the closed set of start words
@task_file n=2 path=src/trust_record.rs
what: viola's trust-record writer shared by verify and the run and revive start, with the home-folder rule and the read-backs
@task_file n=3 path=src/cmd/run.rs
what: the viola run and viola revive start sequence, now with a trust step after the version gate
@task_file n=4 path=src/cmd/verify.rs
what: the viola verify run that writes its own trust record; its config-file lookup moved out to the shared writer
@task_file n=5 path=crates/viola-core/src/obs.rs
what: the closed set of log events, now with the trust-record event
@task_file n=6 path=schemas/diag-line.v1.json
what: the log line schema, its event list and the trust-record line's detail words
@task_file n=7 path=schemas/diag-detail.v1.json
what: the log detail schema's event list
@task_file n=8 path=src/bin/viola-fake-agent.rs
what: the fake agent used by the tests, its trust reading now walking from its cwd to the repository root
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's own test cases for its trust reading
@task_file n=10 path=tests/support/home.rs
what: the test home helpers, now with a test-owned user home and untrusted-folder boot and revive wrappers
@task_file n=11 path=tests/support/cli.rs
what: the test helper that runs CLI verbs, now giving every run and revive a test-owned user home
@task_file n=12 path=tests/run_cli.rs
what: the viola run integration tests, now with the trust-record start cases and the moved log-line counts
@task_file n=13 path=tests/cli_revive.rs
what: the viola revive integration tests, now with the revive's trust record for its recorded cwd
@task_file n=14 path=tests/tui_passthrough.rs
what: the terminal passthrough tests that prove viola adds no bytes, now for a trust-writing start
@task_file n=15 path=tests/contract_diag_schema.rs
what: the check that real log lines validate against the log schema and the event list matches
@task_file n=16 path=tests/contract_bounded_readers.rs
what: the check that each bounded read named by a boundary still stands
@task_file n=17 path=tests/chaos_snapshot_replay.rs
what: the snapshot replay chaos tests, given a test-owned user home
@task_file n=18 path=tests/cli_controls_not_disableable.rs
what: the tests that the CLI's controls cannot be disabled, given a test-owned user home
@task_file n=19 path=tests/cli_fake_agent.rs
what: the fake-agent CLI tests, given a test-owned user home
@task_file n=20 path=tests/cli_version_gate.rs
what: the CLI version gate tests, given a test-owned user home
@task_file n=21 path=tests/cross_exit_causes.rs
what: the cross-process exit cause tests, given a test-owned user home
@task_file n=22 path=tests/cross_killed_test.rs
what: the killed-process tests, given a test-owned user home
@task_file n=23 path=tests/channel_error_bodies.rs
what: the channel error body tests, given a test-owned user home
@task_file n=24 path=crates/viola-e2e/src/harness/boot.rs
what: the end-to-end harness boot, now seeding a session user home
@task_file n=25 path=crates/viola-e2e/src/harness/supervise.rs
what: the end-to-end harness's supervised wrappers, now started under the session user home
@task_file n=26 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig on the real CLI, its two new trust start arms and the verify floor rows
@task_file n=27 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's scratch isolation: the sentinel config entry, a fresh folder and the start's trust word
@task_file n=28 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig command, its new live and rehearse arms
@task_file n=29 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests, now for the two trust start arms
@task_item n=1 part=touches
decision/run-prints-nothing-while-wrapped: the start now writes a trust record and logs one trust-record line, and still prints nothing on the terminal; the rule holds, with a new silent step under it.
@task_item n=2 part=touches
contract/viola-run-output/child-started: run and revive gain a trust step before the child starts; the output after the child starts is unchanged, and the CLI's trust dialog no longer shows first in an untrusted folder.
@task_item n=3 part=touches
contract/viola-run-start-refusals and contract/viola-revive-refusals rows: the trust step adds no refusal and never refuses a start; no refusal, its order or its message moves.
@task_item n=4 part=touches
contract/cli-exit-codes and the revive --json refusal code rows: no exit code and no --json code moves for the trust step; a failed or lost write still starts the child.
@task_item n=5 part=touches
contract/viola-verify-output/trust-record-names-nothing: verify's own trust record and its row stand unchanged; its config-file lookup is now shared with the run and revive start.
@task_item n=6 part=touches
contract/cli-verification/verified and contract/cli-verification/unverified-cli: the trust step runs only on a verified CLI; an unverified CLI reads no config file and writes nothing.
@task_item n=7 part=touches
contract/log-detail-codes rows: a new trust-record log line whose detail is one of 16 fixed start words; no path, key or value in it.
@task_item n=8 part=touches
decision/start-refusals-show-no-path-or-pid and decision/no-internals-in-errors: the new trust-record line names no folder path, config key or value, in line with these rules.
@task_item n=9 part=touches
decision/no-interactive-prompt-or-pager and contract/cli-callers rows: an untrusted folder no longer meets the CLI's trust dialog at a wrapped start, so no key is typed and no prompt is answered on the caller's behalf.
@task_item n=10 part=touches
decision/silent-catch-sites and contract/cli-catch-site/silent-verbs: the trust step's refusals and lost read-backs are silent on the terminal and reach the log only as a start word.
@task_item n=11 part=touches
name/command/run and name/command/revive: both verbs now write the CLI's trust record for their spawn folder; revive keys it by the recorded cwd.
@task_item n=12 part=touches
contract/viola-revive-output/after-child-start: revive's start now carries the same trust step for its recorded cwd; its output after the child starts is unchanged.
@task_item n=13 part=touches
decision/dialog-pending-cocked-strip and contract/flight-progress-strip/dialog: the CLI's trust dialog no longer stands at a wrapped start in an untrusted folder, so no dialog-pending state arises from it.
@task_fact n=1 slug=surface-cli
fact: viola run and viola revive, on a verified CLI, write the CLI's trust record for an untrusted spawn folder before the child starts, setting the trust flag alone, and print nothing for it; the step never refuses the start and moves no exit code, hint or --json code.
@task_fact n=2 slug=surface-cli
fact: the start writes no trust record for the user's home folder, a folder already trusted, a folder under a trusted parent, or an unverified CLI; revive keys its record by the session's recorded cwd.
@task_fact n=3 slug=surface
fact: the trust-record log line carries one of 16 fixed start words as its detail code, one line per run or revive start, with no path, key or value.
@task_item n=1 part=untouched
brand-identity strips, the readback box and the send readback rows: no strip, readback or send form changed.
@task_item n=2 part=untouched
color-palette, typography, spacing and iconography rows: no colour, glyph, separator, column or font rule moved; the trust step prints no byte.
@task_item n=3 part=untouched
the viola list board and table rows: the board, its columns, skip counts and refusals are unchanged.
@task_item n=4 part=untouched
viola link, unlink, wait, last, pause, release, answer and send output rows: none of these verbs changed.
@task_item n=5 part=untouched
the wrapper fault --json detail, peer reply screening and exit 21 cause rows: no IPC method, socket, endpoint or fault form changed.
@task_item n=6 part=untouched
viola --help verb groups and the hidden mcp and hook verb rows: no verb added, renamed or regrouped.
@task_item n=7 part=untouched
the web page absence, the rendered page absence and the self-validation protocol rows: no page or token surface came into being.
@task_item n=8 part=untouched
the CLI colour decision order and colour depth rows: no colour decision or depth rule moved.
