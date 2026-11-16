@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/security.md line=25 class=omission
leaf-text: carried only as its length (`null` when absent, `null` or not an array);
source: dec-woerghls «The StopFailure payload's error text, error_details and last_assistant_message are dropped at the parse; an API-error end's turn-ended data holds only null last_assistant_message and background_tasks»
replace-with: carried only as its length (`null` when absent, `null` or not an array); a StopFailure's error text, `error_details` and `last_assistant_message` are dropped at the parse, its turn-ended data holding only `null` `last_assistant_message` and `background_tasks` and the closed cause word `api-error` (crossing into `events.ndjson`, `wait --json` and MCP `wait`), and its error word's nine closed API-error classes stay inside the Claude adapter crate, never in event data, an event line, the snapshot or a log line (provisional);
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=27 class=omission
leaf-text: never in stderr, `--json` errors, MCP errors, channel `error.data`
source: dec-woerghls «The StopFailure payload's error text, error_details and last_assistant_message are dropped at the parse; an API-error end's turn-ended data holds only null last_assistant_message and background_tasks»
replace-with: never in stderr, `--json` errors, MCP errors, channel `error.data`; a StopFailure's error text, `error_details` and `last_assistant_message` dropped at the parse, only the closed cause word `api-error` crossing (provisional)
@leaf_edit n=3 leaf=.claude/rules/security.md line=19 class=omission
leaf-text: A non-`null` dialog decision requires a `viola verify` stamp for the child's CLI version;
source: dec-t4wfjgzz «No non-null answer decision is sent without a viola verify stamp for the child's CLI version. · except: StopFailure is registered with no capability ledger row and no verify probe, owned by the 2.1.325 recorded fixture set;»
replace-with: A non-`null` dialog decision requires a `viola verify` stamp for the child's CLI version (provisional exception: StopFailure is registered with no capability ledger row and no verify probe, owned by the 2.1.325 recorded fixture set);
