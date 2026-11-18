@row_set n=1 row=contract/fake-agent-cli/trust-config cell=promise seen=1 fact=F4 detector=none
value: The fake agent reads trust from the named config file by the CLI's walk, the canonical cwd's exact key then each parent up to the git root, so inside a repository the cwd's own record counts, through a bounded read; any failure reads untrusted.
basis: fct-vwyqtjra, fct-tvk75cho
because: The fake agent's own key rule (inside a repository only the root's record counts) is retired and it now models the CLI's walk, so the promise must say the walk.
@row_add n=1 row=decision/run-under-test-owned-user-home area=test-data-bootstrap fact=F1 detector=none
name: Test runs and revives under a test-owned user home
rule: Every test-started viola run and viola revive runs under a test-owned user home, never the runner's real HOME.
why: The start writes into the user's Claude config.
basis: fct-o4xnvz66, fct-v4zhvqn7
because: The start's trust step now writes the Claude config, so a test start under the real HOME would write the operator's real config; no row of the area binds run and revive tests to a test-owned user home.
@row_add n=2 row=decision/harness-wrappers-under-session-user-home area=test-data-bootstrap fact=F2 detector=none
name: Harness wrappers under a session user home
rule: The harness's supervised wrappers of a stamped session run under a session user home seeded with an empty config.
why: The start writes into the user's Claude config.
basis: fct-v4zhvqn7, fct-o4xnvz66
because: The harness now starts its supervised wrappers under a session user home so a stamped session's trust step never reaches the real config; no row says it.
@row_add n=3 row=decision/trust-start-folders-outside-repository area=7-test-data-fixtures fact=F3 detector=none
name: Trust-start test folders outside any repository
rule: Trust-start test folders sit under /tmp, outside any repository, an exception to test homes under target/e2e-home.
why: A folder inside the workspace keys at its git root.
basis: fct-mttmg6nh
because: The trust cases need a folder whose own key is written, which a folder under the workspace's target/e2e-home cannot give; no row names this exception.
@row_add n=4 row=contract/log-event/trust-record area=log-format fact=F5 detector=D-tests-obs-harness
boundary: process log event
item: trust-record
promise: An INFO line logged once per viola run or viola revive start on a verified CLI; its detail, required, is one of the 16 start words, and it carries no path, key or value.
basis: fct-xro6zomc, fct-wmizsmr6, fct-vrquvti6
because: The closed log event set gains trust-record with a required closed detail, a log-format contract the harness and the diag-line schema read, and no row of the area states it.
@row_none n=1 fact=F6 type=5
what: An inventory of the live rig's arms for one chunk's proof (S1 and S2 on one ledger); the code and the sidecar hold it, and the standing live rig row already holds the decision.
