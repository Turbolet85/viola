@task_prose part=chunk
Makes every `viola run` wrapper a test boots, and the fake agent under it, end when that test ends — passed, failed, timed out or killed by SIGKILL — so no booted wrapper or fake agent outlives a test run (requirement v2-37).
First step: the shared stamped test home is valid, or is rebuilt, on any host where its home is absent, whatever a cache restored under target/; a test fails before the fix (stamp marker present, home absent) and passes after.
Adds a proof of killed: a harness the test owns SIGKILLs a test process mid-run on the fake agent, then a census shows nothing of that test's own process tree still running.
Re-measures from the code every place a test boots a wrapper, what ends that wrapper on each exit path today, and why a killed test's wrapper spun; tests the CI-red cause hypothesis (a restored stamp marker trusted while its home is gone) against ci#44048204364.
Fixtures and the fake agent only: no product behaviour change is intended; any change `viola run` needs to end once its controlling test is gone is in scope only as P3 measures it.
@task_file n=1 path=tests/support/home.rs mark=edits
what: the shared stamped test home fixture of the booted tests, the stamp marker and the home it stands for — edits: stamp_dir, stamped_home
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the end-to-end harness's wrapper boot over the shared stamped fixture home, the booted wrapper and fake agent a test owns
@task_item n=1 part=needs
The booted `viola run` wrapper and the fake agent under it, as test fixtures: their lifetime is bounded by the test that boots them on every exit path, SIGKILL included.
@task_item n=2 part=needs
The shared stamped fixture home: a stamp marker is trusted only where its home is present; otherwise the home is rebuilt, whatever a restored cache carries.
@task_item n=3 part=needs
The booted wrapper's refusal of an unverified CLI, which a booted test hits when the stamped home is absent: the fix must leave a booted wrapper reading a verified, stamped home.
@task_item n=4 part=needs
The `viola run <name> -- claude` start a test makes: any change to how the wrapper ends once its PTY or parent is gone is in scope only as P3 measures it, with no product behaviour change intended.
@task_item n=5 part=needs
The killed-test proof: a harness the test owns SIGKILLs a test process mid-run on the fake agent; no test sends a kill to a process it does not own.
@task_item n=6 part=needs
The end-of-proof process census over the test's own process tree, as the standing rig's census reads, showing nothing of it still running.
@task_item n=7 part=needs
A new test that waits on a channel or a thread takes a bound under the project's 20 s lint and cannot hang past a failure; a test socket goes under the short /tmp root.
@task_item n=8 part=needs
Each control names its timeout and fires with it on an entry filtered to its own cases, its id within 16 characters; every acceptance predicate can fail and each filter selects at least one test.
@task_item n=9 part=needs
A change to the fake agent's screens puts cli_verify in the gate block; a gate entry naming an installed tool's path is checked against the host first.
@task_item n=10 part=needs
Acceptance evidence for the CI red: a CI run on an exact cache hit that reads green; a green re-run alone does not close the red.
@task_item n=11 part=needs
No live claude start and no mutation run in this chunk.
@task_item n=1 part=not_needed
The web surface: viola renders no page and this chunk adds none.
@task_item n=2 part=not_needed
CLI styling and token rendering rows (SGR colour, token roles, spacing, motion, banner, spinner, glyph choices): the chunk changes no human output.
@task_item n=3 part=not_needed
The viola list board and its JSON document (columns, BAY line, truncation, stale rows, rack separator): not reached by test fixture lifetime.
@task_item n=4 part=not_needed
The viola send output, its readback mirror and its refusals before any request: no send behaviour changes.
@task_item n=5 part=not_needed
The viola mcp tools and their results: the chunk adds no tool and changes no result.
@task_item n=6 part=not_needed
The viola revive refusals, list rows and JSON error codes, and the link / unlink output: no revive or link behaviour is touched.
@task_item n=7 part=not_needed
The wheel verbs (pause, release), viola wait, viola last and viola answer output, and the help text: none is changed by this chunk.
