@task_prose part=session
The session retired the status-line interception whole: viola no longer reads the user's status-line command, writes no per-session settings.json override, and hands the claude child no --settings flag; a start launches the child with --plugin-dir then the caller's args alone.
The hidden viola hook statusline arm and its hidden hook statusline --capture --shell probe arm are gone; the word statusline now maps to no hook event and takes the unknown-event exit 0, reading and writing nothing; viola runs no shell at all, and the hook keeps one capture arm.
The instance snapshot drops statusline_command; viola verify drops Run B's probe settings, the --record status-line variants and the two ledger rows statusline-override and statusline-shell, so the relied set is 21 rows and verify prints 21 step lines and stamped <ver>  21 pass  0 fail; Runs C and D still pass --settings.
The log schema loses the subject statusline-shell, the hook_event word statusline and the field shell_exit_status; the fake agent loses its statusline-echo mode, --statusline-stdin and its statusline receipt kind; the harness boot loses --statusline-echo; the StatusLine fixtures, their schema and two hook_stdin seeds are deleted.
Every own-format v stays 1; the old snapshot and stamps still read, the retired key and row ids skipped. CI green 7/7 on the commit.
@task_file n=1 path=crates/viola-agent-claude/src/statusline.rs
what: the status-line module of the agent crate (deleted) - the /bin/sh -c shell-out argv, the user's status-line command, the settings override and probe documents
@task_file n=2 path=src/cmd/hook/statusline.rs
what: the viola hook statusline arm (deleted) - the status-line shell-out with its 5 s deadline and the hidden --capture --shell probe arm
@task_file n=3 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's statusline-echo mode and --statusline-stdin replay (deleted)
@task_file n=4 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: a recorded StatusLine fixture variant of claude 2.1.287 (deleted)
@task_file n=5 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: a recorded StatusLine fixture variant of claude 2.1.302 (deleted)
@task_file n=6 path=fuzz/corpus/hook_stdin/statusline-absent
what: a hook stdin fuzz seed for the status-line event (deleted)
@task_file n=7 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a hook stdin fuzz seed for the status-line event (deleted)
@task_file n=8 path=schemas/claude-statusline-fixture.v1.json
what: the schema of the StatusLine fixtures (deleted)
@task_file n=9 path=tests/hook_statusline.rs
what: the integration tests of the viola hook statusline arm (deleted)
@task_file n=10 path=tests/support/statusline.rs
what: the test support helpers for the status-line cases (deleted)
@task_file n=11 path=crates/viola-agent-claude/src/ledger.rs
what: the compatibility ledger - the relied rows viola verify steps through and stamps, now 21 rows without statusline-override and statusline-shell
@task_file n=12 path=crates/viola-agent-claude/src/lib.rs
what: the agent crate root - the claude flag literals, --settings now beside --plugin-dir
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's stamp tests - a stamp holding the retired rows read on the relied rows alone
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger's check tests - the relied-row verdict vector
@task_file n=15 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed probe runs - Runs C and D passing --settings
@task_file n=16 path=src/cmd/verify.rs
what: the viola verify verb - its probe runs, step lines, --record variants and summary
@task_file n=17 path=src/run/mod.rs
what: the wrapper's child launch for viola run and viola revive - the claude child's argv
@task_file n=18 path=src/cmd/run.rs
what: the viola run verb - the start that writes the instance snapshot, no longer the status-line source read or the settings override
@task_file n=19 path=src/cmd/hook.rs
what: the hidden viola hook verb - the hook events it takes, the capture arm and the unknown-event exit
@task_file n=20 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot record (snapshot.json) - its keys, written and read
@task_file n=21 path=schemas/diag-line.v1.json
what: the diag log line schema - its subject and hook_event enums and the process-exit fields
@task_file n=22 path=fixtures/viola-0.1.0/retired-words.json
what: the fixture of words viola 0.1.0 used and this version retired
@task_file n=23 path=fixtures/viola-0.1.0/snapshot.json
what: the viola 0.1.0 instance snapshot shape, read for compatibility
@task_file n=24 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary - its mode words, options and receipt kinds
@task_file n=25 path=crates/viola-e2e/src/bin/viola-harness.rs
what: the viola-harness e2e driver - its boot options
@task_file n=26 path=crates/viola-e2e/src/harness/run.rs
what: the harness's live verify expectations - the ledger rows and the expected viola verify step and stamped lines
@task_file n=27 path=crates/viola-e2e/src/harness/session.rs
what: the harness session options
@task_file n=28 path=.config/nextest.toml
what: the nextest config - the ci kill-line override filter of test binaries
@task_file n=29 path=Cargo.toml
what: the workspace manifest - its test targets
@task_file n=30 path=tests/hook_fail_open.rs
what: the hook fail-open tests - a retired hook word as an unknown event writing nothing
@task_file n=31 path=tests/cli_instance_state.rs
what: the viola run instance-state tests - no settings file in the instance dir, the user settings left as found
@task_file n=32 path=tests/cli_verify.rs
what: the viola verify output tests - the step lines and summary counts
@task_file n=33 path=tests/cli_verify/runs.rs
what: the viola verify probe-run tests - the failing rows and summary counts
@task_file n=34 path=tests/cli_fake_agent.rs
what: the fake agent tests - the hangup case's marker hook
@task_item n=1 part=touches
contract/viola-verify-output/step-line, contract/viola-verify-output/summary, contract/viola-verify/summary: the relied ledger moved 23 to 21 rows, verify prints 21 step lines and stamped <ver>  21 pass  0 fail; a row holding the count 23, a [NN/23] example or a ledger order naming statusline-override or statusline-shell is touched (the report found the counter stated symbolically, so a row stating [NN/MM] stands).
@task_item n=2 part=touches
contract/viola-verify-output/record: the --record status-line variants are gone; a row listing the recorded variant families with the status line is touched.
@task_item n=3 part=touches
contract/viola-verify-output/failing-row, contract/viola-verify-output/exit-1, contract/viola-verify-output/stop: Run B's status-line probe settings and the two status-line ledger rows are gone; a row naming a statusline row as a failing row example is touched.
@task_item n=4 part=touches
contract/viola-verify-output/help-paragraph and contract/viola-verify-output/usage: a verify --help paragraph naming the status-line probe or the settings override is touched.
@task_item n=5 part=touches
contract/viola-verify-refusals/fixture-not-clean: the StatusLine fixture variants and their workspace.repo exception left; a row whose except names those fixtures or workspace.repo is touched.
@task_item n=6 part=touches
contract/cli-verbs/run, contract/viola-run/start-output, contract/viola-run/already-live: viola run no longer reads the user's status-line command, writes no settings.json override in the instance dir and passes the child no --settings; a row naming the override, the status-line source or the snapshot's status-line command is touched.
@task_item n=7 part=touches
contract/cli-verbs/revive and contract/viola-revive/start-output: revive's start shares the child launch, which now takes no settings path; a row naming a settings override on a revive start is touched.
@task_item n=8 part=touches
contract/run-revive-start/home-tree-refused and contract/cli-global-flags/home: the home's statusline-source.json and the default home's ~/.claude/settings.json are read by nothing now; a row listing them among the home files a start reads or checks is touched.
@task_item n=9 part=touches
The hidden viola hook verb: the statusline arm and the hook statusline --capture --shell probe form are removed, the word statusline takes the unknown-event exit 0 writing nothing, the hook keeps one capture arm; any row naming viola hook statusline, two capture arms or status-line output passed byte-for-byte is touched, including a hidden-verb row such as decision/mcp-verb-hidden or contract/viola-help/verb-groups where it lists hook arms.
@task_item n=10 part=touches
contract/cli-exit-codes/* and contract/cli-catch-site/internal-error: a row naming the status-line hook's exit, its 5 s deadline or the shell's exit status as an exception to the exit codes is touched.
@task_item n=11 part=touches
decision/stdout-results-stderr-rest, decision/human-output-is-contract, decision/no-persistent-footer, decision/no-banner: a row whose except names the status-line command's stdout passed through to claude, or viola's line in claude's status line, loses that subject - viola now writes nothing into the status line.
@task_item n=12 part=touches
Diag log names: the subject statusline-shell, the hook_event word statusline, the field shell_exit_status and the spans hook.statusline and statusline.shell_out are freed; a row naming them is touched.
@task_item n=13 part=touches
Names freed: the fake agent's statusline-echo mode and --statusline-stdin, the harness's boot --statusline-echo, the snapshot key statusline_command, the ledger ids statusline-override and statusline-shell; a row taking any of these names is touched.
@task_fact n=1 slug=surface-cli
fact: viola intercepts no status line: a wrapped session's claude status line is exactly as unwrapped - viola adds no settings layer, passes the child no --settings on run or revive, reads no status-line command and runs no shell; viola hook statusline is no hook event and takes the unknown-event exit 0, writing nothing.
@task_item n=1 part=untouched
absence/web-surface and the web-spa area: no web surface was added or touched.
@task_item n=2 part=untouched
The viola list board and BAY line rows (columns, widths, truncation, stale rows, colour depth, the --json document): the board reads no status-line or settings value.
@task_item n=3 part=untouched
viola send, the readback mirror and the readback vocabulary: send's issue lines, read back, refusals and --json are unchanged.
@task_item n=4 part=untouched
viola wait, viola last and viola answer output, and the dialog rows: no wait, last or dialog wording moved.
@task_item n=5 part=untouched
The viola mcp tools and their results: no mcp tool or result changed.
@task_item n=6 part=untouched
viola link and unlink output and the wheel verbs viola pause and viola release: unchanged.
@task_item n=7 part=untouched
The CLI token rendering rows (colour roles, spacing, motion, radius) and the styling decisions (hand-written SGR, TTY detection, plain output modes): no styling changed.
@task_item n=8 part=untouched
The exit-21 hints and --json detail rows, the strict-modes refusals and viola plugin install output: none moved.
@task_item n=9 part=untouched
absence/cli-deep-links: no link or URL coupling was added.
