@task_prose part=chunk
Test fixtures and the fake agent only: a `viola run` wrapper a test boots, and the fake agent under it, end when that test ends — passed, failed, timed out or killed by SIGKILL — so no booted wrapper or fake agent is left running after a test run (requirement v2-37).
First step: the shared stamped test home is valid, or rebuilt, on any host where its home is absent, whatever stamp marker a restored build cache carries; a test fails before the fix and passes after it.
A proof of a killed test: a harness the test owns SIGKILLs a test process mid-run on the fake agent, then a census shows nothing of that test's own process tree still runs.
It re-measures from the code every place a test boots a wrapper, what ends it on each exit path, and why a killed test's wrapper spun; and it closes the CI red read after the last wrap (the shared stamped home found absent) against that run.
No product behaviour change is intended; a change `viola run` needs to end once its controlling test is gone is in scope only as P3 measures it. No live `claude` start and no mutation run.
@task_file n=1 path=tests/support/home.rs mark=edits
what: the shared stamped test home fixture every booted test reads — the stamp directory and the stamped home that carry the CLI verification stamps — edits: stamp_dir, stamped_home
@task_file n=2 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the end-to-end harness that boots a `viola run` wrapper over the fake agent on the shared stamped home, and owns the lifetime of what it boots
@task_item n=1 part=needs
The CLI verification state of the wrapped CLI: a booted wrapper refuses to start as unverified-cli when the stamped home is absent or unstamped, and is verified when the stamps hold; the fixture's rebuilt home must reach the verified state.
@task_item n=2 part=needs
The stamps written by `viola verify` (its stamps.json update and pinned hash) that the shared stamped home holds; a rebuilt home carries them as verify writes them, and a stamp marker is not trusted without its home present.
@task_item n=3 part=needs
`viola run` start refusals a booted test can meet (home-tree check, pinned-hash mismatch, already-live, stale heartbeat, squatted name, socket directory) and the unverified-cli refusal hint: a fixture that fails to stamp must surface as the refusal it is, not as a hung wrapper.
@task_item n=4 part=needs
`viola run` prints nothing while wrapped and the child holds the terminal; a test's end of the wrapper reads its exit, not any output it prints after the child started.
@task_item n=5 part=needs
The wrapper's lifetime against its controlling test: once the test, its PTY or its parent is gone, the wrapper and its fake agent end rather than spin; any change to the wrapper loop is bounded to what P3 measures.
@task_item n=6 part=needs
The fake agent's screens: a change to them puts the CLI verification check in the gate block, since verify reads those screens to stamp the CLI.
@task_item n=7 part=needs
CLI exit codes a booted wrapper ends with (0, 1, 2, 20, 21 and the 10 to 14 band), as a test reads them when it ends or kills the wrapper it owns.
@task_item n=8 part=needs
A test kills only processes it owns: the SIGKILL proof and the census act on the test's own process tree, never on another test's wrapper or the standing live rig.
@task_item n=9 part=needs
Every acceptance control names its timeout and fires filtered to its own cases, a control id within 16 characters; each predicate can fail and each filter selects at least one test; holds means those cases ran and failed with no timeout line.
@task_item n=10 part=needs
A test that waits on a channel or thread is bounded under the project's 20 s lint and cannot hang past a failure; a test socket lives under the short `/tmp` root.
@task_item n=11 part=needs
The CI red is closed by a CI run on an exact cache hit that reads green; a green re-run alone does not close it.
@task_standing n=1
convention: each new test's refusal and error text stays within the fixed refusal forms, with no internals, stack traces or paths in what a booted wrapper prints on stderr
home: surface-cli · surface
@task_standing n=2
convention: each new test reads a booted wrapper's data on stdout and its messages on stderr, and treats output as plain, uncoloured when stdout is not a terminal
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
Brand identity rows — strips, readback box, cocked dialog-pending strip, phraseology, expression level — the chunk changes no displayed expression.
@task_item n=2 part=not_needed
Colour palette, typography, spacing, iconography and glyph rows — no output styling, column, glyph or separator changes.
@task_item n=3 part=not_needed
The `viola list` board and table contracts, the fleet console and stable session slots — the chunk renders no board.
@task_item n=4 part=not_needed
Send readback and send mirror contracts, `viola wait`, `viola last`, `viola answer` output — the chunk sends no prompt and reads no turn.
@task_item n=5 part=not_needed
`viola link`/`unlink`, pause/release handoff, revive output and its --json refusal codes, peer reply screening and wrapper fault --json detail — no verb of these is booted or changed here.
@task_item n=6 part=not_needed
The web surface and rendered-page absences and the self-validation protocol's design tests — no page or design change.
