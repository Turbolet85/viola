@task_prose part=chunk
A corrective chunk mending the Epoch 5 audit's ranked defects: no product behaviour, flag, event kind, refusal code or master rule is added.
It moves the root crate's whole-verify test into the verify class's mutants kill, then makes ONE announced in-diff mutation run of the root crate, landed as an addendum with its caught, missed, timeout and unviable counts.
It adds killing tests, each with a remove-the-guard control, for viola-pty's nine restart-path survivors and viola-state's three survivors, the ancestor-walk bound first.
It makes r2-open.sh refuse before any start on a locked desktop or a scratch dir that is not fresh, proved by a check that runs no live arm.
It makes three pure moves and two de-duplications: one shared start prologue for revive and adopt, one refused() helper for open and up, and splits of upped, the PTY pump's restart code and run's window and trust steps; the whole layers and the smoke trio re-run green.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest profiles, the mutants profile's per-test kill lines and overrides — edits: the verify_window_ kill override (gains the whole-verify tests by name; the profile's general limit unchanged)
@task_file n=2 path=tests/cli_answer.rs mark=re-reads
what: the root crate's mutation baseline under the mutants profile, read back for no TIMEOUT on the custom-stamped-home dialog-rows verify test
@task_file n=3 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump, its child restart path — edits: the restart code (split out to its own module), the generation guard of which output copy ended, the reap_within and drain time limits (new killing tests)
@task_file n=4 path=crates/viola-state/src/liveness.rs mark=edits
what: instance liveness, the process ancestor walk — edits: ancestors(pid, max) and its chain-length bound (new killing tests)
@task_file n=5 path=src/cmd/revive.rs mark=edits
what: the viola revive verb, its start preflight — edits: the start prologue (strict-modes check before the log opens), made the shared function
@task_file n=6 path=src/cmd/adopt.rs mark=edits
what: the viola adopt verb — edits: its copy of the start prologue, replaced by a call to the shared one
@task_file n=7 path=src/cmd/up.rs mark=edits
what: the viola up verb — edits: the upped function (split for its complexity), the refused() helper (made the shared one)
@task_file n=8 path=src/cmd/open.rs mark=edits
what: the viola open verb — edits: the refused() helper, replaced by a call to the shared one
@task_file n=9 path=src/cmd/run.rs mark=edits
what: viola run start, on the boot path — edits: the window step and the trust step (split out of the file)
@task_file n=10 path=scripts/r2-open.sh mark=edits
what: the R2 live-rig script for viola open — edits: its preconditions before any start (desktop not locked, scratch dir fresh)
@task_item n=1 part=needs
The mutants nextest profile's kill lines: the whole-verify test and any other test that runs a whole viola verify move by name into the verify class's 20 s x 4 kill; the general 15 s kill is not raised, and every kill stays below cargo-mutants' own timeout.
@task_item n=2 part=needs
The one mutation run: the root crate's in-diff mutants from the recorded base, copy-target off, its own target and a short scratch TMPDIR, a kill deadline, run once behind an announced stop; its verdict read by counts (caught, missed, timeout, unviable) and host-excluded mutants recorded as such; its output never uploaded and no test home kept.
@task_item n=3 part=needs
The mutation gate's thresholds and run verdict that the addendum's score is read against, a baseline-test-failure no longer being the root crate's outcome.
@task_item n=4 part=needs
Killing tests for the PTY pump's restart path: the generation guard deciding which output copy ended, and the reap_within and drain time limits; child exit read on the process handle, real openpty spawns, time limits driven through the injected clock rather than sleeps.
@task_item n=5 part=needs
Killing tests for the liveness ancestor walk's bound, the security limit of 8 ancestor pids, and the other two viola-state survivors; process faults real, no mocked process table.
@task_item n=6 part=needs
Each killing test owes a bounded remove-the-guard witness: red with the guard removed, green with it, a one-off control and no mutation run of its own.
@task_item n=7 part=needs
The r2-open.sh refusal on a locked desktop (hyprctl locked) and on a scratch dir that is not fresh, read by a rehearsal or a real stub executable of hyprctl on a test PATH; the rehearsal reads the refusal's own effects and starts nothing.
@task_item n=8 part=needs
The restructure's invariant: the revive/adopt start prologue with its strict-modes check before any log open, the open/up refused() helper with its --json refusal shape, upped, the pump's restart code, and run's window and trust steps (run after the bind) each do what they did; held by the whole test layers green with no test edited beyond a re-pointed path or name, and for run's boot path by the smoke trio.
@task_item n=9 part=needs
The measure the split is read by: the audit's complexity ceiling and its over-800-lines size count, each moved and each remaining file under them; the duplication read no longer listing the refused() pair or the prologue copies.
@task_item n=10 part=needs
Where a moved unit may live: a new module of its own crate declared in the module tree (no orphan module), exposing no more than its callers need and keeping sync crates free of tokio.
@task_item n=11 part=needs
How tests reach a moved unit: inline unit tests move with it, integration tests keep reaching the public surface; an OS-gated case stays in its binary's root file.
@task_item n=12 part=needs
The declared boot paths of the project read by the phase's boot-path check, run's start being one of them.
@task_item n=13 part=needs
The chunk's tests and checks start no live wrapper, open no window, send no key, run no real claude and read nothing of the host's real claude sessions or viola home; every viola they start is tied to a test-owned home.
@task_standing n=1
convention: each new test function is named by the test-function naming rule and lives inline as a unit test or in a surface-prefixed integration binary, table cases in rstest
home: 2-test-strategy
@task_standing n=2
convention: each new test creates its own private home and directories, seeds its own data and never hand-writes product state
home: 7-test-data-fixtures
@task_standing n=3
convention: each new test waits on logged events or a bounded wait, never on sleep, and is never retried or parked with ignore
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: each new test asserts literal oracles on observable behaviour, never a Debug string or fixture internals
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: each new test fakes time only through the injected clock seam and fakes external processes only as real stub executables
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=6
convention: each new test counts toward the comprehensive coverage floors and passes the zero-flakiness budget under nextest
home: 10-quality-gates-coverage-targets · quality-gates-coverage-targets
@task_item n=1 part=not_needed
MCP server, tool, tools/list and MCP reply-screen contracts: no MCP surface is touched.
@task_item n=2 part=not_needed
Hook dialog, decision-body, answer mapping and dialog hold rows: no dialog or hook behaviour changes.
@task_item n=3 part=not_needed
Send confirmation, paste framing, wheel and release rows and the viola send contracts: nothing of send or the wheel is reached.
@task_item n=4 part=not_needed
Wrapper channel JSON-RPC error codes, link and unlink methods and channel wait contracts: the channel is not changed.
@task_item n=5 part=not_needed
viola list board and list --json contracts and the event-data key contracts: no list or event shape changes.
@task_item n=6 part=not_needed
Fuzzing, fuzz corpus replay and property-test generator rows: no parser surface is added or changed.
@task_item n=7 part=not_needed
Hook performance budgets, hyperfine invocation and perf CI job rows: no hook path is timed or changed.
@task_item n=8 part=not_needed
Harness session record, boot, status, cleanup and supervisor contracts and pre-push detail codes: no harness command is changed.
@task_item n=9 part=not_needed
Process log line fields, log event corr and log stream contracts: the chunk adds no log line.
@task_item n=10 part=not_needed
CI artifact names, retention, secret-scan hit classes and junit copies: no CI artifact or workflow is changed.
@task_item n=11 part=not_needed
Fake agent command-line options, receipts and turn-script contracts: no fake agent option is added.
@task_item n=12 part=not_needed
Absences of a database, outbound HTTP, a browser suite, a deploy stage and Windows or macOS support: nothing of the chunk nears them.
