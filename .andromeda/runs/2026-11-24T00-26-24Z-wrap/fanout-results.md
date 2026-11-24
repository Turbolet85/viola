# Fan-out results — 2026-11-23-viola-open-2

architecture: add 24 · set 7 · retire 0 · none 0 · ops-architecture.md
security-plan: add 11 · set 2 · retire 0 · none 0 · ops-security-plan.md
design-system: add 23 · set 5 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 13 · set 2 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 9 · set 6 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 7 · set 5 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 8 · set 4 · retire 0 · none 0 · ops-a11y-plan.md
test-plan (main): add 1 · set 1 · ops-main-test-plan.md

Homes (check 2, one fact one home; a master's own standing boundary updated in place): the open's exit codes and documents — architecture contract/viola-open-exit-codes/*; the placement and origin words — design-system contract/placement-word/* and contract/origin-word/*; the open's argument allowlist, direct spawn, origin authenticating nothing and the window record's confinement — security-plan; the hidden run flags' names — architecture contract/viola-cli-arguments/*, their checks security-plan (W1); the list item's window and origin — architecture contract/list-payload/items, test-plan contract/viola-list-json/items, security-plan (W3); the diag subject and exit-1 details — obs-plan; the live rig rule — test-plan (main's own, provisional).

ops-architecture.md
decision/snapshot-replay-log-fields-only · apply — check 5: expected amendment (architecture decision/snapshot-replay-log-fields-only), the replay recovers window (report Schema / config)
contract/snapshot-envelope/unreadable-snapshot · apply — check 5: expected amendment, the recovered list gains window
contract/instance-snapshot/data · apply — check 5: expected amendment (contract/instance-snapshot/data gains window?)
contract/list-payload/items · escalate → Q1 — check 1: boundary widening W3 (the list item gains window? and origin?), the operator's own answer arm (a), inputs#I4 · resolved: apply (Q1) — delegate overseer, 2026-11-24, inputs#I9: «An answer card whose every row lies inside his own recorded answers (item 2) is confirmed by the overseer.» — the card confirms rows inside W1–W3 arm (a), the operator's own answers, relayed verbatim by overseer1, 2026-11-23, inputs#I4
decision/wrapper-events-log-only · apply — check 1: routine; the wrapper's log-only window line joins the standing list (report Symbols / APIs)
contract/cli-exit-codes/1 · apply — check 1: routine; the standing exit-1 row in this master gains open's refusals (report Symbols / APIs, exit codes of open)
decision/no-start-over-live-instance · apply — check 1: routine; open refuses already-live and stale-heartbeat (report Symbols / APIs)
name/command/open · apply — check 5: expected amendment (a new name/command/open)
decision/open-closed-terminal-spawn · reject — check 2: the fact's home is security-plan decision/open-terminal-direct-spawn and decision/open-closed-argument-allowlist
decision/open-window-target · reject — check 7: not_master type 4 — the workspace choice is one function's mechanism, its basis stating no reason; escalated (Q1), not asked
decision/wrapper-window-step · reject — check 7: not_master type 4 — the bounded wait and single move retell one mechanism's compiled constants
decision/window-origin-self-reported · reject — check 2: the fact's home is security-plan decision/window-origin-authenticates-nothing
contract/normalised-event-kinds/window · apply — check 5: expected amendment (contract/normalised-event-kinds — window)
contract/window-placement/word-set · reject — check 2: the fact's home is design-system contract/placement-word/* (a closed list, one row an item)
contract/window-origin/word-set · reject — check 2: the fact's home is design-system contract/origin-word/* (a closed list, one row an item)
contract/wrapper-event-appends/window · apply — check 5: expected amendment (contract/wrapper-event-appends — window after launch, under --origin only)
contract/event-data/window · apply — check 1: routine; the standing event-data boundary gains the new kind's data (report Schema / config)
contract/instance-snapshot/window · apply — check 5: expected amendment (the snapshot's window and its replay rule)
contract/list-item/window · reject — check 2: the fact's home is architecture contract/list-payload/items (this file's own set)
contract/list-item/origin · reject — check 2: the fact's home is architecture contract/list-payload/items (this file's own set)
contract/viola-open-exit-codes/0 · apply — check 1: routine; the new verb's exit 0 and ok document, home of the open's documents (report Symbols / APIs)
contract/viola-open-exit-codes/1 · apply — check 1: routine; the new verb's exit-1 codes and refusal document (report Symbols / APIs)
contract/viola-open-exit-codes/2 · apply — check 1: routine; the new verb's usage exit (report Symbols / APIs)
contract/process-exit-detail/terminal-not-found · apply — check 1: routine; the master's standing process-exit-detail register gains the new word (report Schema / config)
contract/process-exit-detail/not-live · apply — check 1: routine; the master's standing process-exit-detail register gains the new word (report Schema / config)
contract/diag-subject/terminal · reject — check 2: the fact's home is obs-plan contract/diag-subject/terminal (architecture holds no diag-subject boundary)
contract/viola-cli-arguments/run-origin · escalate → Q1 — check 1: boundary widening W1 (the origin on the started run's argv), the operator's own answer arm (a), inputs#I4 · resolved: apply (Q1) — delegate overseer, 2026-11-24, inputs#I9: «An answer card whose every row lies inside his own recorded answers (item 2) is confirmed by the overseer.» — the card confirms rows inside W1–W3 arm (a), the operator's own answers, relayed verbatim by overseer1, 2026-11-23, inputs#I4
contract/viola-cli-arguments/run-place · escalate → Q1 — check 1: boundary widening W1 (the target workspace on the started run's argv), the operator's own answer arm (a), inputs#I4 · resolved: apply (Q1) — delegate overseer, 2026-11-24, inputs#I9: «An answer card whose every row lies inside his own recorded answers (item 2) is confirmed by the overseer.» — the card confirms rows inside W1–W3 arm (a), the operator's own answers, relayed verbatim by overseer1, 2026-11-23, inputs#I4
decision/state-depends-on-hyprland · reject — check 7: not_master type 6 — the two crate edges stand in the manifests with no guarantee stated (expected amendment: the crate edge, dispositioned here)
contract/cli-verbs/open · apply — check 1: routine; the master's standing CLI verbs boundary gains the new verb
decision/window-step-in-start-order · reject — check 7: not_master type 4 — the step's place in the start is one mechanism's order; decision/run-start-order stays true (expected amendment dispositioned here)

ops-security-plan.md
decision/hyprctl-read-deadline · apply — check 1: routine; the seam's first product caller resolves hyprctl on PATH and strips the R8 names (report Symbols / APIs, open_window)
decision/state-tree-strict-modes-before-use · apply — check 1: routine; open refuses strict-modes-failed with exit 1 before any log (report Symbols / APIs)
decision/open-closed-argument-allowlist · apply — check 5: expected amendment (security-plan new decision entry: the open's argument surface and allowlist; evidence/v2-08-by-construction.md)
decision/open-terminal-direct-spawn · apply — check 5: expected amendment (security-plan new decision entry: no shell, the one spawn site)
decision/run-window-flags-checked · escalate → Q1 — check 1: boundary widening W1, the operator's own answer arm (a), inputs#I4 · resolved: apply (Q1) — delegate overseer, 2026-11-24, inputs#I9: «An answer card whose every row lies inside his own recorded answers (item 2) is confirmed by the overseer.» — the card confirms rows inside W1–W3 arm (a), the operator's own answers, relayed verbatim by overseer1, 2026-11-23, inputs#I4
decision/board-window-and-origin-re-parsed · escalate → Q1 — check 1: boundary widening W3, the operator's own answer arm (a), inputs#I4 · resolved: apply (Q1) — delegate overseer, 2026-11-24, inputs#I9: «An answer card whose every row lies inside his own recorded answers (item 2) is confirmed by the overseer.» — the card confirms rows inside W1–W3 arm (a), the operator's own answers, relayed verbatim by overseer1, 2026-11-23, inputs#I4
decision/window-origin-authenticates-nothing · apply — check 5: expected amendment (--origin self-reported, authenticating nothing)
decision/window-step-moves-own-terminal-window · apply — check 1: routine; the product's window step moves only its own terminal's window (report Symbols / APIs, src/run/window.rs)
decision/window-record-kept-to-snapshot-and-event-log · apply — check 5: expected amendment (a window record kept to the snapshot and the event log, the launch pair's template)
decision/open-ancestor-walk-in-memory · escalate → Q1 — check 1: boundary widening W2, the operator's own answer arm (a), inputs#I4 · resolved: apply (Q1) — delegate overseer, 2026-11-24, inputs#I9: «An answer card whose every row lies inside his own recorded answers (item 2) is confirmed by the overseer.» — the card confirms rows inside W1–W3 arm (a), the operator's own answers, relayed verbatim by overseer1, 2026-11-23, inputs#I4
contract/open-refusals/terminal-not-found · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/1
contract/open-refusals/not-live · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/1
decision/live-rig-strips-viola-names · reject — check 2: the fact's home is test-plan decision/live-rig-clean-start (main's own, provisional)

ops-design-system.md
contract/help-output/verb-groups · apply — check 5: expected amendment (setup: gains open)
contract/viola-help/verb-groups · apply — check 5: expected amendment (setup: gains open)
contract/cli-exit-codes/1 · apply — check 5: expected amendment (design-system contract/cli-exit-codes/1 — terminal-not-found, not-live)
contract/cli-stderr/fixed-message-refusal · apply — check 1: routine; open's exit-1 refusals use the fixed unable: form
contract/cli-refusal-line/fixed-unable-form · apply — check 1: routine; open's exit-1 refusals use the fixed unable: form
name/command/open · apply — check 1: routine; the master's standing command-name register gains open
decision/open-closed-argument-surface · reject — check 2: the fact's home is security-plan decision/open-closed-argument-allowlist
contract/viola-open-output/usage-refusal · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/2
contract/viola-open-output/json-refusal · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/1
contract/viola-open-output/live-line · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/0
contract/viola-open-json/ok · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/0
contract/viola-open-refusals/strict-modes · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/1
contract/placement-word/placed · apply — check 1: routine; a closed word list the open prints, one row an item (report Symbols / APIs, PLACEMENT_WORDS)
contract/placement-word/unplaced · apply — check 1: routine; one item of the closed placement words
contract/placement-word/window-unseen · apply — check 1: routine; one item of the closed placement words
contract/placement-word/unsupported · apply — check 1: routine; one item of the closed placement words
contract/placement-word/unreachable · apply — check 1: routine; one item of the closed placement words
contract/placement-word/oversize · apply — check 1: routine; one item of the closed placement words
contract/placement-word/malformed · apply — check 1: routine; one item of the closed placement words
contract/placement-word/refused · apply — check 1: routine; one item of the closed placement words
contract/origin-word/cli · apply — check 1: routine; one item of the closed origin words (report Symbols / APIs, ORIGIN_WORDS)
contract/origin-word/agent · apply — check 1: routine; one item of the closed origin words
decision/open-parent-defaults-to-caller · reject — check 2: the fact's home is layout-templates contract/viola-open/parent-default
contract/viola-open-refusals/terminal-not-found · reject — check 2: the fact's home is design-system contract/cli-exit-codes/1 (this file's own set) and architecture contract/viola-open-exit-codes/1
contract/viola-open-refusals/not-live · reject — check 2: the fact's home is design-system contract/cli-exit-codes/1 (this file's own set) and architecture contract/viola-open-exit-codes/1
decision/run-hidden-origin-and-place · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/run-origin and run-place
contract/viola-list-json/window · reject — check 2: the fact's home is architecture contract/list-payload/items
contract/viola-list-json/origin · reject — check 2: the fact's home is architecture contract/list-payload/items

ops-layout-templates.md
contract/cli-verbs/help · apply — check 5: expected amendment (layout-templates contract/cli-verbs/help)
contract/viola-help/verb-groups · apply — check 5: expected amendment (layout-templates contract/viola-help/verb-groups)
contract/cli-verbs/open · apply — check 1: routine; the master's standing CLI verbs boundary gains the new verb
contract/viola-open/usage-refusals · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/2
contract/viola-open-output/live · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/0
contract/viola-open-output/json-ok · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/0
contract/viola-open-json/refusal-document · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/1
contract/viola-open-refusals/terminal-not-found · reject — check 2: the fact's home is design-system contract/cli-exit-codes/1 and architecture contract/viola-open-exit-codes/1
contract/viola-open-refusals/not-live · reject — check 2: the fact's home is design-system contract/cli-exit-codes/1 and architecture contract/viola-open-exit-codes/1
contract/viola-open/parent-default · apply — check 1: routine; the verb's parent default and origin (report Symbols / APIs, own_name)
contract/viola-run/hidden-window-flags · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/run-origin and run-place
contract/viola-revive/no-window · apply — check 1: routine; revive hands no window ask (report Symbols / APIs, revive hands None)
decision/closed-placement-and-origin-words · reject — check 2: the fact's home is design-system contract/placement-word/* and contract/origin-word/*
decision/open-is-cli-only · reject — check 7: not_master type 5 — a record of what one chunk did not add
contract/viola-list-json/window-origin · reject — check 2: the fact's home is architecture contract/list-payload/items (layout's contract/viola-list-json/document defers its item shape to the list payload contract)

ops-test-plan.md
contract/viola-list-json/items · escalate → Q1 — check 1: boundary widening W3 (the list item gains window? and origin?), the operator's own answer arm (a), inputs#I4 · resolved: apply (Q1) — delegate overseer, 2026-11-24, inputs#I9: «An answer card whose every row lies inside his own recorded answers (item 2) is confirmed by the overseer.» — the card confirms rows inside W1–W3 arm (a), the operator's own answers, relayed verbatim by overseer1, 2026-11-23, inputs#I4
contract/run-start-events/order · apply — check 5: expected amendment (test-plan contract/run-start-events/order — an opened start's window line)
decision/run-start-verify-and-trust-after-bind · reject — check 7: not_master type 4 — the added step retells one call's place; the standing rule stays true
decision/fake-agent-on-test-path · apply — check 1: routine; the fake agent also stands as foot (report Symbols / APIs, test-only)
decision/fake-agent-as-claude · apply — check 1: routine; the fake agent also stands as foot
decision/fakes-for-external-services · apply — check 1: routine; the real foot is replaced by the fake in tests
decision/viola-open-argument-allowlist · reject — check 2: the fact's home is security-plan decision/open-closed-argument-allowlist
decision/run-window-flags-closed · reject — check 2: the fact's home is security-plan decision/run-window-flags-checked
contract/open-exit-1/terminal-not-found · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/1
contract/open-exit-1/not-live · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/1
contract/event-log/window · reject — check 2: the fact's home is architecture contract/event-data/window and contract/wrapper-event-appends/window
decision/snapshot-replay-window · reject — check 7: not_master type 4 — the replay's window rule is mechanism; main's set of decision/snapshot-replay-fields carries the recovered field (expected amendment dispositioned there)
decision/window-placement-proven-live · apply — check 1: routine; the acceptance rests real placement on the live reading alone (plan acceptance (fleet), report Outcome)
decision/live-rig-stand-in-parent · reject — check 1: the rule rests on the overseer's relay (inputs#I8), a delegate's word, and must land provisional; main's own decision/live-rig-clean-start carries it so
decision/echo-oracle-skips-allowed-word · apply — check 1: routine; a sweep hazard the chunk met (report Decisions & corrections)

ops-main-test-plan.md
decision/snapshot-replay-fields · apply — check 5: expected amendment (test-plan decision/snapshot-replay-fields); the standing rule's closed list reads false without window
decision/live-rig-clean-start · apply — check 1: no rule; the overseer's relay settles the rig's three mends (R2's red, After implement) · word: delegate overseer, 2026-11-24, inputs#I8

ops-obs-plan.md
contract/process-exit-detail/exit-1 · apply — check 5: expected amendment (obs-plan exit-1 details)
contract/run-start-spans/span-chain · apply — check 5: expected amendment (obs-plan contract/run-start-spans/span-chain, run.window)
contract/diag-subject/hyprctl-probe · reject — check 7: not_master type 5 — the added tail is a call-site inventory; the standing promise stays true
contract/process-log/child-and-probe-lifecycle · apply — check 1: routine; the open's terminal spawn is logged (report Schema / config, subject terminal)
contract/strict-modes-refusal-exit/1 · apply — check 1: routine; open exits 1 on a strict-modes refusal
contract/diag-subject/terminal · apply — check 5: expected amendment (obs-plan the subject terminal)
contract/run-span-attributes/run-window · apply — check 1: routine; run.window's one field outcome (report Symbols / APIs)
contract/cli-log-file/open · apply — check 1: routine; open logs as cli into cli-<name>.ndjson inside cli.open
decision/window-line-untraced · apply — check 5: expected amendment (obs-plan a window line untraced as the launch line)
contract/run-start-events/window-line · apply — check 5: expected amendment (obs-plan contract/run-start-events/order — the window line after launch; the standing order row stays true)
decision/origin-word-self-reported · reject — check 2: the fact's home is security-plan decision/window-origin-authenticates-nothing
decision/window-record-confined · reject — check 2: the fact's home is security-plan decision/window-record-kept-to-snapshot-and-event-log

ops-a11y-plan.md
decision/open-live-wait-bounds-the-driver · reject — check 7: stands as decision/time-bounds-fall-on-the-driver (also time-bounds-never-on-human); each standing list rule's 300-char cell cannot take the three bounds without dropping a name — founder-forks.md F1
decision/window-wait-bounds-the-wrapper-start · reject — check 7: stands as decision/time-bounds-fall-on-the-driver (also time-bounds-never-on-human); the same bound — founder-forks.md F1
decision/hyprctl-deadline-bounds-the-driver · reject — check 7: stands as decision/time-bounds-fall-on-the-driver (also time-bounds-never-on-human); the same bound — founder-forks.md F1 (carry cry-2oejf2md)
contract/open-refusal/terminal-not-found · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/1
contract/open-refusal/not-live · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/1
contract/viola-open/live-line · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/0
contract/viola-open/json-ok · reject — check 2: the fact's home is architecture contract/viola-open-exit-codes/0
decision/placement-carried-by-printed-word · reject — check 2: the fact's home is design-system contract/placement-word/*
contract/cli-exit-codes/2 · apply — check 1: routine; the master's standing exit-2 row gains open's usage refusal
contract/cli-refusal/json · apply — check 1: routine; open's exit-2 path prints a fixed line and no document
decision/json-one-document-no-hint · apply — check 1: routine; the exception names open's exit-2 path
decision/screen-reader-target-tui · reject — check 1: the chunk changed no screen-reader target; the set adds a mention the report does not rest a change on
