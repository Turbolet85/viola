@leaf_group group=architecture+design-system
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=21 class=omission
leaf-text: Nothing is shown as delivered until read back; `turn-ended` never fills a box.
source: dec-dhfntmue «A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is the closed cause word»
replace-with: Nothing is shown as delivered until read back; `turn-ended` never fills a box. A turn ended on an API error shows like any turn end (STATUS `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint); its only trace is the cause word `api-error` in `--json` event data, and the CLI's error text is shown nowhere.
