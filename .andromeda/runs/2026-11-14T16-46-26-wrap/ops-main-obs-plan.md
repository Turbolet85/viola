@row_set n=1 row=decision/diag-schema-additive-only cell=except seen=1 detector=none
value: The budget gate's removal dropped its fields from the diag-line schema (`budget_written`; the send lines' `window`, `used_percentage`, `resets_at`, `paused`), which stays v1.
basis: fct-xalfd6ij
because: The diag-line schema dropped the budget fields with its version kept, on the founder's rulings removing the gate and keeping every v at 1, so the never-remove rule must name this one removal.
@row_set n=2 row=decision/log-fields-additive-only cell=except seen=1 detector=none
value: The budget gate's removal dropped its fields from process-log lines (`budget_written`; the send lines' `window`, `used_percentage`, `resets_at`, `paused`).
basis: fct-xalfd6ij
because: The budget fields left the process-log lines with the gate, so the never-remove rule must name this one removal.
@row_set n=3 row=decision/log-schema-additive-only cell=except seen=1 detector=none
value: The budget gate's removal dropped its fields from the schema (`budget_written`; the send lines' `window`, `used_percentage`, `resets_at`, `paused`).
basis: fct-xalfd6ij
because: The diag-line schema dropped the budget fields with the gate, so the removes-nothing rule must name this one removal.
@row_set n=4 row=decision/obs-aligns-to-tests-log-format cell=except seen=1 detector=none
value: The budget gate's removal dropped its fields from the log format (`budget_written`; the send lines' `window`, `used_percentage`, `resets_at`, `paused`).
basis: fct-xalfd6ij
because: The diag-line schema dropped the budget fields with the gate, so the must-not-remove rule must name this one removal.
