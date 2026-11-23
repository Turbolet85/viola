@row_set n=1 row=contract/viola-list-board/order cell=promise seen=1 detector=none
value: The board is the BAY context line, the uppercase caption row and the wrapped rows drawn as a tree, then the unwrapped read-only rows behind their separator, then the links behind their `-- LINKS --` separator.
basis: fct-yurmbmhi
because: The board now draws its wrapped rows as a tree and prints a `-- LINKS --` block after the unwrapped rack, so the structure row must name both.
@row_set n=2 row=decision/terminal-structural-equivalents cell=rule seen=1 fact=F3 detector=none
value: In place of ARIA roles, terminal structure is carried by the caption line, the unwrapped separator, the `-- LINKS --` separator of the related links, the plain-space NAME indent of tree depth and the --help group labels.
basis: fct-yurmbmhi
because: Tree depth and the links group are now carried by a plain-space NAME indent and a `-- LINKS --` separated block, which the row of terminal structural equivalents must list.
@row_set n=3 row=decision/cli-structural-landmark-equivalents cell=rule seen=1 detector=none
value: The CLI has no landmark roles; its structure is carried by the BAY context line, the uppercase caption line, the `-- UNWRAPPED - READ-ONLY --` separator, the `-- LINKS --` separator, the plain-space NAME indent of tree depth and the `--help` group labels.
basis: fct-yurmbmhi
because: The board gains a `-- LINKS --` separator and a NAME indent for tree depth as structure carriers, so the list of landmark equivalents is incomplete without them.
@row_add n=1 row=contract/viola-list-board/links-separator area=a11y-scope-summary fact=F1 detector=none
boundary: viola list board
item: -- LINKS --
promise: After the unwrapped rack, the board prints the `-- LINKS --` separator then one `<driver> -> <driven>` line per link, with no SGR, in the board's same single write; separator and lines are absent when no link exists.
basis: fct-yurmbmhi
because: The chunk adds a links block to the board, a new item of its printed structure that no standing row promises.
@row_add n=2 row=contract/viola-list-board/tree area=a11y-scope-summary fact=F2 detector=none
boundary: viola list board
item: tree
promise: Wrapped rows are drawn as a tree in the board's own name order: each child under its parent with its NAME cell indented 2 spaces per depth, a row whose parent has no wrapped row as a root, and a cycle drawn once from its first name.
basis: fct-yurmbmhi
because: The chunk draws wrapped rows as a tree, a new promise of the board's reading order that no standing row holds.
@row_add n=3 row=decision/name-column-holds-tree-indent area=visual-design-verification fact=F4 detector=none
name: NAME column holds the tree indent
rule: The NAME column is as wide as its widest cell, indent included; an indent past that width prints whole and moves the columns right, and a stale child's dim span covers its indent.
basis: fct-yurmbmhi
because: The tree indent enters the NAME cell, so the column's width and the stale dim span now include it, which no visual row says.
@row_add n=4 row=contract/viola-run-flags/parent area=cognitive-accessibility fact=F5 detector=none
boundary: viola run flags
item: --parent
promise: A `--parent` equal to the instance's own name is a usage error, exit 2, before anything is read.
basis: fct-evi6pgaa
because: The chunk adds the `--parent` flag to `viola run` with a self-parent usage error, a new promise at the CLI boundary.
@row_add n=5 row=contract/viola-run-flags/role area=cognitive-accessibility fact=F5 detector=none
boundary: viola run flags
item: --role
promise: `--role` accepts only a code word.
basis: fct-yurmbmhi
because: The chunk adds the `--role` flag to `viola run`, accepting only a code word, a new promise at the CLI boundary.
@row_add n=6 row=decision/board-lineage-and-links-name-checked area=cognitive-accessibility fact=F6 detector=none
name: Board lineage and links name-checked
rule: The board shows a parent only when it parses as a viola name other than the row's own, a role only when it is a code word, and a link pair only when both names parse.
basis: fct-yurmbmhi
because: The board now prints logged parents, roles and link pairs, and screens each before showing it, a rule no standing row states.
