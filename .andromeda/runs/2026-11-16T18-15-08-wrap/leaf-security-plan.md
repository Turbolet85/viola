@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=27 class=stale
leaf-text: only the closed cause word `api-error` crossing (provisional)
source: dec-woerghls «an API-error end's turn-ended data holds only null last_assistant_message and background_tasks and one closed nine-word cause set word, which crosses into events.ndjson, wait --json and MCP wait.»
replace-with: only one word of the closed nine-word turn-end cause set crossing
@leaf_edit n=2 leaf=.claude/rules/security.md line=25 class=stale
leaf-text: and the closed cause word `api-error` (crossing into `events.ndjson`, `wait --json` and MCP `wait`)
source: dec-woerghls «an API-error end's turn-ended data holds only null last_assistant_message and background_tasks and one closed nine-word cause set word, which crosses into events.ndjson, wait --json and MCP wait.»
replace-with: and one word of the closed nine-word turn-end cause set (crossing into `events.ndjson`, `wait --json` and MCP `wait`)
@leaf_edit n=3 leaf=.claude/rules/security.md line=25 class=stale
leaf-text: its error word's nine closed API-error classes stay inside the Claude adapter crate, never in event data, an event line, the snapshot or a log line (provisional)
source: dec-oc7btmpx «The CLI's StopFailure error word is classified into one of the nine closed turn-end cause words inside the Claude adapter crate only; only that cause word crosses out of the adapter»
replace-with: its error word is classified into one of those nine cause words inside the Claude adapter crate only, and only that cause word crosses out of the adapter, never the error word or any error text
