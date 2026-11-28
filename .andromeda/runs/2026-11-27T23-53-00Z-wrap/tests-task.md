@task_prose part=session
The session split three oversized parts as pure moves under the Epoch 5 audit's size and complexity ceilings: viola up's planning function, the PTY pump's respawn path, and viola run start's window and trust steps.
No public API, IPC method, endpoint, env var, exit code or event changed; 159 moved bodies read equal, and every touched file now stands at or under 800 code lines and every function at or under cognitive 15.
upped became three private steps (select, plan, act) and keeps the cli.up span; the pump loop took one helper (next_request); the respawn code moved to its own private module, re-exported unchanged.
The pump's and viola run's unit-test modules now live in sibling path files with their module paths kept, the paste and respawn cases and the start cases each in their own child module; test case counts are unchanged.
One log line moved: the trust-record INFO line is still written once per verified start with the same event and detail word, but its tracing target now reads viola::trust_record (was viola::cmd::run).
@task_file n=1 path=src/cmd/up.rs
what: viola up, the fleet bring-up verb: upped and its new private select, plan and act steps under the cli.up span, and its unit tests
@task_file n=2 path=crates/viola-pty/src/pump.rs
what: the PTY pump (viola-pty): the pump loop Pumping::run with its new next_request helper, child exit read on the handle, resize forwarding, paste and human input through the shared writer, and the re-export of Respawn, RespawnHandle and RESPAWN_REAP_BOUND
@task_file n=3 path=crates/viola-pty/src/pump/restart.rs
what: the pump's respawn of its child (new private module): the respawn request and handle, the child swap, reaping the old child within its bound, and draining the old output workers
@task_file n=4 path=crates/viola-pty/src/pump/test_cases.rs
what: the pump's unit tests (pump::tests), moved out of the pump into a path file: exit on the handle, worker panic kill, resize, output drain after exit, enqueue and writer deadlines, and the paste test rig helpers
@task_file n=5 path=crates/viola-pty/src/pump/test_cases_paste.rs
what: the pump's paste unit tests (pump::tests::paste): paste bounds, cut paste framing, human keys during and after a paste, refusals before start and after return, a writer blocked on a child that never reads
@task_file n=6 path=crates/viola-pty/src/pump/test_cases_respawn.rs
what: the pump's respawn unit tests (pump::tests::respawn) with their mock-child helpers: child swap, a child ignoring the signal, a failed spawn, reaping inside the bound, draining the new child's output, flushing host output
@task_file n=7 path=src/cmd/run.rs
what: viola run start, the wrapper start shared by run, revive and adopt: argument parsing, readiness checks, the start's event lines, the child spawn and the pump end
@task_file n=8 path=src/cmd/run/test_cases.rs
what: viola run's unit tests (cmd::run::tests), moved into a path file: the wrapper channel method routing, wheel pause and release, hook event appends and checks, the collision check over a replayed snapshot, the run.start spans
@task_file n=9 path=src/cmd/run/test_cases_start.rs
what: viola run's start unit tests (cmd::run::tests::start): links and cwd in the first snapshot, the order of the wheel, lineage, launch and window lines, run argument parsing of origin, place, parent, role, model and effort, lineage from flag or inherited name
@task_file n=10 path=src/run/window.rs
what: the window step of a start: open_window, moved here from viola run, places the window and keeps the view through the Hyprland requests
@task_file n=11 path=src/trust_record.rs
what: the folder trust record: trust_folder and trust_word, moved here from viola run, write the trust-record line after the bind on a verified CLI and read no file on an unverified one
@task_file n=12 path=src/cmd/mod.rs
what: the CLI verb module list; viola run's module widened to crate visibility so the window step can name the window request
@task_item n=1 part=touches
The process log line's target field: the trust-record line's target value moved from viola::cmd::run to viola::trust_record; it is still the Rust module path and still starts viola, so a row pinning the target field reads true but is touched.
@task_item n=2 part=touches
viola run's verify and trust steps after the bind: the trust step and the window step moved out of viola run into their own modules; order and behaviour are unchanged.
@task_item n=3 part=touches
Inline unit tests: the pump's and viola run's test modules are no longer inline in their source file but in sibling path files that keep the same module paths.
@task_item n=4 part=touches
OS-gated cases stay in a split binary's root file, and one shared copy of each test helper: unit-test modules were split into child modules, and the respawn helpers stand once beside their only users rather than in the root test file.
@task_item n=5 part=touches
Test function naming: every moved case keeps its name; one pump_ case now reads under pump::tests::paste, and case counts are unchanged.
@task_item n=6 part=touches
Child exit read on the process handle and PTY exit from wait, never EOF: the pump loop was restructured (next_request helper, respawn moved out) with the same exit behaviour.
@task_item n=7 part=touches
Pump resize baseline is the spawn size, key after a resize waits for the new size: the pump that holds them was split; behaviour unchanged.
@task_item n=8 part=touches
Paste injection test layers and the blocked-paste send contract: the paste unit cases moved into their own pump test module.
@task_item n=9 part=touches
Mutants tested by their own package and mutation base is the whole chunk: every moved function keeps its tests in its own package (viola-pty or the root crate).
@task_item n=10 part=touches
No orphan modules and the on-demand acyclic boundary review: new modules were added (the respawn module, two pump test modules, the start test module) and a sibling module tree now reaches into viola run through a widened module boundary.
@task_item n=11 part=touches
Real window placement proven only live and the window step keeping the view: the window step moved modules; its unit cases still run.
@task_item n=12 part=touches
Scoped log captures through a pinned dispatch and the run.start span tests: the span and line captures moved with viola run's test module.
@task_item n=13 part=touches
viola up's planning: upped split into select, plan and act, still the verb's sole callee and still under the cli.up span.
@task_fact n=1 slug=test-strategy
fact: A unit-test module too large for one file is split into sibling path files that keep its module path, its cases grouped into child modules, each helper standing once beside its only users.
@task_item n=1 part=untouched
CLI and MCP contracts: exit codes, JSON documents, human lines, refusal words and tool results did not change.
@task_item n=2 part=untouched
Wrapper channel JSON-RPC errors, link methods, wait and last: no method, error code or payload changed.
@task_item n=3 part=untouched
CI integration rows: workflows, artifacts, supply-chain gates, toolchain pins and caches.
@task_item n=4 part=untouched
Coverage floors, perf budgets, hook deadlines and mutation thresholds: no threshold, floor or bound moved.
@task_item n=5 part=untouched
The test harness contract: agent-run boot, status, cleanup, gate, pre-push and session record shapes.
@task_item n=6 part=untouched
Fixtures and the fake agent: recorded payloads, fake-agent command line, receipts and hygiene walks.
@task_item n=7 part=untouched
Security controls: strict modes, socket directory checks, server identity, peer euid, env strip and pinned exe hash.
@task_item n=8 part=untouched
Absences: database, outbound HTTP, browser suite, deploy stage, Windows and macOS.
