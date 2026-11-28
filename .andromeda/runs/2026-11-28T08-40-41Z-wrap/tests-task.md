@task_prose part=session
The session added the terminal fleet console scaffold: a new CLI verb viola console, the human's own terminal surface, listed in the help's board row as list, focus, console.
The console checks stdin and stdout are terminals, then the home's read rule, before any log opens; each refusal is a fixed unable and hint pair on stderr, exit 1, no line written.
It enters the alternate screen, draws a whole frame at or above the 80x24 floor or one too-small line below it, reads keys in raw mode through a bounded 64-byte reader (q quits, Ctrl-C interrupts, anything else unbound), and restores the terminal in a drop guard before any exit or caught-panic line.
It took a fifth diagnostics process role console (its role file console.ndjson) and two new closed events console-frame and console-key, both diagnostics schemas moving with them; the closed event list went 16 to 18 and the roles 4 to 5.
It added a root E2E binary tui_console of eight cases under their own pseudo-terminal and test home, reading the console's log lines; the restore case reads line mode on the console's own terminal through its proc fd, not the PTY master.
Beside it, the relied-row count is now read from one public constant with a single pin at 29, and the verify typed-turn pastes went through one step function (paste sites 6 to 2), guards and ends unchanged.
@task_file n=1 path=src/cmd/console.rs
what: the viola console verb's entry: the terminal check, the home read-rule check before any log, its two refusal pairs, its own process-start and process-exit lines
@task_file n=2 path=src/console/mod.rs
what: the console's run loop and its terminal guard: alternate-screen enter, frame draws on size change, the restore on drop before any exit
@task_file n=3 path=src/console/frame.rs
what: the console's frame floor: whether a terminal size fits 80x24
@task_file n=4 path=src/console/keys.rs
what: the console's key reader: a bounded 64-byte read on its own thread, each read decoded to the closed key class quit, interrupt or unbound
@task_file n=5 path=src/human/console.rs
what: the console's one human-mode writer: enter, frame, too-small line and restore, plain ASCII with no SGR
@task_file n=6 path=tests/tui_console.rs
what: the console's E2E binary under its own outer PTY and test home: frame, floor, resize, unbound key by class, interrupt, terminal restore, and the two refusals, asserted on its diagnostics lines
@task_file n=7 path=crates/viola-core/src/obs.rs
what: the closed diagnostics process roles and closed event enum, and the log-format event literals; gains the console role and the console-frame and console-key events
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostics line schema: its process enum, event enum and per-event fields; gains console, console-frame and console-key
@task_file n=9 path=schemas/diag-detail.v1.json
what: the diagnostics detail-file schema: its process enum and event enum; gains console and the two console events
@task_file n=10 path=src/obs.rs
what: the root obs wiring: role file names per process (console.ndjson) and the catch-site internal-error process-exit line, now for run, cli and console
@task_file n=11 path=src/main.rs
what: the root binary's dispatch and catch site: viola console dispatched with no detail sink, its caught panic restoring before error: internal error, exit 1
@task_file n=12 path=src/cmd/mod.rs
what: the CLI verb table and viola --help's grouped verb rows; the board row now list, focus, console
@task_file n=13 path=src/human.rs
what: the human-mode output module; declares the console writer
@task_file n=14 path=tests/cli_output_plain.rs
what: the plain-output integration check over every served verb and the help literal; the served-verb list gains console
@task_file n=15 path=tests/contract_error_bodies.rs
what: the error-body census naming every product error writer; its not-error-bodies table gains the console's two refusal pairs
@task_file n=16 path=tests/cross_exit_causes.rs
what: the exit-cause table with each cause's own hint; gains the console's no-terminal and strict-modes-failed causes and a piped refusal case
@task_file n=17 path=tests/contract_diag_schema.rs
what: the diagnostics schema contract check binding both schemas' event enums to the pinned log-format literal list, now 18
@task_file n=18 path=Cargo.toml
what: the root crate manifest and its test binaries; gains the tui_console E2E binary requiring the fake-agent feature
@task_file n=19 path=crates/viola-agent-claude/src/ledger.rs
what: the agent ledger and its relied rows; gains the public relied-row count constant beside the relied list
@task_file n=20 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger's stamp test cases; read the relied count from the constant instead of a literal 29
@task_file n=21 path=src/cmd/verify.rs
what: the viola verify verb and its unit cases; read the relied count from the constant instead of a literal 29
@task_file n=22 path=src/cmd/verify/typed/turns.rs
what: verify's typed framing turns against the agent: each paste now through one step of guard, counts, paste, wait and the end, guards and ends unchanged
@task_item n=1 part=touches
contract/log-line/process and every row listing the diagnostics process roles as run, hook, mcp, cli: the roles are now five with console, its role file console.ndjson, with or without an instance.
@task_item n=2 part=touches
decision/closed-enums-need-decisions-log: the closed event enum moved 16 to 18 (console-frame, console-key) and the closed process roles 4 to 5; a row stating those counts or members is stale.
@task_item n=3 part=touches
decision/diagnostics-schema-conformance, decision/diagnostics-schema-validation and decision/jsonschema-validator: console.ndjson lines are validated against the line schema by the console's E2E cases; both schemas' process and event enums grew.
@task_item n=4 part=touches
contract/log-event-corr items: the two new events console-frame and console-key stand outside the listed corr events; a corr row per event may be owed.
@task_item n=5 part=touches
decision/codes-only-role-logs, decision/structured-json-process-logs and decision/panic-counts-read-role-files-only: a new role file console.ndjson joins the role logs these rules read.
@task_item n=6 part=touches
contract/caught-panic-exit-code/hook and contract/caught-panic-exit-code/run and contract/cli-human/internal-error: the console's caught panic now restores the terminal first, then error: internal error once, exit 1, with process-exit internal-error; the catch-site role line owners are run, cli and console.
@task_item n=7 part=touches
contract/viola-help/verb-table: the board row now reads list, focus, console; bare viola still help on stderr, exit 2.
@task_item n=8 part=touches
decision/error-writers-each-named, decision/error-bodies-held-by-surface-witnesses and decision/error-body-check-reports-place-and-class: the console's two refusal writers are named as fixed stderr pairs with no document; the console takes no --json.
@task_item n=9 part=touches
decision/hint-names-its-cause and decision/each-exit-cause-own-hint: two new console causes, no-terminal and strict-modes-failed, each with its own hint; the widened-home cause is held by the console E2E binary since it needs a terminal.
@task_item n=10 part=touches
decision/strict-modes-before-any-log, decision/home-readable-by-others-refused, decision/home-readable-by-other-refused, decision/home-another-user-may-read-refused and contract/strict-modes-refusal/other-verbs and contract/strict-modes-refusal/no-line: the console applies the home's read rule before any log opens, an absent home passing, refused with no line written and the mode left as found.
@task_item n=11 part=touches
decision/bounded-readers and decision/bounded-readers-keep-the-cap: a new external reader, the console's key reader, reads at most 64 bytes per read; the bounded-reader census reads it as bounded without change.
@task_item n=12 part=touches
decision/scan-hits-never-echo-bytes, decision/logs-carry-no-secrets and decision/no-secrets-in-logs: the console logs a key by its closed class word alone, never its bytes, with a negative case.
@task_item n=13 part=touches
decision/e2e-layer-by-binary-prefix, decision/integration-binary-surface-prefix and decision/test-only-e2e-crate: a new root E2E binary tui_console, requiring fake-agent, selected by the tui prefix.
@task_item n=14 part=touches
decision/tui-outer-pty-driver, decision/tui-verdicts-from-records, decision/no-screen-scraping-in-e2e and decision/human-tty-through-outer-pty: the console is driven through an outer PTY and its verdicts read from its log lines and the terminal's line mode, never the screen.
@task_item n=15 part=touches
decision/no-sgr-without-tty, contract/cli-human-output/no-sgr and decision/tty-asserted-only-on-verb-sgr: the console writes plain ASCII with no SGR, but does write the alternate-screen and cursor escapes on its own terminal.
@task_item n=16 part=touches
decision/no-escape-in-plain-output and contract/cli-output/no-escape-without-tty: the console never writes when not on a terminal; it refuses before any screen escape.
@task_item n=17 part=touches
decision/human-output-pinned-by-literals, decision/cli-fixed-strings-pinned-by-literal and decision/human-layouts-pinned-by-literal: the console's refusal pairs, frame and too-small line are pinned by literal.
@task_item n=18 part=touches
decision/no-untied-viola-spawn, decision/tests-create-own-home and decision/per-test-home: the console E2E cases spawn viola under their own outer PTY and a test-owned home.
@task_item n=19 part=touches
decision/version-gate-needs-all-relied-rows and decision/ledger-row-verify-probe: the relied-row count is now one public constant read at every test site, pinned once at 29.
@task_item n=20 part=touches
decision/bounded-remove-the-guard-witness: the console's restore case has a remove-the-guard control, the drop guard's restore emptied, which reads the case red.
@task_item n=21 part=touches
decision/key-after-resize-waits-for-size and decision/resize-baseline-is-spawn-size: the console re-reads its terminal size every 100 ms and on each key, redrawing on a changed size; an unread size reads 0x0 and does not fit.
@task_fact n=1 slug=e2e-test-strategy
fact: The console's terminal restore is asserted by reading line mode on the console's own terminal through its proc fd opened without a controlling tty, raw while the frame stands and cooked after the quit, because the PTY seam exposes no master fd; equality with the mode before the start is not asserted.
@task_fact n=2 slug=e2e-test-strategy
fact: The fleet console's E2E cases run under their own outer PTY and test home, start no real claude, and take verdicts from console.ndjson lines (console-frame, console-key, process-exit) and the terminal's line mode, never from the screen.
@task_fact n=3 slug=e2e-test-strategy
fact: The console's two refusals (no terminal, a home another user may read) are fixed stderr pairs with no document, named among the not-error-bodies, and the console takes no --json.
@task_fact n=4 slug=log-format
fact: The diagnostics process roles are run, hook, mcp, cli and console; the console logs to console.ndjson with or without an instance, its process-start carries no endpoint_kind.
@task_fact n=5 slug=log-format
fact: console-frame carries cols, rows and fits, all required; console-key carries key, one of quit, interrupt or unbound, required, never the key's bytes.
@task_fact n=6 slug=e2e-test-strategy
fact: A caught panic or dispatch error in viola console restores the terminal first, then prints error: internal error once, exit 1, with its process-exit internal-error line.
@task_item n=1 part=untouched
The test harness contract: no harness verb, status shape, session record, boot readiness or exit code changed.
@task_item n=2 part=untouched
CI integration and supply chain: no workflow, artifact, action pin, cache or cargo-deny ban changed; no dependency was added and the advisory ignore list is unchanged.
@task_item n=3 part=untouched
Coverage, mutation and performance gates: no threshold, floor, kill line, hook deadline or perf budget moved.
@task_item n=4 part=untouched
The MCP server, wrapper channel, JSON-RPC errors, send, wait, answer, release, link and dialog contracts: none of their shapes, codes or details changed.
@task_item n=5 part=untouched
Fake agent, fixtures and recorded hook payloads: no fake-agent option, receipt, script, screen or fixture set changed.
@task_item n=6 part=untouched
viola run, revive and list start and refusal contracts, the instance snapshot, heartbeat and event log records: untouched.
@task_item n=7 part=untouched
Mocking discipline and the anti-pattern rules: no seam trait, clock seam or mock was added.
