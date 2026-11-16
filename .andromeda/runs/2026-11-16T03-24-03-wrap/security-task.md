@task_prose part=session
The session documented and pinned the wait wake condition with tests only: every turn-ended record wakes a parked wait whatever its background_tasks count (zero, one, three, null, and a booted case with two), and the activity record and a harness-origin prompt-submitted after a task-free turn end never wake it.
No product symbol, channel method, MCP tool, event kind, field, env var or socket was added or changed; the wake set stays six kinds and the wake rule reads only the event kind.
The stamped-home test fixture now takes one viola verify stamp per built binary, under an exclusive lock on its lock sibling, and copies it into each test's home, never a link; the stamp dir carries no owner record, and other builds' stamps are swept only when their lock is taken with try_lock.
A new fake-agent script with synthetic text only drives a held turn end; the CI per-test kill is unchanged, and the carried CI timing red closed green on measurement.
viola carries no scheduled-wake count beside background_tasks; carrying session_crons as a count is a founder candidate, not taken, and the driver-side fix for a re-parking waiter landed outside viola.
@task_file n=1 path=src/run/wait.rs
what: the CLI wait verb's park-and-wake logic; edited in its test module only (a park bound on the tests' waits and two wake-rule unit tests), its product code unchanged
@task_file n=2 path=fixtures/fake-scripts/wait-held-turn.json
what: a new fake-agent script fixture (fake-script.v1) of synthetic hook steps for a held turn end, a notification and a harness-origin turn; replaced nothing, added new
@task_file n=3 path=tests/cli_wait_last.rs
what: the end-to-end tests of the wait and last verbs against a fake-agent instance; edited to add the booted held-turn and no-false-wake cases
@task_file n=4 path=tests/support/home.rs
what: the test-support stamped-home fixture that gives each test its own viola home from a viola verify stamp; edited so one stamp per built binary is taken under a lock, copied per test, and other builds' stamps swept
@task_file n=5 path=tests/cli_verify.rs
what: the end-to-end tests of viola verify and its stamps; edited to add tests that stamped-home copies share no file with the stamp or each other and that one stamp serves every home
@task_file n=6 path=.config/nextest.toml
what: the test runner's CI profile with the per-test kill for verify-driven tests; edited in a comment only, no kill, filter or period moved
@task_item n=1 part=touches
The wait method of the wrapper channel and the MCP wait tool: the wake condition is now pinned by tests (every turn end wakes whatever its background_tasks; activity and a harness-origin prompt-submitted do not); a row stating which records wake or do not wake a parked wait may need the harness-origin prompt-submitted named.
@task_item n=2 part=touches
The decision that a Stop hook's background tasks cross only as a count: the count still crosses alone and no scheduled-wake count crosses beside it; a session_crons count stands as a founder candidate, a boundary widening not taken.
@task_item n=3 part=touches
The decision that only viola verify writes the stamps file, and that stamps are checked before and under lock: test homes now copy one verify-taken stamp per built binary under an exclusive lock; verify is still the sole writer, the copy is no write of a stamp.
@task_item n=4 part=touches
The decision that tests use their own home: each test's home is now a copy of a shared per-binary stamp, never a link, and the copies share no file with the stamp or each other.
@task_item n=5 part=touches
The rules that test-side removal touches only dirs the tests created, the e2e-home linked-base guard and its keepers, and that viola never touches the e2e-home backing: the fixture now sweeps other builds' stamp dirs under the e2e-home base, each only when its lock is taken with try_lock.
@task_item n=6 part=touches
The decision that test homes are kept out of git: the stamp dirs live under the build target's e2e-home tree.
@task_item n=7 part=touches
The recorded-fixture hygiene rules (fixtures reviewed before commit, viola-format fixtures under the hygiene walk, recordings scrubbed or refused): a new fake-agent script fixture was added, synthetic text only, no recorded fixture.
@task_item n=8 part=touches
The rule that test seams live only under the fake-agent feature: the new cases drive the fake agent by a scripted held turn end, no new seam added.
@task_item n=9 part=touches
The harness-prefix literals that mark a harness-origin prompt-submitted: such a turn after a task-free end is now pinned as non-waking for wait.
@task_item n=10 part=touches
The rule that driver policy sits outside viola (mechanism, not policy): telling a finished turn from a paused one is the driver's, and the re-parking fix was made in the driver's own waiter, not in viola.
@task_item n=11 part=touches
The SHA-256 content-hash rows: the test-side stamp key is a SHA-256 over the built viola and fake-agent binaries, a test-support use, not the pinned-copy key.
@task_fact n=1 slug=input-validation
fact: A turn end carries no scheduled-wake count; background_tasks is the only count that crosses, and carrying session_crons as a second count is a boundary widening left to the founder.
@task_fact n=2 slug=secret-management
fact: Each stamped test home is a copy of one viola verify stamp per built binary, taken under an exclusive lock, never a link, sharing no file with the stamp or other homes; the stamp dir carries no owner record.
@task_fact n=3 slug=security-anti-patterns-never-do-these
fact: The test fixture sweeps another build's stamp dir under the e2e-home base only when it takes that stamp's lock with try_lock, never a stamp a running build holds.
@task_item n=1 part=untouched
Peer euid checks, server verification, socket-directory and socket-mode rules: no channel, socket or client check changed.
@task_item n=2 part=untouched
Strict-modes refusals and their contract items across the CLI, hook, MCP, list, run and revive: no refusal word, exit code or check order moved.
@task_item n=3 part=untouched
Dependency security, pins, cargo-deny, zizmor, toolchains and CI workflows: no dependency, tool pin, workflow or gate changed.
@task_item n=4 part=untouched
Secret handling of the child: the CLAUDE environment strip, the identity floor and the hermetic pre-push environment are unchanged.
@task_item n=5 part=untouched
Paste and dialog text validation, the frame cap and JSON depth rows: no input the product reads was added or changed.
@task_item n=6 part=untouched
MCP tool list, tool inputs, structuredContent and isError contract items: no tool, argument or error code was added or changed.
@task_item n=7 part=untouched
Data protection of the pinned binary, plugin files, event-log rotation and diagnostics modes: nothing under the product's home handling changed.
@task_item n=8 part=untouched
Release, signing and attestation rows and the absences: no release stage, listener, network call or secret was added.
