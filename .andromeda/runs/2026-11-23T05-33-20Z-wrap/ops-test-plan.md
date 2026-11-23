@row_set n=1 row=contract/viola-list-json/items cell=promise seen=2 detector=D-tests-count-literals
value: In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?, parent?, role?}.
basis: fct-sdz3kc3h, fct-uoqmd72r
because: A list item's keys moved up to ten with parent and role when present, so the item shape the row states must name them.
@row_set n=2 row=decision/snapshot-replay-fields cell=rule seen=2 fact=F5 detector=D-tests-count-literals
value: A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id, wheel and lineage with dialog_pending false, the last lineage line winning so a later line with no parent clears it; the replay writes no file and gives no cwd.
basis: fct-sdz3kc3h, fct-yurmbmhi
because: The replay's recovered fields moved from three to four with lineage, taken from the last lineage line, so the fixed field set the row states must follow.
@row_set n=3 row=contract/run-start-events/order cell=promise seen=2 detector=D-tests-count-literals
value: The events log reads wheel with cause start, then lineage with source wrapper, then the agent's session-start with source hook.
basis: fct-sdz3kc3h, fct-yurmbmhi
because: A start's records moved from two to three with the lineage line between the wheel start line and session-start, so the stated order is stale.
@row_set n=4 row=contract/boot-readiness/events cell=promise seen=2 detector=D-tests-count-literals
value: Event lines 1, 2 and 3 are a wheel line with cause start, a lineage line with source wrapper, then session-start with source hook; missing reads events.
basis: fct-ok4jj725, fct-sdz3kc3h
because: Boot readiness now reads three start records (wheel, lineage, session-start), so the row's two-line readiness is stale.
@row_add n=1 row=contract/event-data/lineage area=integration-test-strategy fact=F1 detector=none
boundary: Event data keys
item: lineage
promise: data holds the keys parent and role, each a string or null; the line is written with source wrapper, is log-only and is not a wake kind.
basis: fct-yurmbmhi, fct-uoqmd72r
because: The chunk adds lineage as the sixteenth event kind with its own data keys, a contract the per-kind data rows do not yet hold.
@row_add n=2 row=contract/viola-run-flags/parent area=6-e2e-test-strategy fact=F2 detector=none
boundary: viola run flags
item: --parent
promise: --parent NAME takes a viola name as the instance's parent; a --parent equal to the instance's own name is a usage error, exit 2, before anything is read.
basis: fct-yurmbmhi, fct-evi6pgaa
because: viola run gains the --parent flag with a self-parent usage error, a CLI contract no row holds.
@row_add n=3 row=contract/viola-run-flags/role area=6-e2e-test-strategy fact=F2 detector=none
boundary: viola run flags
item: --role
promise: --role WORD takes a code word as the instance's role.
basis: fct-yurmbmhi
because: viola run gains the --role flag, a CLI contract no row holds.
@row_add n=4 row=decision/run-parent-from-inherited-name area=6-e2e-test-strategy fact=F3 detector=none
name: Inherited viola name is the parent only
rule: With no --parent flag, viola run takes the inherited VIOLA_NAME as the parent only, never as the role, and drops it when it equals the instance's own name.
basis: fct-yurmbmhi
because: The inherited name gains a second use as a self-reported parent, a rule no row of the area states.
@row_add n=5 row=decision/revive-keeps-lineage area=test-scope-summary fact=F4 detector=none
name: Revive keeps the recorded lineage
rule: viola revive keeps the parent and role recorded in the real snapshot.
guard: revive_keeps_the_recorded_parent_and_role
basis: fct-yurmbmhi, fct-ao5bhrj4
because: revive now hands the real snapshot's parent and role into its start, a behaviour no row holds.
@row_add n=6 row=decision/list-board-tree area=e2e-test-strategy fact=F6 detector=none
name: List board drawn as a parent tree
rule: The viola list human board draws each wrapped row under its parent, depth-first in the board's name order, with NAME indented two spaces per depth; a row whose parent has no wrapped row is a root, and cycle members are drawn once from the first in order.
basis: fct-yurmbmhi, fct-gpcfukhg
because: The human board gains a tree layout, a decision no list board row states.
@row_add n=7 row=contract/list-human/links-block area=e2e-test-strategy fact=F7 detector=none
boundary: viola list human board
item: links block
promise: After the board and the unwrapped rack a -- LINKS -- separator line is followed by one driver -> driven line per link, with no SGR; with no links there is no block.
basis: fct-yurmbmhi
because: The human board gains the links block beside the unwrapped separator, a contract no row holds.
@row_add n=8 row=contract/viola-list-json/item-lineage area=1-test-scope-summary fact=F8 detector=none
boundary: viola list --json
item: ok.items parent and role
promise: list --json and MCP list items carry parent and role only when present: a parent is kept only when it parses as a viola name other than the row's own, and a role only when it passes the code-word check.
basis: fct-yurmbmhi, fct-uoqmd72r
because: The list items gain name-checked parent and role, a rule the item shape row cannot hold within its bound.
@row_add n=9 row=contract/viola-list-json/links area=1-test-scope-summary fact=F9 detector=none
boundary: viola list --json
item: ok.links
promise: ok.links is the union of every listed live or stale instance's links, sorted by driver then driven, deduplicated, each pair kept only when both names parse; a gone instance gives none, and the key is omitted when empty.
basis: fct-yurmbmhi, fct-uoqmd72r
because: The list document's ok gains links when non-empty, a contract no row holds.
@row_add n=10 row=decision/lineage-read-by-no-control-path area=test-strategy fact=F10 detector=D-tests-coverage
name: Lineage read by no wheel, link, send or answer path
rule: No wheel, link, send or answer path reads lineage, held by an E2E census test proven to bite by a planted hit.
guard: lineage_is_read_by_no_wheel_link_send_or_answer_path, lineage_census_reads_a_planted_hit
basis: fct-y6npdhbi, fct-ao5bhrj4
because: The chunk adds an E2E-layer census binary holding that lineage is a label only, a decision no row states.
@row_add n=11 row=decision/supervisor-spawn-strips-bridge-env area=5-command-implementation fact=F11 detector=D-tests-obs-harness
name: Harness wrapper spawn strips the bridge env
rule: The harness supervisor's wrapper spawn removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN.
why: so a harness session booted inside a wrapped session records no parent on its instances
guard: harness_session_boots_reports_logs_and_tears_down
basis: fct-dbracsuy, fct-ok4jj725
because: The supervisor's spawn now drops the inherited names as harness status already did, a rule no harness row states for the spawn.
@row_add n=12 row=contract/instance-snapshot/lineage area=e2e-test-strategy detector=none
boundary: instance snapshot
item: lineage
promise: The instance snapshot holds an optional lineage of parent and role, each omitted when absent.
basis: fct-uoqmd72r, fct-yurmbmhi
because: The snapshot gains a lineage field beside the per-field rows the area holds, and none names it.
