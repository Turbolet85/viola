@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/l
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=56 class=stale
leaf-text: Transfer markers — only `link` / `unlink` verb output (`overseer -> builder`), plain, ASCII `->`, no inbound form; `viola list` has no link column
source: dec-m6tks23o «The six shared board columns, their captions and their slot order stay unchanged, with no link column; links show in `link` / `unlink` verb output and in the board's -- LINKS -- block after the unwrapped rack.»
replace-with: Transfer markers — only `link` / `unlink` verb output (`overseer -> builder`) and the `<driver> -> <driven>` lines of `viola list`'s `-- LINKS --` block, plain, ASCII `->`, no inbound form; `viola list` has no link column, its six columns, captions and slot order fixed
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: is cut with `...`; the wrapped rows, then, when one
source: ctr-t7vcwnes «Wrapped rows are drawn as a tree, each child under its parent with its NAME cell indented two spaces per depth; a row whose parent has no wrapped row is a root, and cycle members are drawn once, from the first in order.»
replace-with: is cut with `...`; the wrapped rows drawn as a tree, each child directly under its parent with its NAME cell indented two spaces per depth, then, when one
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: `n/a` in WHEEL, DIALOG and CLI); STATUS
source: ctr-i425qiqe «After the board and the unwrapped rack, `-- LINKS --` then one `<driver> -> <driven>` line per link, sorted, deduplicated and name-checked, with no SGR; absent when there is no link.»
replace-with: `n/a` in WHEEL, DIALOG and CLI), then, when there is a link, the `-- LINKS --` line and one `<driver> -> <driven>` line per link, sorted, deduplicated, no SGR; STATUS
@leaf_stands n=1 leaf=.claude/docs/design-summary.md line=57
because: The line says only that the tower tape is the ndjson tape in `events.ndjson` and the CLI has no tape, which dec-zmmfuujl, ctr-z3syrfob, ctr-7cycjmhp and dec-6dmakk3w still bear out, `events.ndjson` remaining the appended event log the run start writes and replay reads.
