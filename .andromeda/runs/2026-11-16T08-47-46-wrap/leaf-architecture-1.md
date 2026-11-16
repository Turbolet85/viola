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
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: `LedgerRow`, twenty-one closed rows
source: dec-lzixnc6x «decision/local-command-echo-probe · Local command echo ledger row and its probe — The local-command-echo relied ledger row is probed by Run B's seventh paste, /mcp disable viola-probe-absent, and passes»
replace-with: `LedgerRow`, twenty-two closed rows
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=omission
leaf-text: `agents-join` · `local-command-compact`; the relied set `LedgerRow::relied()` (every row of `ALL`, in its order, all twenty-one)
source: dec-lzixnc6x «decision/local-command-echo-probe · Local command echo ledger row and its probe — The local-command-echo relied ledger row is probed by Run B's seventh paste, /mcp disable viola-probe-absent, and passes»
replace-with: `agents-join` · `local-command-compact` · `local-command-echo`; the relied set `LedgerRow::relied()` (every row of `ALL`, in its order, all twenty-two)
@leaf_edit n=3 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=stale
leaf-text: all twenty-one on every build
source: dec-vlpcishg «decision/version-verified-all-relied-rows · A CLI version is verified only when every relied row passes — A version is verified only when every relied ledger row reads pass, so a stamp of an older, shorter row set reads unverified.»
replace-with: all twenty-two on every build (`local-command-echo` among them, so a stamp of the older, shorter row set written before chunk 2026-11-16-local-command-delivery reads unverified)
@leaf_edit n=4 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=omission
leaf-text: Run B's six compiled paste texts (`PROBE_PROMPT`,
source: dec-lzixnc6x «decision/local-command-echo-probe · Local command echo ledger row and its probe — The local-command-echo relied ledger row is probed by Run B's seventh paste, /mcp disable viola-probe-absent, and passes»
replace-with: Run B's seven compiled paste texts (the seventh `/mcp disable viola-probe-absent`, on which `local-command-echo` passes only on a new stdout tag with no UserPromptSubmit within the confirmation window, the paste skipped and the row failing with no readable transcript; `PROBE_PROMPT`,
@leaf_edit n=5 leaf=.claude/docs/services/viola-agent-claude.md line=33 class=omission
leaf-text: so its `send` is confirmed on a verified CLI and `unconfirmable` at once only on an unverified one.
source: dec-mqcmxqng «decision/local-command-echo-confirmation · Local command confirmed by its stdout echo — A local command whose post-condition is a stdout echo is confirmed by counting `<local-command-stdout>` tags in the session»
replace-with: so its `send` is confirmed on a verified CLI and `unconfirmable` at once only on an unverified one; a stdout-echo command's is a new `<local-command-stdout>` tag counted in the session transcript past its end recorded before the paste (only a count crosses, never content, the read bounded by `MAX_FRAME`), and it is `unconfirmable` at once with no readable transcript.
@leaf_edit n=6 leaf=.claude/docs/services/viola-channel.md line=26 class=omission
leaf-text: `hook.event` (re-validated, appended with `source:"hook"`)
source: ctr-fxzohw6c «contract/hook-event/transcript · channel hook.event · transcript — An optional string param beside `event`, never inside it, sent only for a SessionStart whose transcript path passes the shape check against its own session id»
replace-with: `hook.event` (`{ts, event:{kind, data}, transcript?}`, re-validated, appended with `source:"hook"`; the optional string `transcript` sits beside `event`, sent only for a SessionStart whose transcript path passes the shape check against its own session id, re-validated for a session-start line only, any other dropped, never entering event data)
