@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=24 class=omission
leaf-text: only an unwrapped NAME is cut, to the column less 3 characters plus `...`
source: ctr-mp6pifjv «an unwrapped NAME longer than the column (capped at the width less 65) is cut to the column less 3 plus `...`, never inside a `\xHH` escape; a wrapped NAME is never cut.»
replace-with: only an unwrapped NAME longer than its column (capped at the width less 65) is cut, to the column less 3 characters plus `...`
