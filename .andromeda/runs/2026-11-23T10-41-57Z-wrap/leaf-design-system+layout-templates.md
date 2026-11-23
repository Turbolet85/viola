@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: an unusable `viola send` and an oversize `viola answer` instead exit 2 with their fixed stderr line, `UNUSABLE_TEXT` / `UNUSABLE_RESPONSE`, whatever `--json` says, and no document;
source: ctr-nfynnxdl «`UNUSABLE_TEXT` line (unusable `viola send`) or `PAIR_AFTER_DASHES` line (a pair after `--` of `run` or `revive`) on stderr, whatever `--json` says; no document.»
replace-with: an unusable `viola send`, an oversize `viola answer` and a `--model` / `--effort` pair after the `--` of `viola run` or `viola revive` instead exit 2 with their fixed stderr line, `UNUSABLE_TEXT` / `UNUSABLE_RESPONSE` / `PAIR_AFTER_DASHES`, whatever `--json` says, and no document;
