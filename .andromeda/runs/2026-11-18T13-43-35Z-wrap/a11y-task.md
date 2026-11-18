@task_prose part=session
viola run and viola revive, on a verified CLI, now write the CLI's trust record for an untrusted session folder at the start, after the version gate and before the endpoint bind, so the trust dialog no longer stands on the human's first screen and viola types no key.
The write sets the trust flag alone (every other key kept), walks the exact folder then each parent up to the git root, skips the home folder, an already-trusted folder or parent, and reads back three times; revive keys the record by the recorded cwd.
The start's trust step never refuses the start: no exit code, hint, refusal or --json code moves; it emits one new diagnostic event trust-record (INFO, one per start, a detail of 16 closed start words, no path, key or value) to the role file, never the terminal.
Retired: the rule that run and revive write no trust record; the fake agent's oracle that inside a repository only the root's record counts (the fake now walks like the CLI).
Measured live on claude 2.1.325: the wrapped start in a fresh untrusted folder showed the session box first, the plain claude in the same folder shape showed the trust form; no key typed in either.
Tests and harness: every test run or revive and the supervised wrappers now get a test-owned user home; a new passthrough case proves the trust-writing start adds no viola byte; CI green 7/7.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: edited — the Claude-shape trust-record reading: the CLI's folder walk, the flag-only set, the exact read-back and the closed set of 16 start words
@task_file n=2 path=src/trust_record.rs
what: edited — viola's one guarded writer of the Claude config's trust record, now shared by verify and by the run and revive start
@task_file n=3 path=src/cmd/run.rs
what: edited — the viola run and revive start sequence that enters the TUI passthrough, now with a trust step after the version gate
@task_file n=4 path=src/cmd/verify.rs
what: edited — viola verify, its trust-record step now calling the shared config-file lookup
@task_file n=5 path=crates/viola-core/src/obs.rs
what: edited — the closed set of run diagnostics events written to the role file, now with trust-record
@task_file n=6 path=schemas/diag-line.v1.json
what: edited — the schema of a run diagnostics line, now with the trust-record event and its start-word detail
@task_file n=7 path=schemas/diag-detail.v1.json
what: edited — the schema of a diagnostics detail, its event enum now with trust-record
@task_file n=8 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent standing in for claude in tests, its trust reading now walking from the cwd to the git root
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited — the fake agent's own test cases, the trust-walk case renamed
@task_file n=10 path=tests/support/home.rs
what: edited — the test home support, now with an unmade user home and wrappers booted or revived as a user
@task_file n=11 path=tests/support/cli.rs
what: edited — the test CLI driver, giving every test run and revive a test-owned user home
@task_file n=12 path=tests/run_cli.rs
what: edited — the viola run integration suite, its role-line counts moved and the trust-record start cases added
@task_file n=13 path=tests/cli_revive.rs
what: edited — the viola revive integration suite, a case for the record keyed by the recorded cwd
@task_file n=14 path=tests/tui_passthrough.rs
what: edited — the TUI passthrough boundary suite (zero viola bytes on the terminal), a trust-writing start case added
@task_file n=15 path=tests/contract_diag_schema.rs
what: edited — the check that the run diagnostics lines validate against their schema and the event literals
@task_file n=16 path=tests/contract_bounded_readers.rs
what: edited — the check that each bounded read named stands
@task_file n=17 path=tests/chaos_snapshot_replay.rs
what: edited — the snapshot replay chaos suite, given a test-owned user home
@task_file n=18 path=tests/cli_controls_not_disableable.rs
what: edited — the suite that the controls cannot be disabled, given a test-owned user home
@task_file n=19 path=tests/cli_fake_agent.rs
what: edited — the fake agent CLI suite, given a test-owned user home
@task_file n=20 path=tests/cli_version_gate.rs
what: edited — the version gate suite, given a test-owned user home
@task_file n=21 path=tests/cross_exit_causes.rs
what: edited — the exit-cause suite, given a test-owned user home
@task_file n=22 path=tests/cross_killed_test.rs
what: edited — the killed-test suite, given a test-owned user home
@task_file n=23 path=tests/channel_error_bodies.rs
what: edited — the check that error bodies have a writer and a witness, given a test-owned user home
@task_file n=24 path=crates/viola-e2e/src/harness/boot.rs
what: edited — the e2e harness boot, now giving a supervised session its own user home
@task_file n=25 path=crates/viola-e2e/src/harness/supervise.rs
what: edited — the e2e harness's supervised wrappers, now started under the session user home
@task_file n=26 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the live rig against the real CLI (live readings, never a gate), its first-screen word by signature and the new trust arms
@task_file n=27 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the live rig's scratch isolation, a sentinel config entry and a fresh untrusted folder
@task_file n=28 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the live rig's command line, the new S1 and S2 arms
@task_file n=29 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the live rig's rehearsal suite on the fake agent, the ts and tc rehearsals added
@task_item n=1 part=touches
decision/wrapped-tui-reads-as-unwrapped: in an untrusted folder the wrapped start's first screen is now the session box while the unwrapped claude shows the trust form; the first screen differs by design.
@task_item n=2 part=touches
decision/zero-viola-bytes-on-the-terminal and contract/tui-passthrough/viola-terminal-bytes and contract/tui-passthrough/own-bytes: the new start step writes no viola byte to the terminal, held by a new passthrough case.
@task_item n=3 part=touches
contract/tui-passthrough/entry: the passthrough entry now follows a trust step placed after the version gate and before the endpoint bind, on a verified CLI only.
@task_item n=4 part=touches
decision/run-diagnostics-off-terminal and contract/tui-passthrough/run-diagnostics: a new run diagnostics event trust-record (one line per start, 16 closed start words) goes to the role file only.
@task_item n=5 part=touches
decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver: the start's config lock may pause viola's start, at most 2 x 100 ms, a bound on viola's start and never on the human.
@task_item n=6 part=touches
absence/user-time-limit: the new lock pause is no time limit on the user; a row listing the start's bounds may need it beside the others.
@task_item n=7 part=touches
decision/human-keystrokes-never-blocked, decision/tui-terminal-belongs-to-child and decision/critical-path-human-takes-wheel: viola types no key to dismiss the trust dialog; the record is written before the spawn instead.
@task_item n=8 part=touches
decision/cli-not-keyboard-interactive and decision/cli-no-interactive-input: the trust step asks the human nothing and reads no stdin.
@task_item n=9 part=touches
decision/paths-not-carried: the trust-record line carries no path, key or value, only a start word.
@task_item n=10 part=touches
contract/code-word/format and decision/unlisted-code-words-stay-visible: the 16 start words are a new closed kebab-case code-word set, carried in the diagnostics line only.
@task_item n=11 part=touches
contract/start-refusal/home-strict-modes, contract/start-refusal/socket-dir, contract/start-json-refusal/document and the revive --json refusals: the trust step's word never refuses the start, so no refusal, exit code, hint or --json code is added or moved; confirm each row stays true.
@task_item n=12 part=touches
contract/viola-verify/step-line: verify's trust-record step now uses the lookup shared with the start; verify's printed step lines do not move.
@task_item n=13 part=touches
decision/no-screen-content-parsing and decision/boundary-check-never-inspects-claude-content: the live rig names the first screen by signature only (box or trust).
@task_item n=14 part=touches
decision/live-readings-never-gate: the live S1 and S2 readings on 2.1.325 are ledgered evidence and gate no build.
@task_item n=15 part=touches
decision/boundary-tests-sequenced-on-records, decision/a11y-inside-existing-suites and decision/a11y-runs-in-existing-test-suites: the new no-viola-bytes case sits in the existing passthrough suite and sequences on role lines.
@task_item n=16 part=touches
decision/ci-test-gate-no-skips and decision/zero-flakiness: every test start now runs under a test-owned user home; CI green 7/7 with no skip or retry.
@task_fact n=1 slug=7-screen-reader-support
fact: In an untrusted session folder viola's start writes the CLI's trust record (the flag alone) before the spawn, so the wrapped claude opens on its session box with no trust dialog and viola types no key, while an unwrapped claude still opens on the trust form.
@task_fact n=2 slug=8-cognitive-accessibility
fact: The start's trust step may pause viola's start on the Claude config lock, at most 2 x 100 ms; the pause falls on viola's start, never on the human, and never refuses the start.
@task_fact n=3 slug=a11y-scope-summary
fact: The start's trust step reports one of 16 closed start words in one trust-record diagnostics line per start, never printed on the terminal and never a refusal, exit code or hint.
@task_item n=1 part=untouched
Colour and SGR rows (amber DIALOG, bold NAME, dim stale rows, no SGR off a terminal, word beside every SGR): the session prints nothing new.
@task_item n=2 part=untouched
The viola list board, its caption, order, empty board, unwrapped separator and NAME truncation.
@task_item n=3 part=untouched
viola --help rows, viola link and unlink, viola plugin install, viola wait and the MCP wait tool.
@task_item n=4 part=untouched
viola send refusals, hook event and dialog calls, MCP frame tools, frame verb refusals and the instance-unreachable details.
@task_item n=5 part=untouched
The absences of a web or graphical frontend, a touch surface, internationalization and a rendered page, and the N/A contrast, focus and WCAG mapping harnesses.
@task_item n=6 part=untouched
The a11y tier, the compliance and WCAG conformance claims, and the manual Orca and keyboard passes.
@task_item n=7 part=untouched
The a11y CI integration rows (GitHub Actions workflow, coverage suite, no own artifact, gate requires coverage and doctest): no CI step changed.
@task_item n=8 part=untouched
The dialog hold and config.json dialog_hold_s, the dialog-expired event line and the critical path of a pending dialog then answer: the trust dialog is the CLI's own, not a viola dialog.
