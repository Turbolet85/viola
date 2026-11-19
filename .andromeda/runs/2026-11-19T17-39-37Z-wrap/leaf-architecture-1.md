@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fra
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=54 class=omission
leaf-text: which reads the key names and the source word of its own captured UserPromptSubmit payload and the cachedChromeExtensionInstalled key name, never a value
source: dec-uun4jqhm «decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only rig seeds config; UserPromptSubmit/StopFailure key names»
replace-with: which reads the key names of its own captured UserPromptSubmit and StopFailure payloads, the source word, the error word only as `13` or other, two lengths and the cachedChromeExtensionInstalled key name, never a value
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=omission
leaf-text: the live rig in `viola-e2e` reads the key names and the source word of its own captured UserPromptSubmit payload and the cachedChromeExtensionInstalled key name, never a value
source: dec-zypgxbrh «except: Test-only rig seeds config; UserPromptSubmit/StopFailure key names, source word, error word 13|other, 2 lengths, cachedChromeExtensionInstalled name; VH key names, needle; fake agent/rig trust records»
replace-with: the live rig in `viola-e2e` reads the key names of its own captured UserPromptSubmit and StopFailure payloads, the source word, the error word only as `13` or other, two lengths and the cachedChromeExtensionInstalled key name, never a value
