@task_prose part=session
The session is test-only. It pins the wait wake condition: every turn-ended record wakes a parked wait, whatever its background_tasks (unit cases 0, 1, 3 and null, and a booted case with 2), and activity and a harness-origin prompt-submitted after a task-free turn end never wake it.
No product symbol, IPC method, event kind, field, env var, socket, span, log line or detail code is added or changed. The wake set stays 6 kinds, and the wait product code and MCP wait are unchanged.
The stamped-home test fixture now takes one viola verify stamp per built binary, under a lock, at target/e2e-home/viola-stamp-<key16>/home. It copies that stamp into each test's home, never as a link. The stamp dir has no owner record, and another build's stamp is swept only when its lock is free.
Measured: the stamped-home class wall went from 56.069 to 48.381 s (321 tests on both sides), with rest max 10.913 s under the unchanged 20 s ci kill and window max 43.358 s under its 60 s kill. The carried CI timing red is closed (CI green 7/7).
Carried: two older wait unit tests still hang past a failure in a thread-scope join, and the carry is owned by Version done-check. A scheduled-wake count (session_crons) is a founder candidate only, with no row minted.
@task_file n=1 path=src/run/wait.rs
what: the wait verb's park and wake on the instance (the run.wait_dispatch and wait.block path); only its test module was edited: the wake-kind unit table, the no-false-wake unit test, and the park waits now bounded
@task_file n=2 path=fixtures/fake-scripts/wait-held-turn.json
what: a new fake-agent script with synthetic text only: a held turn end with background tasks, a task-free turn end, a Notification and a harness prompt-submitted
@task_file n=3 path=tests/cli_wait_last.rs
what: the CLI wait and last integration tests; edited to add two booted cases, a held turn end waking wait and no false wake after a task-free end
@task_file n=4 path=tests/support/home.rs
what: the stamped-home test fixture that gives each integration test its own home under the e2e-home root; edited so each home is a copy of one viola verify stamp per built binary
@task_file n=5 path=tests/cli_verify.rs
what: the viola verify integration tests; edited to add two stamp-isolation tests (copies share no file; one stamp reused for every home)
@task_file n=6 path=.config/nextest.toml
what: the nextest ci profile and its kill lines for verify-driven tests; edited in a comment only, with no kill, filter or period moved
@task_item n=1 part=touches
verify is the only stamps writer: the stamp is still written only through viola verify. Each test home's copy of it is no write of a stamp. Confirm the row still holds.
@task_item n=2 part=touches
Integration-test homes under one root, and test homes living under the scanned root: stamp dirs now sit beside the homes under the e2e-home root, one per built binary.
@task_item n=3 part=touches
Test homes kept until the gates run: the fixture now removes other builds' stamp dirs under the e2e-home root, but only when their lock is free. It removes no test home.
@task_item n=4 part=touches
Secret scan scope, and uploads gated on the secret scan: the new stamp dirs under the e2e-home root fall inside what the scan reads and what a failing job keeps.
@task_item n=5 part=touches
The zero-panics gate, the panic count reading home-level files only, and the schema conformance gate on real process output: each test home now begins as a copy of the stamp home, so whatever that home holds is read once per copy.
@task_item n=6 part=touches
verify and plugin install log only inside an instance, and no obs gate depends on verify lines: viola verify now runs once per built binary to take the stamp, not once per home.
@task_item n=7 part=touches
CI verify against the fake agent only, and real-CLI verify never in CI: the stamp is taken by viola verify against the fake agent binary, and the binaries are keyed into the stamp name.
@task_item n=8 part=touches
Diagnostics owner-only permissions and file modes: the copied homes are checked for identical bytes, length and mode against the stamp, with no shared file.
@task_item n=9 part=touches
Pinned copy placed, never replaced: each copied home carries its own pinned binary dir, which the stamp-isolation test checks as unshared.
@task_item n=10 part=touches
Recorded fixtures are no telemetry source, and Claude fixtures are scrub-required: a new fake-agent script was added, with synthetic text only.
@task_item n=11 part=touches
The wait log line outcome, and the wait-and-last channel-response outcome over the wake set: the wake set is unchanged, but a turn-ended record holding background tasks now provably ends the wait. Confirm these are true claims with no change.
@task_item n=12 part=touches
events log dialog-expired, a wake kind: the wake set is unchanged at 6 kinds, so this is a true claim with no change.
@task_item n=13 part=touches
viola-mcp stdio server tools, wait: MCP wait shares the CLI wake rule by construction, with no change to the tool.
@task_item n=14 part=touches
The wait span attributes (run.wait_dispatch, wait.block) and the wait and last span chain: the park path is unchanged, and only its tests moved.
@task_item n=15 part=touches
No sleep-based log waits: the new booted tests wait on event records, and the unit park waits are bounded by a 5 s receive timeout, not by a sleep.
@task_item n=16 part=touches
Emitted lines schema-checked in integration and E2E tests: two booted wait cases and two stamp tests join the integration set whose lines the gates read.
@task_fact n=1 slug=9-ci-integration
fact: Integration-test homes are copies of one viola verify stamp per built binary, kept under the same e2e-home root as the homes. A stamp dir holds no owner record, and a stamp of another build is swept only when its lock is free.
@task_item n=1 part=untouched
The diagnostics line schema, its fields, the event enum and the corr-by-event rows: no event, field or value is added or changed.
@task_item n=2 part=untouched
Detail codes, exit codes and refusal words (exit 1, exit 21, strict-modes, socket-dir, server-verify): none moved.
@task_item n=3 part=untouched
The send, dialog, wheel, link, hook and revive path rows, with their spans, lines and budgets.
@task_item n=4 part=untouched
The logging stack and init order, service identity, level gating, redaction, veil, the allow-list and the NEVER-log floor.
@task_item n=5 part=untouched
Error capture and panic rows (panic hook, catch sites, the detail-file line), and the absences: exporters, OTel, Sentry, network egress, metrics pipeline.
@task_item n=6 part=untouched
Perf and hook budgets, the heartbeat and staleness rows, and the SLO thresholds: no budget, threshold or floor moved.
