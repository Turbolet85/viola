@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_read leaf=.claude/rules/ci.md lines=35
last: - 2026-10-13: A CI job that reads `cancelled` with no steps
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: (its `record` capture 0600 in a private directory, never committed)
source: dec-pvrejrf6 «MCP exchange fixtures are recorded by a test-only tap, started live by the standing rig under its private CLI config, that spawns viola mcp directly with no shell, caps each line»
replace-with: (started live by the standing rig under its private CLI config; its `record` capture 0600 in a private directory, never committed)
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: A live session's screen frames are kept only under a gitignored local path
source: dec-w5vqy6om «The live rig's MCP recording arm keeps no CLI output, stdout drained and dropped and stderr kept only as a byte count and a SHA, and its MCP config is a private 0600 file inside the scratch»
replace-with: The live rig's MCP recording arm keeps no CLI output (stdout drained and dropped, stderr kept only as a byte count and a SHA), and its MCP config is a private 0600 file inside the scratch, removed with it. A live session's screen frames are kept only under a gitignored local path
