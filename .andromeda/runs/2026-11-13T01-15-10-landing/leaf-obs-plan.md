@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=17 class=stale
leaf-text: stderr JSON only when that file cannot be opened, D-09
source: dec-4bmxcdmt «MCP logs to a file sink, never stdout — The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with no stderr fallback: a file that cannot be opened exits 1 printing nothing»
replace-with: no stderr fallback: a file that cannot be opened exits 1 printing nothing
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=69 class=stale
leaf-text: three of the eight fail-open codes
source: ctr-adhcpufc «hook-decision detail · fail-open codes — oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, deadline; no hook arm writes strict-modes-failed or unverified-cli.»
replace-with: three of the six fail-open codes
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=51 class=omission
leaf-text: gitignored, outside every upload path (obs-plan §8 item 5)
source: dec-6c72cyee «Mutation diff never uploaded — The mutation run's chunk.diff at the harness capture root is excluded from the harness upload, so it is never scanned and never uploaded.»
replace-with: gitignored, outside every upload path (obs-plan §8 item 5); the mutation run's `chunk.diff` at the harness capture root is excluded from the harness upload, so it is never scanned and never uploaded
