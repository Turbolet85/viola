@task_prose part=session
An internal cleanup chunk: no verb, output line, exit code, refusal word, hint, colour, glyph or JSON shape was added or changed.
The checks that open viola run, viola revive and viola adopt became one shared start prologue (trust of the name, the home-tree check refusing under --json through the caller's strict-modes writer, then the wrapper log), kept in the same order and the same refusals.
The refusal helper that viola up and viola open each held as a byte-identical copy became one, owned by open and imported by up; its call sites and output are unchanged.
Tests were added that kill mutation survivors in the PTY pump's drain and respawn path and in the instance-state liveness walk, board scan and replay reading; the test runner's verify kill class took six tests by name.
The live window-placement script (test-only, operator-fired) gained a refusal verdict line, r2: red precondition=scratch or locked, before it makes or starts anything.
The root crate's mutation reading moved from baseline-test-failure to a score of 100.0, recorded as an addendum to the code audit.
@task_file n=1 path=.config/nextest.toml
what: the test runner's profiles and per-test kill classes (the verify class and its timeouts in the ci and mutants profiles)
@task_file n=2 path=tests/contract_lints.rs
what: the contract lint tests that pin the test runner's verify-class filter line
@task_file n=3 path=crates/viola-pty/src/pump.rs
what: the PTY pump that copies the child's output to the host terminal, drains it at the child's exit and respawns the child
@task_file n=4 path=crates/viola-state/src/liveness.rs
what: the liveness check that walks a process's parent chain (the ancestors used to find the terminal a session runs in)
@task_file n=5 path=crates/viola-state/src/board.rs
what: the board's instance scan behind viola list (tests only)
@task_file n=6 path=crates/viola-state/src/replay.rs
what: the event-log replay that reads a session's state from its log (tests only)
@task_file n=7 path=scripts/r2-open.sh
what: the live window-placement check script (test-only, operator-fired) and its verdict lines
@task_file n=8 path=src/cmd/run.rs
what: the viola run verb and the shared start of run, revive and adopt (now holding the trust check and the start prologue)
@task_file n=9 path=src/cmd/revive.rs
what: the viola revive verb, its start refusals and its --list listing
@task_file n=10 path=src/cmd/adopt.rs
what: the viola adopt verb and its start refusals
@task_file n=11 path=src/cmd/open.rs
what: the viola open verb and its refusal line, now the one refusal helper shared with viola up
@task_file n=12 path=src/cmd/up.rs
what: the viola up verb and its outcome lines, now using open's refusal helper
@task_file n=13 path=tests/contract_error_bodies.rs
what: the contract tests that pin each verb's refusal and error bodies, their codes and exits
@task_item n=1 part=touches
The start-refusal order of viola run, viola revive and viola adopt (trust, then the home-tree check, then the wrapper log): rows stating that order or the refusals it yields now rest on one shared prologue; the order and the words are kept, so a row stays true unless it says each verb runs its own checks.
@task_item n=2 part=touches
The strict-modes refusal of revive and adopt before any log is opened: the shared prologue writes it under --json through each caller's strict-modes writer and refuses the home tree otherwise; rows on the strict-modes refusal, its --json document, exit 21 and the strict-modes-no-log refusal rule over this path.
@task_item n=3 part=touches
The home-tree check refusals of viola run and viola revive (and adopt): the check moved into the shared prologue unchanged; rows naming the home-tree check refusal rule over it.
@task_item n=4 part=touches
Start refusals show no path or pid, and the fixed start-refusal message of run and revive: the refusal path was reshaped, not reworded; a rule over these refusals now rules over the one prologue.
@task_item n=5 part=touches
The viola up and viola open refusal line: the two copies of the refusal helper became one; rows on the up outcome words, the up output line order, the refusal line form and its hint rule over the one helper, the words unchanged.
@task_item n=6 part=touches
The contract tests of refusal bodies no longer pin up's refusal under a constant of its own; its four rows now point at open's refusal, codes, exits and witnesses unchanged; a row whose guard names the up refusal constant is stale.
@task_item n=7 part=touches
The trust record of a session name (trusted or not): its check moved into the shared start and is used by revive's --list; rows on revive --list output and the trust-record row rule over it, behaviour unchanged.
@task_item n=8 part=touches
The viola run and revive output after the child starts and the rule that no document follows once the child held the terminal: the PTY pump's drain at the child's exit now takes its bound from the pump's seams, the product bound still 500 ms; behaviour unchanged.
@task_item n=9 part=touches
The terminal-finding behind viola focus and viola open: the ancestors walk's loop moved into its own function, the 8-process limit kept at the callers; rows on focus exit-1 codes such as window-not-found rule over it, unchanged.
@task_item n=10 part=touches
The viola list board's instance scan and name order: tests were added pinning the scan in name order; rows on the board row order rule over it, unchanged.
@task_item n=11 part=touches
The placement words: the live placement script gained a precondition verdict, red on a scratch home that is not fresh or a locked desktop; it is a test script's line, no placement word of the product, so a row on placement words is touched only if it names the script's verdicts.
@task_item n=12 part=touches
Any row whose guard names a verify-class test, the verify_window_ prefix or the cli_version_gate run tests: the verify kill class now takes six tests by name beside its two terms, timeouts unchanged.
@task_item n=1 part=untouched
Brand identity, the strip bay direction, the readback box and the flight-progress strip contracts: no expression, strip field or readback line changed.
@task_item n=2 part=untouched
Colour palette, CLI colour tokens, colour depth and the colour decision order: no colour, SGR span or token moved.
@task_item n=3 part=untouched
Typography, spacing, iconography, glyphs, separators and the six list columns: no column, glyph, width or separator changed.
@task_item n=4 part=untouched
Send readback and its mirror, viola wait output, viola last output, link and unlink output: no line of these verbs changed.
@task_item n=5 part=untouched
The verify verb's output, refusals and summary lines: only which tests the runner's kill class names changed, no verify output.
@task_item n=6 part=untouched
Wrapper fault --json details, wrapper socket refusals, peer reply screening and log detail codes: no IPC method, endpoint, code word or detail added or changed.
@task_item n=7 part=untouched
The command names (name sheet) and viola --help verb groups: no verb was added, renamed or hidden.
@task_item n=8 part=untouched
The web-spa and rendered-page absences and the self-validation protocol: no page or surface was added.
