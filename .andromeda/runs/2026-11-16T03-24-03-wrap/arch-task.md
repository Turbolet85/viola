@task_prose part=session
The session documented and pinned the wait wake condition by tests only: no product symbol, channel method, event kind, field, env var or socket changed, and the wake set stays 6 kinds.
The wake rule reads only the event kind: every turn-ended wakes a parked wait whatever its background_tasks (zero, one, three, null; a booted case with 2), while activity and a harness-origin prompt-submitted after a task-free turn end never wake it.
Telling a finished turn from a paused one stays the driver's; viola carries no scheduled-wake count on the turn end (a session_crons count was left a founder candidate, not taken).
The test home fixture now takes one viola verify stamp per built test binary, under an exclusive lock on its .lock sibling under the e2e-home base, and copies it into each test's home, never linking; the stamp dir carries no owner record and other builds' stamps are swept only when their lock is taken without waiting.
Measured: the stamped-home class runs every test under its unchanged 20 s ci kill (rest max 10.9 s, verify window max 43.4 s under its 60 s kill, class wall 56.1 to 48.4 s), and CI closed green 7 of 7.
A 5 s park bound was added to the wait unit tests; two older wait unit tests still hang past a failure, carried to a later entry.
@task_file n=1 path=src/run/wait.rs
what: the wrapper channel's wait method: parking on the events log and its wake rule over the wake kinds, with its unit tests (only the test module edited)
@task_file n=2 path=fixtures/fake-scripts/wait-held-turn.json
what: a fake-agent script (fake-script.v1) driving the fake agent through a turn end holding background tasks, a task-free turn end, a Notification and a harness-origin UserPromptSubmit (new)
@task_file n=3 path=tests/cli_wait_last.rs
what: end-to-end tests of the CLI wait and last verbs against the fake agent (edited)
@task_file n=4 path=tests/support/home.rs
what: the test home fixture: each test in its own viola home, started stamped through viola verify, now a copy of one stamp per built binary (edited)
@task_file n=5 path=tests/cli_verify.rs
what: end-to-end tests of viola verify and of stamped-home isolation (edited)
@task_file n=6 path=.config/nextest.toml
what: the nextest profiles: the ci per-test kill line for verify-driven, stamped-home tests (comment only edited)
@task_item n=1 part=touches
contract/channel-wait/wake-kinds and contract/channel-methods/wait-wake-kinds: the wake set is unchanged at 6 kinds, but the row must now say every turn-ended wakes whatever its background_tasks, and activity and a harness-origin prompt-submitted never do.
@task_item n=2 part=touches
contract/event-data/turn-ended-background-tasks: background_tasks gates no wake; it is data for the driver, which decides whether the turn end is the session's last word.
@task_item n=3 part=touches
decision/mechanism-not-policy: applied to wait; viola wakes on every turn end and leaves telling done from paused to the driver, carrying no scheduled-wake count.
@task_item n=4 part=touches
contract/event-data/prompt-submitted-origin and contract/event-prompt-submitted/origin: a harness-origin prompt-submitted is non-waking for a parked wait, now pinned by tests.
@task_item n=5 part=touches
contract/normalised-event-kinds/turn-ended and contract/normalised-event-kinds/activity: their wake behaviour toward a parked wait is pinned (turn-ended always wakes, activity never).
@task_item n=6 part=touches
contract/channel-wait/driver-relevant-events, contract/channel-wait/result and contract/cli-verbs/wait: the held turn end with background_tasks 2 and its last message is returned by wait --json; the rows' statement of what wakes and is returned may need the whatever-its-tasks clause.
@task_item n=7 part=touches
contract/mcp-tools/wait and decision/mcp-wait-cap: the MCP wait carries the same wake rule by construction, through the unchanged channel wait.
@task_item n=8 part=touches
decision/wait-blocks-nothing-polls and decision/channel-blocks-never-polls: the product wait still blocks; the unit tests' parked waits are now bounded by a 5 s park bound, a test-side bound only.
@task_item n=9 part=touches
decision/root-waits-share-one-bound: a second, separate 5 s park bound now governs the wait unit tests' parked waits, beside the root tests' shared bound.
@task_item n=10 part=touches
decision/test-isolated-viola-home: each test still has its own viola home, now a copy of one viola verify stamp per built binary, never a link, sharing no file with the stamp or another home.
@task_item n=11 part=touches
decision/verify-sole-stamps-writer and decision/stamps-single-writer: the stamp is taken once per binary through the unchanged viola verify under an exclusive lock; copying it into a home is no write of stamps.
@task_item n=12 part=touches
decision/test-home-owner-sweep and decision/test-homes-cleaned-by-owner: the stamp dir under the e2e-home base carries no owner record, and other builds' stamps are swept only when their lock is taken without waiting; an exception to removal by owner record.
@task_item n=13 part=touches
decision/separate-lock-files: the stamp is guarded by a .lock sibling of the stamp dir, a new lock under the rule.
@task_item n=14 part=touches
decision/dev-host-e2e-home-on-tmpfs, decision/e2e-home-headroom-floor, decision/e2e-home-owner-only-headroom and decision/linked-e2e-home-base-strict: stamp dirs (viola-stamp-key16/home) now persist under the e2e-home base beside the per-test homes.
@task_item n=15 part=touches
decision/end-to-end-homes-kept-after-each-job: stamp dirs persist across test runs of one build and are not per-job homes.
@task_item n=16 part=touches
decision/tests-against-fake-agent and decision/ci-against-fake-agent: a new fake-agent script drives the held-turn and no-false-wake cases; CI stays on the fake agent.
@task_fact n=1 slug=standard-contracts
fact: Every turn-ended wakes a parked wait whatever its background_tasks (zero, some or null); the wake reads only the event kind, and telling a finished turn from a paused one is the driver's.
@task_fact n=2 slug=standard-contracts
fact: An activity record and a harness-origin prompt-submitted never wake a parked wait, also after a task-free turn end.
@task_fact n=3 slug=standard-contracts
fact: viola carries no scheduled-wake count on turn-ended: a driver cannot learn from the turn end that the CLI has a wake scheduled.
@task_fact n=4 slug=occupied-resources
fact: Test homes are copies of one viola verify stamp per built test binary, kept under the e2e-home base and taken under an exclusive lock on its .lock sibling; a stamp dir holds no owner record and another build's stamp is removed only when its lock is taken without waiting.
@task_item n=1 part=untouched
The hook-to-kind map and hook tiers: no hook, hook mapping or event kind changed; Stop still maps to turn-ended.
@task_item n=2 part=untouched
Send, answer, dialog hold, release, pause, wheel and link rules and their refusal contracts: no send or dialog path changed.
@task_item n=3 part=untouched
Channel framing, protocol version, fault codes and exit codes: no method, field, code or frame shape changed.
@task_item n=4 part=untouched
Build-system, crate dependency direction, dependency policy, toolchain and MSRV pins: no crate, module or dependency added or moved.
@task_item n=5 part=untouched
The CI workflow jobs, action pins, coverage floors, mutation and perf gates: no gate, threshold, floor or workflow moved.
@task_item n=6 part=untouched
Run start, revive, pinned copy, plugin install and child environment rules: no start path changed.
@task_item n=7 part=untouched
The capability ledger's relied row count: no architecture row states it, and the session moved none.
@task_item n=8 part=untouched
Snapshot, list, board and session liveness contracts: no read model or snapshot changed.
