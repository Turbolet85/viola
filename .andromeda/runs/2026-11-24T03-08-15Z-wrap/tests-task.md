@task_prose part=session
Added the CLI verb viola focus <name> [--json]: it raises a wrapped session's window and shows its workspace through Hyprland, finding the window by the recorded address while that window's pid is in the wrapper's ancestry, else the nearest ancestor pid's window.
Focus exits 0 with `<name>  focused` or a one-key ok document, exits 1 with six new refusal codes (window-not-found, unsupported, unreachable, oversize, malformed, refused), each with its own fixed unable and hint pair, and exits 21 for instance-dead, unwrapped-name and strict-modes-failed; it prints and logs no address, pid or workspace, and writes no snapshot or event line.
The fake hyprctl gained a hyprctl-asked receipt of every dispatch argv it was asked, and its terminal mode now answers the window focus and workspace focus from recorded replies; a live reading R3 (a script with a rehearse form) was added beside R2, gated and not run live.
The process-exit detail enum grew from 21 to 27 words, the cross-surface exit causes table from 25 to 31 rows, the error-body census gained six rows, and the help's board row now reads list, focus.
The help row, the six refusal pairs, the unreachable wording and the absence of an MCP focus tool rest on forks F-B, F-C, F-E and F-A, each PROVISIONAL on the delegate overseer's word; matrix v2-11 stays claimed gated on R3.
@task_file n=1 path=src/cmd/focus.rs
what: the new focus verb's implementation and its unit cases over the hyprctl closure seam: the CLI verb that raises a session's window, its ancestry walk, its refusal codes
@task_file n=2 path=tests/cli_focus.rs
what: the new root integration binary for viola focus, run as a subprocess against the fake hyprctl and a fresh test home
@task_file n=3 path=scripts/r3-focus.sh
what: the R3 live reading script for focus, with a rehearse form over the fakes that is not a live reading
@task_file n=4 path=src/cmd/mod.rs
what: the CLI verb table and dispatch, and the help template's grouped verb table (the board row)
@task_file n=5 path=src/human/hints.rs
what: the fixed human unable and hint lines, one pair per exit cause; gained the focus pairs
@task_file n=6 path=src/human.rs
what: the human output module's re-exports of the hint pairs
@task_file n=7 path=src/bin/viola-fake-agent/hyprctl.rs
what: the fake hyprctl stub executable in the fake agent binary that stands in for the compositor; gained the dispatch receipt and the terminal mode's focus answers
@task_file n=8 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's mode unit cases, including the fake hyprctl modes
@task_file n=9 path=schemas/diag-line.v1.json
what: the diagnostics line schema, holding the closed process-exit detail enum
@task_file n=10 path=tests/cli_output_plain.rs
what: the CLI plain-output literals, including the viola --help grouped verb table
@task_file n=11 path=tests/cross_exit_causes.rs
what: the cross-surface exit causes table, one row per exit cause and its hint
@task_file n=12 path=tests/contract_error_bodies.rs
what: the error-body census of every CLI error document, held by surface witnesses
@task_file n=13 path=tests/cli_error_bodies.rs
what: the CLI error-body cases, including the loosened socket directory table run per verb
@task_file n=14 path=tests/contract_diag_schema.rs
what: the diagnostics schema conformance cases for process-exit details
@task_item n=1 part=touches
decision/fakes-for-external-services: the fake hyprctl now keeps a hyprctl-asked receipt of each dispatch argv and its terminal mode answers the focus dispatches; the row should say so.
@task_item n=2 part=touches
decision/external-processes-real-stubs: the fake hyprctl stub executable now also answers dispatches and writes a receipt beside its exe.
@task_item n=3 part=touches
decision/window-placement-proven-live: a real window raise and workspace switch by focus is proven only by the live reading R3, unproven until it runs; the fakes prove the refusals and the dispatch order alone.
@task_item n=4 part=touches
decision/live-failing-reading-not-verification and decision/standing-live-rig: R3 is a second gated live script beside R2, with a rehearse form whose last line says it is not a live reading.
@task_item n=5 part=touches
decision/fake-agent-not-proof-of-real-cli: a green focus over the fake hyprctl is no proof of the real compositor; matrix v2-11 stays gated on R3.
@task_item n=6 part=touches
contract/cli-exit-codes/21 and the exit-21 detail and instance-unreachable cause rows: focus is a new verb that exits 21 for instance-dead, unwrapped-name and strict-modes-failed before any hyprctl read.
@task_item n=7 part=touches
contract/strict-modes-refusal/other-verbs and decision/strict-modes-before-any-log: focus checks strict modes on the home and instance before any log line.
@task_item n=8 part=touches
decision/home-another-user-may-read-refused and decision/home-readable-by-other-refused: focus refuses a home another user may read, asserted by its own integration case.
@task_item n=9 part=touches
contract/viola-help/verb-table: the board row now reads board: list, focus (PROVISIONAL, F-B).
@task_item n=10 part=touches
decision/each-exit-cause-own-hint and decision/hint-names-its-cause: six new exit-1 causes each carry their own fixed unable and hint pair (PROVISIONAL, F-C).
@task_item n=11 part=touches
decision/error-bodies-held-by-surface-witnesses and decision/error-writers-each-named: the error-body census gained six focus documents, each {"v":1,"error":code,"detail":null}.
@task_item n=12 part=touches
decision/diagnostics-schema-conformance and decision/closed-enums-need-decisions-log: the closed process-exit detail enum grew from 21 to 27 words with the six focus codes.
@task_item n=13 part=touches
decision/cli-json-one-document and contract/cli-json/one-document-per-verb: focus --json prints exactly one document on success and on each refusal.
@task_item n=14 part=touches
decision/socket-directory-refusal and decision/socket-dir-check: the loosened socket directory table's focus row reads exit 1 unsupported, not socket-dir-refused, since focus connects to no socket and reads liveness only.
@task_item n=15 part=touches
decision/integration-binary-surface-prefix: a new root integration binary under the cli surface prefix for focus.
@task_item n=16 part=touches
decision/no-untied-viola-spawn, decision/wrapper-drop-guard and decision/test-started-viola-drops-env: the focus integration cases start opened, hand-started and bare wrappers held by a drop guard.
@task_item n=17 part=touches
decision/tests-never-read-host-sessions: the R3 rehearsal runs every child under a scratch HOME and XDG_RUNTIME_DIR so nothing reads the real claude config.
@task_item n=18 part=touches
decision/mocks-only-at-product-seams and decision/rstest-case-tables: focus's unit cases drive the hyprctl seam through a closure, with table rows for each seam word and for unread, special and zero workspaces.
@task_item n=19 part=touches
decision/frame-verbs-reach-only-the-recorded-wrapper and decision/endpoint-read-from-snapshot: focus reads liveness and the snapshot's recorded window address, and uses that address only while its window's pid is in the wrapper's ancestry.
@task_item n=20 part=touches
decision/single-writers and decision/replay-writes-no-file: focus writes no snapshot or event line, asserted by an integration case.
@task_item n=21 part=touches
decision/human-output-pinned-by-literals, decision/cli-outputs-pinned-by-literal and decision/no-sgr-without-tty: focus's success line and refusal pairs are fixed literals, plain ASCII with no SGR, written once.
@task_item n=22 part=touches
decision/mcp-no-control-tools and contract/mcp-tools-list/tool-set: no MCP focus tool was added; a driver-facing focus is left out on fork F-A (PROVISIONAL).
@task_item n=23 part=touches
decision/no-secrets-in-logs and decision/logs-carry-no-secrets: focus logs as cli under span cli.focus with no address, pid or workspace in any line.
@task_fact n=1 slug=8-mocking-stubbing-discipline
fact: The fake hyprctl appends each dispatch argv it is asked to a hyprctl-asked receipt beside its exe, one JSON array line per append, in every mode; its terminal mode answers the window focus on its own window and the workspace focus on its workspace with the recorded FocusWindow and Workspace replies.
@task_fact n=2 slug=e2e-test-strategy
fact: R3 is a gated live reading of viola focus beside R2, raising one hand-started and one opened session on the operator's screen; its rehearse form over the fakes is never a live reading, and its verdict words come only from the plan's closed set (PROVISIONAL, F-D).
@task_fact n=3 slug=e2e-test-strategy
fact: viola focus uses a recorded window address only while that window's pid is in the wrapper's ancestry, else the nearest ancestor pid's window; a recorded address outside the ancestry is never focused, asserted by a negative case.
@task_fact n=4 slug=e2e-test-strategy
fact: viola focus exits 1 with window-not-found (no dispatch), unsupported (off Hyprland, nothing spawned), unreachable (no compositor answering, PROVISIONAL F-E), oversize, malformed or refused, each a fixed pair and a closed process-exit detail word, and exits 21 before any hyprctl read for a session not live.
@task_fact n=5 slug=e2e-test-strategy
fact: viola focus dispatches the workspace before the window, and skips the workspace dispatch for an unread, special or zero workspace.
@task_item n=1 part=untouched
The harness CLI, its session record, boot, cleanup, status, run, gate and pre-push contracts: no harness command or exit code moved.
@task_item n=2 part=untouched
CI integration, coverage floors, mutation, fuzzing, perf budgets and supply-chain gates: no workflow, threshold, tool pin or dependency changed.
@task_item n=3 part=untouched
The send, answer, wait, last, release, link and revive contracts and refusal orders: focus sends no frame to the wrapper channel and none of those verbs changed.
@task_item n=4 part=untouched
Hook contracts, hook deadlines and decision bodies: no hook path changed.
@task_item n=5 part=untouched
Recorded Claude fixtures, verify stamping and fixture scrub or hygiene rules: no claude fixture was recorded or changed.
@task_item n=6 part=untouched
The wrapper channel JSON-RPC errors and the MCP tool result and invalid-params contracts: no MCP tool or channel method was added or changed.
@task_item n=7 part=untouched
PTY, passthrough, vt100 screen and wheel rules: focus never touches the child's terminal or the wheel.
