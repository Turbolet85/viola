@task_prose part=session
The session mended the code audit's ranked defects with no change to any CLI output, exit code, --json document, hint or refusal wording.
The start shared by viola run, viola revive and viola adopt now opens with one start prologue: the instance check, the home tree check (a --json strict-modes refusal from the caller, else the home tree refusal), then the wrapper log, in that order, unchanged.
The trust check of a name, once copied in revive and adopt, now stands once beside the start; the --list arm of viola revive reads the same one.
The refusal writer that viola up held as a byte-identical copy of viola open's is retired; viola up now writes its refusals through viola open's one, its four refusal sites unchanged.
Tests were added for the pty pump's drain and respawn, the parent-chain walk, the instance scan and the replay reading; the verify test class took six tests by name; the operator-fired live window script now refuses a locked desktop or a scratch that is not fresh.
The root crate's mutation reading went from a baseline failure to a score of 100.0, recorded as an addendum to the code audit.
@task_file n=1 path=.config/nextest.toml
what: the test runner's profiles: the verify-driven test class and its per-test kill timeouts in the ci and mutants profiles
@task_file n=2 path=tests/contract_lints.rs
what: the contract lint tests, pinning the verify test class's filter line in the test runner config
@task_file n=3 path=crates/viola-pty/src/pump.rs
what: the pty pump: copying the child's output to the host terminal, the drain bound at a child's exit, and the respawn of the child
@task_file n=4 path=crates/viola-state/src/liveness.rs
what: the liveness reading: the parent-chain walk of a process's ancestors used to find the terminal window of a session
@task_file n=5 path=crates/viola-state/src/board.rs
what: the board's state reading: the scan of checked instances in name order (tests only)
@task_file n=6 path=crates/viola-state/src/replay.rs
what: the replay reading of a session's events: whether a turn ended after the last start (tests only)
@task_file n=7 path=scripts/r2-open.sh
what: the operator-fired live check that viola open places a window on the desktop, now refusing before anything starts on a locked desktop or a scratch home not fresh
@task_file n=8 path=src/cmd/run.rs
what: the viola run verb and the start shared by run, revive and adopt: the trust check, the start prologue's preflight order (instance check, home tree check, wrapper log)
@task_file n=9 path=src/cmd/revive.rs
what: the viola revive verb: its start and its --list arm, now through the shared start prologue and trust check
@task_file n=10 path=src/cmd/adopt.rs
what: the viola adopt verb: its start, now through the shared start prologue and trust check
@task_file n=11 path=src/cmd/open.rs
what: the viola open verb and its refusal writer, now shared with viola up
@task_file n=12 path=src/cmd/up.rs
what: the viola up verb, its refusals now written through viola open's refusal writer
@task_file n=13 path=tests/contract_error_bodies.rs
what: the contract tests of refusal bodies, codes and exits per verb; the viola up rows now pointed at viola open's refusal text
@task_item n=1 part=touches
The preflight order of a viola revive start: the instance check, then the home tree check, then the wrapper log now stand in one shared start prologue; the order and each check are unchanged, so a row stating that order must still read true.
@task_item n=2 part=touches
The starts of viola run and viola revive refusing a home tree not private: the home tree refusal and, under --json, the strict-modes refusal are now chosen inside the shared prologue by the caller's strict-modes writer; wording, code and exit unchanged.
@task_item n=3 part=touches
The --json start refusals shared by viola run and viola revive (squatted-name, socket-dir-refused, pinned-hash-mismatch, internal-error): their start path is now the shared prologue; no document changed.
@task_item n=4 part=touches
viola adopt's start now runs through the same shared prologue as run and revive; a row on adopt's start or its refusals is reached through that shared start.
@task_item n=5 part=touches
viola revive --list and its refusals: the trust check it reads is now the one beside the shared start, byte-equal to the old copy.
@task_item n=6 part=touches
viola revive's strict-modes-failed --json code and its strict-modes refusal: written by the caller's strict-modes writer passed into the shared prologue, the same text as before.
@task_item n=7 part=touches
viola up's refusals: one refusal writer now serves both viola open and viola up; any row on the refusal line form or the shared refusal word column for these verbs is reached, its output unchanged.
@task_item n=8 part=touches
The CLI refusal line form and the hint-for-every-cause rule over viola open and viola up refusals: reached indirectly through the merged refusal writer; no cause, hint or wording moved.
@task_item n=9 part=touches
viola open's window placement and the --parent default: the live window check now refuses a locked desktop or a scratch home not fresh before it places anything; the product's placement is unchanged.
@task_item n=10 part=touches
viola verify's checks of the CLI version stamp and the dialog rows: six of their tests now sit in the verify test class by name with its longer kill timeout; verify's output and refusals unchanged.
@task_item n=11 part=touches
viola focus and viola open's search for a session's terminal through the parent-chain walk: the walk is now its own step; the 8-pid limit and the pid-1 stop are unchanged.
@task_item n=12 part=untouched
The board, viola list and its --json document, the BAY line, column widths, styling and token rendering: no output, width, colour or word changed; the board's scan gained tests only.
@task_item n=13 part=untouched
viola send, viola wait, viola last, viola answer, viola switch and the readback mirror: no send outcome, readback word, waiting line or hint changed; the pump's drain stays 500 ms.
@task_item n=14 part=untouched
viola link and unlink output, viola plugin install output, the viola mcp server and its tools: not reached.
@task_item n=15 part=untouched
The CLI exit codes, the exit-21 --json details, the frame verb hints and the internal error form: no code, detail or hint added, moved or removed.
@task_item n=16 part=untouched
viola --help and per-verb usage, global flags and the flat verb set: no verb, flag or usage line added or removed.
@task_item n=17 part=untouched
The web surface absence and the absence of deep links: no page, URL or link added.
