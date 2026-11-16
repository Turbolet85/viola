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
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=22 class=omission
leaf-text: except a hyprctl reply the Hyprland seam cannot parse, which is the seam's `malformed` error.
source: dec-z5qk62ct «except: A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.»
replace-with: except a hyprctl reply the Hyprland seam cannot parse, which is the seam's `malformed` error, and a StopFailure error word outside the CLI's known words, which reads the turn-end cause `api-error`.
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=stale
leaf-text: `background_tasks` null and `cause` `api-error`;
source: ctr-nyxx7utw «StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null and `cause` one of the nine TurnEndCause words»
replace-with: `background_tasks` null and `cause` one of the nine TurnEndCause words, classified from the CLI's error word inside the Claude adapter (a word outside the CLI's known words reads `api-error`);
@leaf_edit n=3 leaf=.claude/docs/services/viola-core.md line=6 class=stale
leaf-text: one member `api-error`, no Claude-specific name
source: ctr-4lunasqf «`turn-ended` has a closed cause set, TurnEndCause, in viola-core beside UnreachableCause, of nine words (rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error)»
replace-with: nine words `rate-limited` · `overloaded` · `auth-failed` · `billing` · `invalid-request` · `model-not-found` · `server-error` · `output-limit` · `api-error`, no Claude-specific name
