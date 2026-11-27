@task_prose part=session
A cleanup session with no new external surface: no IPC method, endpoint, export, env var, refusal code, output line or config key added or changed.
The start shared by viola run, viola revive start and viola adopt now goes through one start prologue: the instance check, then the strict-modes check (under --json the caller's strict-modes refusal document, else the human refusal), then the wrapper log is opened, in that order, the order unchanged.
The trusted-folder check is held once (it was copied in revive and adopt), and viola up's byte-identical copy of the refusal writer is retired so viola up and viola open write their refusals through one writer; the error-bodies witness rows for viola up's refusals now point at viola open's writer, codes, exits and witnesses otherwise unchanged.
The TUI passthrough pump's exit drain bound became a test seam (the product keeps 500 ms); new crate tests pin the pump's flush after each copy, the respawn reap inside its bound and the exit drain; new state-crate tests pin the parent-chain walk's stop at pid 1, the board scan's name order and a replay reading.
The verify-driven test class in the CI and mutation profiles takes six more tests by name (a dialog-rows path test and five CLI version-gate tests), its 20 s x 4 kill unchanged; the root crate's mutation reading moved from baseline-test-failure to 100.0 (326 caught, 0 missed).
The operator-fired live window-placement script (R2) now refuses with a red precondition verdict, before anything starts, on a locked desktop or a scratch home that is not fresh.
@task_file n=1 path=.config/nextest.toml
what: the test runner's profiles - the CI and mutation profiles' kill timeouts and the verify-driven test class's membership
@task_file n=2 path=tests/contract_lints.rs
what: the contract lints suite - a pin on the verify-driven test class's filter line
@task_file n=3 path=crates/viola-pty/src/pump.rs
what: the TUI passthrough pump - copies the child's output to the human's terminal, the respawn on a switch with its reap wait, the exit drain; its tests
@task_file n=4 path=crates/viola-state/src/liveness.rs
what: the instance liveness read - the parent-process chain walk (8-pid limit at the callers, stop at pid 1) used by viola focus and viola open
@task_file n=5 path=crates/viola-state/src/board.rs
what: the board's instance scan behind the viola list board - tests only
@task_file n=6 path=crates/viola-state/src/replay.rs
what: the event-record replay (start and last-event readings) - tests only
@task_file n=7 path=scripts/r2-open.sh
what: the operator-fired live window-placement reading (R2) on Hyprland - a test-only script, its verdict line
@task_file n=8 path=src/cmd/run.rs
what: viola run - the start shared by run, revive start and adopt; now holds the trusted-folder check and the one start prologue (instance check, strict-modes check, wrapper log)
@task_file n=9 path=src/cmd/revive.rs
what: viola revive - its start path and viola revive --list, both using the shared trusted-folder check and start prologue
@task_file n=10 path=src/cmd/adopt.rs
what: viola adopt - its start path through the shared start prologue
@task_file n=11 path=src/cmd/open.rs
what: viola open - its refusal writer, now the one writer of viola open's and viola up's refusals
@task_file n=12 path=src/cmd/up.rs
what: viola up - its refusals now written through viola open's refusal writer, its own copy removed
@task_file n=13 path=tests/contract_error_bodies.rs
what: the error-bodies witness list - each refusal body's writer and witness; viola up's rows re-pointed to viola open's writer
@task_item n=1 part=touches
The rule that the strict-modes refusal comes before any write (and that an unstattable home is a strict-modes refusal): the check now runs inside one shared start prologue for run, revive start and adopt, before the wrapper log is opened, order unchanged.
@task_item n=2 part=touches
The start refusals of viola run, revive start and verify (home strict-modes failure) and the run or revive start --json refusal document: their writer path moved into the shared start prologue, bodies unchanged.
@task_item n=3 part=touches
The viola revive --json refusals strict-modes-failed and the revive --json start refusal: revive's strict-modes refusal writer is now passed into the shared prologue, the document unchanged.
@task_item n=4 part=touches
The start-against-a-holder refusals already-live and stale-heartbeat: the instance check that raises them now stands first in the shared start prologue.
@task_item n=5 part=touches
The rule that error bodies have a writer and a witness: viola up's refusal rows in the witness list now name viola open's writer, the up copy retired; codes, exits and witnesses unchanged.
@task_item n=6 part=touches
The CLI refusal format, unable line and hint line as written for viola up and viola open: one writer now writes both commands' refusals, byte-identical to before.
@task_item n=7 part=touches
viola revive --list human rows and --json output: the trusted-folder check they use is now the one shared check, moved byte-equal.
@task_item n=8 part=touches
A switch's respawn reap wait bounded at 10 s and the rule that time bounds fall on the driver only: the pump's exit drain bound is now a seam, the product's drain kept at 500 ms, with new tests on the reap inside its bound.
@task_item n=9 part=touches
The TUI passthrough contracts on viola terminal bytes and own bytes, and the rule of zero viola bytes on the terminal: the pump's flush after each copy and its exit drain now pinned by crate tests; no byte added to the terminal.
@task_item n=10 part=touches
The rule that the mutation audit covers invariants held in crate tests: the root crate's mutation reading is now 100.0 (was baseline-test-failure), and the pump's and the state crate's survivors are caught by new crate tests.
@task_item n=11 part=touches
The zero-flakiness and no-skips CI gate rules: the verify-driven test class in the CI and mutation profiles takes six tests by name beside its prefix, its kill timeout 20 s x 4 unchanged, the mutation default 5 s x 3 unchanged.
@task_item n=12 part=touches
The rule that live readings never gate: the operator-fired live window-placement script now refuses with a red precondition verdict on a locked desktop or a non-fresh scratch before it starts anything; it still gates nothing in CI.
@task_item n=13 part=touches
The viola list board structure and order: the board's instance scan in name order now pinned by a crate test, the board unchanged.
@task_fact n=1 slug=cognitive-accessibility
fact: viola run, viola revive start and viola adopt share one start prologue that checks the instance first, then strict modes (the --json caller's own strict-modes refusal document, else the human refusal), and only then opens the wrapper log - adopt now holds the same refusal order as run and revive.
@task_fact n=2 slug=cognitive-accessibility
fact: viola up and viola open write their refusals through one refusal writer, and the error-bodies witness rows for viola up's refusals name that writer.
@task_item n=1 part=untouched
Colour and SGR decisions (amber DIALOG word, bold NAME values, dim stale rows, no SGR off a terminal, failure words never coloured): no output styling changed.
@task_item n=2 part=untouched
CLI exit codes and the code word format: no code, exit or code word added, renamed or moved.
@task_item n=3 part=untouched
viola --help rows, group labels and usage line, and board captions: no help or caption text changed.
@task_item n=4 part=untouched
Screen-reader targets and the manual Orca and keyboard passes: no reading target or manual pass changed.
@task_item n=5 part=untouched
MCP server frame tools, the MCP wait tool, hook event and dialog calls, and frame verb refusals: no MCP or hook surface touched.
@task_item n=6 part=untouched
viola link and unlink, viola plugin install, viola send refusals and viola wait output: none of their writers or lines changed.
@task_item n=7 part=untouched
The absences (internationalization, touch surface, web or graphical frontend, rendered page, user time limit) and the N/A web harness areas: no capability added.
@task_item n=8 part=untouched
The window step's 5 s wait, the 5 s compositor read bound and the dialog hold: no bound's value changed.
