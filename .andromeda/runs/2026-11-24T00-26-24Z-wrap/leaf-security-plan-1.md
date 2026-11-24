@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=17 class=stale
leaf-text: `run`, `revive`'s start and `verify` through `check_home_tree`
source: dec-oeec26zu «run, revive's start, open and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.»
replace-with: `run`, `revive`'s start, `open` and `verify` through `check_home_tree`
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=29 class=omission
leaf-text: leaving only through `list --json` and the MCP list item |
source: dec-4rj6v4ng «An instance's window record (address, workspace, placement and origin) is kept only in the snapshot and the event log's window line and leaves only through list --json and the MCP list item»
replace-with: leaving only through `list --json` and the MCP list item; an instance's window record (address, workspace, placement and origin) kept only in the snapshot and the event log's window line, leaving only through `list --json` and the MCP list item, with no address, pid or workspace in a process-log line |
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=64 class=omission
leaf-text: and a link pair only when both names parse (a gone instance gives no links)
source: dec-kir2odyr «viola list --json and the MCP list item carry a recorded window only when its address re-parses through the one window address validator, and an origin only when it is a closed origin word.»
replace-with: a link pair only when both names parse (a gone instance gives no links), and, in `list --json` and the MCP list item, a recorded window only when its address re-parses through the one window address validator and an origin only when it is a closed origin word
