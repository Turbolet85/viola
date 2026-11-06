@apply_row n=1 ref_file=fanout-a11y-plan--cascade.md ref_n=5 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan--cascade.md ref_n=8 v=applied edit=E2
@apply_row n=3 ref_file=fanout-a11y-plan--cascade.md ref_n=10 v=applied edit=E3
@apply_row n=4 ref_file=fanout-a11y-plan--cascade.md ref_n=11 v=applied edit=E4
@apply_row n=5 ref_file=fanout-a11y-plan.md ref_n=1 v=applied edit=E5
@apply_row n=6 ref_file=fanout-a11y-plan--cascade.md ref_n=3 v=folded_into to_file=fanout-a11y-plan.md to_n=1
@apply_row n=7 ref_file=fanout-a11y-plan.md ref_n=2 v=refused
why: fct-dlpkde76 stands in the key a11y-testing-tool-pick, which is not among this pass's target files (body, a11y-scope-summary, aria-patterns-roles, cognitive-accessibility), so its rev and text are unread here; the step-2 sweep's patterns a11y-01 to a11y-05 reach it
@apply_row n=8 ref_file=fanout-a11y-plan.md ref_n=3 v=applied edit=E6,E7,E8,E9,E10,E11,E12
@apply_row n=9 ref_file=fanout-a11y-plan--cascade.md ref_n=1 v=applied edit=E13
@apply_row n=10 ref_file=fanout-a11y-plan--cascade.md ref_n=2 v=applied edit=E14
@apply_row n=11 ref_file=fanout-a11y-plan--cascade.md ref_n=4 v=applied edit=E15
@apply_row n=12 ref_file=fanout-a11y-plan--cascade.md ref_n=6 v=applied edit=E16
@apply_row n=13 ref_file=fanout-a11y-plan--cascade.md ref_n=7 v=applied edit=E17
@apply_row n=14 ref_file=fanout-a11y-plan--cascade.md ref_n=9 v=applied edit=E18
@fact id=fct-ijnl2u6y expect_rev=2
old: no ARIA; trycmd / assert_cmd assert the non-TTY modes
new: no ARIA; literal pins and assert_cmd assert the non-TTY modes

@fact id=fct-2jdilj7v expect_rev=2
old: **Verification:** trycmd pins the `unable <reason> <detail>` + final `hint:` layout
new: **Verification:** literal pins hold the `unable <reason> <detail>` + final `hint:` layout

@fact id=fct-fhiyvzbh expect_rev=1
old: every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help`, `viola plugin install`, the server-verify, socket-directory, `unwrapped-name` and `endpoint-lost` hints and the `viola link` / `viola unlink` result lines, by literal, so any
new: every fixed CLI string is pinned by literal (`viola list`, `viola --help`, `viola plugin install`, the server-verify, socket-directory, `unwrapped-name` and `endpoint-lost` hints and the `viola link` / `viola unlink` result lines among them), so any

@fact id=fct-yzqlnwsx expect_rev=2
old: assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1; portable-pty outer-PTY SGR checks
new: assert_cmd + predicates + literal pins; portable-pty outer-PTY SGR checks

@fact id=fct-4ktfiloi expect_rev=2
old: boundary driver (inherited `=0.8.1`): zero
new: boundary driver (`=0.9.0`): zero

@fact id=fct-xmkdtzxa expect_rev=1
old: Use the existing `assert_cmd` / `trycmd` / portable-pty output-discipline assertions
new: Use the existing `assert_cmd` / literal-pin / portable-pty output-discipline assertions

@fact id=fct-zj2wxt2x expect_rev=1
old: then `hint:` via trycmd]
new: then `hint:` pinned by literal]

@fact id=fct-6py5v5ks expect_rev=1
old: [cli: `hint:` line pinned by trycmd]
new: [cli: `hint:` line pinned by literal]

@fact id=fct-3xiq4dqd expect_rev=1
old: + `hint:` via trycmd.
new: + `hint:` pinned by literal.

@fact id=fct-cyo3rpos expect_rev=1
old: **Verification sources:** trycmd.
new: **Verification sources:** the `CAPTION` literal.

@fact id=fct-s4vibfo3 expect_rev=2
old: assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1; portable-pty outer PTY
new: assert_cmd + predicates + literal pins; portable-pty outer PTY

@fact id=fct-ms7djxet expect_rev=1
old: assert_cmd / trycmd / portable-pty results
new: assert_cmd / literal-pin / portable-pty results

@fact id=fct-vtox22ts expect_rev=2
old: `assert_cmd` 2.2.2 with `predicates` 3.1.4, `trycmd` 1.2.1, and SGR-only checks
new: `assert_cmd` with `predicates`, the fixed strings pinned by literal in root and unit tests, and SGR-only checks

@fact id=fct-twqx5x46 expect_rev=2
old: portable-pty `=0.8.1` outer-PTY driver
new: portable-pty `=0.9.0` outer-PTY driver

@fact id=fct-uok334yy expect_rev=3
old: No ARIA. trycmd / assert_cmd assert the non-TTY modes
new: No ARIA. Literal pins and assert_cmd assert the non-TTY modes

@fact id=fct-6edxk4iz expect_rev=2
old: `DIALOG_HOLD_MAX`, each PROVISIONAL;
new: `DIALOG_HOLD_MAX`, each PROVISIONAL, owner: 0.2.0's first entry;

@fact id=fct-g3h7n6e2 expect_rev=2
old: **cli:** trycmd cases pin the `unable <reason> <detail>` + final `hint:` layout
new: **cli:** literal pins in root and unit tests pin the `unable <reason> <detail>` + final `hint:` layout

@fact id=fct-oji2cu7x expect_rev=1
old: trycmd pins every fixed CLI string, except `viola list`'s, which literals pin (the board
new: Every fixed CLI string is pinned by literal: `viola list`'s (the board

@pattern id=a11y-01
fixed: trycmd 1.2.1
@pattern id=a11y-02
fixed: via trycmd
@pattern id=a11y-03
regex: trycmd (pins|cases pin)|pinned by trycmd|/ `?trycmd`? /|Verification sources:\*\* trycmd
@pattern id=a11y-04
fixed: inherited `=0.8.1`
@pattern id=a11y-05
fixed: portable-pty `=0.8.1`
