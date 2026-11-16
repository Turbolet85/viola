@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=omission
leaf-text: the root bin's `send` types, classifies and matches that typed text
source: dec-sdymze5l «when equal or when non-empty and equal to the typed text less its trailing run of ASCII space, TAB, CR and LF.»
replace-with: the root bin's `send` types and classifies that typed text and matches a `prompt-submitted` text equal to it, or non-empty and equal to it less its trailing run of ASCII space, TAB, CR and LF (provisional), the text kept as submitted
