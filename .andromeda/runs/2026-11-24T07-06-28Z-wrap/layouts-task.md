@task_prose part=session
The session added the CLI verb viola up [<NAME>...] [--dry-run] [--json]: after a reboot it re-opens the fleet through revive, parents first, skipping live ones, and prints one line per instance.
It added a hidden --origin <cli|agent> flag to viola revive. Under that flag a revive moves its window to the workspace that a real snapshot recorded, and reads unplaced when none was recorded. Without the flag a revive has no window step, as before.
It added the refusal word boot-time-unread to the process-exit detail enum and to the exit-cause table, with its own hint pair. It also added up to the setup row of viola --help.
It measured and mended the test-side teardown of stand-in terminals (one shared teardown) and a settle wait in the fd guard. No product output moved because of these.
It found that the claim that viola open writes a process-exit for its terminal is false: open and up write the terminal's process-start alone. Nothing was retired.
@task_file n=1 path=src/cmd/up.rs
what: the viola up verb: its selection, parents-first order, closed outcome words, line and --json output, exits and refusals
@task_file n=2 path=src/cmd/mod.rs
what: the CLI verb table and dispatch, the --help literal with its setup row
@task_file n=3 path=src/cmd/revive.rs
what: the viola revive verb: its usage, flags, preflight and window placement
@task_file n=4 path=src/cmd/open.rs
what: the viola open verb: its terminal spawn and its wait for the instance to come up live
@task_file n=5 path=src/human/verbs.rs
what: the human output lines of the verbs, now with the up line and the hint write
@task_file n=6 path=src/human/hints.rs
what: the fixed hint and refusal pairs, now with the boot-time-unread pair and the up not-live hint
@task_file n=7 path=src/human.rs
what: the human output module, which exports the up line, the hint, and the boot-time-unread pair
@task_file n=8 path=src/bin/viola-fake-agent/foot.rs
what: the stand-in foot terminal of the test harness, which records each spawn
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the stand-in agent's own unit checks of its modes
@task_file n=10 path=crates/viola-state/src/liveness.rs
what: the liveness reading of an instance, now with the boot time and a heartbeat-before-boot check
@task_file n=11 path=crates/viola-state/src/replay.rs
what: the replay of an instance's event log, now with the check for a session end after the last start
@task_file n=12 path=crates/viola-state/src/board.rs
what: the board scan of the instances under the home
@task_file n=13 path=schemas/diag-line.v1.json
what: the diagnostic line schema, with its process-exit detail enum
@task_file n=14 path=tests/cli_output_plain.rs
what: the check on plain human output and the served-verb list of --help
@task_file n=15 path=tests/contract_error_bodies.rs
what: the check that every CLI refusal document has its exact body
@task_file n=16 path=tests/contract_bounded_readers.rs
what: the check on bounded readers, with their list of boundaries and exceptions
@task_file n=17 path=tests/cross_exit_causes.rs
what: the exit-cause table: each refusal word with its exit code and hint
@task_file n=18 path=tests/cli_revive.rs
what: the integration checks of viola revive, now with its window placement under --origin
@task_file n=19 path=tests/cli_machine_contract.rs
what: the check on the machine contract of --json documents and exit codes
@task_file n=20 path=tests/contract_diag_schema.rs
what: the check that the diagnostic lines match their schema
@task_file n=21 path=tests/tui_channel_fds.rs
what: the e2e guard that a launched child holds no file of the viola home
@task_file n=22 path=tests/support/mod.rs
what: the shared test-support module list
@task_file n=23 path=tests/support/terminal.rs
what: the one shared teardown that ends a stand-in terminal's process tree
@task_file n=24 path=tests/cli_open.rs
what: the integration checks of viola open
@task_file n=25 path=tests/cli_up.rs
what: the integration checks of viola up
@task_file n=26 path=scripts/r4-up.sh
what: the live-rig rehearsal and gate script for viola up and its window placement, which refuses its second phase
@task_item n=1 part=touches
The --help verb group rows (contract/cli-verbs/help, contract/viola-help/verb-groups) are stale. Their setup row lists open, run, revive, verify, plugin install, but the built row is open, run, revive, up, verify, plugin install.
@task_item n=2 part=touches
contract/viola-revive/no-window: a revive now places a window only under the hidden --origin flag. It moves to the workspace a real snapshot recorded, or reads unplaced. Without the flag there is no window step.
@task_item n=3 part=touches
The revive usage and verb rows (contract/cli-verbs/revive, contract/viola-revive/usage, contract/viola-verb-help/usage) gain a hidden --origin <cli|agent>. It takes closed values, conflicts with --list, and has no --place.
@task_item n=4 part=touches
contract/viola-revive/preflight-order: preflight now derives the window ask from --origin and the recovered snapshot.
@task_item n=5 part=touches
The exit-code rows, through the new verb: contract/cli-exit-codes/21 now also covers up's board strict refusal (instance-unreachable or strict-modes-failed). up's exit 1 carries the refusal documents boot-time-unread, strict-modes-failed, pinned-hash-mismatch and terminal-not-found (contract/cli-exit-codes/json-form, contract/instance-unreachable-detail/strict-modes-failed).
@task_item n=6 part=touches
The hint conventions (decision/hint-for-every-cause, decision/hint-last-on-stderr, decision/hints-name-next-verb, decision/hints-never-quote-or-locate) now rule over a new refusal word, boot-time-unread, with its own hint pair, and over up's single not-live hint line on stderr.
@task_item n=7 part=touches
The refusal line form and the shared refusal word column (contract/cli-refusal-line/form, decision/shared-refusal-word-column) now rule over up's refusals. contract/run-revive-start-json/pinned-hash-mismatch now has a second verb, up, that writes the same code.
@task_item n=8 part=touches
The revive --json error words (contract/viola-revive-json/already-live, stale-heartbeat, no-session, cwd-missing, session-live) are reused as up's per-instance outcome words.
@task_item n=9 part=touches
decision/json-on-every-data-verb and contract/cli-json-mode/one-document: up has a --json that gives one document, {"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}.
@task_item n=10 part=touches
decision/cli-holds-every-action and the MCP verb decisions (decision/mcp-hides-wheel-and-handoff-verbs, the contract/mcp-tools rows): up is a CLI-only verb with no MCP tool and no channel method.
@task_item n=11 part=touches
decision/targets-are-wrapped-names and decision/flat-verbs: up takes optional names, each parsed as a ViolaName. Given names narrow the fleet, and a name that is not scanned reads not-taken.
@task_item n=12 part=touches
The plain output rules (decision/plain-output-modes, decision/stdout-results-stderr-rest, decision/cli-field-separators, decision/human-output-is-contract, decision/no-spinner, decision/single-progress-pattern, decision/words-carry-every-state) now rule over up's lines. Each line is plain ASCII with no SGR and one write per line, on stdout, in the form <name>  <word> or <name>  live  <placement>.
@task_item n=13 part=touches
decision/board-ordered-by-name has a near exception in up's line order: every line that is not a spawn comes first, in name order, then the spawned or would-open lines in spawn order (parents first).
@task_item n=14 part=touches
The viola open rows (contract/cli-verbs/open, contract/viola-open/parent-default): open's terminal spawn and its live wait are now shared with up, and open's behaviour is unchanged. Open writes the terminal's process-start alone and no process-exit.
@task_item n=15 part=touches
The name sheet: the command name up is now taken.
@task_fact n=1 slug=surface-cli
fact: viola up [<NAME>...] [--dry-run] [--json] re-opens the fleet after a reboot. Each candidate instance is revived in its own terminal at its recorded working directory, parents before children, and instances already live are skipped. It is CLI-only, with no MCP tool and no channel method.
@task_fact n=2 slug=surface-cli
fact: viola up prints one line per instance, <name>  <word> or <name>  live  <placement>, with the outcome words in a closed set: live, already-live, stale-heartbeat, would-open, no-session, cwd-missing, session-live, not-live, not-taken.
@task_fact n=3 slug=surface-cli
fact: viola up exits 0 when every line reads live, already-live, stale-heartbeat or would-open, and 1 otherwise. It exits 21 for the board's strict refusal. It exits 1 with a document for boot-time-unread, strict-modes-failed, pinned-hash-mismatch and terminal-not-found.
@task_fact n=4 slug=surface-cli
fact: viola up orders its lines with every non-spawn line first in name order, then the spawned or would-open lines in spawn order. With names given, the fleet narrows to them, and an unscanned name reads not-taken.
@task_fact n=5 slug=surface-cli
fact: After its lines, viola up writes one stderr hint line when any line reads not-live, and none under --json. --dry-run prints the plan as would-open lines and spawns nothing.
@task_fact n=6 slug=surface-cli
fact: viola up --json gives one document, {"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}. No line and no document of up carries an address, a pid or a workspace.
@task_fact n=7 slug=surface-cli
fact: boot-time-unread is a refusal word, exit 1, with its own human pair and hint. It is the refusal when the host's boot time cannot be read.
@task_fact n=8 slug=surface-cli
fact: viola revive has a hidden --origin <cli|agent> that conflicts with --list and has no --place. Under it, the revived window moves to the workspace a real snapshot recorded, or reads unplaced when none was recorded.
@task_fact n=9 slug=surface-cli
fact: The command name up is occupied by the viola up verb.
@task_item n=1 part=untouched
The web surface absence and the absence of deep links in the CLI: the session adds no page and no URL.
@task_item n=2 part=untouched
The viola list board rows (columns, BAY line, tree, links block, stale rows, colour depth, truncation, widths): the board's look and order are unchanged.
@task_item n=3 part=untouched
The viola send output, readback mirror and send refusal rows, and the viola wait output rows: no send or wait line moved.
@task_item n=4 part=untouched
The viola verify output and refusal rows, and the viola plugin install rows: neither verb changed.
@task_item n=5 part=untouched
The link and unlink output rows, the wheel verb output rows, the answer output rows, and the frame-verb exit-21 hint rows: none of these verbs moved.
@task_item n=6 part=untouched
The CLI token rendering rows and the colour and styling decisions (amber, SGR module, readback colour): up uses no colour.
@task_item n=7 part=untouched
The viola revive --list rows (list row, list cause, list refusals, list document): --list is unchanged, and --origin conflicts with it.
