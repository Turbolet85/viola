@task_prose part=session
The session split three oversized parts of the code as pure moves, with no user-visible change: the viola up verb's body, the PTY pump's respawn of the child, and viola run's window and trust steps.
viola up's body now selects, plans and acts in three private steps; its output lines, outcome words and dry-run are unchanged.
The pump's respawn moved into its own private module; the pump's loop takes one helper for its next request; paste, resize and respawn behaviour are unchanged.
The window step that opens and places a started instance's window, and the folder-trust step with its trust word, moved out of viola run's start into the window and trust-record modules; run, revive and adopt start as before.
One log line moved: the trust-record INFO line keeps its event and detail word, but its tracing target now reads viola::trust_record (was viola::cmd::run).
Measured: every touched file at or under 800 code lines, every function at or under cognitive 15, test case counts unchanged; no public API, IPC method, exit code, message or output form changed; nothing retired.
@task_file n=1 path=src/cmd/up.rs
what: the viola up verb: which sessions it opens, its plan, its outcome words, its line order and its --dry-run output
@task_file n=2 path=crates/viola-pty/src/pump.rs
what: the PTY pump that relays the terminal between the human and the child: keys, paste, resize and the child's output
@task_file n=3 path=crates/viola-pty/src/pump/restart.rs
what: the pump's respawn of the child (a swap of the child under a running pump, its reap bound and drain)
@task_file n=4 path=crates/viola-pty/src/pump/test_cases.rs
what: the pump's unit tests: output, resize, drain and the shared test rig
@task_file n=5 path=crates/viola-pty/src/pump/test_cases_paste.rs
what: the pump's paste tests: a paste written whole, a cut paste, human keys behind a paste
@task_file n=6 path=crates/viola-pty/src/pump/test_cases_respawn.rs
what: the pump's respawn tests and their helpers
@task_file n=7 path=src/cmd/run.rs
what: the viola run verb's start, the path shared by run, revive and adopt: its start refusals, the child started line and the automatic-verify line
@task_file n=8 path=src/cmd/run/test_cases.rs
what: viola run's unit tests: the wrapper's IPC methods (link, wait, last, hook, dialog, answer, pause, release) and the start's spans
@task_file n=9 path=src/cmd/run/test_cases_start.rs
what: viola run's start tests: the first snapshot's lines, the run arguments (origin, place, parent, role, model, effort) and the lineage
@task_file n=10 path=src/run/window.rs
what: the window step that opens and places a started instance's terminal window and gives its placement word
@task_file n=11 path=src/trust_record.rs
what: the trust record: the folder-trust step at start, its trust word and the trust-record line
@task_file n=12 path=src/cmd/mod.rs
what: the CLI verbs' module list
@task_item n=1 part=touches
The viola up output and outcome-word contracts (live, would-open, not-live, already-live and the rest, line order, --dry-run, not-live hint): the verb's body was split into select, plan and act steps; each output is meant to be unchanged.
@task_item n=2 part=touches
The viola run and revive start contracts (child started line, start refusals, automatic-verify line): the start lost its window and trust steps to other modules; run, revive and adopt booted green and their output is meant to be unchanged.
@task_item n=3 part=touches
The placement-word contracts (placed, unplaced, refused, unreachable, unsupported, window-unseen, malformed, oversize): the window step that yields them moved out of viola run's start, its body byte-equal.
@task_item n=4 part=touches
The viola verify trust-record row contract and any row on the trust word: the folder-trust step and its trust word moved into the trust-record module; the trust-record line's log target now reads viola::trust_record, with the same event and detail word.
@task_item n=5 part=touches
The paste-cut and paste-related refusal hints and the send readback rows that rest on the pump's paste: the pump's paste tests moved into their own file and its respawn into its own module; paste behaviour is meant to be unchanged.
@task_item n=6 part=touches
Rows on a respawned or revived child's strip and its stable session slot: the pump's respawn of the child moved, its bodies byte-equal and its reap bound unchanged.
@task_item n=7 part=touches
The wrapper's IPC method contracts as the CLI sees them (link and unlink output, wait and last output, pause and release, release-from-driver refusal): their tests moved into new test files under unchanged names; no method changed.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, depth, border radius and iconography: no colour, glyph, column, width or styling changed.
@task_item n=2 part=untouched
The anti-patterns and the self-validation protocol: no output form, message, hint or spinner rule was added or broken.
@task_item n=3 part=untouched
The web surface and rendered-page absences: nothing rendered was added.
@task_item n=4 part=untouched
The CLI exit codes, exit 21 causes and their --json causes: no code or cause was added, moved or retired.
@task_item n=5 part=untouched
The viola list board and table, viola focus, viola verify's step and summary lines, and the colour decision order: none of them was reached by the moves.
