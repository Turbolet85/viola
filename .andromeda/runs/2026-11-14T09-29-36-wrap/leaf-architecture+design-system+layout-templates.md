@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=24 class=omission
leaf-text: LIVE `live`, `n/a` in WHEEL, DIALOG and CLI
source: ctr-owhmvkun «One row per unwrapped session, sorted by name: NAME the session's `sessionId`, LIVE `live`, its STATUS, and `n/a` in WHEEL, DIALOG and CLI.»
replace-with: LIVE `live`, its STATUS, `n/a` in WHEEL, DIALOG and CLI
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: every refusal of the start `viola revive` shares with `run` is one stdout document
source: ctr-y3mybizi «A refused strict-modes check, preflight or start of `viola revive --json` prints `{"v":1,"error":"<code>","detail":null}` with exit 1, codes only (no path, pid or id), stderr empty.»
replace-with: every refusal of `viola revive --json` (its strict-modes check, its preflight and the start it shares with `run`) is one stdout document
