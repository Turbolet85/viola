@row_set n=1 row=decision/list-six-fixed-columns cell=rule seen=1 detector=D-design-cli-wording
value: `viola list` carries no link column, so its six columns, their captions and their slot order stay fixed; a tree's indent stands inside the NAME cell and links print apart, in the board's `-- LINKS --` block.
basis: fct-yurmbmhi, fct-j3icgnqw
because: The board now draws trees in NAME and prints a LINKS block, while the six columns, captions and slot order stay as they were, so the row must say where the tree and the links stand without moving a column.
@row_set n=2 row=contract/viola-list/columns cell=promise seen=1 detector=D-design-cli-wording
value: `viola list` carries no link column, so its six columns, their captions and their slot order stay fixed; a tree's indent stands inside the NAME cell and links print apart, in the board's `-- LINKS --` block.
basis: fct-yurmbmhi, fct-j3icgnqw
because: The output's columns stay six with their captions and slot order, while the tree enters the NAME cell and links get a block of their own, so the promise must name both.
@row_set n=3 row=contract/viola-list-table/row-order cell=promise seen=1 detector=D-design-cli-wording
value: Rows keep the board's name order, each wrapped child drawn directly under its parent.
basis: fct-gpcfukhg, fct-yurmbmhi
because: The board draws wrapped rows as a tree in its own name order, so plain name order no longer states the row order; the tree's detail stands in layout-templates.
