@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=28 class=retired
leaf-text: an `IDENTITY_FLOOR` name (11, measured) is removed regardless.
source: dec-x56wogur «decision/r8-identity-floor · R8 parent-identity floor — The identity-floor CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them;»
replace-with: an `IDENTITY_FLOOR` name is removed regardless, even when the persistent environment defines it.
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=omission
leaf-text: the one relied row whose `absent` counts toward verified); the relied set
source: dec-fcus6tgb «decision/identity-floor-ledger-row · Identity-floor ledger row and its readings — The identity-floor relied ledger row passes when every CLAUDE name the CLI adds to a SessionStart hook is an identity-floor»
replace-with: the one relied row whose `absent` counts toward verified) · `identity-floor` (relied; passes when every CLAUDE name the CLI adds to a SessionStart hook is an identity-floor name or a known non-identity name, fails on an unknown name or an unread or off-shape env-names file, kept names dropped before the check; its probe stamped as `measured.env_probe`, names only); the relied set
