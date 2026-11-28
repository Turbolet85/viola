# Fan-out results — wrap 2026-11-28T12-37-34Z-wrap (chunk 2026-11-28-fleet-console-view)

a11y-plan: add 3 · set 9 · retire 0 · none 0 · ops-a11y-plan.md
architecture: add 5 · set 3 · retire 0 · none 1 · ops-architecture.md
design-system: add 10 · set 3 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 6 · set 3 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 2 · set 6 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 3 · set 6 · retire 0 · none 1 · ops-obs-plan.md
security-plan: add 3 · set 6 · retire 0 · none 0 · ops-security-plan.md
main's own: ops-main-design-system.md · ops-main-design-system.2.md · ops-main-layout-templates.md · ops-main-security-plan.md

Q1 — the ONE answer card (inputs#I7 item 2; `rul-bznresq5`): his own answers, relayed verbatim by viola-overseer, inputs#I3 —
(a) the two scoping rows reworded, F1 the two named exceptions widened, F2 `LAST`, F4 the cut, F5 the board's order and
`... <n> more`, F6 the one-second read and the 5 s agents read off the loop (the agents read a new subprocess crossing,
`rul-r74hshif`), F7 the skip line only on a change. The eight verb-output rows: his «Не трогать», rejected below.

ops-a11y-plan.md
decision/static-linear-output · escalate → Q1 — check 1: `rul-r74hshif` a redraw admitted on a no-redraw rule; his own answer F1 «Расширить два» (inputs#I3), the card `rul-bznresq5` · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
decision/cli-no-interactive-input · escalate → Q1 — check 1: as above, his own answer F1 (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
decision/claude-agents-read-fails-open · escalate → Q2 — check 1: `rul-r74hshif` a new subprocess crossing, his own answer F6 (inputs#I3); its empty-list clause rests on the delegate's technical word (inputs#I5), PROVISIONAL · resolved: apply (Q2) — delegate viola-overseer, 2026-11-28: «land it PROVISIONAL — a failed read must not keep showing a stale busy.»
decision/state-carried-by-printed-word · reject — check 2: the fact's home is design-system contract/fleet-console-view/last
decision/strict-modes-refusal-before-any-write · apply — check 1: a correction, the console's mid-run refusal makes the standing rule false without its exception · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
decision/no-spinners · reject — check 1: one of the eight verb-output rows, his own «Не трогать» (inputs#I3): scoped by (a)'s reworded rows, no exception named on it
decision/linear-static-output · reject — check 1: one of the eight verb-output rows, his own «Не трогать» (inputs#I3)
decision/linear-static-cli-output · reject — check 1: one of the eight verb-output rows, his own «Не трогать» (inputs#I3)
contract/viola-list-board/caption · reject — check 2: the fact's home is layout-templates decision/shared-captions-fixed
decision/fleet-console-line-cut · reject — check 2: the fact's home is design-system contract/fleet-console-view/line-cut
decision/fleet-console-more-line · reject — check 2: the fact's home is design-system contract/fleet-console-view/more-line
decision/console-agents-read-off-the-loop · reject — check 2: the fact's home is layout-templates decision/console-agents-read-off-loop; its outlive clause security-plan decision/console-agents-child-outlives-quit

ops-architecture.md
decision/agents-join-read-ungated · escalate → Q1 — check 1: `rul-r74hshif` a new subprocess crossing, his own answer F6 (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
decision/refused-check-writes-no-line · apply — check 1: a correction, the console's mid-run refusal writes one line · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
decision/strict-modes-check-before-log · apply — check 1: a correction, the console's mid-run refusal after its log opens · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
decision/console-rereads-board-from-disk · reject — check 2: the re-read's home is layout-templates decision/console-redraw-on-change, the unchanged-log skip read not_master type 4 at its one home (security-plan decision/list-scan-checks-before-open): the code holds it
decision/console-agents-read-off-loop · reject — check 2: the fact's home is layout-templates decision/console-agents-read-off-loop, its empty list a11y-plan decision/claude-agents-read-fails-open
decision/board-last-activity-never-serialised · reject — check 7: not_master type 4 — a symbol inventory of one field and its skip, stating no reason; the code and its named test `board_read_item_never_serialises_last_activity` hold it
contract/console-end/home-widened-mid-run · reject — check 2: the fact's home is design-system contract/viola-console-refusals/mid-run-home-widened
contract/console-end/scan-error · reject — check 2: the fact's home is obs-plan contract/panic-outcome-by-role/console, standing

ops-design-system.md
decision/truncate-only-unwrapped-name · reject — check 3: the F4 cut needs no exception under his reworded scoping rows (delegate viola-overseer, inputs#I5); its home is design-system contract/fleet-console-view/line-cut
contract/human-tables/name-truncation · reject — check 3: as above
decision/escape-controls-in-human-output · apply — check 1: a code fact (every fleet cell escaped, control `fleet-escape` holds), a narrowing; the one home of two offers
contract/fleet-console-view/body-rows · reject — check 2: the fact's home is layout-templates decision/console-fleet-body-lines
contract/fleet-console-view/captions · escalate → Q1 — check 1: his own answer F2 (inputs#I3), the card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
contract/fleet-console-view/last · escalate → Q1 — check 1: his own answer F2 (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
contract/fleet-console-view/line-cut · escalate → Q1 — check 1: his own answer F4 «Резать строку» (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
contract/fleet-console-view/more-line · escalate → Q1 — check 1: his own answer F5 (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
decision/console-reread-once-a-second · reject — check 2: the fact's home is layout-templates decision/console-redraw-on-change
decision/console-agents-read-off-the-loop · reject — check 2: the fact's home is layout-templates decision/console-agents-read-off-loop
contract/viola-console-refusals/mid-run-home-widened · apply — check 1: the mid-run refusals, the plan's lean (step 7) on the delegate's technical word · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
decision/console-fleet-text-uncoloured · reject — check 2: the fact's home is layout-templates decision/console-own-surface («in plain ASCII with no colour»), standing
decision/last-activity-console-only · reject — check 2: its one home, architecture decision/board-last-activity-never-serialised, read not_master type 4 by fit: the code and its named test hold it

ops-layout-templates.md
decision/shared-captions-fixed · escalate → Q1 — check 1: his own answer F2 «Ячейка LAST в консоли» (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
decision/truncate-only-unwrapped-names · reject — check 3: the F4 cut needs no exception under his reworded scoping rows (delegate viola-overseer, inputs#I5); its home is design-system contract/fleet-console-view/line-cut
contract/cli-token-rendering/motion-tokens · reject — check 1: one of the eight verb-output rows, his own «Не трогать» (inputs#I3)
decision/console-fleet-body-lines · escalate → Q1 — check 1: his own answer F5 «Порядок доски» (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
contract/console-fleet/last-cell · reject — check 2: the fact's home is design-system contract/fleet-console-view/last
contract/console-fleet/more-line · reject — check 2: the fact's home is design-system contract/fleet-console-view/more-line
decision/console-redraw-on-change · escalate → Q1 — check 1: his own answer F6 «Раз в секунду» (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
decision/console-agents-read-off-loop · reject — check 1: rewritten in ops-main-layout-templates.md without its failed-read clause, whose home is a11y-plan decision/claude-agents-read-fails-open
contract/viola-console/mid-run-refusal · reject — check 2: the fact's home is design-system contract/viola-console-refusals/mid-run-home-widened

ops-obs-plan.md
contract/diag-line-fields/console-frame · apply — check 5: the plan's expected amendment, the overseer's technical T2 and inputs#I4 item 1 · word: delegate viola-overseer, 2026-11-28 (inputs#I4)
contract/diag-subject/agents-probe · escalate → Q1 — check 1: `rul-r74hshif` a new subprocess crossing, his own answer F6 (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
decision/symlink-skip-logged-as-count · escalate → Q1 — check 1: his own answer F7 «Только при изменении» (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
contract/state-recovered/scanned · apply — check 1: a correction, the console writes its replay line into console.ndjson with no name · word: delegate viola-overseer, 2026-11-28 (inputs#I4)
contract/process-exit-detail/strict-modes-failed · apply — check 1: a correction, a third carrier of the detail · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
contract/strict-modes-refusal-exit/1 · apply — check 1: a correction, the console's mid-run refusal exits 1 · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
decision/console-mid-run-strict-modes-refusal-logged · reject — check 2: the fact's home is obs-plan contract/process-exit-detail/strict-modes-failed and design-system contract/viola-console-refusals/mid-run-home-widened
decision/console-agents-probe-volume · apply — check 1: the phase card's standing row «The console's log volume», accepted on the delegate's technical word, state provisional · word: delegate viola-overseer, 2026-11-28 (inputs#I4)
decision/console-agents-read-fails-empty · reject — check 2: the fact's home is a11y-plan decision/claude-agents-read-fails-open

ops-security-plan.md
contract/strict-modes-refusal/console · apply — check 1: a correction, «no line written» holds at the start alone · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
decision/claude-agents-resolved-from-path · escalate → Q1 — check 1: `rul-r74hshif` a new subprocess crossing, his own answer F6 (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
decision/claude-agents-read-fail-open · reject — check 2: the fact's home is a11y-plan decision/claude-agents-read-fails-open
decision/list-scan-checks-before-open · reject — check 7: not_master type 4 — the replay-only-a-changed-log clause retells how the console's re-read is built, no reason stated; the code holds it (a set lengthening a true row: nothing follows)
decision/tests-use-own-home · reject — check 2: the fact's home is test-plan decision/tests-never-read-host-sessions
decision/escape-terminal-controls · reject — check 2: the fact's home is design-system decision/escape-controls-in-human-output, the cut's design-system contract/fleet-console-view/line-cut
decision/board-last-activity-in-memory · reject — check 2: its one home, architecture decision/board-last-activity-never-serialised, read not_master type 4 by fit: the code holds it
decision/console-agents-child-outlives-quit · reject — check 1: rewritten in ops-main-security-plan.md with `state: provisional`, the delegate's word
contract/console-board-read/scan-error · reject — check 2: the fact's home is obs-plan contract/panic-outcome-by-role/console, standing

ops-test-plan.md
contract/strict-modes-refusal/no-line · apply — check 1: a correction, the console's start alone stays line-free · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
decision/home-read-rule · apply — check 1: a correction, as above · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
contract/strict-modes-refusal/console · apply — check 1: a correction, the mid-run refusal's one line · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
decision/tests-never-read-host-sessions · apply — check 1: the report's sweep hazard and its fix (Deviations), every console start under a test-scoped PATH; no widening
decision/fake-agent-on-test-path · reject — check 2: the fact's home is test-plan decision/tests-never-read-host-sessions
decision/agents-stub-from-files-no-env · reject — check 2: the fact's home is test-plan decision/tests-never-read-host-sessions
decision/fleet-console-proven-live · reject — check 2: the live half is matrix#v2-14's gated entry and the route's BLOCKED-ON, no master's
contract/list-json/last-activity · reject — check 2: its one home, architecture decision/board-last-activity-never-serialised, read not_master type 4 by fit: the code holds it

ops-main-design-system.md
decision/fleet-console-not-verb-output · escalate → Q1 — check 5: the plan's expected amendment, his own answer (a) «Переформулировать» (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»

ops-main-design-system.2.md
decision/fleet-console-not-verb-output · escalate → Q1 — check 5: its name follows his answer (a) (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»

ops-main-layout-templates.md
decision/no-prompt-or-pager · escalate → Q1 — check 5: the plan's expected amendment, his own answer (a) «Переформулировать» (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»
decision/console-agents-read-off-loop · escalate → Q1 — check 5: the plan's expected surface row, his own answer F6 (inputs#I3) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-28: «confirmed, all 17 — each lies inside the operator's own answers»

ops-main-security-plan.md
decision/console-agents-child-outlives-quit · apply — check 1: the phase card's standing row «An agents child outliving a quit», accepted as built · word: delegate viola-overseer, 2026-11-28 (inputs#I5)
