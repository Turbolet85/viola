# Fan-out results — 2026-11-24-viola-up (wrap 2026-11-24T07-06-28Z)

architecture: add 24 · set 8 · retire 0 · none 1 · ops-architecture.md
security-plan: add 5 · set 7 · retire 0 · none 0 · ops-security-plan.md
design-system: add 22 · set 6 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 12 · set 5 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 23 · set 4 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 3 · set 9 · retire 0 · none 4 · ops-obs-plan.md
a11y-plan: add 8 · set 1 · retire 0 · none 0 · ops-a11y-plan.md

Homes for the facts offered more than once (check 2): the verb's surface, argument list, names narrowing and CLI-only — architecture `contract/cli-verbs/up` (and layouts' per-verb `contract/cli-verbs/up`, its own verb sheet, as for every verb); the occupied name — architecture and design-system `name/command/up` (each master's standing name set); the selection rule, the order and the no-state rule — architecture decisions; the exit codes — architecture `contract/viola-up-exit-codes/*`; the outcome words, the line, its order, the dry run, the hint, the refusal pair and the `--json` document — design-system; the line's plain-ASCII form — a11y `contract/viola-up-output/instance-line`; revive's hidden flag — architecture `contract/viola-cli-arguments/revive-origin`; up's terminal spawn — security `decision/open-terminal-direct-spawn`; the test teardown, the fd settle wait and the live-rig root — test-plan.

Word for the delegate arms: `delegate overseer, 2026-11-24, inputs#I8` (P5 answers and forks U1, U3–U8, T1–T7, the two folds `inputs#I5`–`inputs#I7`, `inputs#I11`–`inputs#I12`), each PROVISIONAL; for U2 arm 1: `relayed verbatim by overseer1, 2026-11-24, inputs#I6` (the operator's own dialog of 2026-11-24T05:21:35Z).

## ops-architecture.md
decision/lineage-label-only · apply — check 5: plan expected amendment; up reads the parent to order only (parents_first, no path join) · word: delegate overseer, 2026-11-24, inputs#I8
decision/revive-resumes-in-place · escalate → Q1 — check 1: boundary widening U2 (the recorded workspace replayed under revive's hidden --origin) · resolved: apply (Q1) — delegate overseer, 2026-11-24T07:45:31Z: «The other eight are confirmed inside his answer, their U3 parts as you marked them» (the card's confirm; the answer it covers is the operator's own, U2 arm 1, relayed verbatim by overseer1, inputs#I6)
contract/cli-verbs/revive · escalate → Q1 — check 1: boundary widening U2 (revive's hidden --origin) · resolved: apply (Q1) — delegate overseer, 2026-11-24T07:45:31Z: «The other eight are confirmed inside his answer, their U3 parts as you marked them» (the card's confirm; the answer it covers is the operator's own, U2 arm 1, relayed verbatim by overseer1, inputs#I6)
contract/cli-exit-codes/1 · apply — check 5: plan expected amendment; up joins exit 1 · word: delegate overseer, 2026-11-24, inputs#I8
contract/instance-snapshot/cwd · escalate → Q2 — check 1: a boundary widening of its own, named outside U2 arm 1 by the card's answer (the recorded cwd onto up's terminal argv) · resolved: apply (Q2) — delegate overseer, 2026-11-24: «Q2: land the three cwd rows PROVISIONAL on the delegate word, the crossing written to founder-forks.md as a fork of its own» (inputs#I19), PROVISIONAL
contract/diagnostics-role-files/cli-name · apply — check 1: routine; up logs under the caller's VIOLA_NAME (T5) · word: delegate overseer, 2026-11-24, inputs#I8
contract/process-exit-detail/not-live · apply — check 1: routine; up writes not-live (src/cmd/up.rs) · word: delegate overseer, 2026-11-24, inputs#I8
contract/process-exit-detail/terminal-not-found · apply — check 1: routine; up refuses terminal-not-found
name/command/up · apply — check 5: plan expected amendment name/command/up (architecture)
contract/cli-verbs/up · apply — check 2: the verb's home; CLI-only (U5), names narrow (Deviations 1) · word: delegate overseer, 2026-11-24, inputs#I8
decision/up-selection-rule · reject — check 4: the rule leaves out the named instance, which skips the session-end reading (src/cmd/up.rs pick); main's ops-main-architecture.md re-adds it
decision/up-parents-first · reject — check 1: rests on T2, a delegate's word, and carries no state: provisional; main's ops-main-architecture.md re-adds it provisional
contract/viola-up-output/human-line · reject — check 2: the fact's home is design-system contract/cli-verb-output/up and contract/viola-up-output/line-order
contract/viola-up-outcome/live · reject — check 2: the fact's home is design-system contract/viola-up-outcome/live
contract/viola-up-outcome/already-live · reject — check 2: the fact's home is design-system contract/viola-up-outcome/already-live
contract/viola-up-outcome/stale-heartbeat · reject — check 2: the fact's home is design-system contract/viola-up-outcome/stale-heartbeat
contract/viola-up-outcome/would-open · reject — check 2: the fact's home is design-system contract/viola-up-outcome/would-open
contract/viola-up-outcome/no-session · reject — check 2: the fact's home is design-system contract/viola-up-outcome/no-session
contract/viola-up-outcome/cwd-missing · reject — check 2: the fact's home is design-system contract/viola-up-outcome/cwd-missing
contract/viola-up-outcome/session-live · reject — check 2: the fact's home is design-system contract/viola-up-outcome/session-live
contract/viola-up-outcome/not-live · reject — check 2: the fact's home is design-system contract/viola-up-outcome/not-live
contract/viola-up-outcome/not-taken · reject — check 2: the fact's home is design-system contract/viola-up-outcome/not-taken
contract/viola-up-exit-codes/0 · apply — check 2: the exit codes' home (architecture §Conventions) · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-exit-codes/1 · apply — check 2: the exit codes' home · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-exit-codes/21 · apply — check 2: the exit codes' home · word: delegate overseer, 2026-11-24, inputs#I8
contract/cli-json-output/up · reject — check 2: the fact's home is design-system contract/cli-verb-output/up (the document), its withheld fields architecture decision/up-writes-no-state
decision/up-writes-no-state · reject — check 1: rests on a delegate's P5 answer and carries no state: provisional; main's ops-main-architecture.md re-adds it provisional with the withheld fields
decision/up-terminal-direct-spawn · reject — check 2: the fact's home is security-plan decision/open-terminal-direct-spawn
contract/viola-cli-arguments/revive-origin · escalate → Q1 — check 1: boundary widening U2 (revive's hidden --origin); its «else none» rests on U3, PROVISIONAL · resolved: apply (Q1) — delegate overseer, 2026-11-24T07:45:31Z: «The other eight are confirmed inside his answer, their U3 parts as you marked them» (the card's confirm; the answer it covers is the operator's own, U2 arm 1, relayed verbatim by overseer1, inputs#I6)
contract/process-exit-detail/boot-time-unread · escalate → Q3 — check 5: plan expected amendment; the enum word (U7); rests on the operator's own answer U7 (inputs#I18), to land as his on the Q3 card · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)
decision/terminal-tree-one-teardown · reject — check 2: the fact's home is test-plan decision/stand-in-terminal-tree-teardown
name/other/foot-spawns-ndjson · apply — check 1: routine; an occupied test-only file name (T7) · word: delegate overseer, 2026-11-24, inputs#I8

## ops-main-architecture.md
decision/up-selection-rule · escalate → Q3 — check 4: the rule as built (named instances skip the session-end reading); rests on the operator's own answer U1 arm 1 (inputs#I18), to land as his on the Q3 card · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)
decision/up-parents-first · apply — check 1: routine, provisional on T2 · word: delegate overseer, 2026-11-24, inputs#I8
decision/up-writes-no-state · apply — check 2: one home for the no-state and withheld-field facts · word: delegate overseer, 2026-11-24, inputs#I8

## ops-security-plan.md
decision/open-terminal-direct-spawn · escalate → Q1 — check 1: boundary widening U2 (up's terminal starts the pinned viola revive --origin) · resolved: apply (Q1) — delegate overseer, 2026-11-24T07:45:31Z: «The other eight are confirmed inside his answer, their U3 parts as you marked them» (the card's confirm; the answer it covers is the operator's own, U2 arm 1, relayed verbatim by overseer1, inputs#I6)
decision/run-window-flags-checked · escalate → Q1 — check 1: boundary widening U2 (revive's hidden --origin) · resolved: apply (Q1) — delegate overseer, 2026-11-24T07:45:31Z: «The other eight are confirmed inside his answer, their U3 parts as you marked them» (the card's confirm; the answer it covers is the operator's own, U2 arm 1, relayed verbatim by overseer1, inputs#I6)
contract/revive-arguments/id · escalate → Q1 — check 1: boundary widening U2 (--list conflicts with the hidden --origin) · resolved: apply (Q1) — delegate overseer, 2026-11-24T07:45:31Z: «The other eight are confirmed inside his answer, their U3 parts as you marked them» (the card's confirm; the answer it covers is the operator's own, U2 arm 1, relayed verbatim by overseer1, inputs#I6)
decision/max-frame-shared-cap · reject — check 7: not_master type 5 — it lengthens the cap's inventory with one more reader, a list the code holds; the row stays true without it
contract/cli-exit-codes/21 · apply — check 5: plan expected amendment; up's board scan exits 21
decision/cwd-only-in-snapshot · escalate → Q2 — check 1: a boundary widening of its own, named outside U2 arm 1 by the card's answer (the recorded cwd on up's terminal argv) · resolved: apply (Q2) — delegate overseer, 2026-11-24: «Q2: land the three cwd rows PROVISIONAL on the delegate word, the crossing written to founder-forks.md as a fork of its own» (inputs#I19), PROVISIONAL
decision/spawn-cwd-snapshot-only · escalate → Q2 — check 1: a boundary widening of its own, named outside U2 arm 1 by the card's answer (the recorded cwd on up's terminal argv) · resolved: apply (Q2) — delegate overseer, 2026-11-24: «Q2: land the three cwd rows PROVISIONAL on the delegate word, the crossing written to founder-forks.md as a fork of its own» (inputs#I19), PROVISIONAL
decision/up-checks-before-log-and-spawn · reject — check 1: rests on a delegate's P5 answer and carries no state: provisional; main's ops-main-security-plan.md re-adds it provisional
contract/strict-modes-refusal/up · apply — check 1: routine; the board scan's refusal (src/cmd/up.rs refuse_board) · word: delegate overseer, 2026-11-24, inputs#I8
decision/up-writes-no-state · reject — check 2: the fact's home is architecture decision/up-writes-no-state
decision/test-teardown-signals-recorded-pids-only · reject — check 2: the fact's home is test-plan decision/stand-in-terminal-tree-teardown
decision/r4-rig-no-credential-path · reject — check 1: rests on U6, a delegate's arm, and carries no state: provisional; main's ops-main-security-plan.md re-adds it provisional

## ops-main-security-plan.md
decision/up-checks-before-log-and-spawn · apply — check 3: the order as built (board::scan, the cli log, check_home_tree, pin_bytes) · word: delegate overseer, 2026-11-24, inputs#I8
decision/r4-rig-no-credential-path · reject — check 7: not_master type 5 — an inventory of a test-only rig's surface the code and the route hold

## ops-design-system.md
contract/help-output/verb-groups · apply — check 5: plan expected amendment; the setup: row gains up · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-help/verb-groups · apply — check 5: plan expected amendment; the setup: row gains up · word: delegate overseer, 2026-11-24, inputs#I8
decision/list-is-the-board · apply — check 1: routine; up acts on the fleet or the names given · word: delegate overseer, 2026-11-24, inputs#I8
contract/exit-21-json-cause/strict-modes-failed · apply — check 5: plan expected amendment; up's board refusal reads strict-modes-failed
contract/cli-refusal-line/fixed-unable-form · apply — check 5: plan expected amendment; up's exit-1 refusals print the fixed form
contract/cli-stderr/fixed-message-refusal · apply — check 5: plan expected amendment; up's exit-1 refusals print the fixed form
decision/up-cli-only · reject — check 2: the fact's home is architecture contract/cli-verbs/up
decision/up-names-narrow-the-fleet · reject — check 2: the fact's home is architecture contract/cli-verbs/up
name/command/up · apply — check 5: plan expected amendment name/command/up (design), beside the master's standing name set
contract/cli-verb-output/up · apply — check 2: the output's home, as contract/cli-verb-output/focus · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-outcome/live · apply — check 5: plan expected amendment, the words (U7, T3) · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-outcome/already-live · apply — check 5: the words (T3) · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-outcome/stale-heartbeat · apply — check 5: the words (T3) · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-outcome/would-open · escalate → Q3 — check 5: the new word (U7); rests on the operator's own answer U7 (inputs#I18), to land as his on the Q3 card · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)
contract/viola-up-outcome/no-session · apply — check 5: the words (T3) · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-outcome/cwd-missing · apply — check 5: the words (T3) · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-outcome/session-live · apply — check 5: the words (T3) · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-outcome/not-live · apply — check 5: the words (T3) · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-outcome/not-taken · reject — check 4: leaves out the named instance whose heartbeat is of this boot (src/cmd/up.rs pick); main's ops-main-design-system.md re-adds it
contract/viola-up-output/line-order · apply — check 2: the order's home (Deviations 4) · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-exit-codes/0 · reject — check 2: the fact's home is architecture contract/viola-up-exit-codes/0
contract/viola-up-exit-codes/1 · reject — check 2: the fact's home is architecture contract/viola-up-exit-codes/1
contract/viola-up-exit-codes/21 · reject — check 2: the fact's home is architecture contract/viola-up-exit-codes/21
contract/viola-up-output/dry-run · apply — check 2: the dry form's home (T1) · word: delegate overseer, 2026-11-24, inputs#I8
decision/up-writes-no-instance-state · reject — check 2: the fact's home is architecture decision/up-writes-no-state
decision/revive-origin-hidden-flag · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/revive-origin
contract/fixed-refusal-pairs/boot-time-unread · escalate → Q3 — check 5: plan expected amendment, the pair (U7); rests on the operator's own answer U7 (inputs#I18), to land as his on the Q3 card · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)
contract/viola-up-output/not-live-hint · escalate → Q3 — check 5: plan expected amendment, the hint (U7, U8); rests on the operator's own answer U7, U8 arm 1 (inputs#I18), to land as his on the Q3 card · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)

## ops-main-design-system.md
contract/viola-up-outcome/not-taken · escalate → Q3 — check 4: the word as built (src/cmd/up.rs pick) (U7); rests on the operator's own answer U7 (inputs#I18), to land as his on the Q3 card · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)

## ops-layout-templates.md
contract/cli-verbs/help · apply — check 5: plan expected amendment; the setup: row gains up
contract/viola-revive/no-window · escalate → Q1 — check 1: boundary widening U2 (a revive places a window under --origin); its «reads unplaced» rests on U3, PROVISIONAL · resolved: apply (Q1) — delegate overseer, 2026-11-24T07:45:31Z: «The other eight are confirmed inside his answer, their U3 parts as you marked them» (the card's confirm; the answer it covers is the operator's own, U2 arm 1, relayed verbatim by overseer1, inputs#I6)
contract/viola-revive/usage · escalate → Q1 — check 1: boundary widening U2 (--list conflicts with the hidden --origin) · resolved: apply (Q1) — delegate overseer, 2026-11-24T07:45:31Z: «The other eight are confirmed inside his answer, their U3 parts as you marked them» (the card's confirm; the answer it covers is the operator's own, U2 arm 1, relayed verbatim by overseer1, inputs#I6)
contract/cli-exit-codes/21 · apply — check 5: plan expected amendment; up's board refusal exits 21
decision/board-vs-strip · apply — check 1: routine; up acts on the fleet or the names given · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-help/setup-row · reject — check 2: the fact's home is layout-templates contract/viola-help/verb-groups, set by main's ops-main-layout-templates.md
contract/cli-verbs/up · apply — check 2: layouts' per-verb sheet, as contract/cli-verbs/focus · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-output/line · reject — check 2: the fact's home is design-system contract/cli-verb-output/up and contract/viola-up-outcome/*
contract/viola-up-output/exit-codes · reject — check 2: the fact's home is architecture contract/viola-up-exit-codes/*
contract/viola-up-output/line-order · reject — check 2: the fact's home is design-system contract/viola-up-output/line-order
contract/viola-up/named-instances · reject — check 2: the fact's home is architecture contract/cli-verbs/up
contract/viola-up-output/not-live-hint · reject — check 2: the fact's home is design-system contract/viola-up-output/not-live-hint
contract/viola-up-output/dry-run · reject — check 2: the fact's home is design-system contract/viola-up-output/dry-run
contract/viola-up-output/json-document · reject — check 2: the fact's home is design-system contract/cli-verb-output/up
contract/viola-up-output/no-address · reject — check 2: the fact's home is architecture decision/up-writes-no-state
contract/viola-up-refusals/boot-time-unread · reject — check 2: the fact's home is design-system contract/fixed-refusal-pairs/boot-time-unread
name/command/up · reject — check 2: the fact's home is architecture name/command/up (layouts holds no name rows)

## ops-main-layout-templates.md
contract/viola-help/verb-groups · apply — check 5: plan expected amendment; the setup: row gains up and the board: row's focus, stale since chunk 2026-11-24-viola-focus (src/cmd/mod.rs HELP)

## ops-test-plan.md
decision/strict-modes-before-any-log · reject — check 4: up's home-tree check runs after its log and writes a process-exit (src/cmd/up.rs), so only the board scan joins; main's ops-main-test-plan.md sets it
contract/strict-modes-refusal/no-line · reject — check 4: up's home-tree refusal writes a process-exit line; main's ops-main-test-plan.md sets the row to the board scan
decision/window-placement-proven-live · apply — check 5: plan expected amendment; up's placement proven live only (R4 gated) · word: delegate overseer, 2026-11-24, inputs#I8
decision/hint-names-its-cause · apply — check 1: routine; boot-time-unread held by its unit pair (Deviations 9)
decision/up-cli-only · reject — check 2: the fact's home is architecture contract/cli-verbs/up
contract/viola-up-exit-codes/0 · reject — check 2: the fact's home is architecture contract/viola-up-exit-codes/0
contract/viola-up-exit-codes/1 · reject — check 2: the fact's home is architecture contract/viola-up-exit-codes/1
contract/viola-up-exit-codes/21 · reject — check 2: the fact's home is architecture contract/viola-up-exit-codes/21
contract/viola-up-outcome/live · reject — check 2: the fact's home is design-system contract/viola-up-outcome/live
contract/viola-up-outcome/already-live · reject — check 2: the fact's home is design-system contract/viola-up-outcome/already-live
contract/viola-up-outcome/stale-heartbeat · reject — check 2: the fact's home is design-system contract/viola-up-outcome/stale-heartbeat
contract/viola-up-outcome/would-open · reject — check 2: the fact's home is design-system contract/viola-up-outcome/would-open
contract/viola-up-outcome/no-session · reject — check 2: the fact's home is design-system contract/viola-up-outcome/no-session
contract/viola-up-outcome/cwd-missing · reject — check 2: the fact's home is design-system contract/viola-up-outcome/cwd-missing
contract/viola-up-outcome/session-live · reject — check 2: the fact's home is design-system contract/viola-up-outcome/session-live
contract/viola-up-outcome/not-live · reject — check 2: the fact's home is design-system contract/viola-up-outcome/not-live
contract/viola-up-outcome/not-taken · reject — check 2: the fact's home is design-system contract/viola-up-outcome/not-taken
decision/up-names-narrow-the-fleet · reject — check 2: the fact's home is architecture contract/cli-verbs/up
decision/up-selects-pre-boot-unended · reject — check 2: the fact's home is architecture decision/up-selection-rule
decision/up-spawns-parents-first · reject — check 2: the fact's home is architecture decision/up-parents-first
contract/viola-revive/origin-window · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/revive-origin
decision/stand-in-terminal-tree-teardown · reject — check 1: a delegate's fold, no state: provisional; main's ops-main-test-plan.md re-adds it provisional
decision/child-fd-table-read-after-launch-settles · reject — check 1: a delegate's fold, no state: provisional; main's ops-main-test-plan.md re-adds it provisional
decision/window-reopen-proven-live · apply — check 1: routine, provisional (U6) · word: delegate overseer, 2026-11-24, inputs#I8
decision/live-rig-root-outside-repositories · reject — check 1: a delegate's P5 review, no state: provisional; main's ops-main-test-plan.md re-adds it provisional
contract/fake-foot/spawn-record · apply — check 1: routine; the fake foot's record (T7) · word: delegate overseer, 2026-11-24, inputs#I8
name/command/up · reject — check 2: the fact's home is architecture name/command/up (area=new, not asked)

## ops-main-test-plan.md
decision/strict-modes-before-any-log · apply — check 5: plan expected amendment, the board scan alone (src/cmd/up.rs)
contract/strict-modes-refusal/no-line · apply — check 5: plan expected amendment, the board scan alone (src/cmd/up.rs)
decision/stand-in-terminal-tree-teardown · apply — check 1: routine, provisional (fold, inputs#I12) · word: delegate overseer, 2026-11-24, inputs#I12
decision/child-fd-table-read-after-launch-settles · apply — check 1: routine, provisional (fold, inputs#I6) · word: delegate overseer, 2026-11-24, inputs#I7
decision/live-rig-root-outside-repositories · apply — check 5: plan expected amendment decision/live-rig-clean-start, carried as its own row · word: delegate overseer, 2026-11-24, inputs#I2

## ops-obs-plan.md
contract/diag-subject/terminal · apply — check 6: the disproved terminal pair, corrected to the code (the close's Decisions & corrections after implement)
contract/process-log/child-and-probe-lifecycle · apply — check 6: the disproved terminal pair, corrected to the code
decision/strict-modes-failure-lines · apply — check 1: routine; up's home-tree refusal writes its own process-exit (src/cmd/up.rs refused)
contract/diag-detail/strict-modes-failed · apply — check 1: routine; up writes strict-modes-failed on its exit-1 process-exit
contract/process-exit-detail/strict-modes-failed · apply — check 1: routine; up is a second carrier
contract/strict-modes-refusal-exit/21 · apply — check 5: plan expected amendment; up's board scan exits 21
contract/strict-modes-refusal-exit/1 · apply — check 1: routine; up's home-tree check exits 1
decision/strict-modes-before-diagnostics · apply — check 1: routine; up's later home-tree refusal is the named exception
decision/symlink-skip-logged-as-count · apply — check 1: routine; up never takes the strict board read (Deviations 8)
contract/cli-log-file/up · apply — check 5: plan expected amendment contract/cli-log-file/up (T5) · word: delegate overseer, 2026-11-24, inputs#I8
contract/process-exit-detail/exit-1-up · apply — check 5: plan expected amendment process-exit-detail/exit-1 (Deviations 3) · word: delegate overseer, 2026-11-24, inputs#I8
contract/process-exit-detail/boot-time-unread · escalate → Q3 — check 5: plan expected amendment; the new word (U7); rests on the operator's own answer U7 (inputs#I18), to land as his on the Q3 card · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)

## ops-a11y-plan.md
contract/viola-up-output/instance-line · apply — check 2: the plain-ASCII, no-SGR, one-write form is a11y's · word: delegate overseer, 2026-11-24, inputs#I8
contract/viola-up-output/line-order · reject — check 2: the fact's home is design-system contract/viola-up-output/line-order
contract/viola-up-output/outcome-word · reject — check 2: the fact's home is design-system contract/viola-up-outcome/*
contract/viola-up-output/exit · reject — check 2: the fact's home is architecture contract/viola-up-exit-codes/0
contract/viola-up-output/json · reject — check 2: the fact's home is design-system contract/cli-verb-output/up
contract/viola-up-output/not-live-hint · reject — check 2: the fact's home is design-system contract/viola-up-output/not-live-hint
contract/viola-up-refusal/boot-time-unread · reject — check 2: the fact's home is design-system contract/fixed-refusal-pairs/boot-time-unread
contract/viola-up-refusal/exit-1-documents · reject — check 2: the fact's home is architecture contract/viola-up-exit-codes/1
contract/cli-exit-codes/21 · apply — check 5: plan expected amendment; up's board refusal exits 21

## ops-main-a11y-plan.md
decision/time-bounds-never-on-human · escalate → Q3 — check 7: not_master type 5 (an inventory); check 5: the operator's own answer to fork F1 of run 2026-11-24T00-26-24Z-wrap, option 1 (inputs#I18): the list row reworded, «Every» → «Eight» (the store's row form refuses a lead «These») · resolved: apply (Q3) — relayed verbatim by overseer1, 2026-11-24, inputs#I18: «Три отдельные строки (Recommended)» — F1 option 1, the two list rows reworded; the lead «Eight» in place of his «These» PROVISIONAL on the delegate word (overseer, 2026-11-24T07:49:50Z), set by ops-main-a11y-plan.3.md
decision/time-bounds-fall-on-the-driver · escalate → Q3 — check 7: not_master type 5 (an inventory); check 5: F1 option 1, the operator's own answer (inputs#I18): the list row reworded, «Every» → «Eight» · resolved: apply (Q3) — relayed verbatim by overseer1, 2026-11-24, inputs#I18: «Три отдельные строки (Recommended)» — F1 option 1, the two list rows reworded; the lead «Eight» in place of his «These» PROVISIONAL on the delegate word (overseer, 2026-11-24T07:49:50Z), set by ops-main-a11y-plan.3.md

## ops-main-a11y-plan.2.md
decision/open-live-wait-bound · escalate → Q3 — check 5: F1 option 1, the operator's own answer (inputs#I18): open's 30 s live wait a row of its own; up's reuse adds nothing (no new bound) · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)
decision/window-wait-bound · escalate → Q3 — check 5: F1 option 1, the operator's own answer (inputs#I18): the wrapper's 5 s window wait a row of its own · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)
decision/hyprctl-read-deadline · escalate → Q3 — check 5: F1 option 1, the operator's own answer (inputs#I18): the seam's 5 s read deadline a row of its own, viola focus among its bearers (no row of its own: no new bound) · resolved: apply (Q3) — delegate overseer, 2026-11-24T07:49:50Z: «every operation of the list lies inside his answers and lands as his» (the card's confirm; the answers it covers are the operator's own, relayed verbatim by overseer1, inputs#I18)

## ops-main-a11y-plan.3.md
decision/time-bounds-never-on-human · apply — the lead «Eight» marked provisional on the delegate word (Q3's answer, overseer, 2026-11-24T07:49:50Z) · word: delegate overseer, 2026-11-24T07:49:50Z
decision/time-bounds-fall-on-the-driver · apply — the lead «Eight» marked provisional on the delegate word (Q3's answer) · word: delegate overseer, 2026-11-24T07:49:50Z

## ops-main-security-plan.2.md
decision/cwd-only-in-snapshot · apply — Q2: the crossing of the recorded cwd onto up's terminal argv, PROVISIONAL on the delegate word (inputs#I19) · word: delegate overseer, 2026-11-24, inputs#I19
decision/spawn-cwd-snapshot-only · apply — Q2: the same crossing, PROVISIONAL on the delegate word (inputs#I19) · word: delegate overseer, 2026-11-24, inputs#I19
