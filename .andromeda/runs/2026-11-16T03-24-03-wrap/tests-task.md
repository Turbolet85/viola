@task_prose part=session
The session documented and pinned the wait wake condition with tests alone: no product symbol, IPC method, event kind, data key, env var or socket changed, and the wake set stays 6 kinds.
It pinned that every turn-ended wakes a parked wait whatever its background_tasks (unit cases zero, one, three and null, and a booted case holding 2 tasks with a closing question), and that no false wake comes from the Notification activity or a harness-origin prompt-submitted after a task-free turn end.
It added a fake agent turn script with five gated steps (a held turn end, a default stop, a notification, a harness-injected prompt, a default stop), synthetic text only.
It changed the stamped-home fixture: one viola verify stamp per built binary, taken once under an exclusive lock and copied into each test's home, never linked; the stamp dir carries no owner record and another build's stamp is swept only when its lock is free.
It measured the stamped-home class under the unchanged ci kills: rest max 10.913 s under the 20 s kill, verify_window_ max 43.358 s under the 60 s kill, 321 tests on both sides, class wall 56.069 to 48.381 s; CI read green 7 of 7 and the carried CI timing red closed.
Nothing retired; a carry stands: two older wait unit tests still hang past a failure under a clock that never moves, owned by a later route entry.
@task_file n=1 path=src/run/wait.rs
what: the wait verb's park-and-wake over the instance event log and its inline unit tests; edited in the test module only (an rstest case table of turn-end wakes by background_tasks, a task-free-end park test, park waits bounded by a 5 s bound)
@task_file n=2 path=fixtures/fake-scripts/wait-held-turn.json
what: a fake agent turn script (fake-script format, gated steps) driving a held turn end with background tasks, then a notification and a harness-injected turn; new
@task_file n=3 path=tests/cli_wait_last.rs
what: the viola wait and viola last E2E suite, the CLI run as a subprocess over the fake agent; edited with two booted path3 wait cases (wake on a turn end holding background tasks, no false wake after a task-free end)
@task_file n=4 path=tests/support/home.rs
what: the shared test helper holding the stamped-home rstest fixture chain and the per-test home under target/e2e-home; edited so each home copies one verify stamp per built binary, taken under a lock, with the sweep of other builds' stamps
@task_file n=5 path=tests/cli_verify.rs
what: the viola verify integration suite; edited with two root tests that stamped-home copies share no file with the stamp or each other and that one stamp serves every home
@task_file n=6 path=.config/nextest.toml
what: the nextest configuration holding the test layer filtersets and the per-test kill lines of the ci profile; edited in the verify-driven override's comment only, no kill, filter or period moved
@task_item n=1 part=touches
contract/viola-wait/non-waking-records: the activity and wheel records row does not name the harness-origin prompt-submitted nor the Notification activity after a task-free turn end; both are now pinned non-waking by a booted case and a unit test.
@task_item n=2 part=touches
contract/viola-wait/wake, contract/viola-wait/turn-ended, contract/wait/wake-kinds and contract/channel-wait/wake-kinds: the wake set is unchanged at 6 kinds, but every turn-ended now provably wakes whatever its background_tasks (0, 1, 3, null, and 2 booted); a row restating the wake set may want that clause.
@task_item n=3 part=touches
contract/turn-ended/background-tasks, contract/event-data/turn-ended and contract/dialog-data/null-details: the wake rule reads only the event kind, so background_tasks (a count or null) never gates a wake; no scheduled-wake count was added and the turn-ended data keys are unchanged.
@task_item n=4 part=touches
decision/fresh-home-per-test, decision/per-test-fresh-home, decision/per-test-home and decision/tests-create-own-home: each stamped home is now a copy of one viola verify stamp per built binary, never a link, in place of a verify run per home.
@task_item n=5 part=touches
decision/rstest-fixture-chain, decision/fixture-chain-exists-once and decision/rstest-fixtures-for-constructors: the stamped_home fixture takes the stamp once per binary and copies it; its fields and its 91 consumers are unchanged.
@task_item n=6 part=touches
decision/verify-sole-stamp-writer, decision/stamps-written-only-by-verify, decision/stamps-only-from-verify, decision/stamps-from-fake-agent-verify, decision/fake-agent-replay-for-stamps, decision/single-writer-state-files and decision/single-writers: the stamp is still taken only through viola verify against the fake agent; the copy into a home writes no stamp.
@task_item n=7 part=touches
decision/guarded-paths-start-stamped: check-guarded cases still start stamped, now from the copied stamp.
@task_item n=8 part=touches
decision/ci-stamped-home-kill-line and decision/verify-driven-tests-20s-kill: the 20 s kill is unchanged, its rationale moves to the stamp taken once per binary; measured rest max 10.913 s under 20 s.
@task_item n=9 part=touches
decision/verify-window-60s-kill: the 60 s kill for the verify_window_ class holds unchanged; measured max 43.358 s.
@task_item n=10 part=touches
decision/test-home-owner-record-sweep, decision/owner-record-removed-last, decision/e2e-homes-under-one-root, decision/e2e-homes-under-target-e2e-home and decision/e2e-home-base-checks: a new stamp dir lives under target/e2e-home with no owner record, and stamps of other builds are swept only when their lock is taken; the owner-record sweep rule may want that exception.
@task_item n=11 part=touches
decision/bounded-remove-the-guard-witness: a remove-the-guard control on a park test needs a bounded park wait and an entry that runs only the guard's cases; a waiter with no deadline hangs the entry past the bound (120 s timeout against 2.21 s on the filtered entry).
@task_item n=12 part=touches
decision/no-test-retries-or-sleeps, decision/no-sleep-synchronization, decision/waits-key-on-events, decision/root-wait-bound and decision/within-7s-wait-bound: the booted wait cases wait on logged events, and the unit park waits are bounded by a 5 s receive timeout.
@task_item n=13 part=touches
decision/injected-clock, decision/test-clocks-behind-clock-seam and decision/zero-flakiness: two older wait unit tests still hang past a failure, joining a waiter parked on a huge timeout under an injected clock that never moves; a carry, not fixed.
@task_item n=14 part=touches
contract/fake-script/format and contract/fake-script/gated-step: a new committed script uses five gated steps across Stop, Notification and UserPromptSubmit.
@task_item n=15 part=touches
decision/synthetic-fixture-text, decision/scrubbed-fixtures and decision/fixtures-never-from-real-prompts: the new fake agent script carries synthetic text only.
@task_item n=16 part=touches
decision/harness-turn-keeps-the-wheel, decision/injected-turn-keeps-wheel, contract/prompt-submitted/origin-harness and contract/events/harness-prompt: a harness-origin prompt-submitted is now also pinned as never waking a parked wait.
@task_item n=17 part=touches
decision/e2e-for-critical-paths, decision/e2e-over-fake-agent and decision/cli-tests-as-subprocess: path3 gains two booted wait cases over the fake agent.
@task_item n=18 part=touches
decision/cli-mcp-identical-payloads and contract/mcp-cli-parity/refusal: MCP wait holds by construction, its code untouched; the parity witness ran in pre-push.
@task_item n=19 part=touches
decision/shared-e2e-helpers and decision/one-shared-test-helper: the stamp, copy-tree and file-hash helpers were added once to the shared test support, not copied.
@task_item n=20 part=touches
decision/no-shared-mutable-state and decision/tests-run-in-parallel: one stamp is shared across parallel tests behind an exclusive lock, and the copies share no file with it or each other.
@task_item n=21 part=touches
decision/test-layer-filtersets and decision/os-gated-cases-in-binary-root: the two stamp tests sit at the verify suite's root because a test(=name) filter matches no test inside a module.
@task_fact n=1 slug=e2e-test-strategy
fact: A turn-ended wakes a parked viola wait whatever its background_tasks (zero, any count, or null); telling a finished session from a paused one is the driver's, and viola carries no scheduled-wake count.
@task_fact n=2 slug=test-data-bootstrap
fact: Stamped test homes copy one viola verify stamp per pair of built binaries, kept under target/e2e-home as viola-stamp-<key16>/home keyed by the binaries' hashes and taken under an exclusive lock on its .lock sibling.
@task_fact n=3 slug=test-data-bootstrap
fact: The stamp dir carries no owner record; another build's stamp is swept only when its lock is taken without waiting.
@task_fact n=4 slug=test-data-bootstrap
fact: Each stamped home is a copy of the stamp, never a link: no file is shared between the stamp and any home, or between two homes.
@task_item n=1 part=untouched
Send, send confirmation, answer, release and dialog rows: no send, answer or dialog path changed.
@task_item n=2 part=untouched
MCP tool rows (tools list, invalid-params fields, tool results, wait tool bound): no MCP code or schema moved.
@task_item n=3 part=untouched
CI workflow, artifact, supply-chain, fuzz, coverage, mutation and performance gate rows: no workflow, gate, threshold or budget changed.
@task_item n=4 part=untouched
Hook exec, plugin, strict-modes, socket-directory, peer euid and server identity security rows: no security control moved.
@task_item n=5 part=untouched
Harness CLI contract rows (boot, cleanup, status, run document, pre-push detail and reasons, supervisor): the harness and its shim are unchanged.
@task_item n=6 part=untouched
Process log line, log corr and diagnostics schema rows: no log field or event was added.
@task_item n=7 part=untouched
viola run start, revive, list board and exit-code rows: no start, revive or list path changed.
@task_item n=8 part=untouched
Absence rows (browser suite, database, outbound HTTP, Windows and macOS, deploy stage): nothing was added that fills them.
