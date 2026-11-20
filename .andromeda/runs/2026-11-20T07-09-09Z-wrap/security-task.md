@task_prose part=session
The session makes a CLI-scheduled wake's prompt keep the driver's wheel under viola run: a keyless human-filed prompt is told as a fired wake and relabelled origin harness, with no wheel move, only when no key was typed on stdin since the last human-filed prompt, the last turn end counted at least one scheduled wake, and the CLI is verified; otherwise it takes the wheel as before.
The wrapper now keeps two pieces of in-memory state: a stdin typed mark, set by the stdin observer's key move and cleared only at the next human-filed prompt (never at a turn end or a release), and the scheduled-wake count recorded from each turn end, reset at session start and end.
The capability ledger gains a 26th relied row, stop-session-crons, which checks only that Stop carries session_crons as an array, and reads no element; the relied set moves 25 to 26 and the floor rows 21 to 22; viola verify gains no run, paste, wait or step.
Test-only additions: the fake agent's argv-only, capped --stop-crons and --wake-after-ms options, and the live rig's WS, HB and R3W labels, its hb arm and the wheel-after-fire reading; no IPC method, event kind, socket, env var, CLI flag or config key is added to the product.
Measured live on claude 2.1.328 under the rig's private config and scratch homes: WS 26/26 rows passed, HB read 22 dim hint cells in a fresh box with no send, and R3W saw one interval wake fire, read origin harness, the wheel kept and the next send answered; the user's live stamp home stayed unmoved at 24 rows.
The driver's own rule against setting a scheduled wake is not relaxed: the running product is the pinned one without this change, so it waits for the product re-pin.
@task_file n=1 path=src/run/wheel.rs
what: the wrapper's wheel state, which says who holds the wheel; the human takes it when they type on stdin; now also the stdin typed mark and the counted scheduled wakes
@task_file n=2 path=src/run/send/slot.rs
what: the wrapper's hook.event handling, which records hook events and moves the wheel on a human-filed prompt-submitted; now the told-wake rule that relabels a keyless prompt origin harness
@task_file n=3 path=src/run/send/relabel.rs
what: unit tests of the wrapper's prompt-submitted origin relabel and the driver's send claim
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger, the relied rows viola verify checks and stamps per claude version, and the floor rows
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the capability ledger rows
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit tests of each ledger row's check over captured hook payloads
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the ledger stamp: verified and unverified ids and the stamped pass and fail counts
@task_file n=8 path=src/cmd/verify.rs
what: viola verify, the command that runs the probes and writes the stamps file; only its tests and step-line counts changed here
@task_file n=9 path=src/run/version_gate.rs
what: viola run's version gate, which reads the stamps file to decide whether the CLI is verified; only its tests changed here
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent under the fake-agent feature, with its argv test seams; now the synthetic Stop session_crons and the one fired keyless wake
@task_file n=11 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness runner and its expected live verify step lines
@task_file n=12 path=crates/viola-e2e/src/rig/mod.rs
what: the live round rig: its arms, labels, private config and scratch homes, the kept stamp it seeds into its own home, and the closed words it records
@task_file n=13 path=crates/viola-e2e/src/rig/wake.rs
what: the live rig's wake arm: it fires a scheduled wake and reads viola's own wheel lines and origin word from the events log in its scratch home
@task_file n=14 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary, with its rehearse words and help text
@task_file n=15 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsals, which run the live rig against the fake agent
@task_file n=16 path=tests/cli_wheel.rs
what: integration tests of the wheel and the driver's sends and dialogs through the CLI
@task_file n=17 path=tests/tui_wheel.rs
what: end-to-end tests of the wheel through the outer terminal, with human keys and queued messages
@task_file n=18 path=tests/cli_verify.rs
what: integration tests of viola verify's step lines and stamped summary
@task_file n=19 path=tests/cli_verify/runs.rs
what: integration tests of viola verify's probe runs and failing steps
@task_file n=20 path=tests/cli_verify/stamps.rs
what: integration tests of the stamps viola verify writes
@task_file n=21 path=tests/support/verify.rs
what: test support: the spine set of synthetic hook payloads the fake agent replays for verify
@task_file n=22 path=tests/contract_ledger_probes.rs
what: contract test that stamps every recorded fixture set against the ledger rows
@task_item n=1 part=touches
decision/human-typing-takes-wheel gains an exception. A keyless human-filed prompt does not take the wheel when it is told as a fired scheduled wake: no stdin key typed since the last human-filed prompt, at least one counted wake at the last turn end, and a verified CLI.
@task_item n=2 part=touches
contract/wheel-event/human-input: a prompt told as a wake emits no human-input wheel move; every other human-filed prompt moves the wheel as before.
@task_item n=3 part=touches
contract/send-refusal/human-typing: after a fired wake, the driver's next send is no longer refused as human-typing, because the wheel stays with the driver.
@task_item n=4 part=touches
contract/wrapper-channel-methods/hook-event: hook.event now records a turn-ended event's scheduled_wakes count in the wrapper. An unclaimed human-filed prompt-submitted reads and clears the typed mark, and it may be relabelled origin harness.
@task_item n=5 part=touches
decision/stop-session-crons-count-only: the count now also decides the wheel rule; no element is read. The new ledger row checks only that session_crons is an array.
@task_item n=6 part=touches
decision/hook-payload-is-content, decision/upstream-text-is-content and decision/from-field-authenticates-nothing: the relabel is decided from wrapper state (the typed mark, the wake count, the verify stamp), never from prompt text or a self-reported field.
@task_item n=7 part=touches
decision/unverified-cli-gate and decision/dialog-answers-need-verify-stamp: the told-wake rule requires a verified CLI, so on an unverified CLI a fired wake takes the wheel. The ledger relied rows 25 to 26 and the floor rows 21 to 22 are values a row cell may state.
@task_item n=8 part=touches
decision/stdin-closed-non-editing-list: the stdin observer's existing classifier, unchanged, now also sets the typed mark under the same lock; the hook path's human move sets no mark.
@task_item n=9 part=touches
decision/stamps-written-only-by-verify and decision/version-gate-strict-stamps-read: the stamp gains the stop-session-crons row key, written by viola verify only. The user's live stamp home stayed unmoved, and its re-stamp is left to the product re-pin.
@task_item n=10 part=touches
decision/verify-signature-only, decision/verify-reads-only-signatures and decision/verify-never-answers-dialogs: viola verify gains a ledger check over a captured payload, with no new run, paste, wait or step.
@task_item n=11 part=touches
decision/test-seams-fake-agent-only, decision/test-seams-compiled-out and decision/release-build-no-test-binaries: the fake agent gains --stop-crons (0 to 4 synthetic elements) and --wake-after-ms (capped at 5000 ms), argv-only, with no env read and no fixture written.
@task_item n=12 part=touches
decision/new-env-read-needs-log-entry and decision/new-listener-needs-log-entry: the new seams read no env and add no listener or channel method, so no log entry is owed. A picker should confirm that no row claims one.
@task_item n=13 part=touches
decision/live-round-rig-keys-own-pty: the new HB arm types x and del into its own unwrapped pty's fresh box before any send, and reads cell attributes and counts only, never text.
@task_item n=14 part=touches
decision/live-rig-isolated-child and decision/live-rig-seed-from-bundle-logic: the HB arm runs unwrapped, trusted, marker-seeded and with no viola home. WS and R3W run under the rig's private config and scratch homes.
@task_item n=15 part=touches
decision/live-rig-kept-stamp-owner-only: the kept stamp is now one file per label pair, a wake stamp beside the context stamp, each seeded only into the rig's own home.
@task_item n=16 part=touches
decision/live-rig-prompt-capture-closed: R3W records the captured prompt's key names only, plus capture-source absent and the closed wake-origin and wheel-after-fire words (kept, human, other, unread).
@task_item n=17 part=touches
decision/input-box-readiness-from-literals and decision/verify-screen-read-against-literals: the HB arm requires the first frame to show the box up and no trust prompt, else it records refused shape.
@task_item n=18 part=touches
decision/driver-permission-policy-outside-viola: the driver's rule against setting a scheduled wake stays in force until the product re-pin; this wrap does not relax it.
@task_item n=19 part=touches
decision/controls-not-switchable and decision/no-control-switch-off: the told-wake rule adds no config key, env var or flag, so nothing switches the wheel rule.
@task_item n=20 part=touches
contract/wheel-event/human-input: a known false positive stands, unmeasured. A prompt submitted by the mouse alone, with no key typed, after a counted wake on a verified CLI reads as a wake and keeps the driver's wheel. A draft typed and then erased makes the next wake take the wheel once.
@task_fact n=1 slug=threat-model-summary
fact: A keyless human-filed prompt is told as a CLI-scheduled wake and relabelled origin harness, the wheel staying with the driver, only when no stdin key was typed since the last human-filed prompt, the last turn end counted at least one scheduled wake, and the CLI is verified; the typed mark clears only at the next human-filed prompt, and a mouse-only submitted prompt is the one named false positive, unmeasured.
@task_fact n=2 slug=threat-model-summary
fact: The driver's rule never to set a scheduled wake stays in force until the product re-pin, because the running pinned product does not hold the told-wake rule.
@task_fact n=3 slug=input-validation
fact: The stop-session-crons ledger row checks only that Stop's session_crons is an array, and a missing key reads fail; no element of session_crons is read anywhere, and its count feeds only the wheel rule.
@task_fact n=4 slug=input-validation
fact: The fake agent's --stop-crons (0 to 4 synthetic elements) and --wake-after-ms (capped at 5000 ms, fires once) are argv-only test seams under the fake-agent feature, read no env and write no fixture.
@task_fact n=5 slug=authentication-authorization
fact: The live rig's HB arm types x and del into a fresh box in its own unwrapped pty before any send, and records cell attributes and counts only, never screen text.
@task_fact n=6 slug=data-protection
fact: The live rig's R3W arm reads only viola's own wheel lines and origin word from the events log in its scratch home, and records the captured prompt's key names, never its values.
@task_item n=1 part=untouched
Socket, peer-euid, server-verification and socket-directory rows: no socket, IPC method, endpoint or peer check was added or changed.
@task_item n=2 part=untouched
Dependency security, CI workflows, toolchain pins, cargo-deny and zizmor rows: no dependency, workflow or tool pin changed.
@task_item n=3 part=untouched
Secret management, credential strip and child environment rows: no env var is read or stripped anew, and no credential path changed.
@task_item n=4 part=untouched
MCP tools, their inputs and their isError contracts: no MCP tool or argument changed.
@task_item n=5 part=untouched
Paste text character set, control-character and empty-text refusals: typed-text validation is unchanged.
@task_item n=6 part=untouched
CLI exit codes, error display, refusal words and channel error codes: none was added or moved.
@task_item n=7 part=untouched
Pinned binary, plugin install, trust record and marketplace rows: none is reached.
@task_item n=8 part=untouched
Revive, snapshot, link and unlink rows: none is reached.
@task_item n=9 part=untouched
config.json keys and configuration rows: no config key was added.
