@task_prose part=session
The session added a human CLI verb, viola focus <name> [--json]: it raises the named session's window and shows its workspace through Hyprland, finding the window by the recorded address only while that window's pid is in the wrapper's ancestry, else by the nearest ancestor's window.
On success it prints <name>  focused (or {"v":1,"ok":{"name":"<name>"}} under --json); it prints and logs no window address, pid or workspace, and writes no snapshot or event line.
It refuses with exit 1 under six new code words (window-not-found, unsupported, unreachable, oversize, malformed, refused), each with a fixed unable/hint pair as planning forms, PROVISIONAL on fork F-C; and with exit 21 (instance-dead, unwrapped-name, strict-modes-failed) before any compositor read or log.
The viola --help board row moved from list to list, focus (PROVISIONAL, fork F-B); the process-exit detail enum grew 21 to 27 words; the exit-cause table grew 25 to 31 rows; the error-body census grew by six rows.
A human verb now bears the compositor read deadline (HYPRCTL_DEADLINE) on up to three reads; by the wrap directive the a11y time-bound lists are NOT grown or split, this waits on fork F1 of run 2026-11-24T00-26-24Z-wrap.
The verb itself is PROVISIONAL on fork F-A (a boundary widening of the human controls); the live reading R3 has not run, so the real raise on a screen is unproven; nothing retired.
@task_file n=1 path=src/cmd/focus.rs
what: the new viola focus verb: its argument parsing, strict-modes refusal before any log, liveness check, compositor reads and dispatches under the read deadline, the ancestry walk held in memory, and the exit-1 and exit-21 refusal mapping
@task_file n=2 path=src/cmd/mod.rs
what: the CLI command list and dispatch, and the viola --help template whose board group row lists the served verbs
@task_file n=3 path=src/human/hints.rs
what: the fixed CLI refusal copy, the unable/hint pair per code word, now with focus's six pairs and their literal pins
@task_file n=4 path=src/human.rs
what: the human output module that re-exports the refusal pairs to the verbs
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs
what: the test-only fake compositor (hyprctl) answering focus dispatches and recording each dispatch it was asked
@task_file n=6 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's own unit cases for its modes, now for the fake compositor's focus answers and receipt
@task_file n=7 path=schemas/diag-line.v1.json
what: the diagnostic log line schema, whose process-exit detail enum names the closed exit detail words
@task_file n=8 path=tests/cli_focus.rs
what: the integration suite for viola focus: success line, each refusal pair, one clean --json document, no address or pid printed, no state written
@task_file n=9 path=tests/cli_output_plain.rs
what: the CLI literal output-discipline test holding the viola --help text verbatim
@task_file n=10 path=tests/cross_exit_causes.rs
what: the cross-check table of every exit cause with its code word and exit code
@task_file n=11 path=tests/contract_error_bodies.rs
what: the error-body census pairing each CLI refusal document with its writer and witness
@task_file n=12 path=tests/cli_error_bodies.rs
what: the CLI error-body tests, including the loosened socket-directory table, now with a focus row
@task_file n=13 path=tests/contract_diag_schema.rs
what: the diagnostic line schema contract test, now taking focus's exit detail words
@task_file n=14 path=scripts/r3-focus.sh
what: the gated live reading R3 for focus on a real Hyprland screen, with a rehearse form; a live reading that gates nothing
@task_item n=1 part=touches
contract/cli-help/served-verbs-only, contract/cli-help/usage-line, contract/cli-help/group-labels and contract/cli-help/handoff-row: the board group row now reads list, focus (PROVISIONAL, fork F-B) - a served verb added to the help.
@task_item n=2 part=touches
contract/cli-exit-codes/21 and the instance-unreachable-detail rows instance-dead, unwrapped-name and strict-modes-failed: focus is a new verb that refuses at exit 21 with these details, before any compositor read.
@task_item n=3 part=touches
decision/strict-modes-refusal-before-any-write and contract/frame-verb-refusal/strict-modes-failed: focus checks strict modes on the home and instance before any log line or read - a verb added to the set the rule covers.
@task_item n=4 part=touches
contract/cli-refusal/unable-line, contract/cli-refusal/hint-line, contract/cli-refusal/stderr-layout, contract/cli-refusal/exit-code and contract/cli-fixed-message/form: six new exit-1 refusals in the fixed unable/hint form (PROVISIONAL, fork F-C).
@task_item n=5 part=touches
contract/cli-refusal/json, contract/cli-json-failure/document, decision/json-one-document-no-hint and decision/json-failure-documents-are-clean: focus's refusals print one {"v":1,"error":"<code>","detail":null} document with no hint.
@task_item n=6 part=touches
decision/one-hint-per-code, decision/reason-code-beside-unable, decision/refusal-ends-with-hint and decision/codes-beside-plain-word: six new code words each with one hint beside its unable line.
@task_item n=7 part=touches
contract/code-word/format and decision/unlisted-code-words-stay-visible: six new kebab-case code words (window-not-found, unsupported, unreachable, oversize, malformed, refused).
@task_item n=8 part=touches
decision/fixed-cli-strings-pinned-by-literal, decision/fixed-cli-strings-pinned, decision/fixed-copy-from-design and decision/refusal-layout-literal-pins: focus's pairs are pinned by literal as planning forms, the design copy owed (fork F-C).
@task_item n=9 part=touches
decision/error-bodies-have-writer-and-witness: the error-body census grew by six focus documents, each with its writer and witness.
@task_item n=10 part=touches
decision/time-bounds-fall-on-the-driver, decision/time-bounds-never-on-human and absence/user-time-limit: a human verb (focus) now bears the compositor read deadline on up to three reads; the lists are NOT grown or split, held on fork F1 of run 2026-11-24T00-26-24Z-wrap.
@task_item n=11 part=touches
decision/peer-replies-screened-to-code-words: compositor replies are screened to code words (malformed, refused, oversize, unreachable), a reply of another kind maps to malformed.
@task_item n=12 part=touches
decision/printed-names-are-parsed-arguments and decision/paths-not-carried: focus's success line prints only the parsed name; no window address, pid or workspace is printed or logged.
@task_item n=13 part=touches
decision/static-linear-output, decision/linear-static-cli-output, decision/no-sgr-in-plain-modes and contract/cli-output/control-characters: focus's output is plain ASCII, no SGR, one write - a new output under these rules.
@task_item n=14 part=touches
decision/linux-only: focus is Hyprland-only - off Hyprland it refuses unsupported and spawns nothing; with no compositor answering it refuses unreachable (PROVISIONAL, fork F-E).
@task_item n=15 part=touches
decision/live-readings-never-gate: a new gated live reading R3 for focus, beside R2, unrun and gating nothing.
@task_item n=16 part=touches
contract/tui-passthrough/focus-mouse-resize-sequences and decision/wheel-ignores-terminal-events: raising a wrapped session's foot window makes the terminal send a focus-in sequence to the wrapper, which must not move the wheel - indirect, through the new verb.
@task_item n=17 part=touches
decision/screen-reader-target-tui and decision/manual-orca-pass: focus raises a foot window on Hyprland, the TUI screen-reader target; its live effect is unmeasured until R3.
@task_item n=18 part=touches
decision/cli-not-keyboard-interactive and decision/cli-no-interactive-input: a new verb that takes one name and --json only, no interactive input.
@task_fact n=1 slug=cognitive-accessibility
fact: viola focus refuses at exit 1 under window-not-found, unsupported, unreachable, oversize, malformed and refused, each with one fixed unable/hint pair (planning forms, PROVISIONAL on fork F-C) and under --json one {"v":1,"error":"<code>","detail":null} document with no hint.
@task_fact n=2 slug=cognitive-accessibility
fact: viola focus refuses at exit 21 (instance-dead, unwrapped-name, strict-modes-failed) before any compositor read, and a home another user may read before any log line.
@task_fact n=3 slug=a11y-scope-summary
fact: viola focus result line - <name>  focused, or {"v":1,"ok":{"name":"<name>"}} under --json, in one write with no SGR; no window address, pid or workspace is printed (PROVISIONAL, fork F-A).
@task_fact n=4 slug=aria-patterns-roles
fact: the viola --help board group row reads board: list, focus (PROVISIONAL, fork F-B).
@task_fact n=5 slug=a11y-scope-summary
fact: viola focus raises a session's window only through Hyprland; off Hyprland it refuses unsupported and starts nothing; a footclient window and a special workspace are unmeasured live (fork F-E).
@task_item n=1 part=untouched
The viola list board rows (colour, order, caption, empty board, unwrapped separator, stale-row escape sequences, NAME truncation): the board's output is unchanged.
@task_item n=2 part=untouched
The visual design SGR rows (amber DIALOG, bold NAME, dim stale rows, colour decision inputs): focus prints no SGR and no board.
@task_item n=3 part=untouched
viola revive refusals, revive --list, link and unlink, plugin install, send refusals, wait and verify outputs: none of those verbs changed.
@task_item n=4 part=untouched
MCP server tools and hook calls: no MCP tool was added (a driver-facing focus was left out) and no hook call changed.
@task_item n=5 part=untouched
The TUI passthrough byte contracts (viola terminal bytes, vendor content, human keystrokes, vt100 feed panic, run diagnostics): the wrapper's passthrough did not change.
@task_item n=6 part=untouched
The scope absences (internationalization, touch surface, web or graphical frontend, rendered page) and the no-compliance and no-WCAG-claim rows: still true.
@task_item n=7 part=untouched
config.json dialog_hold_s, the MCP wait timeout_ms and the dialog-expired event: no dialog or wait bound moved.
