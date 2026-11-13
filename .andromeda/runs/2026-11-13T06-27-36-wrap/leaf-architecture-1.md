@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=79 class=stale
leaf-text: (quiet period + input-box signature + no modal signature)
source: dec-xsqsrc2s «a screen is ready when no row holds a modal literal and a footer literal stands or the box's own rows do: a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char.»
replace-with: (quiet period + no modal signature + the input box: a footer literal, or the box's own rows — a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char)
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=11 class=stale
leaf-text: a quiet screen with no input-box row waits for one until the maximum wait
source: dec-xsqsrc2s «a screen is ready when no row holds a modal literal and a footer literal stands or the box's own rows do: a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char.»
replace-with: a quiet screen with neither a footer literal nor the box's own rows (a rule row of the compiled rule char alone, at least 40, above a row opening the compiled prompt char) waits for the input box until the maximum wait
