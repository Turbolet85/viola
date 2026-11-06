@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/security.md line=31 class=stale
leaf-text: the only ignore is RUSTSEC-2017-0008 via portable-pty `=0.8.1`
source: fct-serp72ds «`[advisories]`: `unmaintained = "all"` (the default), `unsound = "all"`, `yanked = "deny"`, and `ignore = []`: no advisory is ignored»
replace-with: no advisory is ignored (`ignore = []`; portable-pty pinned `=0.9.0`)
