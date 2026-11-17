@leaf_group group=architecture+security-plan+test-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=134
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=27 class=omission
leaf-text: `viola-harness` behind `scripts/agent-run.sh`, `viola-mcp-tap`, `viola-hypr-tap` and the Tokio-based E2E clients
source: dec-k4rtddnt «decision/standing-live-rig · One standing committed live rig — Live rounds against the real claude reuse one committed test-only rig, viola-live-rig in viola-e2e: its rehearsals over the fake agent run»
replace-with: `viola-harness` behind `scripts/agent-run.sh`, `viola-mcp-tap`, `viola-hypr-tap`, `viola-live-rig` (the one standing live rig: rehearsals over the fake agent in CI, live starts by hand only, its live arm refusing under CI) and the Tokio-based E2E clients
