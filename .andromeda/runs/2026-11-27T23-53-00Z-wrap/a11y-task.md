@task_prose part=session
The session split three oversized pieces of code as pure moves with no behaviour change: the up command's planner, the PTY pump's respawn, and the run start's window and trust steps.
The up command's top function now hands off to three private helpers (select, plan, act) and still opens the cli.up span; the pump loop takes one helper that reads the next respawn request.
The pump's respawn, its reap wait and its output drain moved into a private restart module of the pump crate, which still exports the reap bound, the Respawn outcome and the respawn handle under the same names.
The start's window step moved beside the window-step code, and the folder-trust step moved into the trust record module; the trust-record INFO log line keeps its event and detail word, but its tracing target now reads viola::trust_record (was viola::cmd::run).
The pump's and the run command's unit tests moved into separate files and new submodules (pump::tests::paste, pump::tests::respawn, cmd::run::tests::start; one case went to trust_record::tests), so those cases' test paths changed. Case counts did not change and no public API, IPC method, CLI output, exit code or config changed.
Measured: every file it touched is at or under 800 code lines, and every function it touched is at or under cognitive complexity 15.
@task_file n=1 path=src/cmd/up.rs
what: the viola up command, which plans and starts the fleet's instances and prints the instance line
@task_file n=2 path=crates/viola-pty/src/pump.rs
what: the TUI passthrough pump, which copies child output to the terminal, forwards human keystrokes, pastes and resizes, and ends the run with its pump_child outcome
@task_file n=3 path=crates/viola-pty/src/pump/restart.rs
what: the pump's respawn (a switch's child swap), the switch's reap wait under its 10 s bound, and the drain of the old child's output
@task_file n=4 path=crates/viola-pty/src/pump/test_cases.rs
what: the pump's unit tests for output copy, resize forwarding, exit drain and the human-keystroke writer
@task_file n=5 path=crates/viola-pty/src/pump/test_cases_paste.rs
what: the pump's paste tests: the paste bound, a cut paste closing its bracket, human keys waiting behind at most one chunk
@task_file n=6 path=crates/viola-pty/src/pump/test_cases_respawn.rs
what: the pump's respawn tests: the child swap, the reap bound, a failed spawn, the exit drain after a switch
@task_file n=7 path=src/cmd/run.rs
what: the viola run start, shared by run, revive and adopt, and its run diagnostics
@task_file n=8 path=src/cmd/run/test_cases.rs
what: the run command's unit tests for its IPC methods, the wheel's pause and release, hook events and the start spans
@task_file n=9 path=src/cmd/run/test_cases_start.rs
what: the run start's unit tests for the first snapshot's lines, run argument parsing (origin, place, parent, role) and lineage
@task_file n=10 path=src/run/window.rs
what: the window step of a start, which places the new window under its 5 s compositor wait and keeps the view
@task_file n=11 path=src/trust_record.rs
what: the folder-trust record a verified start writes, and its trust-record INFO log line
@task_file n=12 path=src/cmd/mod.rs
what: the CLI command module list
@task_item n=1 part=touches
decision/respawn-reap-bound: the switch's 10 s reap wait and its bound constant moved, unchanged, into the pump's new restart module; a guard cell naming the old pump test path for the reap case now reads pump::tests::respawn::respawn_reaps_a_child_that_exits_inside_the_bound.
@task_item n=2 part=touches
decision/window-wait-bound: the start's window step that sits under the 5 s wait moved, unchanged, from the run command into the window-step code; the bound and its behaviour did not change.
@task_item n=3 part=touches
Guards naming pump paste cases moved: each now reads pump::tests::paste::{name}, including pump::tests::paste::pump_returns_with_the_writer_blocked_on_a_child_that_never_reads; rows over the paste bound and human keys waiting behind one chunk (decision/human-keystrokes-never-blocked, decision/human-key-takes-the-wheel, contract/tui-passthrough/human-keystrokes) may name them.
@task_item n=4 part=touches
Guards naming pump respawn cases moved to pump::tests::respawn::{name} (child swap, a child ignoring the signal, a failed spawn, the exit drain, respawn without a running pump); rows over a switch's respawn or contract/tui-passthrough/pump-child-error may name them.
@task_item n=5 part=touches
Guards naming run start cases moved to cmd::run::tests::start::{name} (the first snapshot's lines, run argument parsing, lineage), and one trust case now reads under trust_record::tests::; rows over the start shared by run, revive and adopt may name them.
@task_item n=6 part=touches
The trust-record INFO log line's tracing target is now viola::trust_record (was viola::cmd::run); rows over log identity or run diagnostics (contract/log-identity/process, contract/log-identity/service-name, decision/run-diagnostics-off-terminal, contract/tui-passthrough/run-diagnostics) are checked against it, and none pins that target to a value.
@task_item n=7 part=touches
contract/tui-passthrough/viola-terminal-bytes and decision/zero-viola-bytes-on-the-terminal: the pump that writes the terminal had its loop split into a helper, with its bytes unchanged.
@task_item n=8 part=touches
decision/portable-pty-outer-pty-driver: the outer-PTY pump crate gained a private restart module; its exported names stay the same.
@task_item n=9 part=touches
contract/viola-up-output/instance-line: the up command's top function was split into select, plan and act, with its printed instance line and its cli.up span unchanged.
@task_item n=10 part=touches
decision/mutation-audit-covers-crate-invariants: the crate tests that hold the pump invariants now live in separate test files and submodules, with case counts unchanged (pump 33, crate 49).
@task_item n=1 part=untouched
CLI refusal and fault output (unable and hint lines, reason codes, exit codes, --json failure documents, fixed-message forms): no string, code or exit changed.
@task_item n=2 part=untouched
Colour and SGR rows (amber on DIALOG, bold NAME, dim stale rows, no SGR off a terminal) and the viola list board's captions, order and truncation.
@task_item n=3 part=untouched
Cognitive-accessibility decisions on reading level and AAA escalation: the session's complexity figures are code-complexity measures, not reading-level or cognitive-load claims.
@task_item n=4 part=untouched
CI integration and SLO rows (the coverage suite, no skips, no retries, the fail-fast gate): no gate, threshold or workflow changed.
@task_item n=5 part=untouched
Screen-reader targets and manual passes, absences (web frontend, touch surface, i18n, rendered page, user time limit), and the MCP, hook, link and frame-verb refusal contracts.
