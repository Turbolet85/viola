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
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=13 class=stale
leaf-text: its value read by `viola verify` alone, only when `claude_env_keep` keeps that name, to locate the Claude config file
source: dec-wjypwipg «viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.»
replace-with: its value read by `viola verify`, and by the `viola run` and `viola revive` start, only when `claude_env_keep` keeps that name, to locate the Claude config file
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=45 class=stale
leaf-text: keeping `CLAUDE_CONFIG_DIR` makes `viola verify` read its value to locate the Claude config file
source: ctr-szg56kho «Keeping CLAUDE_CONFIG_DIR makes verify, run and revive read its value to find the Claude config.»
replace-with: keeping `CLAUDE_CONFIG_DIR` makes `viola verify`, `viola run` and `viola revive` read its value to locate the Claude config file
@leaf_edit n=3 leaf=.claude/docs/services/viola-agent-claude.md line=28 class=stale
leaf-text: only `viola verify` reads the value of `CLAUDE_CONFIG_DIR`, only when `claude_env_keep` keeps that name, to locate the Claude config file
source: dec-ysr3fsif «viola verify, and the run and revive start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.»
replace-with: only `viola verify`, and the `viola run` and `viola revive` start, read the value of `CLAUDE_CONFIG_DIR`, only when `claude_env_keep` keeps that name, to locate the Claude config file
