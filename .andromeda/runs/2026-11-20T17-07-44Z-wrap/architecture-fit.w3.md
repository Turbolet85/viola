@fit n=1 row=decision/stop-during-start-verify sha=7dafd6b86dfa v=not_master type=4
why: How the start's stop handler tears down its verify child (order, TERM, 30 s grace, KILL, re-raise) is one mechanism whose basis, a test case's outcome, states no reason for it, so the code and its test hold it.
