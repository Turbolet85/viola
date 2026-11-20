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
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=stale
leaf-text: classifies `origin` driver/human/harness
source: ctr-u3uf5uxc «driver, human or harness; the hook files only human or harness; the wrapper sets driver only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake harness.»
replace-with: classifies `origin` human/harness; the wrapper sets `driver` only on the in-flight send's own prompt and relabels a CLI-fired scheduled wake `harness`
