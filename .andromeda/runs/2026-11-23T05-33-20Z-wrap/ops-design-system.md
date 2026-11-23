@row_set n=1 row=decision/stable-session-slots cell=rule seen=1 detector=none
value: Every session keeps a slot in the strip bay stable across state changes: wrapped sessions in name order, each child drawn under its parent, then unwrapped ones in name order; nothing is re-sorted by state, and a slot shifts only when a session arrives or leaves.
basis: fct-yurmbmhi, fct-gpcfukhg
because: The board now draws wrapped rows as a tree in its own name order, children under their parents, so the row's plain name order is incomplete.
@row_set n=2 row=contract/cli-transfer-arrow/ascii-arrow cell=promise seen=1 detector=D-design-cli-wording
value: The CLI prints transfers with ASCII `->` and has no inbound form; links print only as `link` / `unlink` verb output (`overseer -> builder`) and as the `<driver> -> <driven>` lines of `viola list`'s `-- LINKS --` block.
basis: fct-yurmbmhi, fct-ugo53gel
because: viola list now prints each link as a `driver -> driven` line in its LINKS block, so links no longer print only as link and unlink output.
@row_set n=3 row=contract/viola-list-table/row-order cell=promise seen=1 fact=F2 detector=D-design-cli-wording
value: Rows are ordered by name; a wrapped row is drawn under its parent, its NAME cell indented two spaces per depth, siblings in name order; a row whose parent has no wrapped row is a root, and a cycle is drawn once from its first member.
basis: fct-yurmbmhi, fct-gpcfukhg
because: The board's wrapped rows are now drawn as a tree under their parents, so plain name order no longer states the row order.
@row_set n=4 row=contract/viola-list-board/name-column-width cell=promise seen=1 detector=D-design-cli-wording
value: On a terminal of known width NAME's column is the largest of the caption's 4, the longest wrapped NAME cell with its tree indent included, and the longest unwrapped NAME capped at the width less the 65 columns the rest of an unwrapped row takes.
basis: fct-yurmbmhi
because: The NAME column is now the widest cell with its tree indent included, so a wrapped NAME's width alone no longer sets it.
@row_add n=1 row=contract/viola-run-flags/parent area=surface-cli fact=F1 detector=none
boundary: viola run flags
item: --parent
promise: `viola run --parent <NAME>` declares the instance's parent; a parent equal to the instance's own name is a usage error with exit 2; without the flag the inherited session name is the parent, dropped when it equals the instance's own name.
basis: fct-yurmbmhi, fct-evi6pgaa
because: viola run gains the --parent flag and its self-parent usage error, a new contract of the CLI surface no row states.
@row_add n=2 row=contract/viola-run-flags/role area=surface-cli fact=F1 detector=none
boundary: viola run flags
item: --role
promise: `viola run --role <WORD>` declares the instance's role, a word that must pass the code-word check.
basis: fct-yurmbmhi
because: viola run gains the --role flag checked as a code word, a new contract of the CLI surface no row states.
@row_add n=3 row=contract/viola-list-board/links-block area=surface fact=F3 detector=D-design-cli-wording
boundary: viola list board
item: links block
promise: After the board and the unwrapped rack, `-- LINKS --` then one uncoloured `<driver> -> <driven>` line per link, sorted and deduplicated, both names checked, in the board's one write; with no link the block is absent.
basis: fct-yurmbmhi, fct-i5eugfgb
because: viola list now prints a LINKS block whose wording no master row states.
@row_add n=4 row=contract/viola-list-json/item-parent-role area=surface-cli fact=F4 detector=none
boundary: viola list --json
item: item parent and role
promise: A `list --json` item carries `parent` and `role` only when present; the addition does not move the document version.
basis: fct-uoqmd72r, fct-yurmbmhi
because: list --json items gain optional parent and role keys, an additive contract no row states.
@row_add n=5 row=contract/viola-list-json/ok-links area=surface-cli fact=F4 detector=none
boundary: viola list --json
item: ok links
promise: The `ok` object of `list --json` carries `links` only when non-empty; the addition does not move the document version.
basis: fct-uoqmd72r, fct-yurmbmhi
because: list --json ok gains optional links, an additive contract no row states.
@row_add n=6 row=decision/lineage-shown-never-acted-on area=brand-identity fact=F5 detector=none
name: Declared lineage shown, never acted on
rule: A declared parent and role are shown and never acted on: no wheel, link, send or answer path reads them, and the parent edge is drawn apart from send links.
guard: lineage_is_read_by_no_wheel_link_send_or_answer_path
basis: fct-ao5bhrj4, fct-yurmbmhi
because: The chunk adds lineage as a display-only label held by a contract test, a decision future work must respect.
@row_add n=7 row=decision/revive-keeps-lineage area=surface-cli fact=F6 detector=none
name: Revive keeps the recorded lineage
rule: `viola revive` keeps the recorded parent and role on the revived start, and a reused name started without a parent shows none.
basis: fct-yurmbmhi, fct-ao5bhrj4
because: revive now carries the recorded parent and role forward, a behaviour of the CLI surface no row states.
