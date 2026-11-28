@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/r
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fra
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: also `viola console`'s refusals, each an `unable:` / `hint:` pair on stderr with no path
source: dec-yohi2hpj «A home that fails the strict-modes check while viola console runs, after its log is open, ends the console with exit 1, the home pair and one process-exit line of detail strict-modes-failed.»
replace-with: also `viola console`'s refusals, each an `unable:` / `hint:` pair on stderr with no path, and a home failing the strict-modes check while `viola console` runs, after its log is open, which ends the console with the home pair and one `process-exit` line of detail `strict-modes-failed`
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=14 class=omission
leaf-text: `viola adopt`'s `session-live` check and `viola verify`'s Run B
source: dec-xz5qksls «viola list, viola revive and viola console do not gate their claude agents --json read on a ledger stamp.»
replace-with: `viola adopt`'s `session-live` check, `viola console` (its read, like `list`'s and `revive`'s, not gated on a ledger stamp) and `viola verify`'s Run B
