@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=54 class=omission
leaf-text: which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value, and seeds its private Claude config
source: dec-uun4jqhm «Test-only: the rig seeds its private config and reads, no value, its UserPromptSubmit key names, source word and the cachedChromeExtensionInstalled key name; the fake agent and rig read trust records.»
replace-with: which reads the key names and the source word of its own captured UserPromptSubmit payload and the cachedChromeExtensionInstalled key name, never a value, and seeds its private Claude config
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=omission
leaf-text: reads the key names and the source word of its own captured UserPromptSubmit payload, never a value, and seeds its private Claude config
source: dec-uun4jqhm «Test-only: the rig seeds its private config and reads, no value, its UserPromptSubmit key names, source word and the cachedChromeExtensionInstalled key name; the fake agent and rig read trust records.»
replace-with: reads the key names and the source word of its own captured UserPromptSubmit payload and the cachedChromeExtensionInstalled key name, never a value, and seeds its private Claude config
@leaf_edit n=3 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: `verified` (every row of `LedgerRow::relied()` `pass`,
source: dec-vlpcishg «A version is verified only when every relied ledger row counts, reading pass or, on browser-tools-prompt alone, absent, so a stamp of an older, shorter row set reads unverified.»
replace-with: `verified` (every row of `LedgerRow::relied()` `pass`, or `absent` on `browser-tools-prompt` alone,
@leaf_edit n=4 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=omission
leaf-text: and Run F's start shows no modal and the input box);
source: dec-e65gmsyj «The browser-tools-prompt relied ledger row reads pass when some verify run's first screen is the CLI's browser-tools prompt, absent when no run met it, and fail when any first screen reads none; it is the»
replace-with: and Run F's start shows no modal and the input box) · `browser-tools-prompt` (relied; `pass` when some verify run's first screen is the CLI's browser-tools prompt, `absent` when no run met it, `fail` when any first screen reads none; the one relied row whose `absent` counts toward verified);
