@task_prose part=chunk
Makes the owed live re-read of the relied capability-ledger row trust-record on the real 2.1.325 claude CLI, built so far only on the fake agent.
viola verify keeps one closed first-screen word per interactive run (trust, imports, box, none; null for a run that did not start) in the capability stamp's measured block, from compiled screen signatures, keeping no screen text.
The test rig's V arm prints the six words before teardown and runs on a git-initialised scratch root seeded with the trust record at that root's key; the verify window kill and its 52 s test case get bounds from the measured walls; verify --record gets a witness of where its record went.
Then one base live verify start (plus two spares only after a named cause), read against the floor of 21 rows the live home's stamp passes; nothing of viola run or revive changes and no capability is claimed.
@task_file n=1 path=src/cmd/verify/typed/run.rs mark=edits
what: viola verify's interactive child runs against the claude CLI, which receive the kept CLAUDE_CONFIG_DIR — edits: the per-run first-screen word kept in the stamp's measured block, the verify window kill
@task_item n=1 part=needs
viola verify's capability stamp: a new closed per-run first-screen word in its measured block; verify stays the only writer of the stamps and the stamp path and contents are never written into a log line.
@task_item n=2 part=needs
The first-screen word is derived from compiled screen signatures only: no screen text, no PTY bytes, nothing of the founder's config reaches the stamp, a log line or the rig's printout; only closed words and numbers (the child's exit before the read, the count of settled frames, the run's wall).
@task_item n=3 part=needs
viola verify's log lines carry no content and are written only inside an instance; any new verify line or field for the first-screen word or the closed facts stays a closed code within the diagnostics line schema and its additive-only rule.
@task_item n=4 part=needs
viola verify's child spawns (version probe, print probe, PTY probe) and their process-exit lines with child_exit_status and duration_ms: the closed facts the plan uses to tell whether a child exited or was cut before the screen read.
@task_item n=5 part=needs
The verify window kill: the kill of an interactive run whose screens never settle, bounded or moved by the measured 52 s walls; a killed run reads as a closed outcome, not an error with a chain on stderr.
@task_item n=6 part=needs
viola verify's exit codes 0 and 1 and its stdout step and summary lines: the live reading is taken from them (verify-exit, rows passed out of 23, trust-record pass or fail).
@task_item n=7 part=needs
The CLAUDE environment strip in verify's interactive children: CLAUDE_CONFIG_DIR is kept, stripped values never logged and only names or counts are recorded; the P3 comparison of verify's child against the rig's own wrapped start reads argv, env after the strip, cwd and pty size without logging any value.
@task_item n=8 part=needs
The trust-record write and its seed stay uninstrumented: the rig's seed at the scratch root's key adds no log line, and the --record witness reports where the record went as a closed fact, no path in any home-level line.
@task_item n=9 part=needs
Real-CLI verify never runs in CI; CI verify runs against the fake agent only: the live start S0 prime is a local run under the rig's private config, and the fake-agent work lands green in the tree before it.
@task_item n=10 part=needs
A change to the fake agent's screens or to verify's runs puts cli_verify in the gate block; a test that waits for an outcome takes an explicit bound under the 20 s lint and never waits by sleeping on a log.
@task_item n=11 part=needs
Unreadable or failing ledger rows: an unreadable ledger counts as an unverified CLI, and the row trust-record stays unverified unless S0 prime reads it pass with the floor held.
@task_item n=12 part=needs
No telemetry is taken from screen content: the first-screen word is a classification kept in the stamp, never a log field carrying what the screen showed.
@task_standing n=1
convention: every new or changed integration test that drives a process checks the diagnostics lines it emits against the line schema
home: ci-integration
@task_standing n=2
convention: every new test home lives under the one integration-test root the secret scan covers and is kept until the gates run
home: 9-ci-integration
@task_standing n=3
convention: no test waits for a log line by sleeping; every wait has an explicit bound
home: 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
viola run's start path, its spans and span attributes, run exit-1 detail codes and the run process log: nothing of viola run changes.
@task_item n=2 part=not_needed
viola revive, its preflight order and its process log: revive does not change.
@task_item n=3 part=not_needed
The send, wait, last, dialog, wheel and pause or release paths, their log fields, span chains and refusal codes: the chunk touches none of them.
@task_item n=4 part=not_needed
viola mcp tools, its stdin framing and its isError results, and the viola-channel JSON-RPC lines and correlation: not reached.
@task_item n=5 part=not_needed
Heartbeat liveness, list status, link and unlink, and state recovery from the events log: not reached.
@task_item n=6 part=not_needed
A future error reporter, OTel export, network egress and hosted deployment rows: the chunk adds no exporter or reporter.
@task_item n=7 part=not_needed
Hook latency budgets and the perf job's hyperfine gates: the chunk adds no hook work.
