@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` ·
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=omission
leaf-text: a print-mode sender's message to Run B, started with the crossSessionInbound accept settings,
source: dec-wgz7t5no «A local claude session is addressed for a cross-session send by its inbox socket address, uds:<runtime>/cc-socks/<pid>.sock, composed from the runtime dir and the session's pid»
replace-with: a print-mode sender's message to Run B (the sender addressing Run B by its inbox socket address `uds:<runtime>/cc-socks/<pid>.sock`, composed from the runtime dir and Run B's pid, never by a bare session id), Run B started with the crossSessionInbound accept settings,
