# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the panic hook's raw-backtrace line is an unchanged component (Not needed: the panic hook unchanged), and the chunk's new reads neither call into it nor pass it a value
error-capture-reporting#2.1 · yes · the hook's fail-open exit 0 and the CLI verbs' exit 21 must return through each role's own exit, never an Err the catch site maps to process-exit internal-error; the run-worker and vt100 clauses are not reached
error-capture-reporting#3.1 · no · the window before init step 4 holds no verification code the chunk adds, since its mismatch lines go to a role file that exists only after step 4; the statusline re-read checks read_snapshot's outcome, not this window's logging rule
error-capture-reporting#4.1 · no · the vt100 feed-thread catch sits in run's server side, which the chunk leaves unchanged, and run.rs is re-read only at its collision check
