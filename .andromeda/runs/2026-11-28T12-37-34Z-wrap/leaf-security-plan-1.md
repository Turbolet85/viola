@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=17 class=stale
leaf-text: (a refusal exit 1, no line, the modes as found)
source: ctr-fh4crnmq «on stderr; at start with no line written and the mode left as found, mid-run with one process-exit line of detail strict-modes-failed.»
replace-with: (a refusal exit 1; at start no line and the modes as found, mid-run one process-exit line of detail `strict-modes-failed`)
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=19 class=omission
leaf-text: `adopt`'s live-elsewhere check and the MCP list tool;
source: dec-otmvj2vn «list, revive's preflight, adopt's live-elsewhere check, the MCP list tool and viola console resolve claude for claude agents --json from the calling process's PATH and current directory»
replace-with: `adopt`'s live-elsewhere check, the MCP list tool and `viola console`;
