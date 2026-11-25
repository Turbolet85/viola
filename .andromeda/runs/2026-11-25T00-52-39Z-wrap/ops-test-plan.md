@row_add n=1 row=decision/r5-needs-no-unlocked-desktop area=e2e-test-strategy fact=F1 detector=none
name: Gated live reading R5 needs no unlocked desktop
rule: The gated live reading R5 of viola adopt is valid with the screen locked: its script hosts adopt in a pseudo-terminal of its own and opens no window, so an unlocked desktop is no precondition of the reading.
why: The script types into its own pseudo-terminal and opens no window.
basis: fct-kphna6yl, fct-qmtw73em
because: The chunk's green R5a firing ran with the screen read locked, against the block's word «unlocked», and the report states the script needs no window, so future R5 firings must not wait on an unlocked desktop.
@row_add n=2 row=decision/r5a-refire-on-a-fresh-instance area=e2e-test-strategy fact=F2 detector=none
name: R5a re-fires only on a fresh instance and base
rule: Before any re-fire of the gated live reading R5a, the earlier firing's instance and base are moved aside whole; a re-fire on a reused instance is no reading.
why: The R5a script reads the first session-start in the adopted instance's events, so an earlier firing's line would be read in place of the new one.
basis: fct-fxry6mhk
because: The report records that the earlier firings' instance and base were moved aside because the script reads the first session-start, a precondition every later R5a firing must respect.
@row_add n=3 row=decision/r5a-adopts-a-kept-transcript area=e2e-test-strategy fact=F3 detector=none
name: R5a adopts only a session with a kept transcript
rule: The gated live reading R5a adopts only a plain session whose transcript the CLI's store holds, one the operator started, made one exchange in and ended himself; a red reading on a session id with no kept transcript behind it proves nothing about adopt.
why: Both earlier red R5a readings resumed session ids with no kept transcript behind them, while the green one resumed a session whose transcript the store holds.
basis: fct-qmtw73em, fct-pbupwawq, fct-stqnphx3
because: The report shows the two earlier reds on ids with no kept transcript and the green on an operator-made session the store holds, which binds what a valid R5a firing adopts.
