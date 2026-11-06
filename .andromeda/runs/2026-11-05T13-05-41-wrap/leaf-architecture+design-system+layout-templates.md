@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=24 class=retired
leaf-text: `TERM=dumb` · non-TTY stdout · Windows VT enable failed.
source: fct-kbiy6u23 «6. stdout is not a terminal»
replace-with: `TERM=dumb` · non-TTY stdout.
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=24 class=retired
leaf-text: from `COLORTERM` / `TERM` (256 on Windows once VT enabling has succeeded) (`COLUMNS`
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: from `COLORTERM` / `TERM` (`COLUMNS`
