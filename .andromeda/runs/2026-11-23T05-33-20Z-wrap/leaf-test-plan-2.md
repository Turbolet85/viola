@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=80
last: - 2026-11-20: A plan's non-leg entry that reads a file a roun
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=29 class=stale
leaf-text: then `events.ndjson` lines 1 and 2 = `wheel{cause:start}` → `session-start{source:hook}`
source: ctr-ephtbyyb «Event lines 1, 2 and 3 are a wheel line with cause start, a lineage line with source wrapper, then session-start with source hook; missing reads events.»
replace-with: then `events.ndjson` lines 1, 2 and 3 = `wheel{cause:start}` → `lineage{source:wrapper}` → `session-start{source:hook}`
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=28 class=omission
leaf-text: it hands the fake agent `--screens --trusted-root <cwd>`;
source: dec-kmv5o4xs «The harness supervisor's wrapper spawn removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN.»
replace-with: it hands the fake agent `--screens --trusted-root <cwd>`; its wrapper spawn removes the inherited `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, so a session booted inside a wrapped session records no parent on its instances;
@leaf_stands n=1 leaf=.claude/rules/verification-harness.md line=39
because: The line describes only the fake agent's scripted-turn format and control-file gating, and makes no claim about `viola list` columns, so the change to ctr-q5dspexy leaves it true.
@leaf_stands n=2 leaf=.claude/rules/verification-harness.md line=35
because: The line states only how `logs` merges and filters the events and diagnostics files, which no changed row on revive, snapshot replay, run-start order or link-name logging contradicts.
