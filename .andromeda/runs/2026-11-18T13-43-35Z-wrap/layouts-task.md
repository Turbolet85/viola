@task_prose part=session
viola run and viola revive now write the claude CLI's trust record for an untrusted session folder at a wrapped start, so no trust dialog stands on the child's first screen and no key is typed (matrix v2-33, implemented).
The start's trust step runs after the version gate and before the endpoint is bound, on a verified CLI only; it walks the CLI's own way (the exact folder key, then each parent up to the git root), skips the user's home folder, and sets the trust flag alone in the entry, every other key kept.
revive keys the record by the session's recorded working directory; the child's config file is read through CLAUDE_CONFIG_DIR when the strip plan keeps that name.
The step never refuses a start: no exit code, refusal line, hint or --json code moves, and no viola byte reaches the terminal; its outcome is one new trust-record diagnostic log line carrying one of a new closed set of 16 start words, with no path, key or value.
The closed diagnostic event set moved 15 to 16; the rule that run and revive write no trust record (verify's own trust record as the one write) is retired.
Proven on the fake agent and live on claude 2.1.325 only (a wrapped start reads the input box first, a plain claude in the same folder shape reads the trust form); CI green 7/7.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the Claude-shape trust-record logic with no file I/O: the CLI's folder walk, the flag-only entry edit, the exact read-back, and the closed set of start trust words
@task_file n=2 path=src/trust_record.rs
what: the guarded trust-record writer and the Claude config file resolver, now shared by viola verify and the run and revive start
@task_file n=3 path=src/cmd/run.rs
what: the viola run and viola revive start sequence (version gate, then the new trust step, then the endpoint bind)
@task_file n=4 path=src/cmd/verify.rs
what: the viola verify verb, which keeps its own trust record writer and now calls the shared config file resolver
@task_file n=5 path=crates/viola-core/src/obs.rs
what: the closed set of diagnostic log events, now with trust-record
@task_file n=6 path=schemas/diag-line.v1.json
what: the diagnostic log line schema: the event enum, the trust-record message and its detail enum of 16 start words
@task_file n=7 path=schemas/diag-detail.v1.json
what: the diagnostic detail schema's event enum
@task_file n=8 path=src/bin/viola-fake-agent.rs
what: the test fake agent's trust reading, now walking from its cwd to the git root like the CLI
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's own tests of its trust reading
@task_file n=10 path=tests/support/home.rs
what: the test home helpers: an unmade user home and wrapper boots and revives run as a test-owned user
@task_file n=11 path=tests/support/cli.rs
what: the test CLI runner, which now gives every test run and revive an unmade user home
@task_file n=12 path=tests/run_cli.rs
what: the viola run integration cases, including the new trust-record cases and the start-line counts
@task_file n=13 path=tests/cli_revive.rs
what: the viola revive integration cases, with the revive writing the record for its recorded cwd
@task_file n=14 path=tests/tui_passthrough.rs
what: the terminal passthrough check that a trust-writing start adds no viola bytes
@task_file n=15 path=tests/contract_diag_schema.rs
what: the diagnostic schema contract check against the log format events and real runs
@task_file n=16 path=tests/contract_bounded_readers.rs
what: the bounded-readers boundary list check
@task_file n=17 path=tests/chaos_snapshot_replay.rs
what: the snapshot replay chaos tests, given a test-owned user home
@task_file n=18 path=tests/cli_controls_not_disableable.rs
what: the controls-not-disableable CLI tests, given a test-owned user home
@task_file n=19 path=tests/cli_fake_agent.rs
what: the fake agent CLI tests, given a test-owned user home
@task_file n=20 path=tests/cli_version_gate.rs
what: the version gate CLI tests, given a test-owned user home
@task_file n=21 path=tests/cross_exit_causes.rs
what: the cross-cutting exit cause tests, given a test-owned user home
@task_file n=22 path=tests/cross_killed_test.rs
what: the killed-test cleanup tests, given a test-owned user home
@task_file n=23 path=tests/channel_error_bodies.rs
what: the channel error body tests, given a test-owned user home
@task_file n=24 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot, which now gives a stamped session a session user home
@task_file n=25 path=crates/viola-e2e/src/harness/supervise.rs
what: the e2e harness's supervised wrappers, run under the session user home
@task_file n=26 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's arms and rounds, with the new wrapped-start and plain-claude trust arms and the verify floor rows
@task_file n=27 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's scratch isolation: the sentinel config entry, fresh untrusted folders and the entry and trust word reads
@task_file n=28 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary's command arms
@task_file n=29 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests for the new trust arms
@task_item n=1 part=touches
The viola run start (the run verb and its passed start output): a new trust step between the version gate and the endpoint bind writes the CLI's trust record on a verified CLI; it prints nothing and changes no start output, refusal or exit code, so a row stating what a run start does or writes may need it beside it.
@task_item n=2 part=touches
The viola revive start (the revive verb, its passed start output and its preflight order): the same trust step runs at a revive, keyed by the session's recorded working directory, after the version gate; a row ordering the revive's start steps or naming what the recorded cwd is used for is touched.
@task_item n=3 part=touches
The run and revive start refusals and their --json codes shared between run and revive: the trust step never refuses a start, so no new refusal, hint or code joins them; a row enumerating a start's refusal causes stays true but may need that stated.
@task_item n=4 part=touches
viola verify output, its --record step: verify no longer holds the only trust-record write; the config file resolver it uses is now shared with the run and revive start, and verify's own record writer and its words are unchanged.
@task_item n=5 part=touches
The rule that a refusal or a state is carried by words: a new closed set of 16 start trust words exists, carried only in the diagnostic log line, never on the CLI surface.
@task_item n=6 part=touches
Hints never quote text or name paths, and the CLI writes nothing on the terminal beyond its own forms: the new trust-record log line carries no path, key or value, and the wrapped start adds no viola byte to the passthrough terminal.
@task_item n=7 part=touches
The trust dialog the claude child used to show at a start in an untrusted folder: it no longer stands at a wrapped start, so a row that reads a child's first screen or its dialog at a start may be stale.
@task_item n=8 part=touches
The CLAUDE_CONFIG_DIR name, when the strip plan keeps it: its value is now read by the run and revive start too, not by verify only.
@task_fact n=1 slug=surface-cli
fact: viola run and viola revive, on a verified CLI, write the claude CLI's trust record for an untrusted session folder at the start (flag only, every other key of the entry kept), so no trust dialog stands and no key is typed; a folder already trusted, a trusted parent up to the git root, the user's home folder and an unverified CLI write nothing.
@task_fact n=2 slug=surface-cli
fact: The start's trust step never refuses or alters a run or revive start: no exit code, refusal line, hint or --json code, and no viola byte on the terminal; its only output is one trust-record diagnostic log line with one of 16 start words.
@task_fact n=3 slug=surface-cli
fact: viola revive keys the trust record by the session's recorded working directory, the same directory it spawns the child in.
@task_item n=1 part=untouched
The web surface absence: viola still renders no page.
@task_item n=2 part=untouched
The viola list board and its --json document, its columns, widths, colour, truncation and the BAY line.
@task_item n=3 part=untouched
viola send, wait, last and answer outputs, the readback mirror and its vocabulary, and the send refusals.
@task_item n=4 part=untouched
The viola mcp server, its hidden verb and its tool list and results.
@task_item n=5 part=untouched
CLI token rendering, styling, TTY detection, spinner, banner, footer and pager decisions.
@task_item n=6 part=untouched
viola link and unlink output, viola plugin install output, the pause and release wheel verb output, and the help and per-verb usage.
@task_item n=7 part=untouched
The CLI exit code table, the internal error forms, the frame verb exit-21 hints and the exit-21 --json details.
