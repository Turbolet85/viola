@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=33 class=stale
leaf-text: `strict-modes-failed` · `session-live` · `already-live` · `stale-heartbeat`
source: ctr-yzm3hmbz «`<code>` is one of `strict-modes-failed` · `session-live` · `not-resumed` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
replace-with: `strict-modes-failed` · `session-live` · `not-resumed` · `already-live` · `stale-heartbeat`
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=omission
leaf-text: it is the one product home of the four flag literals `--resume`, `--fork-session`, `--model` and `--effort` (`MODEL_FLAG`, `EFFORT_FLAG`)
source: dec-wm2dwyha «except: viola adopt's fixed not-resumed hint names claude --resume <id> as words for the human to type, never as an argument viola passes.»
replace-with: it is the one product home of the four flag literals `--resume`, `--fork-session`, `--model` and `--effort` (`MODEL_FLAG`, `EFFORT_FLAG`; `viola adopt`'s fixed `not-resumed` hint names `claude --resume <id>` only as words for the human to type, never as an argument viola passes)
