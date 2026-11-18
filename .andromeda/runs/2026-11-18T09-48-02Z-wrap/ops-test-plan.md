@row_set n=1 row=decision/record-only-at-zero-failing-rows cell=except seen=1 fact=F1 detector=none
value: A failed recording verify by the live rig alone writes a first-screen file of whole scrubbed rows per run short of the input box (Run A's trust excepted), refused whole when unclean, never committed.
basis: fct-xaokr7kk, fct-vynq47jv
because: The hidden rig-only --record-first-screens option now writes files on a FAILED recording verify, so the zero-failing-rows rule must name that one exception.
@row_retire n=1 row=decision/verify-window-60s-kill seen=1 heir=decision/verify-window-80s-kill detector=D-tests-count-literals
why: The verify_window_ class kill is no longer 60 s; it moved to 80 s in both the ci and mutants profiles.
basis: fct-s7pcz72p, fct-vynq47jv, fct-utzixcto
because: The row's name and rule state the retired 60 s literal, and its key is made from that value, so the row is replaced by its 80 s heir.
@row_add n=1 row=decision/verify-window-80s-kill area=bootstrap-phases-derive-for-route-setup-project detector=D-tests-count-literals
name: 80 s kill for the verify_window_ class
rule: Tests named verify_window_ take an 80 s per-test kill, in both the ci and mutants profiles, through an override that stands first.
why: Their cases wait out the gate's 8.5 s maximum several times by design.
guard: mutants_verify_window_override_stands_before_the_harness_package_override
basis: fct-s7pcz72p, fct-vynq47jv
because: The verify_window_ nextest kill moved 60 s to 80 s in ci and mutants, and the heir carries the standing rule at its new bound.
@row_add n=2 row=decision/instrumented-probe-runs-in-scratch-cwd area=quality-gates-coverage-targets fact=F2 detector=D-tests-coverage
name: Instrumented probe runs in a scratch cwd
rule: A test probe that spawns an instrumented binary under a cleared environment runs with a scratch directory as its cwd, so no coverage profile lands in the tree.
why: Under llvm-cov a child left with no LLVM_PROFILE_FILE writes its profile into its cwd, which moved the tree's status.
basis: fct-yqzdiq7x, fct-s44rn2ws
because: The pre-push red from stray profraw files in the tree was fixed by giving the version probe a scratch cwd, a convention no standing coverage row states.
@row_none n=1 fact=F3 type=5
what: Which CLI version the trust-record row is verified live on, and by which rig start, is a version-by-version inventory the ledger stamps and the route hold.
