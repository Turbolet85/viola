@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/testing.md lines=117
last: - 2026-11-15: A removal sweeps every test that reads the remo
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: (after a `prompt-submitted` of any origin, before `turn-ended`) → exit 13 `turn-running`
source: ctr-iirgwvem «contract/viola-send/turn-running · viola send · turn-running refusal — While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first;»
replace-with: (after a `prompt-submitted` of any origin, before `turn-ended`, an API-error turn end included, after which a send is accepted with no pause or release) → exit 13 `turn-running`
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=27 class=omission
leaf-text: a Stop's `turn-ended` data carries `background_tasks`, the Stop array's length or `null`
source: ctr-gyhkfi7n «contract/event-data/turn-ended · Event data keys · turn-ended — data holds the keys last_assistant_message, background_tasks and cause; cause is present on an API-error turn end only and absent on a Stop.»
replace-with: a Stop's `turn-ended` data carries `background_tasks`, the Stop array's length or `null`, and an API-error turn end's carries `cause` too, absent on a Stop
@leaf_edit n=3 leaf=.claude/rules/testing.md line=24 class=omission
leaf-text: (the CLI's PascalCase hook event name, e.g. `PreToolUse.ask.json`; no mapping table)
source: dec-gf63skju «decision/hook-payloads-recorded-by-verify · Hook payloads recorded by verify, replayed by CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are commit»
replace-with: (the CLI's PascalCase hook event name, e.g. `PreToolUse.ask.json`; no mapping table; StopFailure, provisionally, has no recorded payload, ledger row or verify probe, and its tests use a synthetic StopFailure payload)
@leaf_edit n=4 leaf=.claude/rules/testing.md line=35 class=omission
leaf-text: Fixtures are synthetic probe text or a test-only recorder's capture (`viola-mcp-tap`, `viola-hypr-tap`) only,
source: dec-ngqgaouf «decision/send-cases-copy-recorded-fixtures · Send-after-turn cases copy the recorded fixtures — A fake-agent fixtures root for a case that sends after the first turn starts as a copy of the recorded fixture set»
replace-with: Fixtures are synthetic probe text or a test-only recorder's capture (`viola-mcp-tap`, `viola-hypr-tap`) only (a fake-agent fixtures root for a case that sends after the first turn starts as a copy of the recorded fixture set, its synthetic payloads added to it: synthetic payloads alone draw no input box and the send reads input-not-ready),
