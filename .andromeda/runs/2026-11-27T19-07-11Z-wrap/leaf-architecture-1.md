@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` ·
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=11 class=stale
leaf-text: `COMPACT_CONFIRM_WINDOW` 40 s (PROVISIONAL, a `/compact` send's own window
source: dec-xr4hahiu «A /compact send waits a compiled 40 s window for its post-condition, validated per CLI version by the local-command-compact row against the measured compact latency.»
replace-with: `COMPACT_CONFIRM_WINDOW` 40 s (standing, a `/compact` send's own window
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=33 class=stale
leaf-text: its own 40 s PROVISIONAL `COMPACT_CONFIRM_WINDOW`
source: dec-xr4hahiu «A /compact send waits a compiled 40 s window for its post-condition, validated per CLI version by the local-command-compact row against the measured compact latency.»
replace-with: its own compiled 40 s `COMPACT_CONFIRM_WINDOW`
