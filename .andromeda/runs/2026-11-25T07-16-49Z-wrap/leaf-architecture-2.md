@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-mcp.md line=23 class=omission
leaf-text: The `rmcp` log target is OFF.
source: ctr-v6wlxdat «viola mcp's codes-only lines, ending with a self process-exit (exit_code 0, no detail) once stdin closes.»
replace-with: The `rmcp` log target is OFF. `mcp.ndjson` holds codes-only lines and ends with a self `process-exit` (exit_code 0, no detail) once stdin closes.
@leaf_edit n=2 leaf=.claude/docs/services/viola-pty.md line=11 class=omission
leaf-text: libc `=0.2.189` (termios raw mode, terminal size).
source: dec-u4fx7kjh «The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves.»
replace-with: libc `=0.2.189` (termios raw mode, terminal size); a code-free `fake-agent = []` feature, so a package-scoped `--features fake-agent` build resolves.
@leaf_edit n=3 leaf=.claude/docs/services/viola-state.md line=11 class=omission
leaf-text: dev-dependency tracing-subscriber (unit-test line capture only, no product edge).
source: dec-u4fx7kjh «The sync crates viola-pty, viola-agent-claude and viola-state, and viola-mcp and viola-e2e, each declare a code-free fake-agent feature, so a package-scoped fake-agent build of any of them resolves.»
replace-with: dev-dependency tracing-subscriber (unit-test line capture only, no product edge); a code-free `fake-agent = []` feature, so a package-scoped `--features fake-agent` build resolves.
@leaf_edit n=4 leaf=.claude/docs/services/viola-state.md line=28 class=omission
leaf-text: the replay writes no file, rebuilds `lineage` from the log
source: dec-vo27o42l «A snapshot rebuilt by replay recovers only links, agent_session_id, wheel, lineage, launch and window from the log; a field no line gave is absent, never a default.»
replace-with: the replay writes no file, recovers only `links`, `agent_session_id`, `wheel`, `lineage`, `launch` and `window` (a field no line gave is absent, never a default), rebuilds `lineage` from the log
