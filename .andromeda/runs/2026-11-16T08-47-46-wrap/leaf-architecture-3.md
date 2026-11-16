@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_read leaf=.claude/rules/events.md lines=41
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=20 class=omission
leaf-text: SessionStart → `session-start` (`cause` startup|clear|resume|compact|unknown, `agent_session_id`);
source: ctr-fxzohw6c «contract/hook-event/transcript · channel hook.event · transcript — An optional string param beside `event`, never inside it, sent only for a SessionStart whose transcript path passes the shape check»
replace-with: SessionStart → `session-start` (`cause` startup|clear|resume|compact|unknown, `agent_session_id`; the hook may send an optional string `transcript` param beside `event`, never inside it, only when the transcript path passes the shape check against its own session id; the wrapper re-validates it for a session-start line only, drops any other, and it never enters event data);
