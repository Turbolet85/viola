@row_set n=1 row=decision/diag-schema-additive-only cell=rule seen=1 detector=none
value: The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove a field the product still emits.
basis: fct-xalfd6ij
because: The diag-line schema dropped the retired budget gate's fields, so the never-remove rule stands false as written; fit read an exception naming the removal as history.
@row_set n=2 row=decision/log-fields-additive-only cell=rule seen=1 detector=none
value: Fields may be added to process-log lines, but a required one the product still emits is never renamed or removed.
basis: fct-xalfd6ij
because: The budget fields left the process-log lines with the gate, so the never-remove rule stands false as written.
@row_set n=3 row=decision/log-schema-additive-only cell=rule seen=1 detector=none
value: The observability log schema only adds fields and enum values to the tests' log format; it renames nothing and removes only a field no build emits.
basis: fct-xalfd6ij
because: The diag-line schema dropped the retired budget gate's fields, so the removes-nothing rule stands false as written.
@row_set n=4 row=decision/obs-aligns-to-tests-log-format cell=rule seen=1 detector=none
value: The log format is bound by the test plan's harness contract; observability may add fields but must not rename them or remove one the product still emits.
basis: fct-xalfd6ij
because: The diag-line schema dropped the retired budget gate's fields, so the must-not-remove rule stands false as written.
