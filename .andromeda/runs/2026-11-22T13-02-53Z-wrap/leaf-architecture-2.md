@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-pty.md line=6 class=stale
leaf-text: as one `write_all` under the lock (span `pty.paste_write`), so a key that arrives during a paste waits only for that paste
source: dec-zzgfrbyq «Human keystrokes are never blocked, refused or delayed past the current paste's in-flight chunk and closing marker; automation is refused with a typed reason instead of competing with the human.»
replace-with: under the lock (span `pty.paste_write`) in chunks of at most 4096 bytes, so a human key is never delayed past the current paste's in-flight chunk and closing marker
@leaf_edit n=2 leaf=.claude/docs/services/viola-pty.md line=6 class=omission
leaf-text: `NoInput` before the pump opens the input or after it returns.
source: dec-wyrjbend «A send's paste not finished within the compiled 5 s paste bound, counted from the call, is cut: the writer stops after its in-flight chunk of at most 4096 bytes and writes the closing paste marker with no Enter»
replace-with: `NoInput` before the pump opens the input or after it returns. A paste not finished within the compiled 5 s paste bound, counted from the call, is cut: the writer stops after its in-flight chunk and writes the closing paste marker with no CR; a paste still queued at the deadline is cut with nothing written.
