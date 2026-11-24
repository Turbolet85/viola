@task_prose part=session
The session added a new flat CLI verb, viola focus <name> [--json], that raises a wrapped session's window and shows its workspace on Hyprland, chosen by the recorded window while its pid is in the wrapper's ancestry, else by the nearest ancestor window.
On success it prints <name>  focused on stdout, or {"v":1,"ok":{"name":"<name>"}} under --json; it prints and logs no window address, pid or workspace, and writes no snapshot or event line.
It refuses with exit 1 and six codes (window-not-found, unsupported, unreachable, oversize, malformed, refused), each a fixed unable/hint pair in human mode and {"v":1,"error":"<code>","detail":null} under --json; exit 21 for instance-dead, unwrapped-name and strict-modes-failed, before any compositor read.
The viola --help board row moved from list to list, focus; the process-exit log detail enum grew from 21 words to 27; the fake hyprctl gained a dispatch receipt and focus answers, and a gated live reading R3 script was added, not fired.
Everything resting on the operator's forks stands PROVISIONAL on the delegate overseer's word: F-A the widening to a focus verb, F-B the help row, F-C the refusal wording, F-E what outside Hyprland verifies; no driver-facing (MCP) focus was added.
@task_file n=1 path=src/cmd/focus.rs
what: the viola focus verb, new: its closed arguments, the strict-modes check, the liveness read, the window choice by ancestry, the workspace then window dispatch, the success line and the exit-1 and exit-21 refusals
@task_file n=2 path=src/cmd/mod.rs
what: the CLI verb list and dispatch, edited: the focus verb added after list, and the viola --help template whose board row now reads list, focus
@task_file n=3 path=src/human/hints.rs
what: the fixed refusal hint pairs for human output, edited: the six focus unable/hint pairs added
@task_file n=4 path=src/human.rs
what: the human output module, edited: re-exports the focus hint pairs
@task_file n=5 path=schemas/diag-line.v1.json
what: the diagnostic log line schema, edited: the process-exit detail enum gains the six focus exit-1 codes
@task_file n=6 path=src/bin/viola-fake-agent/hyprctl.rs
what: the test-only fake hyprctl, edited: answers the window and workspace focus dispatches in terminal mode and writes a receipt of every dispatch it is asked
@task_file n=7 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's mode tests, edited: tests of the fake hyprctl's focus answers and dispatch receipt
@task_file n=8 path=tests/cli_focus.rs
what: the integration tests of viola focus, new: success, ancestry fallback, each refusal, the clean --json documents and no state write
@task_file n=9 path=scripts/r3-focus.sh
what: the gated live reading R3 of focus on the operator's screen, new, with a rehearse form over the fakes; not fired live
@task_file n=10 path=tests/cli_output_plain.rs
what: the plain-output check of viola --help, edited: the board row reads list, focus
@task_file n=11 path=tests/cross_exit_causes.rs
what: the cross check of exit causes across verbs, edited: six focus exit-1 rows added
@task_file n=12 path=tests/contract_error_bodies.rs
what: the contract census of CLI --json error documents, edited: six focus refusal documents added
@task_file n=13 path=tests/cli_error_bodies.rs
what: the CLI error-body tables, edited: focus rows added, the loosened-socket-directory row for focus reading exit 1 unsupported
@task_file n=14 path=tests/contract_diag_schema.rs
what: the contract check of the diagnostic log line schema, edited: the focus exit details validate
@task_item n=1 part=touches
The name/command set holds no focus: the session took the command name focus (PROVISIONAL, F-A).
@task_item n=2 part=touches
The viola --help verb-groups contracts (both the help-output and the viola-help rows) are stale: the board row moved from list to list, focus (PROVISIONAL, F-B).
@task_item n=3 part=touches
contract/cli-exit-codes/1 now also covers focus's six refusal codes: window-not-found, unsupported, unreachable, oversize, malformed, refused.
@task_item n=4 part=touches
contract/cli-exit-codes/21 and the exit-21 cause and exit-21 --json cause rows for instance-dead, unwrapped-name and strict-modes-failed: focus now raises each, before any compositor read.
@task_item n=5 part=touches
decision/flat-cli-verbs and decision/list-is-the-board: a second verb joins the board group, a flat verb taking one name and --json only.
@task_item n=6 part=touches
The refusal-hints rows, contract/cli-refusal-line rows and contract/cli-stderr refusal and hint lines: focus brings six new fixed unable/hint pairs (PROVISIONAL, F-C), human mode only, no hint under --json.
@task_item n=7 part=touches
decision/hint-never-quotes-text, decision/no-internals-in-errors and decision/start-refusals-show-no-path-or-pid: focus's output and log carry no window address, pid or workspace.
@task_item n=8 part=touches
The fixed refusal pairs (home-state, instance-state), the strict-modes refusal rows (exit 21, human output, --json document) and contract/frame-verb-refusals/strict-modes: focus refuses a widened home or instance with strict-modes-failed before any log.
@task_item n=9 part=touches
contract/frame-verb-refusals/socket-dir and the socket-dir-refused cause: focus connects to no wrapper socket, so a loosened socket directory does not refuse it with socket-dir-refused; an exception beside that convention.
@task_item n=10 part=touches
decision/json-one-document-on-stdout, decision/json-failure-documents-clean and contract/cli-catch-site/json-flag-scope: focus's --json success and refusals are each one clean document.
@task_item n=11 part=touches
The placement-word contracts (unsupported, unreachable, oversize, malformed, refused, window-unseen): focus's refusal codes reuse five of those words for its own exit-1 codes and add window-not-found beside window-unseen.
@task_item n=12 part=touches
contract/log-detail-codes rows: the process-exit detail enum grew from 21 words to 27 with the six focus codes; any row holding that count or word list is stale.
@task_item n=13 part=touches
contract/cli-verb-output rows: no row yet holds viola focus's output, the success line <name>  focused and its --json ok document.
@task_item n=14 part=touches
decision/no-success-before-confirmation and decision/confirmations-carry-context: focus prints focused only after the compositor answered the window dispatch, and names the session.
@task_item n=15 part=touches
decision/bare-strips-never-target: focus of an unwrapped name is refused exit 21 unwrapped-name, never a target.
@task_item n=16 part=touches
contract/cli-callers/human and contract/cli-callers/llm-driver: focus is a human CLI verb; no driver-facing MCP focus exists (PROVISIONAL, F-A).
@task_item n=17 part=touches
contract/cli-streams/stdout and stderr, decision/stdout-data-stderr-messages: focus writes its success to stdout and its refusal pair to stderr, plain ASCII, no SGR, one write.
@task_item n=18 part=touches
decision/fixed-phraseology-vocabulary: the word focused joins the CLI's fixed result words.
@task_fact n=1 slug=surface-cli
fact: The command name focus is occupied: viola focus <name> [--json] brings a wrapped session's window forward and shows its workspace on Hyprland, a board verb beside list (PROVISIONAL, F-A).
@task_fact n=2 slug=surface-cli
fact: viola focus output: on success <name>  focused on stdout, or {"v":1,"ok":{"name":"<name>"}} under --json; no window address, pid or workspace is ever printed or logged.
@task_fact n=3 slug=surface-cli
fact: viola focus exit-1 refusal codes are window-not-found, unsupported, unreachable, oversize, malformed and refused, each {"v":1,"error":"<code>","detail":null} under --json and a fixed unable/hint pair in human mode (pairs PROVISIONAL, F-C; unreachable PROVISIONAL, F-E).
@task_fact n=4 slug=surface-cli
fact: viola focus refuses exit 21 with instance-dead or unwrapped-name for a name not live, and strict-modes-failed for a home or instance failing strict modes, each before any compositor read.
@task_fact n=5 slug=surface-cli
fact: viola focus connects to no wrapper socket: a loosened socket directory does not refuse it with socket-dir-refused, the exception to the frame-verb socket-directory refusal.
@task_fact n=6 slug=surface-cli
fact: viola focus raises only a window of its session: the recorded window while its pid is in the wrapper's ancestry, else the nearest ancestor's window; a recorded window outside the ancestry is never focused, and none found is window-not-found with nothing dispatched.
@task_fact n=7 slug=surface-cli
fact: No driver-facing focus exists: focus is a human CLI verb with no MCP tool, a further widening only on the operator's word (PROVISIONAL, F-A arm 2).
@task_item n=1 part=untouched
The brand-identity strip and readback rows (flight-progress strip fields, readback box, send readback, handoff marker): focus draws no strip and fills no readback.
@task_item n=2 part=untouched
Colour, typography, spacing and iconography rows (amber, dim, bold NAME, glyphs, separators): focus's output uses no colour or glyph.
@task_item n=3 part=untouched
The viola list board and table rows (columns, widths, NAME cut, row order, skip counts, escaping, BAY line): the board's own output did not change.
@task_item n=4 part=untouched
The run, revive, verify, wait, send, last, link and unlink output and refusal rows, and the revive --json exit-1 codes: none of those verbs changed.
@task_item n=5 part=untouched
Wrapper fault --json detail, wrapper socket and peer reply screening rows: focus asks no wrapper and screens no peer reply.
@task_item n=6 part=untouched
The automatic-verify line and CLI verification state rows, the hook verb and mcp verb rows: untouched by focus.
@task_item n=7 part=untouched
The web-page, rendered-page and tape absences, and the self-validation protocol checks: no page, tape or design token was added.
