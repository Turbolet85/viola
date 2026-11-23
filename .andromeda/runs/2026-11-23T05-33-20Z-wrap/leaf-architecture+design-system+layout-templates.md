@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=24 class=omission
leaf-text: the BAY line, caption, empty-board line and separator carry no SGR
source: ctr-i425qiqe «contract/viola-list-board/links-block · viola list board · -- LINKS -- block — After the board and the unwrapped rack, `-- LINKS --` then one `<driver> -> <driven>` line per link, sorted, deduplicated and name-checked, with no SGR; absent when there is no link.»
replace-with: the BAY line, caption, empty-board line and separator carry no SGR, and after the board and the unwrapped rack a `-- LINKS --` block prints one `<driver> -> <driven>` line per link, sorted, deduplicated and name-checked, with no SGR, absent when there is no link
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=24 class=omission
leaf-text: a wrapped NAME is never cut
source: ctr-t7vcwnes «contract/viola-list-board/tree · viola list board · tree — Wrapped rows are drawn as a tree, each child under its parent with its NAME cell indented two spaces per depth; a row whose parent has no wrapped row is a root, and cycle»
replace-with: wrapped rows are drawn as a tree, each child under its parent with its NAME cell indented two spaces per depth (the indent counted in NAME's column width), and a wrapped NAME is never cut
