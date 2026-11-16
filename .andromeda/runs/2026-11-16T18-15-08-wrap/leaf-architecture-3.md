@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_read leaf=.claude/rules/events.md lines=41
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=16 class=stale
leaf-text: (`cause` present only on an API-error turn end, as `api-error`, one word of the closed `TurnEndCause` set; absent on a Stop end)
source: ctr-2rjrmok6 «`cause` is present only on an API-error turn end, one of the nine TurnEndCause words, and absent on a Stop end; it crosses into `events.ndjson`, CLI `wait --json` and MCP `wait`.»
replace-with: (`cause` present only on an API-error turn end, one word of the closed nine-word `TurnEndCause` set: `rate-limited`, `overloaded`, `auth-failed`, `billing`, `invalid-request`, `model-not-found`, `server-error`, `output-limit`, `api-error`; absent on a Stop end)
@leaf_edit n=2 leaf=.claude/rules/events.md line=20 class=stale
leaf-text: data exactly `last_assistant_message` null, `background_tasks` null and `cause` `api-error`)
source: ctr-nyxx7utw «StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null and `cause` one of the nine TurnEndCause words»
replace-with: data exactly `last_assistant_message` null, `background_tasks` null and `cause` one of the nine `TurnEndCause` words, classified from the CLI's error word inside the Claude adapter)
@leaf_edit n=3 leaf=.claude/rules/events.md line=36 class=omission
leaf-text: (except a hyprctl reply the Hyprland seam cannot parse, which is the seam's `malformed` error)
source: dec-z5qk62ct «A hyprctl reply the Hyprland seam cannot parse is the seam's malformed error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`.»
replace-with: (except a hyprctl reply the Hyprland seam cannot parse, which is the seam's `malformed` error; a StopFailure error word outside the CLI's known words reads the turn-end cause `api-error`)
