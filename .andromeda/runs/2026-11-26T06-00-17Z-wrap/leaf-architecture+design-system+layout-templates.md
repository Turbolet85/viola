@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: every refusal of `viola open --json` and every exit-1 refusal of `viola focus --json` (one of `refused` · `unsupported` · `window-not-found` · `unreachable` · `malformed` · `oversize`)
source: ctr-jayaxyks «contract/viola-open-exit-codes/1 · viola open exit codes · 1 — A refusal: strict-modes-failed before any log, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live or internal-error»
replace-with: every refusal of `viola open --json` (one of `strict-modes-failed` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `terminal-not-found` · `not-live` · `internal-error`) and every exit-1 refusal of `viola focus --json` (one of `refused` · `unsupported` · `window-not-found` · `unreachable` · `malformed` · `oversize`)
@leaf_stands n=1 leaf=.claude/rules/frontend.md line=8
because: The sources still hold one built cli surface, no web-spa surface (abs-kesgqpv5, abs-jb3mdokp) and the terminal fleet console as the one further view (dec-ubc5oo6l).
@leaf_stands n=2 leaf=.claude/rules/frontend.md line=12
because: dec-n47v3ec6 keeps amber for dialog_pending and blue for handoff only, and ctr-p5tul4pv keeps handoff blue unused with link output plain.
@leaf_stands n=3 leaf=.claude/rules/frontend.md line=17
because: dec-nlw4yedf still says the cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
@leaf_stands n=4 leaf=.claude/rules/frontend.md line=20
because: The captions (dec-lz3h724z), readback words (ctr-kwnmlrk5), open line and eight placement words (ctr-jezharna, placement-word rows), focus (ctr-x63m6t5f), up (ctr-z7n7sjhl) and switch (ctr-vpsfkquh) outputs read as the leaf says.
@leaf_stands n=5 leaf=.claude/rules/frontend.md line=21
because: dec-nlkfcb3b, dec-nmciaxz5 and dec-dhfntmue still state no success before read-back, no turn-ended fill and the API-error turn end shown as any turn end.
@leaf_stands n=6 leaf=.claude/rules/frontend.md line=24
because: The colour decision order rows, the cli colour tokens, and the viola list board rows (tree, status join, unwrapped rack, LINKS block, NAME cut) read as the leaf says.
@leaf_stands n=7 leaf=.claude/rules/frontend.md line=26
because: dec-naqagg5b, ctr-pnj46xxs, dec-naojisrn, dec-mort5xha and ctr-6iwmj65y still state the escaping and hint rules the leaf gives.
