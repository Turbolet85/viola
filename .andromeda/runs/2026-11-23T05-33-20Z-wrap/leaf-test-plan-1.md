@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=132
last: - 2026-11-22: A rename of a product constant or a move of a 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=25 class=stale
leaf-text: event order `wheel{start}` → `session-start`
source: ctr-fkljkm43 «contract/run-start-events/order · run start events · order — The events log reads wheel with cause start, then lineage with source wrapper, then the agent's session-start with source hook.»
replace-with: event order `wheel{start}` → `lineage{source:wrapper}` → `session-start`
@leaf_stands n=1 leaf=.claude/docs/tests-summary.md line=21
because: The line's `v1` tokens are fixture schema file names, not the retired `in v1` qualifier of the list-columns row, and nothing in it describes `viola list` columns.
@leaf_stands n=2 leaf=.claude/docs/tests-summary.md line=28
because: The line's `v1-15` and `v1-16` are matrix row ids, unrelated to the `in v1` qualifier the list-columns row dropped.
@leaf_stands n=3 leaf=.claude/docs/tests-summary.md line=42
because: The line's `matrix#v1-44` is a matrix row id for the peer admission survivor, unrelated to the list-columns row.
@leaf_stands n=4 leaf=.claude/docs/tests-summary.md line=19
because: `events.ndjson` still is the arch event line log the rows name; the Log format statement claims nothing about link-name placement, replay fields or start order.
@leaf_stands n=5 leaf=.claude/docs/tests-summary.md line=27
because: Its `events.ndjson` mention (a Stop's turn-ended data reaching it) is untouched by the changed rows, which only added lineage and dropped where link names appear.
@leaf_stands n=6 leaf=.claude/docs/tests-summary.md line=50
because: Verdicts still come from `events.ndjson` among other surfaces, which no changed row contradicts.
@leaf_stands n=7 leaf=.claude/rules/testing.md line=26
because: The line's `v1` tokens are MCP fixture schema file names, not the `in v1` qualifier the list-columns row dropped.
@leaf_stands n=8 leaf=.claude/rules/testing.md line=48
because: The line's `matrix#v1-44` is a matrix row id, unrelated to the list-columns row.
@leaf_stands n=9 leaf=.claude/rules/testing.md line=20
because: It names the shared `events.ndjson` reader in `tests/support/events.rs`, a helper no changed row speaks to.
@leaf_stands n=10 leaf=.claude/rules/testing.md line=29
because: Waiting on an `events.ndjson` byte offset stays the sync rule; the changed rows only add a lineage line and drop where link names appear.
@leaf_stands n=11 leaf=.claude/rules/testing.md line=40
because: Verdicts still come from `events.ndjson` among other surfaces, which no changed row contradicts.
