@row_set n=1 row=decision/input-box-readiness-from-literals cell=rule seen=1 fact=F1 detector=none
value: The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars right above a prompt-char row, never while a modal literal stands: option-shaped, or no box row under it, a footer being none; one predicate decides for verify and viola run's pre-send gate.
basis: fct-xxy4my3t, fct-pn22p663
because: The readiness predicate's modal refusal moved from any row holding a modal literal to a standing modal (option-shaped, or no input-box row below it), so a label quoted mid-row above the box no longer refuses and the rule must say what standing means.
@row_set n=2 row=decision/verify-reads-only-signatures cell=rule seen=1 fact=F2 detector=none
value: verify's PTY output is read only against the compiled signature literals and compiled characters, and no row text is kept or returned.
basis: fct-ph5bkvoz, fct-y5bdjkji, fct-pn22p663
because: The standing-modal rule verify's paste guard now reads decides on compiled characters (selector glyph, numbering, spaces) beside the compiled literals and keeps or returns no row text, so a row saying literals alone is incomplete.
@row_set n=3 row=decision/verify-pty-output-caught cell=rule seen=1 detector=none
value: verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signature literals and compiled characters.
basis: fct-ph5bkvoz, fct-pn22p663
because: The readiness predicate verify reads its screen through now decides on compiled characters as well as compiled literals, so the row's literals-only reading is incomplete.
@row_set n=4 row=decision/verify-screen-read-against-literals cell=rule seen=2 detector=none
value: verify's children's screens are read only against compiled signature literals and compiled characters, and their row text leaves the process only as signature-only record fixtures.
basis: fct-ph5bkvoz, fct-pn22p663
because: The standing-modal rule under the moved readiness predicate reads compiled characters beside the compiled literals, so the row's literals-only reading is incomplete.
@row_set n=5 row=decision/verify-signature-only cell=rule seen=2 detector=none
value: viola verify's interactive children are read only against the compiled signature literals and compiled characters, their row text leaving the process only as signature-only record fixtures.
basis: fct-ph5bkvoz, fct-pn22p663
because: The standing-modal rule under the moved readiness predicate reads compiled characters beside the compiled literals, so the row's literals-only reading is incomplete.
