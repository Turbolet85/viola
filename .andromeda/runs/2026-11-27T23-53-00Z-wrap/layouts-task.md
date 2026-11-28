@task_prose part=session
The session split three oversized parts of the code as pure moves, under the Epoch 5 audit's size and complexity ceilings: the up verb's body, the pump's respawn, and the run start's window and trust steps.
No public API, CLI verb, flag, exit code, refusal, hint, --json document, IPC method, env var or human output line changed; 159 moved bodies read equal.
The up verb's body now reads as three private steps (select, plan, act) under the same cli.up span; the pump's main loop takes one helper that picks the next respawn request.
The respawn code (the child swap, the bounded reap and the drain) moved to its own private module of the pty crate; the window step and the trust-folder record moved out of the run verb into the run window and trust record modules.
One log line moved module: the trust-record INFO line keeps its event and detail word, its tracing target now reads viola::trust_record (was viola::cmd::run).
Test case counts are unchanged; the moved tests keep their names in new test files.
@task_file n=1 path=src/cmd/up.rs
what: the viola up verb — the planning and starting of the named instances, its --dry-run plan and its --json output
@task_file n=2 path=crates/viola-pty/src/pump.rs
what: the pty pump — the loop that copies the child's output to the host, forwards keys, resizes and pastes, and ends on the child's exit
@task_file n=3 path=src/cmd/run.rs
what: the viola run verb and the start shared by run, revive and adopt — its preflight, its refusals and its passed start output
@task_file n=4 path=src/run/window.rs
what: the window step of a start — placing the new instance's window without moving the view
@task_file n=5 path=src/trust_record.rs
what: the trust record — the folder-trust check and the record written once per verified start, behind viola verify's untrusted-folder refusal
@task_file n=6 path=src/cmd/mod.rs
what: the CLI verb module list — the module boundary that holds each verb
@task_file n=7 path=crates/viola-pty/src/pump/restart.rs
what: the pump's respawn — swapping the child under a running pump, the bounded reap of the old child and the drain of its output
@task_file n=8 path=crates/viola-pty/src/pump/test_cases.rs
what: the pump's unit tests — exit, resize, drain and writer cases
@task_file n=9 path=crates/viola-pty/src/pump/test_cases_paste.rs
what: the pump's paste tests — the paste bound, a cut paste, human keys behind a paste, a paste refused before or after the pump
@task_file n=10 path=crates/viola-pty/src/pump/test_cases_respawn.rs
what: the pump's respawn tests — the child swap, the reap bound, a failed spawn and the drain after an exit
@task_file n=11 path=src/cmd/run/test_cases.rs
what: the run verb's unit tests — the instance's channel methods, hook events, wheel methods and the start spans
@task_file n=12 path=src/cmd/run/test_cases_start.rs
what: the run start's tests — the first snapshot's lines, the lineage, the hidden origin and place arguments, the launch pair refusal
@task_item n=1 part=touches
The viola up verb's contract (cli-verbs/up): its body was split into select, plan and act steps; the dry-run plan, the start order and the --json output are unchanged.
@task_item n=2 part=touches
The viola run start's passed start output and its already-live refusal: the start lost its window and trust steps to other modules, the output and refusals unchanged.
@task_item n=3 part=touches
The start refusals and preflight shared by run and revive (home tree refused, socket directory refused, squatted name, pinned hash mismatch, internal error, the revive preflight order): the shared start was reorganised, the order and the words unchanged.
@task_item n=4 part=touches
Window placement of a start (viola revive's window placement row and the run start's window step): the window step moved module, its behaviour and the view kept unchanged.
@task_item n=5 part=touches
viola verify's untrusted-folder refusal and the trust record behind a verified start: the trust-folder record moved into the trust record module; the record is still written once per verified start with the same word.
@task_item n=6 part=touches
viola switch output (switched, --json ok): the switch respawns the child through the pump's respawn, which moved to its own module with the same reap bound and the same outcomes.
@task_item n=7 part=touches
viola send's paste-cut hint and paste outcomes: the pump's paste path and its tests moved files, the paste bound and the cut behaviour unchanged.
@task_item n=8 part=touches
viola adopt's start (cli-verbs/adopt): it uses the same start shared with run and revive that was reorganised.
@task_item n=1 part=untouched
The board and BAY line of viola list, its columns, styling, truncation and colour depth: no list code changed.
@task_item n=2 part=untouched
CLI exit codes, the refusal line form, hints and the shared word column: no code, word or hint changed.
@task_item n=3 part=untouched
The viola mcp tools and their results, and the MCP-hidden verbs: no MCP surface changed.
@task_item n=4 part=untouched
viola wait, viola last, viola answer, link and unlink, pause and release output: no code behind them moved.
@task_item n=5 part=untouched
The token rendering rows, the readback mirror and vocabulary, plugin install and viola --help output: no rendering or help text changed.
@task_item n=6 part=untouched
The web surface absence and the deep-links absence: no capability added or removed.
