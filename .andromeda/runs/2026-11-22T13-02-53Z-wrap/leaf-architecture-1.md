@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fra
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `.
@leaf_edit n=1 leaf=.claude/docs/services/viola-core.md line=21 class=stale
leaf-text: (six values, `empty-text` the sixth: an empty typed text on `send`)
source: ctr-thxblc25 «contract/refusal-reason/not-delivered · RefusalReason · not-delivered — Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut.»
replace-with: (seven values, `empty-text` an empty typed text on `send`, `paste-cut` the seventh: a paste cut at its bound or a standing cut mark)
