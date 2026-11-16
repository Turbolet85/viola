@task_prose part=session
The session documented and pinned the wake condition of viola wait: every turn-ended record wakes a parked wait whatever its background_tasks count (zero, one, three, null, and a booted held turn with 2), and an activity record or a harness-origin prompt-submitted after a task-free turn end never wakes it.
No product code moved: no command, IPC method, event kind, field, output line, refusal, exit code or env var is added or changed; the wake set stays six kinds; the MCP wait tool is unchanged.
The change is test-only: new unit and booted CLI wait tests, a new fake-agent script for a held turn, and a bound on the unit tests' park waits.
The stamped-home test fixture now takes one viola verify stamp per built test binary under a lock and copies it into each test's fresh home, never a link; two new verify-suite tests pin that copies share no file and reuse one stamp.
Measured: the stamped-home class runs under the unchanged 20 s ci kill (rest max 10.9 s, verify-window max 43.4 s under its own 60 s kill), the class wall fell 56.1 to 48.4 s, and the carried CI timing red closed green 7 of 7 checks.
Carried, not fixed: two older wait unit tests still hang past a failure; owner the route entry Version done-check. Nothing is retired.
@task_file n=1 path=src/run/wait.rs
what: viola wait, the command that parks until a waking record arrives and prints the waiting line; only its unit-test module changed (new wake-rule cases, park waits bounded)
@task_file n=2 path=fixtures/fake-scripts/wait-held-turn.json
what: a fake-agent script driving a held turn end, a notification and a harness turn, used by the booted wait tests; synthetic text only
@task_file n=3 path=tests/cli_wait_last.rs
what: the CLI wait and last integration suite, the critical path for wait/last output, booted against the fake agent and sequenced on records
@task_file n=4 path=tests/support/home.rs
what: the stamped-home test fixture giving each test a fresh home, now a copy of one viola verify stamp per built binary; viola verify stays the sole stamp writer
@task_file n=5 path=tests/cli_verify.rs
what: the viola verify CLI integration suite; two new tests pin that stamped homes share no file and reuse one stamp
@task_file n=6 path=.config/nextest.toml
what: the nextest configuration, its ci profile and the verify-driven tests' 20 s kill override; only that override's comment changed, no kill, filter or period moved
@task_item n=1 part=touches
contract/viola-wait/waiting-line and contract/viola-wait/timed-out: the wait command's output is unchanged; the new tests run wait --json against a held turn end and read its turn-ended result, so these rows stand and a picker should confirm no line moved.
@task_item n=2 part=touches
decision/critical-path-cli-board-output: the wait/last leg of this critical path gained two booted cases (a held turn end with background tasks wakes wait; a task-free end does not falsely wake it) inside the existing suite.
@task_item n=3 part=touches
decision/time-bounds-fall-on-the-driver and decision/time-bounds-never-on-human: the session restates that telling a finished turn from a paused one is the driver's job; viola wakes on every turn end and carries no scheduled-wake count; no time bound moves onto the human.
@task_item n=4 part=touches
contract/mcp-wait-tool/timeout-ms: MCP wait is held equal to CLI wait by construction and the parity witness ran green; timeout_ms is unchanged.
@task_item n=5 part=touches
decision/a11y-inside-existing-suites and decision/a11y-runs-in-existing-test-suites: new wait and stamp tests were added inside the existing nextest suites, no separate suite.
@task_item n=6 part=touches
decision/boundary-tests-sequenced-on-records: the booted wait tests wait on the event-log records (session-start, the two turn-ended records, the activity and harness-turn records) before asserting.
@task_item n=7 part=touches
decision/zero-flakiness and decision/a11y-fail-fast-gate: unit park waits are now bounded at 5 s so a wake-rule failure fails instead of hanging, but two older wait unit tests still hang past a failure, a standing carry owned by Version done-check.
@task_item n=8 part=touches
decision/a11y-ci-github-actions, decision/cli-invariants-pass-in-ci and decision/ci-test-gate-no-skips: the push ran CI green 7 of 7 checks with the stamped-home class inside the unchanged 20 s ci kill; no test was skipped or retried.
@task_item n=9 part=touches
decision/no-a11y-performance-budget: the class wall and per-test times were measured and reported only; no budget or threshold was set.
@task_item n=10 part=touches
contract/viola-verify/step-line and contract/viola-verify/interrupted-pair: verify's output is unchanged; the stamped-home fixture takes its stamp through the unchanged verify and the verify suite gained two stamp tests.
@task_item n=11 part=touches
contract/event-line/dialog-expired: dialog-expired still wakes a parked wait; the wake set stays six kinds, a true claim with no change.
@task_item n=12 part=touches
decision/a11y-failures-are-nextest-failures and decision/gate-requires-coverage-doctest: the new cases are ordinary nextest tests under the same gate; pre-push and the unit run read green.
@task_item n=1 part=untouched
Colour and SGR rows (amber DIALOG word, bold NAME, dim stale rows, word beside every SGR span, no SGR off a terminal): no output styling moved.
@task_item n=2 part=untouched
Refusal and failure rows (unable and hint lines, reason codes, fault bodies, --json failure documents, exit codes, the unknown refusal, socket-dir and strict-modes refusals): no refusal, code word or exit code changed.
@task_item n=3 part=untouched
TUI passthrough rows (entry, human keystrokes, focus/mouse/resize sequences, vendor content, viola terminal bytes, vt100 feed panic, the stdin non-editing list, the human taking the wheel): the passthrough was not reached.
@task_item n=4 part=untouched
viola list board, revive, link/unlink, plugin install and --help contracts: none of those commands or their output changed.
@task_item n=5 part=untouched
Screen-reader targets, manual keyboard and Orca passes, no-frontend absences, internationalization, touch surface and WCAG or compliance claims: nothing in this session bears on them.
@task_item n=6 part=untouched
Send and dialog rows (confirmed send and readback, input-not-ready, no-prompt-submitted, dialog hold bound, dialog_hold_s): send and the dialog path were not touched.
