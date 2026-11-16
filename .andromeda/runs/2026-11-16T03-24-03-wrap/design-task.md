@task_prose part=session
A test-only chunk that documents and pins when viola wait wakes; no product code, verb, flag, output form, exit code, event kind or field changed.
Measured and pinned: every turn-ended record wakes a parked wait whatever its background_tasks (zero, one, three, null, and a booted held turn with two tasks and the question Shall I go on?).
Pinned the other side: after a task-free turn end, a Notification activity record and a harness-origin prompt-submitted never wake a parked wait.
Telling a finished session from a paused one stays the driver's job; viola carries no scheduled-wake count (a session_crons count is only a candidate for the founder, not taken).
The stamped-home test fixture now copies one viola verify stamp per built binary into each test home, never a link; viola verify itself is unchanged and stays the sole stamp writer, and the CI per-test kill is unchanged.
The CI timing red carried on the wait chunk closed green (checks 7/7); two older wait unit tests that hang past a failure are carried to a later route entry.
@task_file n=1 path=src/run/wait.rs
what: the viola wait verb, parking until a waking record (turn end, dialog) or its timeout; edited in its test module only (a park bound and the wake-whatever-the-tasks and no-false-wake unit tests)
@task_file n=2 path=fixtures/fake-scripts/wait-held-turn.json
what: a new fake-agent script for tests, a held turn end with background tasks followed by a notification and a harness turn
@task_file n=3 path=tests/cli_wait_last.rs
what: the end-to-end tests of the viola wait and viola last verbs; edited, two booted wait cases added
@task_file n=4 path=tests/support/home.rs
what: the test fixture that gives each test a fresh home started from a viola verify stamp; edited to copy one stamp per built binary
@task_file n=5 path=tests/cli_verify.rs
what: the end-to-end tests of the viola verify verb and its stamp; edited, two stamp-isolation tests added
@task_file n=6 path=.config/nextest.toml
what: the test runner configuration holding the CI per-test kill for verify-driven tests; edited, comment only
@task_item n=1 part=touches
viola wait output, event form: wait --json returns a held turn end with its background_tasks count and its last message as the event; the form is unchanged, now pinned by tests.
@task_item n=2 part=touches
The wait command name and what wakes it: every turn end wakes a parked wait whatever its background-task count; activity records and a harness-origin prompt-submitted never wake it. Pinned, not changed.
@task_item n=3 part=touches
viola wait output, waiting line and timed out: a wait armed after a task-free turn end stays parked through activity and a harness turn rather than returning early; no output line changed.
@task_item n=4 part=touches
CLI callers, LLM driver sessions: deciding whether a turn end with background tasks is done or paused is the driver's; viola gives no scheduled-wake count, and a driver that re-parks on a non-zero task count misses a turn that asks.
@task_item n=5 part=touches
Rules over how a turn end is shown (a turn end never fills the readback box; an API-error turn end shown as any turn end): the wake rule reads only the event kind, so every turn-ended kind still wakes alike.
@task_item n=6 part=touches
viola last output, message: the held turn's question is the message a woken wait reports; last itself is unchanged.
@task_item n=7 part=touches
viola verify output and refusals (step line, summary, stamps update, home-tree check, unclean fixture): verify is unchanged, now run once per built binary under a lock to take the stamp test homes are copied from.
@task_item n=8 part=touches
The MCP wait tool and the mcp verb: unchanged, parity with CLI wait holds by construction (the parity witness ran green).
@task_item n=9 part=touches
CLI exit codes for wait (0 on a wake, the timeout code on timed out): unchanged; the booted cases return a wake.
@task_fact n=1 slug=surface-cli
fact: viola wait wakes on every turn end whatever its background-task count, and never on an activity record or a harness-origin prompt-submitted; telling done from paused is the driver's, and viola carries no scheduled-wake count.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, depth, border radius and iconography rows: no styling, glyph, colour token or column changed.
@task_item n=2 part=untouched
The list board and list table rows: no column, width, row order, skip count or strict-modes scan changed.
@task_item n=3 part=untouched
Send readback, readback box and send mirror rows: no send, confirmation or readback form changed.
@task_item n=4 part=untouched
revive, run start, link, unlink, pause, release, answer and plugin install output and refusal rows: none of those verbs changed.
@task_item n=5 part=untouched
exit 21 causes, refusal hints, peer reply screening, wrapper fault detail and CLI colour decision order rows: no refusal, hint or cause was added or reworded.
@task_item n=6 part=untouched
The web-spa surface absence, the anti-patterns and the self-validation protocol: no surface or design check was added or removed.
