@task_prose part=session
The session built the scaffold of the terminal fleet console as a new CLI verb, viola console, the human's own terminal surface: an alternate screen with the cursor hidden, a frame at or above an 80x24 floor (title on the first row, q quit on the last), one line naming both sizes below it, a redraw on resize, and the terminal restored on every end before anything else is printed.
The console reads keys in raw mode through a bounded reader (reads of at most 64 bytes, each decoded to a closed class quit, interrupt or unbound; key bytes never logged); q, Ctrl-C and input end exit 0.
Before any log opens it refuses, exit 1 with a fixed unable and hint pair and no line written, when stdin or stdout is no terminal or the home fails the strict-modes read rule; it then logs under its own fifth process role, console, to console.ndjson, with its own process-start (no endpoint_kind) and process-exit.
The obs process roles moved 4 to 5 and the closed diagnostics event enum 16 to 18 (console-frame, console-key) in both diagnostics schemas; the help's board group now lists list, focus, console; the catch-site internal-error role line owners are now run, cli and console.
Two internal mends rode along: the capability ledger's relied-row count is now one public constant with one pinned value (29), and verify's typed local-command probe turns share one private paste-and-wait step (latencies read at the answering poll); no dependency was added and nothing was retired.
@task_file n=1 path=src/cmd/console.rs
what: the viola console verb's start: the terminal check, the home's strict-modes read rule before any log, the console's own process-start and process-exit, and its two fixed refusal pairs
@task_file n=2 path=src/console/mod.rs
what: the terminal fleet console's loop and the guard that enters the alternate screen and restores the terminal on drop, size re-read and redraw
@task_file n=3 path=src/console/frame.rs
what: the console's 80x24 floor, the fit check on both axes
@task_file n=4 path=src/console/keys.rs
what: the console's key reader thread, a bounded 64-byte read decoded to a closed key class
@task_file n=5 path=src/human/console.rs
what: the console's terminal writer inside the one human writer of the root bin, plain ASCII with no SGR
@task_file n=6 path=tests/tui_console.rs
what: the end-to-end tests of the console under its own pseudo-terminal and test home (frames, keys, restore, refusals, schema check)
@task_file n=7 path=crates/viola-core/src/obs.rs
what: the obs process roles and the closed diagnostics event catalog (ObsProcess, ObsEvent) in viola-core
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostics line schema: its process enum, event enum, per-event fields and role-file list
@task_file n=9 path=schemas/diag-detail.v1.json
what: the instance diagnostics detail-line schema: its process and event enums
@task_file n=10 path=src/obs.rs
what: the root bin's diagnostics setup: role file names per process role and the catch-site internal-error process-exit line
@task_file n=11 path=src/main.rs
what: the viola binary's entry, verb dispatch and internal-error catch site with its role mapping
@task_file n=12 path=src/cmd/mod.rs
what: the CLI verb set and the help's verb groups
@task_file n=13 path=src/human.rs
what: the one human writer module of the root bin
@task_file n=14 path=tests/cli_output_plain.rs
what: the plain human output check over every served verb and the help text
@task_file n=15 path=tests/contract_error_bodies.rs
what: the error-body census, its table of fixed stderr pairs with no document
@task_file n=16 path=tests/cross_exit_causes.rs
what: the exit-cause table test across verbs
@task_file n=17 path=tests/contract_diag_schema.rs
what: the contract test binding both diagnostics schemas' event enums to the log-format literals
@task_file n=18 path=Cargo.toml
what: the root crate manifest, its test binaries list and their required features
@task_file n=19 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger rows and the relied rows list, now with a public relied-row count
@task_file n=20 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger stamps test cases
@task_file n=21 path=src/cmd/verify.rs
what: viola verify, the one stamps writer and its relied-row checks
@task_file n=22 path=src/cmd/verify/typed/turns.rs
what: verify's typed probe turns, the local command echo and context probes and their paste and wait
@task_item n=1 part=touches
decision/terminal-fleet-console: the console now exists as a verb with a scaffold frame; no fleet view, board read or action yet.
@task_item n=2 part=touches
The project-intent overview says a keyboard-driven terminal fleet console is planned, not built: the scaffold is built, the view is not.
@task_item n=3 part=touches
The codes-only process and role log rows (decision/codes-only-process-logs, decision/codes-only-role-logs, decision/process-logs-hold-codes-only) list the roles run, hook, mcp, cli: a fifth role console now logs home-level.
@task_item n=4 part=touches
The home-level diagnostics role-file contract (contract/diagnostics-role-files items): a new role file console.ndjson, with or without an instance.
@task_item n=5 part=touches
decision/silent-roles-at-internal-error-catch and the catch-site rows (contract/cli-internal-error/output, contract/human-stderr/internal-error): the console prints error: internal error once after the restore, exit 1, and owns a process-exit internal-error line beside run and cli.
@task_item n=6 part=touches
decision/four-interfaces-one-binary and decision/new-capabilities-as-verbs: a new human-facing interface arrives as a subcommand of the one binary.
@task_item n=7 part=touches
The CLI verb rows (contract/cli-verbs items, name/command items): a new verb console takes the name, with no arguments but the global home flag and no --json.
@task_item n=8 part=touches
decision/cli-human-text-json-exit-codes and decision/cli-output-conventions: the console has no --json form; its refusals are fixed unable and hint pairs on stderr with exit 1.
@task_item n=9 part=touches
decision/run-terminal-child-only and decision/cli-verbs-human-stderr: the console writes its screen to stdout as a full-screen redrawing surface.
@task_item n=10 part=touches
decision/single-human-writer: the console's terminal writes go through a new writer inside the one human writer.
@task_item n=11 part=touches
decision/bounded-readers: a new boundary reader, the console's key input, bounded at 64 bytes a read.
@task_item n=12 part=touches
The strict-modes rows (decision/strict-check-before-log-open, decision/strict-modes-check-before-log, decision/strict-read-rule, decision/refused-check-writes-no-line): the console holds its home to the read rule before any log opens and writes no line on a refusal.
@task_item n=13 part=touches
contract/cli-exit-codes/0 and contract/cli-exit-codes/1: the console exits 0 on q, Ctrl-C or input end and 1 on no terminal, a refused home or an internal error.
@task_item n=14 part=touches
contract/diag-line/format and contract/diagnostics-lines/process-log-line: the diagnostics line gains process console and the events console-frame (cols, rows, fits) and console-key (key quit, interrupt or unbound); the console's process-start carries no endpoint_kind.
@task_item n=15 part=touches
decision/obs-event-only-emitter and decision/event-kinds-agent-neutral: two new diagnostics events added in viola-core and emitted through the one emitter.
@task_item n=16 part=touches
decision/colour-env-decides-only-sgr, decision/clap-without-color-feature and the human-output colour environment rows: the console writes plain ASCII with no SGR at all.
@task_item n=17 part=touches
decision/hybrid-concurrency and decision/sync-core-tokio-only-in-mcp: the console's key reader is a std thread; no Tokio outside mcp.
@task_item n=18 part=touches
The capability ledger and relied-rows rows (decision/cli-capability-ledger, decision/version-verified-all-relied-rows, decision/capability-ledger-gate): the relied-row count is now one public constant read at every test site, pinned at 29.
@task_item n=19 part=touches
decision/local-command-echo-probe and decision/local-command-context-probe: verify's probed local command pastes through one shared step; the echo and context latencies are read at the poll that answered the wait.
@task_item n=20 part=touches
decision/test-isolated-viola-home and decision/root-waits-share-one-bound: a new end-to-end test binary runs the console under its own pseudo-terminal and test home, gated on the fake-agent feature.
@task_item n=21 part=touches
decision/human-controls-cli-only: the console reads keys from the human, q quit and Ctrl-C only, no wheel or link control.
@task_fact n=1 slug=established-decisions
fact: viola console is the terminal fleet console's own surface: alternate screen with the cursor hidden, a whole frame at or above an 80x24 floor, one line naming both sizes below it, a redraw on resize, and the terminal restored on every end before any error line.
@task_fact n=2 slug=occupied-resources
fact: The console checks that stdin and stdout are terminals, then holds the home to the strict-modes read rule, both before any log opens; either refusal exits 1 with a fixed unable and hint pair and writes no line.
@task_fact n=3 slug=occupied-resources
fact: The console is a fifth home-level process role, console, logging codes only to console.ndjson, with its own process-start without endpoint_kind and a process-exit at its end.
@task_fact n=4 slug=cross-cutting-patterns
fact: The console's key reader is bounded: reads of at most 64 bytes, each decoded to a closed class quit, interrupt or unbound, logged by class alone and never by its bytes.
@task_fact n=5 slug=conventions
fact: At the internal-error catch site the console behaves as cli: it restores the terminal, prints error: internal error once, exits 1 and writes the process-exit internal-error line.
@task_item n=1 part=untouched
The wrapper channel: its frames, methods, params, fault codes and results; the console opens no channel connection.
@task_item n=2 part=untouched
The MCP server, its tools, list results and exit codes.
@task_item n=3 part=untouched
Hooks: the hook contract, tiers, hook-to-kind map and hook exit codes.
@task_item n=4 part=untouched
The normalised event kinds, the events log and its event data; the console reads no board and appends no event.
@task_item n=5 part=untouched
The instance snapshot, the list payload and the board; no fleet view is shown yet.
@task_item n=6 part=untouched
The wheel, dialog hold and send path decisions; the send path was read, not changed.
@task_item n=7 part=untouched
The pty seam and the wrapped child's argv and environment; the console drives the host terminal, not a wrapped child.
@task_item n=8 part=untouched
CI workflows, coverage floors, mutation, fuzzing and the pre-push gate; no gate threshold moved.
@task_item n=9 part=untouched
The Hyprland seam and window features.
@task_item n=10 part=untouched
The web, graphical and mobile frontend absences; the console is a terminal surface.
@task_item n=11 part=untouched
The cargo-deny dependency policy and its advisory ignores; no dependency was added and the chrono ignore stands.
