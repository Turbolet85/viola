@task_prose part=session
The session added a new CLI verb, viola focus <name> [--json], which raises the named session's window and shows its workspace through Hyprland. It finds the window by the recorded address only while that window's pid is in the wrapper's ancestry, and otherwise by the nearest ancestor pid's window.
On success it prints `<name>  focused` on stdout, or {"v":1,"ok":{"name":"<name>"}} under --json. No window address, pid or workspace is ever printed or logged.
It adds six exit-1 refusal codes (window-not-found, unsupported, unreachable, oversize, malformed, refused), each with a fixed unable/hint pair. It uses exit 21 for a name that is not live (instance-dead, unwrapped-name) and for strict-modes-failed.
The viola --help board row moved from `board:    list` to `board:    list, focus`. The process-exit detail enum grew from 21 words to 27. The exit-cause table grew from 25 rows to 31.
The verb, the help row (F-B), the refusal wording (F-C) and unreachable (F-E) rest PROVISIONAL on the delegate overseer's word. A driver-facing focus (an MCP tool) was left out. Nothing was retired.
@task_file n=1 path=src/cmd/focus.rs
what: the viola focus verb: its args, its strict-modes check, its liveness check, the window lookup by recorded address or ancestry, the workspace and window dispatches, and its refusal codes
@task_file n=2 path=src/cmd/mod.rs
what: the CLI verb set and dispatch, and the viola --help template with its verb group rows
@task_file n=3 path=src/human/hints.rs
what: the fixed unable/hint pairs for refusals, the hint for every refusal cause, which now include focus's six pairs
@task_file n=4 path=src/human.rs
what: the human output module that re-exports the hint pairs
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake hyprctl test double: terminal mode's window and workspace focus answers, and a receipt of each dispatch it was asked
@task_file n=6 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: unit tests of the fake hyprctl's modes, its focus answers and its dispatch receipt
@task_file n=7 path=schemas/diag-line.v1.json
what: the diag line schema, whose process-exit detail enum gains the six focus exit-1 words
@task_file n=8 path=tests/cli_focus.rs
what: integration tests of viola focus: success, ancestry lookup, the refusals, the --json documents, and no state write
@task_file n=9 path=tests/cli_output_plain.rs
what: the plain-output check of viola --help's text, including the board verb group row
@task_file n=10 path=tests/cross_exit_causes.rs
what: the cross-check table of exit causes, with each cause's exit code and detail word
@task_file n=11 path=tests/contract_error_bodies.rs
what: the census of CLI --json error documents ({"v":1,"error":...,"detail":...}) by verb
@task_file n=12 path=tests/cli_error_bodies.rs
what: the CLI refusal body tables, including the loosened-socket-directory table, now with a focus row
@task_file n=13 path=tests/contract_diag_schema.rs
what: the contract check of the diag line schema, which now takes focus's exit details
@task_file n=14 path=scripts/r3-focus.sh
what: the gated live reading R3 of viola focus on the operator's screen, with a --rehearse form
@task_item n=1 part=touches
contract/viola-help/verb-groups: the board verb group row now reads `board:    list, focus`, not `board:    list` (PROVISIONAL, F-B).
@task_item n=2 part=touches
contract/cli-verbs/help: viola --help lists a new verb, focus, in the board group.
@task_item n=3 part=touches
The CLI verbs set (contract/cli-verbs/*): there is a new verb, viola focus <name> [--json], and no row holds it yet. The nearest sibling is contract/cli-verbs/open.
@task_item n=4 part=touches
decision/board-vs-strip: focus joins list in the board group of verbs, which changes which verbs the board group holds.
@task_item n=5 part=touches
contract/cli-exit-codes/21: focus now exits 21 for a name that is not live and for a home or instance that fails strict modes.
@task_item n=6 part=touches
contract/instance-unreachable-detail/instance-dead, unwrapped-name and strict-modes-failed: focus is a new verb that emits each of these exit-21 details.
@task_item n=7 part=touches
contract/frame-verb-hints/* (instance-not-running, not-wrapped): focus's not-live refusals go through the shared unreachable path and its hints. The rows' list of verbs may need focus.
@task_item n=8 part=touches
contract/frame-verbs/strict-modes-refused and decision/state-tree-strict-modes-before-use: focus refuses a home another user may read before it writes any log. That is a new verb under the strict-modes refusal.
@task_item n=9 part=touches
contract/cli-exit-codes/json-form and contract/cli-json-mode/one-document: focus's --json refusals are {"v":1,"error":"<code>","detail":null}, and its success is {"v":1,"ok":{"name":"<name>"}}. Each is one clean document.
@task_item n=10 part=touches
decision/json-on-every-data-verb: a new verb now carries --json.
@task_item n=11 part=touches
decision/hint-for-every-cause, decision/hints-name-next-verb and decision/hint-last-on-stderr: six new refusal causes, each with a fixed unable/hint pair (PROVISIONAL, F-C).
@task_item n=12 part=touches
decision/hints-never-quote-or-locate: focus never prints the window address, pid or workspace in any output or log line.
@task_item n=13 part=touches
decision/board-planning-forms-wording: the six focus pairs stand as planning forms, not ratified wording (F-C).
@task_item n=14 part=touches
contract/cli-refusal-line/form and decision/shared-refusal-word-column: focus's refusals follow the shared refusal line and word column.
@task_item n=15 part=touches
decision/spoken-phraseology-verb-output and decision/cli-field-separators: the success line `<name>  focused` uses the two-space field separator and a spoken outcome word.
@task_item n=16 part=touches
decision/stdout-results-stderr-rest and decision/plain-output-modes: the success goes to stdout and refusals go to stderr, in plain ASCII with no SGR and one write.
@task_item n=17 part=touches
decision/targets-are-wrapped-names and decision/flat-verbs: focus takes one ViolaName positional target and nothing else besides --json.
@task_item n=18 part=touches
decision/cli-holds-every-action: focus is a human control held on the CLI only (PROVISIONAL, F-A, a boundary widening).
@task_item n=19 part=touches
decision/mcp-hides-wheel-and-handoff-verbs and decision/mcp-server-without-human-surface: no MCP tool was added for focus. It is a CLI-only verb, and a driver-facing focus waits on the operator's word (F-A arm 2).
@task_item n=20 part=touches
contract/viola-verb-help/usage: there is a new per-verb usage, viola focus <name> [--json].
@task_item n=21 part=touches
contract/viola-revive/no-window: window placement. focus is the first verb that raises a session's window and switches its workspace through Hyprland.
@task_fact n=1 slug=surface-cli
fact: viola focus <name> [--json] brings the named session's window forward and shows its workspace on Hyprland. It takes one ViolaName and --json only, and it is a CLI-only human control (PROVISIONAL, F-A).
@task_fact n=2 slug=surface-cli
fact: viola focus success prints `<name>  focused` on stdout, or {"v":1,"ok":{"name":"<name>"}} under --json. No window address, pid or workspace is printed.
@task_fact n=3 slug=surface-cli
fact: viola focus exit-1 refusals are window-not-found, unsupported, unreachable (PROVISIONAL, F-E), oversize, malformed and refused. Each is {"v":1,"error":"<code>","detail":null} under --json, with a fixed unable/hint pair as a planning form (F-C).
@task_fact n=4 slug=surface-cli
fact: viola focus exits 21 (instance-dead, unwrapped-name) for a name not live, before any compositor read. It exits 21 strict-modes-failed for a home or instance failing strict modes, before any log.
@task_fact n=5 slug=surface-cli
fact: viola focus off Hyprland refuses with unsupported and spawns nothing. With no window of its session, it refuses with window-not-found and dispatches nothing.
@task_fact n=6 slug=surface-cli
fact: The name focus is occupied as a CLI verb in the board group of viola --help (PROVISIONAL, F-B).
@task_fact n=7 slug=surface-cli
fact: Absence: no MCP tool focuses a window. A driver-facing focus is left out until the operator's word (F-A arm 2).
@task_item n=1 part=untouched
absence/web-surface: viola still renders no page.
@task_item n=2 part=untouched
The send, readback mirror, readback vocabulary and send refusal rows: send's output and refusals did not move.
@task_item n=3 part=untouched
The viola wait output rows and decision/turn-ended-not-send-outcome: wait is unchanged.
@task_item n=4 part=untouched
The viola revive and viola run start rows, refusals and --json codes: start paths are unchanged.
@task_item n=5 part=untouched
The viola verify output and refusal rows: verify is unchanged.
@task_item n=6 part=untouched
The viola link/unlink output rows and the viola plugin install rows: unchanged.
@task_item n=7 part=untouched
The viola list board rows (columns, tree, BAY line, truncation, colour depth, LINKS block, stale rows): the board's content is unchanged. Only the help group row gains focus.
@task_item n=8 part=untouched
The CLI token rendering rows (colours, roles, spacing, radius, motion): no token was added or changed.
@task_item n=9 part=untouched
The mcp-tools and mcp-tool-results rows: no MCP tool was added or changed.
@task_item n=10 part=untouched
The wheel verb output rows (pause, release) and decision/no-hint-names-release: unchanged.
@task_item n=11 part=untouched
absence/cli-deep-links: focus adds no link or URL.
