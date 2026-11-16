@task_prose part=session
The session documented and pinned the wake condition of viola wait with tests only; no product code, verb, flag, output line, exit code, MCP tool or event kind changed.
It pinned that every turn-ended record wakes a parked wait whatever its background_tasks (zero, one, three, null, and a booted case holding two), and that activity and a harness-origin prompt-submitted after a task-free turn end never wake it.
It added a fake-agent script that holds a turn with background tasks, two booted wait cases, and two stamped-home isolation tests.
It changed the stamped-home test fixture to copy one viola verify stamp per built test binary, taken under a lock, in place of a stamp per test; the CI kill is unchanged and the carried CI timing red closed green.
It measured the stamped-home class: rest max 10.9 s under 20 s, window max 43.4 s under 60 s, class wall 56.1 to 48.4 s; nothing was retired.
@task_file n=1 path=src/run/wait.rs
what: the wait verb's wake loop, edited in its unit test module only — a park bound and tests that a turn end wakes whatever its background tasks and a task-free end parks through activity and a harness turn
@task_file n=2 path=fixtures/fake-scripts/wait-held-turn.json
what: a new fake-agent script for tests, a turn end held with background tasks then a notification and a harness prompt
@task_file n=3 path=tests/cli_wait_last.rs
what: the integration tests of viola wait and viola last, edited to add two booted wait cases
@task_file n=4 path=tests/support/home.rs
what: the stamped-home test fixture, edited to copy one viola verify stamp per built binary into each test's home
@task_file n=5 path=tests/cli_verify.rs
what: the integration tests of viola verify, edited to add the stamped-home copy and one-stamp tests
@task_file n=6 path=.config/nextest.toml
what: the test runner's kill and filter config, a comment edited on the verify-driven override only
@task_item n=1 part=touches
contract/cli-verbs/wait: viola wait now has a pinned wake rule — every turn-ended wakes it whatever its background_tasks; the verb's form is unchanged.
@task_item n=2 part=touches
contract/mcp-tools/wait: the MCP wait tool shares the same wake rule by construction; its parity holds and its shape is unchanged.
@task_item n=3 part=touches
contract/viola-wait-output/turn-ended: a wait now returns a turn end that still holds background tasks, carrying background_tasks as it stands (2 in the booted case) with its message; the output form is unchanged.
@task_item n=4 part=touches
contract/viola-wait-output/waiting-line: a wait armed after a task-free turn end keeps waiting through an activity record and a harness-origin prompt-submitted.
@task_item n=5 part=touches
decision/turn-ended-not-send-outcome: a turn end is a wake for wait, not a verdict of done; telling done from paused is the driver's, not viola's.
@task_item n=6 part=touches
contract/viola-wait-output/timed-out: indirect, through the wake rule — a wait after a task-free end with only activity and a harness turn ends timed out, never on a false turn end.
@task_item n=7 part=touches
contract/viola-verify-output/summary and the verify output rows: indirect only — the test fixture now takes the verify stamp once per binary through the unchanged verify; no verify output line moved.
@task_fact n=1 slug=surface-cli
fact: viola wait (CLI and MCP) wakes on every turn-ended record whatever its background_tasks, and never on an activity record or a harness-origin prompt-submitted; whether the turn end means done or paused is the driver's to tell, and viola carries no scheduled-wake count.
@task_item n=1 part=untouched
The web-spa surface and its absence row: no page or web surface is reached.
@task_item n=2 part=untouched
viola list board, BAY line and list --json rows: no column, colour, width or status word moved.
@task_item n=3 part=untouched
viola send, readback mirror, readback vocabulary and send refusal rows: no send output or refusal changed.
@task_item n=4 part=untouched
viola run, revive and plugin install rows, with their start refusals and --json error codes: no start path changed.
@task_item n=5 part=untouched
CLI exit codes, refusal line form, internal error, frame-verb hints and exit-21 detail rows: no code, hint or detail added or moved.
@task_item n=6 part=untouched
CLI token rendering, styling, spinner, banner, footer and hint wording decisions: no styling or wording changed.
@task_item n=7 part=untouched
viola answer, link/unlink, wheel verb (pause/release) and help/usage rows: no verb, flag or usage line added or moved.
