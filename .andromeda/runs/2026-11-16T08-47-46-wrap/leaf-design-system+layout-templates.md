@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=7 class=omission
leaf-text: and for `/compact` until its same-session compact post-condition does;
source: ctr-cbg4mzia «On a verified CLI a `/mcp` send counts, and reads back, only once a new local-command stdout tag appears in the session transcript after the send; it prints no new word.»
replace-with: for `/compact` until its same-session compact post-condition does and for `/mcp` until a new local-command stdout tag appears in the session transcript after the send;
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=7 class=omission
leaf-text: on an unverified CLI both `/clear` and `/compact` stay `unconfirmable`
source: ctr-qcnoq66l «A local command with no post-condition (`/remote-control`, `/cost`, `/usage`) stays `unconfirmable`; so does a `/compact` send on an unverified CLI, and a `/mcp` send on an unverified CLI or in a session that declared no transcript.»
replace-with: on an unverified CLI `/clear`, `/compact` and `/mcp` stay `unconfirmable`, as does `/mcp` in a session that declared no transcript and, always, `/remote-control`, `/cost` and `/usage`
