@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=20 class=omission
leaf-text: `hook.event` (notification)
source: ctr-fxzohw6c «An optional string param beside `event`, never inside it, sent only for a SessionStart whose transcript path passes the shape check against its own session id; the wrapper re-validates it for a se»
replace-with: `hook.event` (notification, `{ts, event:{kind, data}, transcript?}`: the optional string `transcript` beside `event`, never inside it, sent only for a SessionStart whose transcript path passes the shape check against its own session id, re-validated by the wrapper for a session-start line only, any other dropped, never entering event data)
