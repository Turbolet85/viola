@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=30 class=stale
leaf-text: strict-modes-checked before any snapshot, `statusline_command` or ledger read
source: dec-p7mi6b2i «--home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot or ledger read.»
replace-with: strict-modes-checked before any snapshot or ledger read
@leaf_edit n=2 leaf=.claude/rules/api.md line=30 class=retired
leaf-text: `viola hook <event>`'s event and dialog paths and `hook statusline` canonicalise the instance directory
source: dec-pdeany7i «Hook event and dialog paths canonicalise the instance directory and run the instance strict-modes check before diagnostics level and obs init; a missing, malformed or refused instance exits 0 with no»
replace-with: `viola hook <event>`'s event and dialog paths canonicalise the instance directory
@leaf_edit n=3 leaf=.claude/rules/api.md line=30 class=retired
leaf-text: a malformed, absent or refused instance a silent exit 0 (`hook statusline` before its log; its hidden `hook statusline --capture <DIR> [--shell <WORD>]` probe form, called only by Run B's probe settings, runs no instance check, reads no `VIOLA_*` and requires an absolute, existing `<DIR>` instead, its `--shell` word mapped only into the closed `ShellVerdict`);
source: ctr-zgw4cnv3 «The retired hook word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0 and reads and writes nothing.»
replace-with: a malformed, absent or refused instance a silent exit 0 (the retired word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0 and reads and writes nothing);
