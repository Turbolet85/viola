@leaf_group group=design-system
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=68 class=omission
leaf-text: `turn-ended` never fills a readback box;
source: dec-dhfntmue «A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is the closed cause word `api-error`»
replace-with: `turn-ended` never fills a readback box; a turn the CLI ends on an API error shows like any turn end (STATUS `idle`, `wait` `turn-ended`, no distinct status, colour or hint; the only trace the cause word `api-error` in `--json` event data, the CLI's error text shown nowhere);
