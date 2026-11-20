@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/r
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=20 class=omission
leaf-text: PermissionRequest `decision.behavior` allow/deny + `message` (plan revise)
source: ctr-xy2r6esu «PermissionRequest returns `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`; for a question the decision is `{"behavior":"allow","updatedInput":<the tool's input plus answers and annotations>}`.»
replace-with: PermissionRequest `decision.behavior` allow/deny + `message` (plan revise), and for a question first raised by PermissionRequest with no matching armed dialog `decision` `{"behavior":"allow","updatedInput":…}` (the tool's input plus answers and annotations)
