@leaf_group group=design-system
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/l
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=50 class=stale
leaf-text: prints nothing once the child starts, and a `pump_child` error under `--json` prints no document)
source: ctr-pjujcdbl «`run`, `revive` and `adopt` print nothing once the child starts, except `adopt`'s not-resumed end, which writes its fixed pair or `--json` document after the child released the terminal.»
replace-with: prints nothing once the child starts except at its not-resumed end, which after the child released the terminal exits 1 with `unable: <name>'s session did not resume` / `hint: check the session id - claude --resume <id> must open it by hand, under the same claude config` or its `--json` document, and a resumed child ends exit 0; a `pump_child` error under `--json` prints no document)
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=50 class=stale
leaf-text: a start refusal of `viola adopt` exits 1 with its fixed `unable:` / `hint:` pair
source: ctr-utjna4le «A start refusal of `viola adopt`, or its not-resumed end after the child ran, exits 1 with its fixed `unable:` / `hint:` pair, or under `--json` one stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; a resumed child ends exit 0.»
replace-with: a start refusal of `viola adopt`, or its not-resumed end after the child ran, exits 1 with its fixed `unable:` / `hint:` pair
