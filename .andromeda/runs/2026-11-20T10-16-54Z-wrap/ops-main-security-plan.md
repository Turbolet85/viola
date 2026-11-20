@row_set n=1 row=decision/input-box-readiness-from-literals cell=rule seen=1 detector=none
value: The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars directly above a row opening the prompt char, and never while a standing modal is up; one predicate decides for verify and for viola run's pre-send gate.
basis: fct-xxy4my3t, fct-pn22p663
because: The predicate's modal clause moved from any modal literal on the screen to a standing modal, rule (A), and verify's paste guard still calls the same predicate, so «one predicate» stands.
@row_set n=2 row=decision/verify-reads-only-signatures cell=rule seen=1 detector=none
value: verify's PTY output is read only against the compiled signatures: the literals, and the compiled shapes over them (a modal label row's option shape; the box's rows below it).
basis: fct-xxy4my3t, fct-pn22p663, fct-ph5bkvoz
because: Verify's paste guard reads the moved predicate, which decides on the compiled shapes over the compiled literals, so a reading of literals alone is incomplete.
@row_set n=3 row=decision/verify-pty-output-caught cell=rule seen=1 detector=none
value: verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signatures: the literals, and the compiled shapes over them (a modal label row's option shape; the box's rows below it).
basis: fct-xxy4my3t, fct-pn22p663, fct-ph5bkvoz
because: Verify's paste guard reads the moved predicate, which decides on the compiled shapes over the compiled literals, so a reading of literals alone is incomplete.
@row_set n=4 row=decision/verify-screen-read-against-literals cell=rule seen=2 detector=none
value: verify's children's screens are read only against compiled signatures: the literals, and the compiled shapes over them (a modal label row's option shape; the box's rows below it), and their row text leaves the process only as signature-only record fixtures.
basis: fct-xxy4my3t, fct-pn22p663, fct-ph5bkvoz
because: Verify's paste guard reads the moved predicate, which decides on the compiled shapes over the compiled literals, so a reading of literals alone is incomplete.
@row_set n=5 row=decision/verify-signature-only cell=rule seen=2 detector=none
value: viola verify's interactive children are read only against the compiled signatures: the literals, and the compiled shapes over them (a modal label row's option shape; the box's rows below it), their row text leaving the process only as signature-only record fixtures.
basis: fct-xxy4my3t, fct-pn22p663, fct-ph5bkvoz
because: Verify's paste guard reads the moved predicate, which decides on the compiled shapes over the compiled literals, so a reading of literals alone is incomplete.
