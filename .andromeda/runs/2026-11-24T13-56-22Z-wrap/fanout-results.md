# Fan-out results — 2026-11-24-switching-a-session-s-model-and-effort (wrap 2026-11-24T13-56-22Z-wrap)

architecture: add 8 · set 10 · retire 0 · none 1 · ops-architecture.md
security-plan: add 8 · set 4 · retire 0 · none 0 · ops-security-plan.md
design-system: add 6 · set 10 · retire 0 · none 2 · ops-design-system.md
layout-templates: add 5 · set 4 · retire 0 · none 1 · ops-layout-templates.md
test-plan: add 11 · set 4 · retire 1 · none 1 · ops-test-plan.md
obs-plan: add 2 · set 7 · retire 0 · none 3 · ops-obs-plan.md
a11y-plan: add 6 · set 4 · retire 0 · none 1 · ops-a11y-plan.md

Check 5 (expected amendments): every entry of the plan's list is met by an operation below, save test-plan `contract/viola-help/verb-table`, layout-templates `contract/viola-help/verb-groups` and a11y-plan `contract/cli-help/served-verbs-only`, each read whole (`rec.py get`): none lists the setup verbs, each holds as written — no operation owed; obs-plan `contract/process-exit-detail/exit-1-switch` is not carried (the report: switch has no exit-1 refusal of its own; obs-plan's `@row_none` F6 agrees).
Check 6 (disproved claims): «respawns … in the same PTY» stands in the plan's Goal alone — no master site (`same-pty` 0, the pre-pass control refusal); disposed: no master operation — the plan is the chunk's own document and the report records the measured shape (a new PTY pair under the same host terminal); the a11y-plan add that restated it read `not_master type 4`.
Check 3 (intent): the scope record's six lines (companion 3 · in-intent 3 · widening 0) each serve their named file; no line outside the intent.

ops-architecture.md
decision/send-never-queued · apply — check 1: the report's fact, a send refused turn-running while a switch stands (fct-ionzjh33) · word: delegate overseer, 2026-11-24, fork F3 arm 1
decision/launch-pair-handed-not-verified · apply — check 1: a switch hands the respawned child a pair, unverified on the real CLI (fct-bs3a4rfe) · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/hook-event/appended-line · apply — check 6: the report's insufficient fix, the one-shot late session-end screen as built; its limit is the R6a carry's · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/refusal-reason/not-delivered · apply — check 1: the four switch details as built (fct-xvxcbpio) · word: delegate overseer, 2026-11-24, fork F3 arm 1
contract/wrapper-event-appends/launch · apply — check 5: the plan's expected amendment («once at every start and once at every switch», PROVISIONAL) · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/wrapper-event-appends/window · apply — check 1: the report's Schema / config bullet (the window line again after a switch's launch line) · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/instance-snapshot/launch · apply — check 1: the report's Schema / config bullet (the snapshot's launch rewritten by a switch) · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/instance-snapshot/child-pid · apply — check 1: the report's Schema / config bullet (child_pid rewritten by a switch) · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/child-argv/resume · escalate → Q1 — check 1: boundary widening F1 (rul-r74hshif, rul-p2zl2kgl), a new crossing of the session id onto a respawned child's argv · resolved: apply (Q1) — delegate overseer, 2026-11-24: confirmed PROVISIONAL on fork F1 arm 1, the founder's answer owed (inputs#I7)
contract/claude-child-argv/resume · escalate → Q1 — check 1: boundary widening F1 (rul-r74hshif, rul-p2zl2kgl), the same crossing · resolved: apply (Q1) — delegate overseer, 2026-11-24: confirmed PROVISIONAL on fork F1 arm 1, the founder's answer owed (inputs#I7)
decision/switch-in-place · escalate → Q1 — check 1: boundary widening F1 (rul-r74hshif, rul-p2zl2kgl), a wrapper ending its own live child and resuming in place · resolved: apply (Q1) — delegate overseer, 2026-11-24: confirmed PROVISIONAL on fork F1 arm 1, the founder's answer owed (inputs#I7)
decision/switch-refused-never-queued · reject — check 2: the fact's home is security-plan decision/switch-refusal-order
decision/switch-cli-only · apply — check 1: F5 as built, provisional; the one home of the five offered (architecture's resource surface) · word: delegate overseer, 2026-11-24, fork F5 arm 1
decision/respawn-reap-bound · reject — check 2: the fact's home is a11y-plan decision/respawn-reap-bound
decision/switch-writes-no-user-settings · apply — check 5: the chunk's acceptance as built (switch_keeps_the_user_settings_byte_and_mtime_equal)
contract/channel-methods/switch · escalate → Q1 — check 1: boundary widening F1 (rul-r74hshif, rul-p2zl2kgl), a new wrapper channel method · resolved: apply (Q1) — delegate overseer, 2026-11-24: confirmed PROVISIONAL on fork F1 arm 1, the founder's answer owed (inputs#I7)
contract/cli-verbs/switch · apply — check 5: the plan's expected amendment (a `switch` row)
name/command/switch · apply — check 1: the occupied command name

ops-security-plan.md
decision/switch-ends-only-its-own-child · escalate → Q1 — check 1: boundary widening F1 (rul-r74hshif, rul-p2zl2kgl); the plan's expected amendment (PROVISIONAL, F1) · resolved: apply (Q1) — delegate overseer, 2026-11-24: confirmed PROVISIONAL on fork F1 arm 1, the founder's answer owed (inputs#I7)
contract/wrapper-channel-methods/switch · escalate → Q1 — check 1: boundary widening F1 (rul-r74hshif, rul-p2zl2kgl), a new wrapper channel method · resolved: apply (Q1) — delegate overseer, 2026-11-24: confirmed PROVISIONAL on fork F1 arm 1, the founder's answer owed (inputs#I7)
contract/channel-frames/switch-params · apply — check 1: the method's params re-checked in the wrapper, as built (fct-xvxcbpio) · word: delegate overseer, 2026-11-24, fork F1 arm 1
decision/switch-refusal-order · apply — check 1: F3 as built, provisional; the one home of the five offered · word: delegate overseer, 2026-11-24, fork F3 arm 1
decision/launch-pair-kept-to-snapshot-and-event-log · apply — check 1: one launch line per switch, nothing in the user's settings (fct-vzga25j6) · word: delegate overseer, 2026-11-24, fork F1 arm 1
decision/switch-cli-only · reject — check 2: the fact's home is architecture decision/switch-cli-only
decision/respawn-reap-bound · reject — check 2: the fact's home is a11y-plan decision/respawn-reap-bound
contract/send-refusal/turn-running · reject — check 2: the fact's home is architecture decision/send-never-queued
decision/switch-method-behind-ipc-controls · escalate → Q1 — check 1: boundary widening F1 (rul-r74hshif, rul-p2zl2kgl); the plan's expected amendment (the Security Decisions Log entry, PROVISIONAL, F1) · resolved: apply (Q1) — delegate overseer, 2026-11-24: confirmed PROVISIONAL on fork F1 arm 1, the founder's answer owed (inputs#I7)
decision/launch-flags-checked · apply — check 5: the plan's expected amendment (`switch` named)
decision/cli-frame-verbs-strict-modes-first · apply — check 5: the plan's expected amendment (`switch` named)
contract/cli-exit-codes/12 · apply — check 1: switch refused unverified-cli at exit 12, as built · word: delegate overseer, 2026-11-24, fork F3 arm 1

ops-design-system.md
contract/help-output/verb-groups · apply — check 5: the plan's expected amendment (`switch` in `setup:`)
contract/viola-help/verb-groups · apply — check 5: the plan's expected amendment (`switch` in `setup:`)
contract/cli-exit-codes/13 · apply — check 1: the four not-delivered details at exit 13, as built · word: delegate overseer, 2026-11-24, fork F3 arm 1
contract/cli-refusal-line/fixed-unable-form · apply — check 1: switch's exit-21 strict-modes refusal through client::refuse_untrusted (fct-pmolmu2p)
contract/cli-stderr/fixed-message-refusal · apply — check 1: the same refusal (fct-pmolmu2p)
contract/cli-strict-modes-refusal/instance-state-pair · apply — check 1: the same refusal (fct-pmolmu2p)
contract/strict-modes-refusal/exit-21 · apply — check 1: the same refusal (fct-pmolmu2p)
contract/frame-verb-refusals/strict-modes · apply — check 1: the same refusal (fct-pmolmu2p)
contract/exit-21-json-cause/strict-modes-failed · apply — check 1: six frame verbs' strict-modes refusals become seven with switch
contract/exit-21-json-cause/server-verify-failed · apply — check 1: eight frame-writing verbs become nine with switch (client::connect)
name/command/switch · reject — check 2: the fact's home is architecture name/command/switch
contract/cli-verb-output/switch · reject — check 2: the fact's home is layout-templates contract/viola-switch-output/switched
decision/switch-refusal-order · reject — check 2: the fact's home is security-plan decision/switch-refusal-order
contract/viola-send-refusal/switch-standing · reject — check 2: the fact's home is test-plan contract/viola-send/turn-running
decision/switch-cli-only · reject — check 2: the fact's home is architecture decision/switch-cli-only
decision/switch-keeps-the-slot · reject — check 2: the fact's home is architecture decision/switch-in-place (and a11y-plan decision/switch-respawn-same-terminal for the terminal)

ops-layout-templates.md
contract/cli-verbs/help · apply — check 5: the plan's expected amendment (`switch` in `setup:`)
contract/viola-help/setup-row · apply — check 5: the plan's expected amendment (`switch` in `setup:`)
contract/frame-verbs/strict-modes-refused · apply — check 1: switch's strict-modes refusal first (fct-pmolmu2p)
contract/cli-verbs/run · apply — check 1: viola writes no byte to the terminal at a respawn (the report's coverage row) · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/cli-verbs/switch · apply — check 5: the plan's expected amendment (the verb's row in the CLI layouts) · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/viola-switch-output/switched · apply — check 1: the verb's human line as built (human::write_switched); the one home of the output line
contract/viola-switch-output/json-ok · apply — check 1: the verb's --json document as built; the one home of the document
decision/switch-refusal-order · reject — check 2: the fact's home is security-plan decision/switch-refusal-order
decision/switch-cli-only · reject — check 2: the fact's home is architecture decision/switch-cli-only

ops-test-plan.md
contract/viola-send/turn-running · apply — check 1: a send refused turn-running while a switch stands (fct-ionzjh33) · word: delegate overseer, 2026-11-24, fork F3 arm 1
contract/json-rpc-errors/-32602 · apply — check 1: the switch method's invalid params -32602 with data null, as built
decision/frame-verbs-check-instance-first · apply — check 1: eight frame verbs become nine with switch (fct-pmolmu2p)
contract/exit-21-detail/socket-dir-refused · apply — check 1: eight frame verbs become nine with switch
decision/unverified-cli-gates-answer-only · reject — check 7: its heir below stands as decision/unverified-cli-send-only; main's own retire (heir that row) replaces it, ops-main-test-plan.md
decision/unverified-cli-gates-answer-and-switch · reject — check 7: stands as decision/unverified-cli-send-only (fit); main sets that row, ops-main-test-plan.md
decision/switch-refusal-order · reject — check 2: the fact's home is security-plan decision/switch-refusal-order
decision/respawn-reap-bound · reject — check 2: the fact's home is a11y-plan decision/respawn-reap-bound
contract/switch-method/params · reject — check 2: the fact's home is security-plan contract/channel-frames/switch-params
contract/cli-human-lines/switched · reject — check 2: the fact's home is layout-templates contract/viola-switch-output/switched
contract/viola-switch/exit-codes · reject — check 2: the fact's home is architecture contract/cli-verbs/switch
contract/event-log/switch-launch-line · reject — check 2: the fact's home is architecture contract/wrapper-event-appends/launch
contract/viola-switch/settings · reject — check 2: the fact's home is architecture decision/switch-writes-no-user-settings
decision/switch-ends-only-its-own-child · reject — check 2: the fact's home is security-plan decision/switch-ends-only-its-own-child
decision/switch-proven-live · reject — check 1: a route carry, no master fact — R6a and R6b are carried on «The fleet on the founder's host»
decision/switch-cli-only · reject — check 2: the fact's home is architecture decision/switch-cli-only

ops-obs-plan.md
contract/claude-child-process-log/handle-wait-exit · apply — check 1: the pump owns a respawned child, each child's process-start and process-exit (fct-xvxcbpio) · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/run-process-log/process-exit-self · apply — check 1: a switch ends a child without ending run · word: delegate overseer, 2026-11-24, fork F1 arm 1
decision/launch-line-untraced · apply — check 1: the switch's launch line, log-only (fct-vzga25j6) · word: delegate overseer, 2026-11-24, fork F1 arm 1
decision/window-line-untraced · apply — check 1: the window line again after a switch's launch line · word: delegate overseer, 2026-11-24, fork F1 arm 1
contract/send-refused-detail/turn-running · apply — check 1: a third cause, a switch standing (fct-ionzjh33) · word: delegate overseer, 2026-11-24, fork F3 arm 1
decision/instance-check-before-log-init · apply — check 1: switch runs the instance check first (fct-pmolmu2p)
contract/cli-json-error/strict-modes-failed · apply — check 1: switch's exit-21 strict-modes document (fct-pmolmu2p)
contract/cli-log-file/switch · apply — check 5: the plan's expected amendment (`contract/cli-log-file/switch`)
decision/switch-method-lines-carry-no-pair · reject — check 7: stands as decision/no-params-in-channel-lines (fit) — that rule already keeps every channel line free of params and result bodies, the switch's included

ops-a11y-plan.md
contract/frame-verb-refusal/strict-modes-failed · apply — check 1: switch's strict-modes refusal first (fct-pmolmu2p)
contract/frame-verb-refusal/server-verify-failed · apply — check 1: switch served only after the peer and server checks (fct-pmolmu2p)
contract/cli-exit-codes/10 · apply — check 1: switch refused human-typing at exit 10, as built · word: delegate overseer, 2026-11-24, fork F3 arm 1
decision/send-refused-while-human-typing · apply — check 1: switch refused human-typing too · word: delegate overseer, 2026-11-24, fork F3 arm 1
decision/respawn-reap-bound · apply — check 5: the plan's expected amendment, the 10 s bound a row of its own (F6); the two list rows untouched (inputs#I6 item 3) · word: delegate overseer, 2026-11-24, fork F6 at 10 s
decision/switch-refusal-order · reject — check 2: the fact's home is security-plan decision/switch-refusal-order
contract/viola-switch-output/result-line · apply — check 1: the verb's output discipline (plain ASCII, no SGR), the a11y fact
contract/viola-switch-output/json-output · reject — check 2: the fact's home is layout-templates contract/viola-switch-output/json-ok
decision/switch-respawn-same-terminal · reject — check 7: not_master type 4 — the respawn's mechanism is the code's (crates/viola-pty/src/pump.rs); the plan Goal's disproved «same PTY» is disposed with no master site (check 6 above)
decision/send-refused-while-switching · reject — check 2: the fact's home is architecture decision/send-never-queued

ops-main-test-plan.md
decision/unverified-cli-send-only · apply — check 1: switch refused unverified-cli beside answer, as built (fct-xvxcbpio); fit read the proposer's heir as standing here
decision/unverified-cli-gates-answer-only · apply — check 2: one rule under one key, its heir decision/unverified-cli-send-only
