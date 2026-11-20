@task_prose part=session
Under viola run, a keyless prompt that a scheduled wake fires is now told apart from a typed one, so the driver keeps the wheel and its sends and dialogs still work after the wake fires (matrix v2-36).
The human's stdin key now sets a typed mark on the wheel. A human-filed prompt-submitted that the driver did not claim is relabelled origin harness, with no wheel move, only when no key was typed, the last turn end counted at least one scheduled wake, and the CLI is verified. Otherwise the human takes the wheel as before.
The typed mark clears only at the next human-filed prompt, never at a turn end or a release. A message queued during a turn therefore still reads human. A prompt submitted by the mouse alone is the one named false positive, and it is unmeasured.
The capability ledger gains a 26th relied row, stop-session-crons, which checks that a Stop's session_crons is an array. The relied rows went from 25 to 26 and the floor rows from 21 to 22, so verify's step lines now read [NN/26].
The test-only fake agent gains two argv options, --stop-crons and --wake-after-ms. The test-only live rig gains the WS, HB and R3W labels and an hb arm, which reads hint-cell attributes in a fresh box before any send.
Measured live on claude 2.1.328 in a scratch home: WS gave 26/26. HB gave 22 hint cells, all dim, with typed text plain and the two told apart. R3W saw one fired wake with origin harness, the wheel kept and the second send ok. No IPC method, event kind, CLI flag, config key or terminal byte was added to the product.
@task_file n=1 path=src/run/wheel.rs
what: the wheel, which records who holds the human or driver turn. It now also keeps the typed mark from human keystrokes and the scheduled-wake count from the last turn end.
@task_file n=2 path=src/run/send/slot.rs
what: the hook-event filer. It now holds the told-wake rule that relabels an unclaimed keyless prompt-submitted as origin harness instead of letting it take the wheel.
@task_file n=3 path=src/run/send/relabel.rs
what: the prompt-submitted origin relabel and its unit cases. It gains the case for the keyless human prompt after a counted wake.
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger's relied rows. It gains stop-session-crons as the 26th row.
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's unit cases.
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger row check cases. It gains the stop-session-crons pass and fail cases.
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger stamp cases. Their row counts moved to 26.
@task_file n=8 path=src/cmd/verify.rs
what: viola verify, whose tests pin its step lines and its stamped pass/fail summary. The counts moved to 26.
@task_file n=9 path=src/run/version_gate.rs
what: the version gate's tests over the stamped ledger rows. The counts moved to 26.
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent. It gains synthetic session_crons on Stop and a once-fired keyless wake prompt.
@task_file n=11 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run helpers, which hold the live verify step-line expectation at 26 rows.
@task_file n=12 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig. It gains the labels, the hb arm that reads hint-cell attributes in a fresh box before any send, and the wake-stamp keep or seed.
@task_file n=13 path=crates/viola-e2e/src/rig/wake.rs
what: the live rig's wake leg. It now reads viola's own wheel lines after a fired wake (kept, human, other or unread) and a second send.
@task_file n=14 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary. It gains the ws, r3w and hb rehearsal words and their help text.
@task_file n=15 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent. It gains the WS, HB and R3W rehearsals.
@task_file n=16 path=tests/cli_wheel.rs
what: the CLI wheel integration cases. It gains the Path 5 cases: a fired wake keeps the driver wheel, a keyless prompt with no counted wake takes the wheel, and an unverified CLI takes the wheel.
@task_file n=17 path=tests/tui_wheel.rs
what: the outer-PTY TUI wheel boundary cases. It gains: a human key before a fired wake takes the wheel, and a message queued during a turn reads human after the turn ends, with or without a release between.
@task_file n=18 path=tests/cli_verify.rs
what: the viola verify output cases, covering step lines and stamped summaries. The counts moved to 26.
@task_file n=19 path=tests/cli_verify/runs.rs
what: the viola verify run cases. Their step and failing counts moved to 26.
@task_file n=20 path=tests/cli_verify/stamps.rs
what: the viola verify stamp cases. A set without a Stop now reads 7 pass 19 fail.
@task_file n=21 path=tests/support/verify.rs
what: the verify test support spine set. It gains a synthetic Stop with session_crons.
@task_file n=22 path=tests/contract_ledger_probes.rs
what: the ledger probe contract over the stamped fixture sets. They now stamp 26 pass 0 fail.
@task_item n=1 part=touches
decision/human-key-takes-the-wheel: a human key still takes the wheel, and it now also sets a typed mark. A keyless prompt-submitted after a counted wake on a verified CLI, with no key typed since the last human-filed prompt, no longer takes the wheel. This is a new exception to the rule.
@task_item n=2 part=touches
decision/critical-path-human-takes-wheel: the human-takes-the-wheel path now has the told-wake exception. Its cases include the outer-PTY cases where a key before a fired wake takes the wheel and a queued message reads human.
@task_item n=3 part=touches
decision/no-screen-content-parsing: the rig's HB arm reads hint-cell and typed-cell attributes in a fresh box before any send. This is the planned addition to this decision's existing exception.
@task_item n=4 part=touches
decision/live-readings-never-gate: the new live HB, WS and R3W readings were recorded as readings only. HB's hint-cells-attr dim(22), typed-attr plain and hint-told-apart yes gate nothing.
@task_item n=5 part=touches
decision/boundary-check-never-inspects-claude-content: HB reads cell attributes and counts only, never text, in a fresh box. R3W reads viola's own wheel lines and origin word.
@task_item n=6 part=touches
decision/stdin-classifier-non-editing-list: the stdin classifier itself is unchanged, but its human move now also sets the typed mark. Check that the row still holds.
@task_item n=7 part=touches
decision/boundary-tests-sequenced-on-records: the new outer-PTY wheel cases are sequenced on wheel and prompt-submitted records.
@task_item n=8 part=touches
decision/send-refused-while-human-typing: a typed key now marks the slot, and a keyless wake prompt only stays harness when no key was typed. Check that the row's send-refusal claim still holds next to the told-wake rule.
@task_item n=9 part=touches
contract/viola-verify/step-line: verify's step lines now read [NN/26], and the stamped summary reads 26 pass 0 fail. This touches the row if its cell holds the 25 count.
@task_item n=10 part=touches
contract/tui-passthrough/human-keystrokes: human keystrokes still pass through unblocked. They now also mark the wheel slot as typed, which decides whether a later keyless prompt is a told wake.
@task_item n=11 part=touches
decision/wrapped-tui-reads-as-unwrapped and decision/zero-viola-bytes-on-the-terminal: the told-wake rule adds no terminal byte. Re-read them only to confirm the wheel change wrote nothing to the terminal.
@task_item n=12 part=touches
decision/tui-boundary-assertions-only and decision/portable-pty-outer-pty-driver: the new told-wake and queued-message cases are outer-PTY boundary assertions driven by the same driver.
@task_fact n=1 slug=keyboard-test-harness
fact: A keyless prompt-submitted that the driver did not claim is a told wake and is relabelled origin harness with no wheel move only when three things hold: no human key was typed since the last human-filed prompt, the last turn end counted at least one scheduled wake, and the CLI is verified. Otherwise it takes the wheel for the human.
@task_fact n=2 slug=keyboard-test-harness
fact: The typed mark set by a human key clears only at the next human-filed prompt, never at a turn end or a release. So a message queued during a turn reads human, and a draft typed then erased makes the next wake take the wheel once. A prompt submitted by the mouse alone, with no key, is the one named false positive, and it is unmeasured.
@task_fact n=3 slug=11-a11y-anti-patterns-never-do-these
fact: The no-screen-content-parsing exception also covers the live rig's fresh-box reading before any send. It reads the dim hint cells against the plain typed cells by attribute and count only, never text, and the reading gates nothing.
@task_item n=1 part=untouched
CLI refusal layout, hint lines, reason codes, exit codes and the --json failure documents: no refusal, code word, hint or exit code was added or moved.
@task_item n=2 part=untouched
The viola list board, its caption, colour, dim stale rows, NAME truncation and the SGR rules: no board output or colour changed.
@task_item n=3 part=untouched
The revive, link, unlink and plugin install contracts and the start refusals: none of their outputs changed.
@task_item n=4 part=untouched
The control-character escaping and C0/C1 rules: no printed string changed.
@task_item n=5 part=untouched
The a11y CI integration, gate, flakiness and suite placement rows: no CI step, suite or a11y driver was added.
@task_item n=6 part=untouched
The scope-summary absences (i18n, touch surface, web or graphical frontend, rendered page) and the screen-reader setup rows: nothing here adds a frontend or changes the screen-reader targets.
@task_item n=7 part=untouched
The time-bound rows and dialog hold: the driver's dialog flow after a fired wake is kept, and no timeout or dialog_hold_s value moved.
