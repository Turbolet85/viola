@apply_row n=1 ref_file=fanout-architecture--cascade-g7.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g7.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g7.md ref_n=3 v=folded_into to_file=fanout-architecture--cascade-g7.md to_n=2
@apply_row n=4 ref_file=fanout-architecture--cascade-g7.md ref_n=4 v=applied edit=E3
@fact id=fct-k6htnrfj expect_rev=1
old: (the Unix R8 pass-list of
new: (the R8 pass-list of

@fact id=fct-wfge3rsz expect_rev=1
old: relied on every OS and stamped live
new: relied and stamped live
old: sixth paste, relied on every OS, stamped
new: sixth paste, relied, stamped
old: relied on Unix only and stamped live
new: relied and stamped live
old: (`LedgerRow::relied()`: 24 rows on Unix, 21 on Windows;
new: (`LedgerRow::relied()`: all 24 rows on every build;
old: agents read, relied on every OS, stamped
new: agents read, relied, stamped
old: `evidence/live-round.md`), its Windows reading owed to "Windows-only live measurements" ([Delivery Confirmation])
new: `evidence/live-round.md`; [Delivery Confirmation])
old: ), its Windows reading owed to "Windows-only live measurements". `/compact` is
new: ). `/compact` is
old: ), its Windows reading owed on "Windows-only live measurements"; so
new: ); so
old: stamped row; the Windows legs stay owed to "Windows-only live measurements" (the operator's direction at the 2026-10-10-statusline-pass-through wrap). The
new: stamped row. The
old: ), its Windows reading owed on "Windows-only live measurements"; neither
new: ); neither

@fact id=fct-xxlqtlbc expect_rev=1
old: (so `feature = "unix-like"` is no hit).
new: (so `feature = "unix-like"` is no hit). A gate whose predicate names `windows` or `target_os = "macos"` reads as a fault (`another OS: …`) even when a `GATES` row names it, which `census_reads_a_gate_naming_another_os_as_a_fault_even_when_named` proves fires; the planted Windows cases build the word from a `WINDOWS` const.

@pattern id=arch6-01
fixed: Unix R8 pass-list
@pattern id=arch6-02
fixed: 24 rows on Unix, 21 on Windows
@pattern id=arch6-03 ignore_case=true
regex: relied on (Unix only|every OS)
@pattern id=arch6-04
regex: Windows (reading owed|legs stay owed)
