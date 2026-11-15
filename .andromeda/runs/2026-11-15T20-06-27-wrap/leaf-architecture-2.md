@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fra
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope, i
@leaf_edit n=1 leaf=.claude/docs/services/viola-channel.md line=41 class=omission
leaf-text: real endpoint in a temp home; 0700 / 0600 read-back
source: dec-elm6ki3k «decision/test-sockets-under-short-tmp-root · Test channel sockets under a short /tmp root — Test channel sockets bind under a test-owned short root directory made under /tmp, never under TMPDIR; the product endpoint s»
replace-with: real endpoint in a temp home, its socket bound under a test-owned short root directory made under /tmp, never under TMPDIR (so the socket path length bound holds whatever TMPDIR's length); 0700 / 0600 read-back
