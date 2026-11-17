@task_prose part=chunk
A `viola run` wrapper a test boots, and the fake agent under it, end when that test ends: passed, failed, timed out or killed by SIGKILL; after a test run no booted wrapper or fake agent is left running (requirement v2-37).
First step: the shared stamped test home is valid, or is rebuilt, on any host where its home is absent, whatever a cache restored under target/; a test fails before the fix (stamp marker present, home absent) and passes after it.
A proof of killed: a harness the test owns SIGKILLs a test process mid-run on the fake agent, then a census at its end shows nothing of that test's own process tree still running.
Re-measures from the code every place a test boots a wrapper, what ends that wrapper today on each exit path, and why a killed test's wrapper spun rather than idled; re-reads the CI red of the shared stamped home on run ci#44048204364 and closes it against that run.
@task_file n=1 path=tests/support/home.rs mark=edits
what: the shared stamped test home fixture of the root integration tests — the stamp marker and the stamped home every booted test reads — edits: stamp_dir, stamped_home
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the E2E harness boot of the test-only E2E crate — makes the test home, stamps it by verify against the fake agent and starts the wrappers
@task_item n=1 part=needs
A booted wrapper does not outlive its test: on every exit path of a test that boots `viola run` (pass, fail, timeout, SIGKILL with no destructor run) the wrapper and its fake agent end; a drop guard alone cannot hold the SIGKILL path.
@task_item n=2 part=needs
The stamped home's validity: a stamp marker is trusted only when the home it stamps is present; where the home under the target/e2e-home link (under the runner's /tmp) is absent, the fixture rebuilds it instead of reading a stale marker restored from a CI cache.
@task_item n=3 part=needs
A regression test for the stamped home: a stamp marker present with its home absent fails before the fix and passes after it.
@task_item n=4 part=needs
The killed proof: a test-owned harness SIGKILLs a test process mid-run on the fake agent and a census of that test's own process tree at its end finds nothing still running, as the standing live rig's census does.
@task_item n=5 part=needs
Process hygiene of kills: no test sends a kill to a process it does not own; a kill targets only the test's own process tree, verified by pid and start time before it is sent.
@task_item n=6 part=needs
The fake agent's children and process group: how the fake agent and the wrapper's child sit in process groups and how they exit when their parent or PTY is gone, as fixtures may change them.
@task_item n=7 part=needs
The wrapper's end once its controlling test is gone: a `viola run` change to end when its PTY or parent is gone is in scope only as P3 measures it; otherwise fixtures and the fake agent only, no product behaviour change.
@task_item n=8 part=needs
How a test boots a wrapper: the shared fixtures and any test that spawns a wrapper itself, the harness boot that makes, stamps and starts, and the cleanup that stops agents and removes the home.
@task_item n=9 part=needs
Stamps are written only by verify against the fake agent, one verify stamp per build; the rebuild of an absent home goes through that same path, never a hand-written stamp.
@task_item n=10 part=needs
Test homes live under target/e2e-home, whose base may be a link to /tmp; the deletion of a home through that link and the checks of the base bind the rebuild.
@task_item n=11 part=needs
CI's only cache is rust-cache over target/; the acceptance evidence is a CI run on an exact cache hit that reads green — a green re-run alone does not close the red.
@task_item n=12 part=needs
Directive controls: each control names its --timeout S and fires with that flag on an entry filtered to its own cases, a control id within 16 characters; holds means those cases ran and failed with no TIMEOUT line; every acceptance predicate can fail and each filter selects at least one test.
@task_item n=13 part=needs
Bounded waits: a test waiting on a channel, a thread or a process exit takes a bound under the project's 20 s lint, root waits bounded below the per-test kill lines, and none can hang past a failure.
@task_item n=14 part=needs
A test socket goes under the short /tmp root, in a test endpoint directory made private.
@task_item n=15 part=needs
A change to the fake agent's screens puts cli_verify in the gate block; a gate entry naming an installed tool's path is checked against the host before the card.
@task_item n=16 part=needs
Child exit is read on the process handle, never on reader EOF, in any test or fixture that waits for a wrapper or fake agent to end.
@task_item n=17 part=needs
No live claude start and no mutation run in this chunk; real claude stays local-only and never in CI.
@task_item n=18 part=needs
A killed test child is uninstrumented: a test process killed by the proof leaves no coverage profile the coverage run must count.
@task_standing n=1
convention: each new test waits on logged events or file-state probes, never on sleep, with no retry
home: 11-test-anti-patterns-never-do-these
@task_standing n=2
convention: each new test creates its own fresh private home and shares no mutable state with tests run in parallel
home: test-data-fixtures
@task_standing n=3
convention: each new test function follows the project's test function naming and its root integration binary the surface prefix
home: 2-test-strategy
@task_standing n=4
convention: each new test that starts a long-lived wrapper holds it in a drop guard
home: integration-test-strategy
@task_standing n=5
convention: each new test that clears the child environment re-adds LLVM_PROFILE_FILE
home: 6-e2e-test-strategy
@task_standing n=6
convention: each new OS-gated test is registered with its reason
home: test-strategy
@task_item n=1 part=not_needed
MCP server, tool list, tool result and invalid-params contracts: the chunk changes no MCP surface.
@task_item n=2 part=not_needed
Send, answer, release and wait verb contracts and their confirmation windows: no verb behaviour changes.
@task_item n=3 part=not_needed
Revive refusal codes, exit codes and its human and JSON output: revive is not touched.
@task_item n=4 part=not_needed
The list board, its JSON items and its control-character escaping: not reached.
@task_item n=5 part=not_needed
Hook decision bodies, dialog events and hook perf budgets: no hook or dialog behaviour changes.
@task_item n=6 part=not_needed
Fuzz corpus replay, nightly fuzzing and property-test generators over parser surfaces: no parser changes.
@task_item n=7 part=not_needed
Pre-push stages, reasons and detail codes: the local pre-push gate is not changed.
@task_item n=8 part=not_needed
Channel error bodies, JSON-RPC error codes and the link and unlink methods of the wrapper channel: the channel is not changed.
