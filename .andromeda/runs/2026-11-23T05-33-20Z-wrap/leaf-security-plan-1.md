@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=64 class=omission
leaf-text: joins paths only from a parsed `ViolaName`, follows no link, and escapes every control character of a row (`escape_row`).
source: dec-ihvcipcw «The board shows a recorded parent only when it re-parses as a viola name other than the row's own, a role only when it is a code word, and a link pair only when both names parse»
replace-with: joins paths only from a parsed `ViolaName`, follows no link, shows a recorded parent only when it re-parses as a viola name other than the row's own, a role only when it is a code word and a link pair only when both names parse (a gone instance gives no links), and escapes every control character of a row (`escape_row`).
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=29 class=omission
leaf-text: | Low (operational) | pids, `agent_session_id`, OS username in paths | Typed fields in diagnostics; paths never in external errors |
source: dec-cngvoifp «An instance's lineage (parent and role names) is kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte.»
replace-with: | Low (operational) | pids, `agent_session_id`, OS username in paths, an instance's lineage (parent and role names) | Typed fields in diagnostics; paths never in external errors; lineage kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte |
@leaf_stands n=1 leaf=.claude/docs/security-summary.md line=63
because: The line states the `release` refusal as an affordance guard among trusted same-user processes, while ctr-q5dspexy only dropped "in v1" from the `viola list` column promise, which this line does not describe.
@leaf_stands n=2 leaf=.claude/docs/security-summary.md line=27
because: The line confines user content to contract payloads such as `events.ndjson`, and none of the named rows changed that: dec-6dmakk3w still keeps link names out of process logs, and the other rows only add lineage to revive, replay and start order.
@leaf_stands n=3 leaf=.claude/docs/security-summary.md line=52
because: The retention question still holds open with every scheme keeping 0600 and valid offsets, and the named rows only add lineage to `events.ndjson` lines and replay without touching retention or modes.
