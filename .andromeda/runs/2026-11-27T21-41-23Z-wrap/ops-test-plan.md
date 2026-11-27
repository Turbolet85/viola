@row_set n=1 row=decision/verify-window-80s-kill cell=rule seen=2 detector=D-tests-count-literals
value: Tests named verify_window_, the fake agent dialog and framing replay tests over every recorded set, and the verify-driven tests the class's filter line names one by one, take an 80 s per-test kill, in both the ci and mutants profiles, through an override that stands first.
basis: fct-rr5mjbee, fct-pla7hbyi
because: The verify class's filter line, identical in the ci and mutants profiles, now takes six verify-driven tests by name beside its prefix and the replay tests, so the row naming the class's members must name that third member too.
@row_set n=2 row=decision/verify-driven-tests-20s-kill cell=except seen=3 detector=D-tests-count-literals
value: A verify-driven test of the verify_window_ class, by its prefix or named in that class's filter line, takes that class's 80 s kill instead.
basis: fct-rr5mjbee, fct-pla7hbyi
because: Six verify-driven tests outside the verify_window_ prefix now take the class's kill by name, so the exception naming who leaves the 20 s bound must cover the named members as well.
@row_add n=1 row=decision/r2-refuses-before-anything-starts area=e2e-test-strategy fact=F1 detector=none
name: R2 live reading refuses an unfit desktop or scratch
rule: The R2 live reading refuses before anything is made or started, printing r2: red precondition=scratch when its scratch home or its runtime-dir entry already exists, or r2: red precondition=locked when the desktop's locked reading is not exactly false, and exits 1 closing no window.
basis: fct-pbmtkmpr, fct-l4xhjiiw
because: The R2 script gained a precondition refusal with its own verdict line ahead of any start, and no row of the E2E strategy states what the gated live reading refuses.
@row_add n=2 row=decision/start-paths-declared-once area=3-test-harness-contract fact=F2 detector=none
name: Start paths declared once
rule: viola run, viola revive and viola adopt start through one shared start prologue (trust check, instance check, pinned-exe check with the strict-modes refusal under --json, then the wrapper log), so a start-order test over one verb holds for all three.
basis: fct-xoaulb66, fct-nypfwqez
because: The start checks of revive and adopt now run through the one prologue in the shared start, and no row of the harness contract declares the start paths once.
@row_add n=3 row=decision/skipped-drain-case-consumes-output-end area=unit-test-strategy fact=F3 detector=none
name: Skipped-drain pump case consumes the output end first
rule: A pump case asserting that the exit drain was skipped has the run loop consume the output-ended signal before the child exits, the case's child exiting only after the reader has ended.
why: Otherwise the exit-path drain reads the signal and the mutant passes too.
basis: fct-ic35l2mp
because: The chunk's pump cases established this mutation-test design rule, and no unit-strategy row states how a skipped-drain case must be built to kill its mutant.
