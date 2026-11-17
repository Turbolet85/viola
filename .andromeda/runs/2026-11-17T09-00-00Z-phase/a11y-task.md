@task_prose part=chunk
Makes every `viola run` wrapper a test boots, and the fake agent under it, end with that test on every exit path: passed, failed, timed out or killed by SIGKILL. After a test run no booted wrapper or fake agent is left running.
First step: the shared stamped test home is valid, or is rebuilt, on any host where its home is absent, whatever a restored `target/` cache carries. A test fails before the fix (stamp marker present, home absent) and passes after it.
Adds a proof of «killed»: a harness the test owns SIGKILLs a test process mid-run on the fake agent, then a census at its end shows nothing of that test's own process tree is still running.
Re-measures from the code every place a test boots a wrapper, what ends it today on each exit path, and why a killed test's wrapper spun. Closes the CI red ci#44048204364 (shared stamped home absent) against that run.
Fixtures and the fake agent only. No product behaviour change is intended. A change to how `viola run` ends once its controlling test is gone is in scope only as P3 measures it.
@task_file n=1 path=tests/support/home.rs mark=edits
what: the shared stamped test home fixture that every booted test reads, its stamp marker and the home it points at — edits: stamp_dir, stamped_home
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness's boot of a `viola run` wrapper over the fake agent on the shared stamped home, and what ends that wrapper when its test ends
@task_item n=1 part=needs
A booted `viola run` wrapper and its fake agent end when the test that booted them ends, on every exit path: pass, fail, timeout, and SIGKILL, which runs no destructor. After a test run no booted wrapper or fake agent is left running.
@task_item n=2 part=needs
The shared stamped fixture home is trusted only when its home exists. A stamp marker restored under `target/` whose home is gone (the runner's `/tmp`, reached through the `target/e2e-home` link) leads to a rebuild, not to a verified-home read. A booted wrapper must not refuse with `unverified-cli` for that reason.
@task_item n=3 part=needs
New test for the stamp fix: it fails before the fix (marker present, home absent) and passes after it. It is the chunk's first step.
@task_item n=4 part=needs
Proof of «killed»: a harness the test owns SIGKILLs a test process mid-run on the fake agent, and a census at its end shows none of that test's own process tree still running. No test sends a kill to a process it does not own.
@task_item n=5 part=needs
Re-measure: every place a test boots a wrapper (the shared fixtures and any test that spawns one itself), what ends each one today on each exit path, and what a wrapper loops on once its test is gone, the reason it spun near a full core rather than idled.
@task_item n=6 part=needs
If `viola run` itself has to change to end once its PTY or parent is gone (its loop), that change is held to P3's measure. Its exit path still writes no viola bytes of its own to the human's terminal, and its run diagnostics still go only off the terminal.
@task_item n=7 part=needs
Any test that waits on a channel or a thread takes a bound under the project's 20 s lint and cannot hang past a failure. Time bounds fall on the test driver, never on a human. A test socket goes under the short `/tmp` root.
@task_item n=8 part=needs
The test drives the wrapper's outer side through the existing boundary driver, the outer PTY, and is sequenced on records, not on reading claude's screen content.
@task_item n=9 part=needs
No live `claude` starts. A booted test runs on the fake agent only, and live readings gate nothing.
@task_item n=10 part=needs
A change to the fake agent's screens puts the `cli_verify` checks in the gate block.
@task_item n=11 part=needs
Acceptance of the CI red: the stamped-home hypothesis is tested from the code, not assumed. A green re-run does not close the red. The acceptance evidence is a CI run on an exact cache hit that reads green, with the `test` job's coverage run passing every booted test.
@task_item n=12 part=needs
Each directive control names its `--timeout S` and fires with that flag on an entry filtered to its guard's own cases, each filter selecting at least one test. A control id is 16 characters at most. «Holds» means those cases ran and failed with no TIMEOUT line. Each acceptance predicate states the change that would turn it red.
@task_item n=13 part=needs
The standing live rig keeps owning only its own processes and is not reworked. Its census is the model for this chunk's census, not a component the chunk changes.
@task_standing n=1
convention: no retries and no flaky tests: every new test reads the same on every run, and a red is never closed by a re-run
home: 10-slo-invariants-a11y-budgets
@task_standing n=2
convention: the CI test gate allows no skipped tests, so no new test is ignored or skipped by environment
home: 1-a11y-scope-summary
@task_standing n=3
convention: new tests live inside the existing test suites and their failures are ordinary nextest failures
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=4
convention: the gate a new test joins requires the coverage run and the doctest run
home: ci-integration
@task_item n=1 part=not_needed
Colour, SGR, dim, bold, amber and NAME truncation decisions, and the `viola list` board and caption contracts. The chunk prints nothing to a user and changes no board.
@task_item n=2 part=not_needed
`viola --help`, `viola link` / `unlink`, `viola plugin install`, `viola revive` and `viola revive --list` output contracts. The chunk changes no verb's output.
@task_item n=3 part=not_needed
Refusal layout, hint, reason-code, exit-code and `--json` failure-document contracts, and their fixed-string pins. The chunk changes no refusal. It only makes the stamped home valid so a booted wrapper does not reach one.
@task_item n=4 part=not_needed
Screen-reader targets and setups, and the manual Orca and keyboard passes. These are human supplements that this fixture work does not touch.
@task_item n=5 part=not_needed
Terminal structural equivalents of ARIA roles, landmarks and linear static output, and the C0/C1 escaping of CLI output. The chunk changes no printed output.
@task_item n=6 part=not_needed
Structured violation JSON rows, and the absences of a web, graphical or touch frontend, internationalization and a rendered page. None is reached by test fixtures.
@task_item n=7 part=not_needed
The send-refusal, dialog-hold, wait-timeout and MCP tool contracts. The chunk changes no send, answer, wait or MCP behaviour.
