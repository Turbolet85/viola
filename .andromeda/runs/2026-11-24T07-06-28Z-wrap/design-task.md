@task_prose part=session
The session added the CLI verb viola up [NAME...] [--dry-run] [--json]: after a reboot it re-opens the fleet through revive, parents first, live and stale instances skipped, one line per instance on stdout.
It added a hidden --origin <cli|agent> flag to viola revive: with it, the revived window is moved to the workspace the instance last recorded, else unplaced; without it there is no window step, as before.
It added a new refusal word boot-time-unread (the host boot time could not be read), with its own fixed unable/hint pair and exit 1 document, and one not-live hint line after up's lines.
The help's setup: verb group now reads open, run, revive, up, verify, plugin install.
It measured that viola open (and now up) write the terminal's process-start alone and no process-exit; the observability rule is corrected to the code.
Test-only: one shared teardown for a stand-in terminal's process tree, a settle wait in the child fd guard, and a gated live rehearsal script for R4; matrix v2-12 stays GATED, proven over the fakes only.
@task_file n=1 path=src/cmd/up.rs
what: the viola up verb — selection of pre-boot instances, parents-first order, the closed outcome words, its per-instance lines, --json document, exits and refusals
@task_file n=2 path=tests/cli_up.rs
what: the integration cases of viola up — its lines, closed words, --json document, dry run, refusals and exits over the fakes
@task_file n=3 path=scripts/r4-up.sh
what: the test-only live rehearsal harness for the R4 gate of viola up (windows and plant/after phases)
@task_file n=4 path=tests/support/terminal.rs
what: the test-only shared teardown of a stand-in terminal's process tree
@task_file n=5 path=src/cmd/mod.rs
what: the CLI verb table and dispatch — the flat verb list, the help text with its verb groups
@task_file n=6 path=src/cmd/revive.rs
what: the viola revive verb — its flags, its refusals and its window placement under the hidden --origin flag
@task_file n=7 path=src/cmd/open.rs
what: the viola open verb — the terminal spawn, the wait until the instance is live, the recorded window read
@task_file n=8 path=src/human/verbs.rs
what: the human output lines of each verb — the two-space columns, one write per line, the hint line
@task_file n=9 path=src/human/hints.rs
what: the fixed unable/hint refusal pairs and the fixed hint literals of the human CLI
@task_file n=10 path=src/human.rs
what: the human output module — the exports of the line writers and hint pairs
@task_file n=11 path=src/bin/viola-fake-agent/foot.rs
what: the stand-in foot terminal of the test fakes, recording each spawn
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the unit cases of the test fakes' modes
@task_file n=13 path=crates/viola-state/src/liveness.rs
what: the instance liveness reading — live, stale heartbeat, and now the host boot time and a heartbeat older than the boot
@task_file n=14 path=crates/viola-state/src/replay.rs
what: the replay of an instance's event log into its state, and whether a session ended after its last start
@task_file n=15 path=crates/viola-state/src/board.rs
what: the board's strict-modes scan of the instances under the home
@task_file n=16 path=schemas/diag-line.v1.json
what: the diagnostic log line schema — the process-exit detail word list
@task_file n=17 path=tests/cli_output_plain.rs
what: the check that human output is plain ASCII and the help lists only served verbs
@task_file n=18 path=tests/contract_error_bodies.rs
what: the check of every CLI --json refusal document and its fixed body
@task_file n=19 path=tests/contract_bounded_readers.rs
what: the check that every reader of an outside source is bounded, with its boundaries and exceptions list
@task_file n=20 path=tests/cross_exit_causes.rs
what: the exit-cause table check — each refusal word with its exit code and hint
@task_file n=21 path=tests/cli_revive.rs
what: the integration cases of viola revive, now its window placement under --origin
@task_file n=22 path=tests/cli_machine_contract.rs
what: the check of the CLI machine contract — --json documents on stdout and exit codes
@task_file n=23 path=tests/contract_diag_schema.rs
what: the check that log lines match the diagnostic line schema
@task_file n=24 path=tests/tui_channel_fds.rs
what: the e2e guard that the child holds no file of viola's once its launch settles
@task_file n=25 path=tests/support/mod.rs
what: the shared test-support module list
@task_file n=26 path=tests/cli_open.rs
what: the integration cases of viola open
@task_item n=1 part=touches
The help verb-group rows (contract/help-output/verb-groups, contract/viola-help/verb-groups) are stale: the setup: group now lists open, run, revive, up, verify, plugin install.
@task_item n=2 part=touches
The command-name rows: the name up is now taken by a served verb; no name/command row holds it yet.
@task_item n=3 part=touches
decision/flat-cli-verbs: a new flat top-level verb up joins the verb set.
@task_item n=4 part=touches
contract/cli-exit-codes/1: up exits 1 when any line is not live, already-live, stale-heartbeat or would-open, and 1 with a document for boot-time-unread, strict-modes-failed, pinned-hash-mismatch and terminal-not-found.
@task_item n=5 part=touches
contract/cli-exit-codes/0 and /21: up exits 0 when every line is fine and 21 on the board's strict refusal (instance-unreachable, strict-modes-failed) with no line.
@task_item n=6 part=touches
The exit-21 strict-modes cause rows and decision/list-refuses-board-on-strict-modes-failure: up now shares the board's whole-board strict refusal.
@task_item n=7 part=touches
The fixed refusal pairs and CLI refusal line rows: a new home-wide fixed pair for boot-time-unread joins them.
@task_item n=8 part=touches
The revive --json refusal code rows: up reuses already-live, stale-heartbeat, no-session, cwd-missing and session-live as its own per-line outcome words.
@task_item n=9 part=touches
The placement word rows: up's live line and its --json placement field carry the placement word of the revived window.
@task_item n=10 part=touches
The origin word rows (cli, agent): revive now takes them through a hidden --origin flag, which up passes to each terminal it spawns.
@task_item n=11 part=touches
The viola revive output and refusal rows: a revive under --origin moves its window to the recorded workspace, else unplaced; no window step without it.
@task_item n=12 part=touches
decision/hints-human-mode-only, decision/no-hint-under-json and contract/cli-stderr/hint-line: up writes one stderr hint line after its lines when any reads not-live, none under --json.
@task_item n=13 part=touches
decision/json-one-document-on-stdout and decision/stdout-data-stderr-messages: up's --json is one document of closed words; its lines are stdout data.
@task_item n=14 part=touches
decision/cli-two-space-columns and decision/ascii-only-human-output: up's line is name, two spaces, word (and two spaces, placement), plain ASCII, no SGR, one write per line.
@task_item n=15 part=touches
decision/start-refusals-show-no-path-or-pid and decision/no-internals-in-errors: up's lines and document name no address, pid or workspace.
@task_item n=16 part=touches
decision/no-spinner, decision/one-progress-pattern and decision/step-counter-only-progress-pattern: up waits per spawned instance on open's live bound and prints nothing while it waits.
@task_item n=17 part=touches
decision/no-success-before-confirmation: up prints live for an instance only once it has come up live.
@task_item n=18 part=touches
contract/viola-run-start-refusals/pinned-hash-mismatch and home-tree-check rows: up runs the same home-tree check and pinned-copy check before its first spawn and refuses with the same words.
@task_item n=19 part=touches
decision/no-hint-names-viola-release and decision/no-hint-for-faults: the new not-live and boot-time-unread hints stand under these rules.
@task_item n=20 part=touches
The CLI verb output rows (contract/cli-verb-output/*): no row yet holds viola up's output.
@task_fact n=1 slug=surface-cli
fact: viola up [NAME...] [--dry-run] [--json] is a CLI-only verb in the setup: group, with no MCP tool and no channel method; names narrow the fleet to those named, and a name not on the board reads not-taken.
@task_fact n=2 slug=surface-cli
fact: viola up prints one stdout line per instance, name two spaces word, or name two spaces live two spaces placement; --json gives one document {"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}.
@task_fact n=3 slug=surface-cli
fact: viola up's outcome words are a closed set: live, already-live, stale-heartbeat, would-open, no-session, cwd-missing, session-live, not-live, not-taken.
@task_fact n=4 slug=surface-cli
fact: viola up's line order: every line that is no spawn first, in name order, then the spawned or would-open lines in spawn order, parents before children.
@task_fact n=5 slug=surface-cli
fact: viola up exits 0 when every line is live, already-live, stale-heartbeat or would-open, else 1; 21 with no line on the board's strict refusal; 1 with a document for boot-time-unread, strict-modes-failed, pinned-hash-mismatch and terminal-not-found.
@task_fact n=6 slug=surface
fact: boot-time-unread is a refusal word with its own fixed unable/hint pair and exit 1 --json document: the host boot time could not be read, so up selects nothing.
@task_fact n=7 slug=surface
fact: viola up writes one fixed stderr hint line after its lines when any line reads not-live, and none under --json.
@task_fact n=8 slug=surface-cli
fact: viola up --dry-run prints the plan as would-open lines and spawns nothing; up writes no snapshot, event line or stamp.
@task_fact n=9 slug=surface-cli
fact: viola revive takes a hidden --origin <cli|agent> (closed values, conflicts with --list, not listed in help): only under it does a revive place a window, at the workspace a real snapshot recorded, else unplaced.
@task_fact n=10 slug=surface-cli
fact: viola up mints no time bound of its own: each spawned instance waits on viola open's live bound.
@task_item n=1 part=untouched
Brand identity rows: the flight-progress strip, the readback box, the send readback and handoff rows; up draws no strip and no readback.
@task_item n=2 part=untouched
Colour palette and colour token rows: up prints no SGR and no colour.
@task_item n=3 part=untouched
Typography, spacing beyond the two-space column, depth, border radius and iconography rows; no glyph was added.
@task_item n=4 part=untouched
The viola send, wait, last, link, unlink, pause, release, answer and focus output rows.
@task_item n=5 part=untouched
viola verify output and refusal rows, and the automatic-verify line rows.
@task_item n=6 part=untouched
Wrapper fault detail, peer reply screening and wrapper socket rows.
@task_item n=7 part=untouched
The mcp and hook verb rows and their colour decision order rows.
@task_item n=8 part=untouched
The web-page and rendered-page absences and the self-validation protocol rows.
