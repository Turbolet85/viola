# Fan-out results — 2026-11-22-lineage-registry (wrap 2026-11-23T05-33-20Z-wrap)

architecture: add 13 · set 9 · retire 0 · none 0 · ops-architecture.md (re-spawned once, RULED: `decision/lineage-line-at-start` gained `state: provisional`)
security-plan: add 6 · set 0 · retire 0 · none 1 · ops-security-plan.md (re-spawned once, RULED: three adds gained `state: provisional`, `decision/lineage-parent-authenticates-nothing` re-worded)
design-system: add 7 · set 4 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 6 · set 6 · retire 0 · none 0 · ops-layout-templates.md (re-spawned once, RULED: `decision/board-ordered-by-name`, `decision/no-link-column` re-worded)
test-plan: add 12 · set 4 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 3 · set 3 · retire 0 · none 0 · ops-obs-plan.md (re-spawned once, RULED: two adds gained `state: provisional`)
a11y-plan: add 6 · set 3 · retire 0 · none 0 · ops-a11y-plan.md
main's own: ops-main-design-system.md (set 3) · ops-main-layout-templates.md (set 1) · ops-main-obs-plan.md (set 1; its first draft, a `state` set, withdrawn before any fit or landing) · ops-main-architecture.md (set 2)
Removed by a fix: none. Prior files kept as `.raw-ops-{doc}.md` for the four re-spawned masters.

ops-architecture.md
decision/wrapper-events-log-only · apply — check 5: the plan's Expected entry; `lineage` is a wrapper-appended, log-only kind (report Symbols) · word: delegate overseer, 2026-11-22 (F3)
decision/snapshot-replay-log-fields-only · reject — check 7: not_master type 4 — «the added clause that the last lineage line replaces the held value retells how the replay's lineage step is built»; the standing row is false, its shorter set in ops-main-architecture.md
contract/snapshot-envelope/unreadable-snapshot · apply — check 1: routine «accurate this-chunk addition»; the closed list of recovered fields restated with `lineage`
contract/instance-snapshot/data · apply — check 5: the plan's Expected entry; snapshot `data.lineage?`
contract/list-payload/items · apply — check 5: the plan's Expected entry; item `parent?`, `role?`
contract/list-payload/ok · escalate → Q1 — check 1: the boundary widening of logged names to MCP `list` (F6, the plan's own escalation line); the shape restated · resolved: apply (Q1) — delegate overseer, 2026-11-22: F6 arm (a), «Additive: parent and role on a wrapped item when present, a top-level links array … no v move» — PROVISIONAL, the most conservative option of founder-forks.md row W1
contract/mcp-tool-results/success · escalate → Q1 — check 1: the same widening, MCP `list`'s success shape · resolved: apply (Q1) — delegate overseer, 2026-11-22: F6 arm (a) — PROVISIONAL, founder-forks.md row W1
decision/board-read-model-shape · reject — check 7: not_master type 4 — «the added clause lists the board link's fields and its name-check filtering»; escalated (Q1), not asked; the standing row's ban on links is false, its shorter set in ops-main-architecture.md
decision/revive-in-place · apply — check 5: the plan's Expected entry; `recorded_lineage` (report Symbols) · word: delegate overseer, 2026-11-22 (F7)
contract/normalised-event-kinds/lineage · apply — check 5: the plan's Expected entry; the sixteenth kind, its own row as each kind stands — the route entry's own «a `lineage` event at launch» and v2-09's requirement, no fork
contract/event-data/lineage · apply — check 2: the wire contract's home is architecture; test-plan's twin rejected — its keys are v2-09's requirement («a declared parent and role … as a `lineage` event»), no fork
contract/instance-snapshot/lineage · apply — check 2: the snapshot field's home is architecture; test-plan's twin rejected
contract/list-item/parent · reject — re-added in ops-main-architecture.md on Q1's route (a contract has no `state` cell): an add on the overseer's F6 and F8, a delegate's word
contract/list-item/role · reject — re-added in ops-main-architecture.md on Q1's route (a contract has no `state` cell): an add on the overseer's F6, a delegate's word
contract/list-payload/links · escalate → Q1 — check 1: `ok.links` reaches MCP `list` (F6) · resolved: apply (Q1) — delegate overseer, 2026-11-22: F6 arm (a) — PROVISIONAL, founder-forks.md row W1
contract/viola-cli-arguments/run-parent · escalate → Q2 — check 1: the inherited `VIOLA_NAME`'s second use, a boundary widening the operator answered himself (F1); the exit 2 on a self-parent is F8 · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F1 «Флаг и наследование (Recommended)» (the card confirmed by the overseer, inputs#I7)
contract/viola-cli-arguments/run-role · apply — check 1: routine «accurate this-chunk addition»; `--role`, the operator's F2 · word: relayed verbatim by overseer, 2026-11-22, inputs#I2 (F2)
decision/lineage-line-at-start · reject — check 7: not_master type 4 — «retells how the start function orders its snapshot, wheel line and lineage line»; decision/run-start-order stands («start events» covers the line)
decision/lineage-label-only · apply — check 1: routine; the requirement's «a parent edge is a drawing label only», held by the census (report Symbols, Outcome); this fact's one home — design-system, layout-templates and test-plan twins rejected
decision/list-board-tree · reject — check 2: the fact's home is layout-templates contract/viola-list-board/tree
decision/list-board-links-block · reject — check 2: the fact's home is layout-templates contract/viola-list-board/links-block
decision/harness-wrapper-drops-viola-env · reject — check 2: the fact's home is test-plan decision/supervisor-spawn-strips-bridge-env

ops-security-plan.md
decision/run-lineage-flags-checked · apply — check 1: routine; the flags' input checks, security-plan's question; `state: provisional` · word: delegate overseer, 2026-11-22 (F8)
decision/lineage-parent-authenticates-nothing · escalate → Q2 — check 1: the inherited `VIOLA_NAME`'s second use, a widening the operator answered himself (F1): read as the parent only, never the instance, never an identity, never joined into a path (read in `src/cmd/run.rs` `lineage_of` and `board.rs` `shown_lineage`: a string, re-parsed, never a path) · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F1 «Флаг и наследование (Recommended)» (the card confirmed by the overseer, inputs#I7)
decision/board-lineage-and-links-name-checked · apply — check 2: the display filter's home is security-plan's input validation; a11y-plan's twin rejected; `state: provisional` · word: delegate overseer, 2026-11-22 (F8)
decision/lineage-kept-to-snapshot-and-event-log · apply — check 1: routine; where the new names stand at a start (report Symbols: no process-log line, span or terminal byte)
decision/list-carries-lineage-and-links · reject — check 7: not_master type 4 — «retells the additive list --json and MCP list schema fields»; escalated (Q1), not asked; the widening's provisional home is obs-plan decision/lineage-names-confined
decision/lineage-from-real-snapshot-only · reject — check 7: not_master type 4 — «retells how the replay's lineage line handling and revive's recorded-lineage read are built»

ops-design-system.md
decision/stable-session-slots · reject — check 3: the strip bay is the fleet console not yet built; the report carries no fact of its order
contract/cli-transfer-arrow/ascii-arrow · escalate → Q2 — check 5: the plan's Expected entry, the operator's own row moved by F5 (logged link names shown on `list`, a widening he answered himself) · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F5 «Блок LINKS под доской (Recommended)» (the card confirmed by the overseer, inputs#I7)
contract/viola-list-table/row-order · reject — check 2: the tree's home is layout-templates contract/viola-list-board/tree; this row's own claim set shorter in ops-main-design-system.md
contract/viola-list-board/name-column-width · apply — check 1: routine; the NAME column counts the tree indent (report Symbols) · word: relayed verbatim by overseer, 2026-11-22, inputs#I2 (F4)
contract/viola-run-flags/parent · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/run-parent
contract/viola-run-flags/role · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/run-role
contract/viola-list-board/links-block · reject — check 2: the fact's home is layout-templates contract/viola-list-board/links-block
contract/viola-list-json/item-parent-role · reject — check 2: the fact's home is architecture contract/list-item/parent and contract/list-item/role
contract/viola-list-json/ok-links · reject — check 2: the fact's home is architecture contract/list-payload/links
decision/lineage-shown-never-acted-on · reject — check 2: the fact's home is architecture decision/lineage-label-only
decision/revive-keeps-lineage · reject — check 2: the fact's home is architecture decision/revive-in-place

ops-layout-templates.md
decision/board-ordered-by-name · escalate → Q2 — check 5: the plan's Expected entry, the operator's own row moved by F4; the order as BUILT (the report's first deviation): the board's own name order, no second sort, a child sorting before its parent drawn after it, under it (`board_tree_siblings_and_roots_are_in_name_order`) · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F4 «Отступ в NAME везде (Recommended)» (the card confirmed by the overseer, inputs#I7)
decision/no-link-column · reject — check 7 form: the re-spawn copied the direction's «in the same cell» into the rule; the row set in ops-main-layout-templates.md
contract/viola-list-board/single-write · apply — check 1: routine; the single write gains the tree order and the LINKS block (report Symbols) · word: relayed verbatim by overseer, 2026-11-22, inputs#I2 (F4, F5)
contract/viola-list-board/column-widths · apply — check 1: routine; NAME counts its indent · word: relayed verbatim by overseer, 2026-11-22, inputs#I2 (F4)
decision/mcp-hides-wheel-and-handoff-verbs · escalate → Q1 — check 1: MCP `list`'s result now carries link pairs (F6) · resolved: apply (Q1) — delegate overseer, 2026-11-22: F6 arm (a) — PROVISIONAL, founder-forks.md row W1
contract/cli-verbs/revive · reject — check 2: the fact's home is architecture decision/revive-in-place; the verb row stays true as written
contract/viola-run-flags/parent-and-role · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/run-parent and run-role
decision/inherited-session-name-as-parent · reject — check 2: the fact's home is security-plan decision/lineage-parent-authenticates-nothing and obs-plan decision/inherited-name-default-parent
contract/viola-list-board/tree · escalate → Q2 — check 1: the tree in the NAME cell, a moved row family of the operator's F4 · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F4 «Отступ в NAME везде (Recommended)» (the card confirmed by the overseer, inputs#I7)
contract/viola-list-board/links-block · escalate → Q2 — check 1: logged link names shown on `list`, a widening the operator answered himself (F5) · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F5 «Блок LINKS под доской (Recommended)» (the card confirmed by the overseer, inputs#I7)
contract/viola-list-board/parent-and-role · reject — check 2: the fact's home is security-plan decision/board-lineage-and-links-name-checked
decision/lineage-display-only · reject — check 2: the fact's home is architecture decision/lineage-label-only

ops-test-plan.md
contract/viola-list-json/items · apply — check 1: routine; the item shape restated with `parent?`, `role?` (Counts moved: keys up to 10)
decision/snapshot-replay-fields · apply — check 5: the plan's Expected entry; the replay's fourth field
contract/run-start-events/order · apply — check 5: the start's three records (Counts moved 2 → 3) · word: delegate overseer, 2026-11-22 (F3)
contract/boot-readiness/events · apply — check 5: the plan's Expected entry; `start_records` reads three lines
contract/event-data/lineage · reject — check 2: the fact's home is architecture contract/event-data/lineage
contract/viola-run-flags/parent · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/run-parent
contract/viola-run-flags/role · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/run-role
decision/run-parent-from-inherited-name · reject — check 2: the fact's home is security-plan decision/lineage-parent-authenticates-nothing and obs-plan decision/inherited-name-default-parent
decision/revive-keeps-lineage · reject — check 2: the fact's home is architecture decision/revive-in-place
decision/list-board-tree · reject — check 2: the fact's home is layout-templates contract/viola-list-board/tree
contract/list-human/links-block · reject — check 2: the fact's home is layout-templates contract/viola-list-board/links-block
contract/viola-list-json/item-lineage · reject — check 2: the fact's home is architecture contract/list-item/parent and contract/list-item/role
contract/viola-list-json/links · reject — check 2: the fact's home is architecture contract/list-payload/links
decision/lineage-read-by-no-control-path · reject — check 2: the fact's home is architecture decision/lineage-label-only
decision/supervisor-spawn-strips-bridge-env · apply — check 1: routine; the operator pass's harness fix (After implement), the test-plan row that now names the supervisor beside the three «test-started viola drops» rows
contract/instance-snapshot/lineage · reject — check 2: the fact's home is architecture contract/instance-snapshot/lineage

ops-obs-plan.md
contract/run-start-events/order · apply — check 5: the plan's Expected entry; the lineage line in the start order · word: delegate overseer, 2026-11-22 (F3)
decision/product-order-from-events-log · apply — check 5: the plan's Expected entry · word: delegate overseer, 2026-11-22 (F3)
decision/link-names-never-in-process-log · reject — check 7: not_master type 4 — «lengthens a fitting confinement rule with a list of the board's read surfaces»; escalated (Q1), not asked; the standing row's «only in the link and unlink event lines» is false, its shorter set in ops-main-obs-plan.md
decision/lineage-line-untraced · apply — check 1: routine; no line or span for the lineage line (report Symbols); `state: provisional` · word: delegate overseer, 2026-11-22 (F3)
decision/lineage-names-confined · escalate → Q1 — check 1: parent and role names reach MCP `list` (F6) · resolved: apply (Q1) — delegate overseer, 2026-11-22: F6 arm (a) — PROVISIONAL (`state: provisional`), founder-forks.md row W1
decision/inherited-name-default-parent · reject — check 7: not_master type 4 — «only retells how run's lineage_of picks a parent … the instance never comes from VIOLA_NAME already stands in decision/instance-source-per-role»; escalated (Q2), not asked

ops-a11y-plan.md
contract/viola-list-board/order · escalate → Q2 — check 5: the plan's Expected entry, the operator's own row moved by F4 and F5 · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F4 «Отступ в NAME везде (Recommended)»; F5 «Блок LINKS под доской (Recommended)» (the card confirmed by the overseer, inputs#I7)
decision/terminal-structural-equivalents · escalate → Q2 — check 5: the plan's Expected entry, moved by F4 and F5 · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F4 «Отступ в NAME везде (Recommended)»; F5 «Блок LINKS под доской (Recommended)» (the card confirmed by the overseer, inputs#I7)
decision/cli-structural-landmark-equivalents · apply — check 1: routine; the same structure list restated with the LINKS separator and the indent · word: relayed verbatim by overseer, 2026-11-22, inputs#I2 (F4, F5)
contract/viola-list-board/links-separator · reject — check 2: the fact's home is layout-templates contract/viola-list-board/links-block
contract/viola-list-board/tree · reject — check 2: the fact's home is layout-templates contract/viola-list-board/tree
decision/name-column-holds-tree-indent · reject — check 2: the NAME width's home is layout-templates contract/viola-list-board/column-widths; the dim span over the indent tells how one function is built
contract/viola-run-flags/parent · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/run-parent
contract/viola-run-flags/role · reject — check 2: the fact's home is architecture contract/viola-cli-arguments/run-role
decision/board-lineage-and-links-name-checked · reject — check 2: the fact's home is security-plan decision/board-lineage-and-links-name-checked

ops-main-design-system.md
decision/list-six-fixed-columns · escalate → Q2 — check 5: the plan's Expected entry, the operator's own row moved by F4 and F5: the six columns, their captions and slot order unchanged in the same cell · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F4 «Отступ в NAME везде (Recommended)»; F5 «Блок LINKS под доской (Recommended)» (the card confirmed by the overseer, inputs#I7)
contract/viola-list/columns · escalate → Q2 — check 5: the plan's Expected entry, moved by F4 and F5 · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F4 «Отступ в NAME везде (Recommended)»; F5 «Блок LINKS под доской (Recommended)» (the card confirmed by the overseer, inputs#I7)
contract/viola-list-table/row-order · apply — check 1: the row's own claim, «ordered by name», now short of the tree as built · word: relayed verbatim by overseer, 2026-11-22, inputs#I2 (F4)

ops-main-layout-templates.md
decision/no-link-column · escalate → Q2 — check 5: the plan's Expected entry, the operator's own row moved by F5: the six columns, captions and slot order unchanged in the same cell · resolved: apply (Q2) — relayed verbatim by overseer, 2026-11-22, inputs#I2: F5 «Блок LINKS под доской (Recommended)» (the card confirmed by the overseer, inputs#I7)

ops-main-obs-plan.md
decision/link-names-never-in-process-log · apply — check 7's owed correction: the standing row's false «only in the link and unlink event lines» cut to its true half, nothing added

ops-main-architecture.md
decision/snapshot-replay-log-fields-only · apply — check 7's owed correction: the closed list gains `lineage`, nothing else added
decision/board-read-model-shape · apply — check 7's owed correction: the false ban on links cut, nothing added
contract/list-item/parent · escalate → Q1 — check 1: the item's `parent` reaches MCP `list` (F6); a contract has no `state` cell, so the delegate's word rides Q1 · resolved: apply (Q1) — delegate overseer, 2026-11-22: F6 arm (a) — PROVISIONAL, founder-forks.md row W1
contract/list-item/role · escalate → Q1 — check 1: the item's `role` reaches MCP `list` (F6); a contract has no `state` cell, so the delegate's word rides Q1 · resolved: apply (Q1) — delegate overseer, 2026-11-22: F6 arm (a) — PROVISIONAL, founder-forks.md row W1
