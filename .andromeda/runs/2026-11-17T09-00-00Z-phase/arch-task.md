@task_prose part=chunk
Makes a `viola run` wrapper booted by a test, and the fake agent under it, end when that test ends — passed, failed, timed out or SIGKILLed — so no booted wrapper or fake agent outlives a test run (requirement v2-37).
First step: the shared stamped test home is valid, or rebuilt, wherever its home is absent whatever a restored `target/` cache holds, proven by a test red before the fix (marker present, home absent) and green after.
Adds a proof of «killed»: a test-owned harness SIGKILLs a test process mid-run on the fake agent and a census at its end shows nothing of that test's process tree still running.
Re-measures from the code every place a test boots a wrapper, what ends it on each exit path today, and why a killed test's wrapper spun; closes the folded CI red (ci#44048204364) against that run.
Fixtures and the fake agent only; a `viola run` change to end once its PTY or parent is gone is in scope only as P3 measures it.
@task_file n=1 path=tests/support/home.rs mark=edits
what: the shared stamped test home of the root tests — the fixture home stamped once by verify with its pinned copy and ledger stamps, behind the e2e-home link — edits: `stamp_dir`, `stamped_home`
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the end-to-end harness's boot of a `viola run` wrapper over the fake agent in a stamped test home, in the test-only viola-e2e crate
@task_item n=1 part=needs
A wrapper a test boots, and its fake agent child, end when the test ends on every exit path: pass, fail, timeout and SIGKILL (which runs no destructor); after a test run no booted wrapper or fake agent is left running.
@task_item n=2 part=needs
The shared stamped fixture home trusts a stamp marker only when the home it names is present: a marker whose home is absent (a `target/` cache restored with the home under the runner's `/tmp` gone through the `target/e2e-home` link) leads to a rebuild, never to reads that fail NotFound on the stamps or the pinned copy.
@task_item n=3 part=needs
A booted wrapper over a fixture home with absent or untrusted stamps reads the CLI unverified and refuses with unverified-cli; the fixture must hand every booted test a home whose stamps verify wrote and whose pinned copy is present and re-hashes.
@task_item n=4 part=needs
The stamped home's stamps come only from verify as the one stamps writer, recorded only from a clean verify; the rebuild path re-runs that, never writes stamps by hand.
@task_item n=5 part=needs
The killed proof: a harness the test owns sends SIGKILL to a test process mid-run on the fake agent, then a census of that test's own process tree, as the standing rig's census does, shows nothing still running.
@task_item n=6 part=needs
No test sends a kill or a stop to a process it does not own; root tests that freeze a wrapper keep doing so through their one SIGSTOP guard.
@task_item n=7 part=needs
A possible change to the `viola run` wrapper's loop so it ends once its PTY, its child or its parent is gone (and stops spinning), only as P3's re-measure finds it needed: child exit read on the process handle, run writing only the child's output to the terminal.
@task_item n=8 part=needs
Test homes are removed only by their owner record and test-side removals stay outside the working directory; evidence a killed test leaves (its kept home, its wait report) survives the kill.
@task_item n=9 part=needs
The linked e2e-home base stays a private real directory checked before use, with its headroom floor; dev-host test homes on the tmpfs link.
@task_item n=10 part=needs
Every test wait on a channel or a thread takes a bound under the project's 20 s lint and the one shared root wait bound, and none can hang past a failure.
@task_item n=11 part=needs
A test channel socket goes under the short `/tmp` root, outside the product endpoint namespace.
@task_item n=12 part=needs
Tests run against the fake agent only: no live `claude` start, none in CI; any fake-agent seam it adds is a named test seam, never a configuration channel, never read by the product.
@task_item n=13 part=needs
A change to the fake agent's screens puts `cli_verify` in the gate block; the fake-agent feature stays on its named member crates and out of product and release builds.
@task_item n=14 part=needs
Each directive control names its `--timeout S` and fires with that flag on an entry filtered to its own cases, its id within 16 characters; «holds» means those cases ran and failed with no TIMEOUT line; each acceptance predicate says what change turns it red and each filter selects at least one test.
@task_item n=15 part=needs
Acceptance of the CI red: a CI run on an exact cache hit reads green; a green re-run alone does not close it; no mutation run.
@task_item n=16 part=needs
End-to-end runs keep zero panic lines and the coverage run's floors hold with the new tests in it.
@task_standing n=1
convention: each new test runs in its own viola home
home: established-decisions
@task_standing n=2
convention: each new test's channel or thread wait is bounded under the shared root wait bound and the 20 s lint
home: occupied-resources
@task_standing n=3
convention: each new test socket goes under the short `/tmp` root, outside the product namespace
home: occupied-resources
@task_standing n=4
convention: each new harness knob carries the AGENT_RUN_ prefix and each new fake-agent seam follows the fake-agent seam naming
home: conventions
@task_standing n=5
convention: each new OS-gated test names its reason and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=6
convention: each new end-to-end test leaves zero panic lines
home: project-directory-structure
@task_item n=1 part=not_needed
MCP server rows (its tools, results, JSON-RPC errors, stdin caps): no MCP behaviour is changed or proven.
@task_item n=2 part=not_needed
Hook contracts and the hook-to-kind map, dialog holds and answers: the chunk changes no hook or dialog behaviour.
@task_item n=3 part=not_needed
Wheel decisions (byte-source claim, release, human take): untouched by fixture lifetime work.
@task_item n=4 part=not_needed
Channel method contracts for send, wait, last, answer, link and unlink, and their refusal details: no channel verb changes.
@task_item n=5 part=not_needed
The board and list payloads, agents listing and session liveness states: not read or changed.
@task_item n=6 part=not_needed
Revive rows (its refusals, JSON error codes, resume in place): revive is not touched.
@task_item n=7 part=not_needed
The Hyprland seam and window features: no compositor work.
@task_item n=8 part=not_needed
Plugin install and the embedded plugin's marketplace rows: not touched.
@task_item n=9 part=not_needed
Release tooling, dist and crates.io publishing rows: no release work.
@task_item n=10 part=not_needed
Fuzz workspace and weekly fuzzing rows: no fuzz target changes.
@task_item n=11 part=not_needed
Event-data per kind and normalised event kinds: no event kind or payload is added or changed.
