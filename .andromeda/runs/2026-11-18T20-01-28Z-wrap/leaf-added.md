@leaf_group group=added
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/r
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/workflow.md line=49 class=omission
leaf-text: the test-only `viola-mcp-tap` recording a live CLI's MCP exchange for the `Mcp.` fixtures
source: dec-pvrejrf6 «MCP exchange fixtures are recorded by a test-only tap, started live by the standing rig under its private CLI config, that spawns viola mcp directly with no shell, caps each line,»
replace-with: the test-only `viola-mcp-tap`, started live by the standing rig under its private CLI config, recording a live CLI's MCP exchange for the `Mcp.` fixtures
