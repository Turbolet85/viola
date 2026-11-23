@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=80
last: - 2026-11-20: A plan's non-leg entry that reads a file a rou
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=29 class=stale
leaf-text: then `events.ndjson` lines 1, 2 and 3 = `wheel{cause:start}` → `lineage{source:wrapper}` → `session-start{source:hook}`
source: ctr-ephtbyyb «Event lines 1, 2, 3 and 4 are a wheel line with cause start, a lineage line with source wrapper, a launch line with source wrapper, then session-start with source hook; missing reads events.»
replace-with: then `events.ndjson` lines 1, 2, 3 and 4 = `wheel{cause:start}` → `lineage{source:wrapper}` → `launch{source:wrapper}` → `session-start{source:hook}`
