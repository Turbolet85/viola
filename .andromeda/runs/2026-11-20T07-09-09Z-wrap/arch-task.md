@task_prose part=session
Under viola run, a keyless prompt-submitted that Claude Code fires for a CLI-scheduled wake is now told apart from a prompt the human typed, so the driver keeps the wheel, its next send and its dialogs (matrix v2-36).
The wrapper keeps a typed mark set by every human key the stdin observer passes; it clears only when a human-filed prompt takes it, never at a turn end or a release; hook-path human moves set no mark.
A human-filed prompt-submitted the in-flight send did not claim is relabelled origin harness with no wheel move only when no key was typed, the last turn end counted at least one scheduled wake, and the CLI is verified; otherwise it takes the wheel as before.
The capability ledger gains its 26th relied row, stop-session-crons (Stop carries session_crons as an array, checked by is_array only); relied rows 25 to 26, the floor row set 21 to 22; no verify run, step, IPC method, event kind, CLI flag, env var or config key added.
The test-only fake agent gains argv-only, capped options for synthetic Stop crons and one fired keyless wake; the live rig gains the WS, HB and R3W labels and their rehearsals.
Measured live on claude 2.1.328 under a scratch home: one interval-form wake fired, read origin harness with the wheel kept and the next send ok; the stop-session-crons row passed at 26 of 26; the live home stamp left unmoved.
@task_file n=1 path=src/run/wheel.rs
what: the wheel held in wrapper memory (the wheel slot): the human key move, the new typed mark and the scheduled-wakes count
@task_file n=2 path=src/run/send/slot.rs
what: the hook-event append path in the wrapper: prompt-submitted origin relabel, the turn-ended scheduled-wakes count and the new told-wake rule
@task_file n=3 path=src/run/send/relabel.rs
what: the driver relabel of prompt-submitted origin; tests only, a told-wake case added
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger rows of the Claude agent crate: the relied row set and each row's check
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger unit cases
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the capability ledger row-check cases
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger stamps cases: verified versions, row counts and stamped summaries
@task_file n=8 path=src/cmd/verify.rs
what: viola verify, the one stamps writer; tests only, its step lines and summary moved to 26 rows
@task_file n=9 path=src/run/version_gate.rs
what: the version gate that reads stamps strictly; tests only, row counts moved
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent binary: two new argv-only seams for Stop session_crons and a fired keyless wake
@task_file n=11 path=crates/viola-e2e/src/harness/run.rs
what: the test harness run of the test-only e2e crate: the live verify pass text at 26 rows
@task_file n=12 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig: its labels, arms, stamp shapes and summary words
@task_file n=13 path=crates/viola-e2e/src/rig/wake.rs
what: the live rig's wake reading: the wheel after a fired wake
@task_file n=14 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the viola-live-rig binary: rehearsal words and help text
@task_file n=15 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsals against the fake agent
@task_file n=16 path=tests/cli_wheel.rs
what: the wheel integration cases through the CLI: fired-wake cases added
@task_file n=17 path=tests/tui_wheel.rs
what: the wheel end-to-end cases through the outer PTY: typed key, queued message and fired-wake cases
@task_file n=18 path=tests/cli_verify.rs
what: viola verify integration cases: step lines and summary at 26 rows
@task_file n=19 path=tests/cli_verify/runs.rs
what: viola verify run cases: the step and failing sets
@task_file n=20 path=tests/cli_verify/stamps.rs
what: viola verify stamped-summary cases
@task_file n=21 path=tests/support/verify.rs
what: the verify test support spine payload set, a synthetic Stop with session_crons added
@task_file n=22 path=tests/contract_ledger_probes.rs
what: the ledger probe contract check over the recorded fixture sets, each stamped 26 pass 0 fail
@task_item n=1 part=touches
decision/human-prompt-confirms-wheel-take gains its exception: a keyless human-filed prompt with no typed mark, a counted scheduled wake and a verified CLI is a told wake, relabelled harness, and moves no wheel.
@task_item n=2 part=touches
decision/byte-source-wheel-claim and decision/wheel-byte-source-claim: a stdin human key now also sets the typed mark, which lasts until the next human-filed prompt takes it, never cleared by a turn end or a release.
@task_item n=3 part=touches
contract/event-data/prompt-submitted-origin and contract/event-prompt-submitted/origin: the wrapper now relabels a told wake origin harness.
@task_item n=4 part=touches
contract/event-data/turn-ended-scheduled-wakes: the count is now read by the wrapper's wheel rule (session-start and session-end reset it); still no wait wake condition.
@task_item n=5 part=touches
decision/capability-ledger, decision/cli-capability-ledger and decision/capability-ledger-gate: the relied set gains stop-session-crons, 25 to 26 rows; any cell stating the relied count or the floor row set (21 to 22) moves.
@task_item n=6 part=touches
decision/version-verified-all-relied-rows: a version now verifies only with all 26 relied rows counted; a stamp of 25 rows reads the new id unverified.
@task_item n=7 part=touches
contract/viola-verify-output/step-line and contract/viola-verify-output/summary-line: step lines read [NN/26] and a clean summary reads 26 pass 0 fail.
@task_item n=8 part=touches
contract/ledger-stamps/rows: the stamp gains the stop-session-crons row key, written by viola verify only.
@task_item n=9 part=touches
decision/harness-prompts-not-human and decision/harness-prompt-prefixes: a second road to harness origin beside the raw prefix, the told-wake rule over a keyless prompt.
@task_item n=10 part=touches
decision/human-always-wins-wheel and decision/wheel-classifier-only-observes: a human-filed prompt can now leave the wheel with the driver; a typed key still takes it, and a prompt submitted by the mouse alone is the one named false positive, unmeasured.
@task_item n=11 part=touches
contract/hook-event/driver-relabel and contract/channel-methods/hook-event-driver-origin: driver-claimed and hook-harness prompts neither read nor clear the typed mark.
@task_item n=12 part=touches
decision/wheel-in-wrapper-memory: the wheel slot now also holds the typed mark and the last scheduled-wakes count, in memory only.
@task_item n=13 part=touches
decision/fake-agent-test-seams and decision/test-seams-not-configuration: the fake agent gains two argv-only seams, no env read, capped (0 to 4 crons, a wake at most 5000 ms after).
@task_item n=14 part=touches
name/binary/viola-live-rig and decision/e2e-test-only: the rig gains the hb arm, the WS, HB, HB-2, R3W and R3W-2 labels and the ws, r3w and hb rehearsal words; still test-only.
@task_item n=15 part=touches
decision/run-never-reads-screen-content and decision/no-live-claude-in-ci: the rig's HB arm reads a fresh box's cell attributes before any send, local-only and never in CI.
@task_fact n=1 slug=established-decisions
fact: Under viola run a keyless human-filed prompt-submitted the in-flight send did not claim is a CLI-fired scheduled wake, relabelled origin harness with no wheel move, only when no human key was typed since the last human-filed prompt, the last turn end counted at least one scheduled wake and the CLI is verified; otherwise it takes the wheel.
@task_fact n=2 slug=established-decisions
fact: The stdin typed mark clears only when a human-filed prompt takes it, never at a turn end or a release: a message queued during a turn reads human, and a draft typed and erased makes the next wake take the wheel once.
@task_fact n=3 slug=design-philosophy
fact: The relied ledger row stop-session-crons checks only that Stop carries session_crons as an array; no element is read, and a missing key reads fail.
@task_fact n=4 slug=occupied-resources
fact: The fake agent's wake seams are argv-only options, --stop-crons and --wake-after-ms, capped and synthetic, never an env variable or fixture.
@task_item n=1 part=untouched
MCP tools, their results and the CLI verbs, exit codes and --json documents: no verb, flag, code or document shape changed.
@task_item n=2 part=untouched
The wrapper channel frames, methods, fault codes and protocol version: no method or parameter added.
@task_item n=3 part=untouched
Dialog hold, answer and hook tiers and exit bodies: dialogs reach the driver as before.
@task_item n=4 part=untouched
Snapshots, liveness, revive, list and the board: no field or read rule moved.
@task_item n=5 part=untouched
Stack, dependencies, toolchain, cargo-deny, lint bans and the CI workflow: no dependency, step or job changed.
@task_item n=6 part=untouched
Trust writer, socket directory checks, strict-modes rules, child argv and environment, and config.json keys.
@task_item n=7 part=untouched
Hyprland seam, fuzzing, release and deployment rows.
