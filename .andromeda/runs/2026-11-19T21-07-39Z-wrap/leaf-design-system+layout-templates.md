@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=7 class=omission
leaf-text: for `/mcp` until a new local-command stdout tag appears in the session transcript after the send;
source: ctr-ezrgwvxf «On a verified CLI a `/context` send counts, and reads back, only once one new local-command stdout tag appears in the session transcript with no prompt submitted;»
replace-with: for `/mcp` until a new local-command stdout tag appears in the session transcript after the send, and for `/context` until one new local-command stdout tag appears there with no prompt submitted;
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=7 class=omission
leaf-text: on an unverified CLI `/clear`, `/compact` and `/mcp` stay `unconfirmable`, as does `/mcp` in a session that declared no transcript and, always,
source: ctr-qcnoq66l «so do a `/compact` send on an unverified CLI, a `/mcp` send on an unverified CLI or with no declared transcript, and a `/context` send on an unverified CLI or before its transcript exists.»
replace-with: on an unverified CLI `/clear`, `/compact`, `/mcp` and `/context` stay `unconfirmable`, as do `/mcp` in a session that declared no transcript, `/context` before its transcript exists and, always,
