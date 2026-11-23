@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=29 class=omission
leaf-text: the `-- UNWRAPPED - READ-ONLY --` separator stands too, printed only when unwrapped rows exist
source: ctr-2t7hahyo «the wrapped rows drawn as a tree, then the unwrapped read-only rows behind their separator, then the links behind their `-- LINKS --` separator.»
replace-with: the wrapped rows are drawn as a tree, their depth carried by a plain-space NAME indent; the `-- UNWRAPPED - READ-ONLY --` separator stands too, printed only when unwrapped rows exist, and the links follow behind their `-- LINKS --` separator
