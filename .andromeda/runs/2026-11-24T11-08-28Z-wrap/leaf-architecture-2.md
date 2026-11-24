@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=40
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=48
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY
@leaf_read leaf=.claude/docs/services/viola-state.md lines=50
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-state.md line=6 class=omission
leaf-text: run before a bind (`viola run`, `revive`'s start)
source: ctr-uwztwbnt «Exit 21 with during connect, at WARN, no channel-request line; run's, revive's and adopt's start write it with exit_code 1.»
replace-with: run before a bind (`viola run`, `revive`'s and `adopt`'s start)
@leaf_edit n=2 leaf=.claude/docs/services/viola-state.md line=22 class=omission
leaf-text: (`run`'s and `revive`'s starts and `verify` through `check_home_tree`
source: ctr-yzm3hmbz «`viola adopt --json` refusal codes — `<code>` is one of `strict-modes-failed` · `session-live` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused`»
replace-with: (`run`'s, `revive`'s and `adopt`'s starts and `verify` through `check_home_tree`
