@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/l
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pres
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=59 class=stale
leaf-text: then the cause's `hint:` line last;
source: ctr-onpg3y3e «A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail»
replace-with: then the cause's `hint:` line last (`send` and `answer` print the same hint for the same reason or detail; a `not-delivered` refusal with an opaque detail or none prints no hint);
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=26 class=omission
leaf-text: no hint for `unknown`, `wrapper fault` or `internal error`.
source: ctr-6iwmj65y «A human-mode `viola answer` refused not-delivered ends with its own hint for `unknown-dialog`, and for `control-character`, `empty-text`, `input-not-ready`, `no-prompt-submitted` and `turn-running` with»
replace-with: no hint for `unknown`, `wrapper fault`, `internal error` or a `not-delivered` refusal with an opaque detail or none; a refused `viola answer` ends with its own hint for `unknown-dialog` and, for any other named detail, the hint `viola send` holds for it.
