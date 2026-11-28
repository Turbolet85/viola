@task_prose part=session
The session added the terminal fleet console scaffold: a new verb, viola console, the human's full-screen surface in the alternate screen, reading keys in raw mode, plain ASCII with no colour, a floor of 80x24 below which it draws one line.
It added a fifth obs process role, console, logging home-level to console.ndjson with no instance, and two events, console-frame (cols, rows, fits) and console-key (a closed key word, never its bytes); the closed event enum moved 16 to 18 in both diagnostic schemas.
The console refuses with exit 1 and an unable/hint pair, writing no line, when stdin or stdout is no terminal or when the home fails its read rule; both checks run before any log opens.
At the catch site the console counts as the cli role; a dispatch error or caught panic restores the terminal first, then prints error: internal error once, exit 1, with a process-exit internal-error line, so the catch-site role line owners are now run, cli and console.
It also moved two test-side carries: the relied ledger row count read from one public constant, and the verify typed turns' paste step folded into one private step function (paste_text sites 6 to 2); no dependency added, nothing retired.
@task_file n=1 path=src/cmd/console.rs
what: the viola console verb's entry: terminal check, home read-rule check, the console role's obs init and its own process-start and process-exit lines
@task_file n=2 path=src/console/mod.rs
what: the console's loop and its terminal guard: alternate-screen enter and restore, frame draws on start and on a changed size, the console-frame line per draw
@task_file n=3 path=src/console/frame.rs
what: the console's size floor of 80x24, the rule a frame fits by
@task_file n=4 path=src/console/keys.rs
what: the console's key reader: a bounded 64-byte read decoded to a closed key class, the console-key line by class word alone
@task_file n=5 path=src/human/console.rs
what: the console's one terminal writer: enter, frame, too-small line and restore, plain ASCII with no SGR
@task_file n=6 path=tests/tui_console.rs
what: the console's E2E tests under a test-owned pseudo-terminal and home: frames, keys, restore, refusals, each line checked against the diag-line schema
@task_file n=7 path=crates/viola-core/src/obs.rs
what: the closed obs process roles and closed event enum (ObsProcess, ObsEvent) and the log format event list
@task_file n=8 path=schemas/diag-line.v1.json
what: the diagnostic log line schema: its process enum, event enum and per-event additive fields
@task_file n=9 path=schemas/diag-detail.v1.json
what: the detail-file line schema: its process enum and event enum
@task_file n=10 path=src/obs.rs
what: the root binary's obs module: role file name per role, the catch-site internal-error process-exit line per role
@task_file n=11 path=src/main.rs
what: the root binary's dispatch and catch site, where a role's panic outcome and exit are decided
@task_file n=12 path=src/cmd/mod.rs
what: the CLI verb list and the help's verb groups, the board row now list, focus, console
@task_file n=13 path=src/human.rs
what: the human output module, the plain terminal writers of every verb
@task_file n=14 path=tests/cli_output_plain.rs
what: the plain CLI output test and its served-verb list
@task_file n=15 path=tests/contract_error_bodies.rs
what: the error-body contract census and its list of fixed stderr pairs that carry no document
@task_file n=16 path=tests/cross_exit_causes.rs
what: the exit-cause cross test and its table of causes per verb
@task_file n=17 path=tests/contract_diag_schema.rs
what: the diag schema contract test binding both schemas' event enums to the log format event list
@task_file n=18 path=Cargo.toml
what: the root crate's manifest and its test binaries, gaining the tui_console E2E binary
@task_file n=19 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger rows and the relied rows, gaining the public relied row count
@task_file n=20 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the capability ledger stamp test cases, reading the relied count from the ledger
@task_file n=21 path=src/cmd/verify.rs
what: the viola verify verb's tests over the relied rows, reading the relied count from the ledger
@task_file n=22 path=src/cmd/verify/typed/turns.rs
what: the verify typed turns: the framing turns' paste, guard and wait steps
@task_item n=1 part=touches
absence/interactive-terminal-interface is broken: viola console is an interactive full-screen terminal interface reading keys in raw mode; the absence leaves for a decision naming the console's own surface.
@task_item n=2 part=touches
Every row listing the obs process roles as run, hook, mcp, cli is stale: the roles are five, console added (the process field of the diag line, every log line, the process log line, the diagnostics line and the detail-file line).
@task_item n=3 part=touches
The diagnostics files rows per role and their paths gain a console role file, console.ndjson under the home's diagnostics directory, home-level with or without an instance; one log file per role and file-only per-role sinks now cover five roles.
@task_item n=4 part=touches
The stdout by role rows gain a console role whose stdout is its screen; a row counting stdout on three of four roles is stale.
@task_item n=5 part=touches
contract/panic-outcome-by-role gains a console item: terminal restored first, then error: internal error once on stderr, exit 1, a process-exit internal-error line; the catch-site role line, owned by run and cli alone, now also by console.
@task_item n=6 part=touches
The closed event enum moved 16 to 18 (console-frame, console-key) in both schemas; rows on the closed event values, the event field, enum values and the diag schema matching the event enum are reached.
@task_item n=7 part=touches
The diagnostic line additive fields per event gain console-frame (cols, rows, fits, all required) and console-key (key, one of quit, interrupt, unbound, required); the corr-by-event rows hold no rule for the two new events.
@task_item n=8 part=touches
The rule that typed keys are never logged now has a console leg: console-key carries only the closed class word, never the key's bytes.
@task_item n=9 part=touches
The rule of no telemetry from screen content: console-frame logs only the terminal size and whether it fits, nothing of what is drawn.
@task_item n=10 part=touches
The process init order and strict-modes-before-diagnostics rules gain the console: a terminal check, then the home's read rule, both before any log opens, a refusal writing no line and leaving the home's mode as found.
@task_item n=11 part=touches
The rule of no process log without an instance and the instance source per role: the console writes a home-level process log with no instance.
@task_item n=12 part=touches
The process-start line contract (service_name, version, os, pid) gains the console's own process-start, which carries no endpoint_kind; its process-exit is subject self, exit_code 0.
@task_item n=13 part=touches
The CLI stderr unable/hint refusal pair rows gain two console pairs (no terminal; home not usable), fixed and pathless, with no detail code and no line; the distinct detail code per exit cause rule meets two causes that write none.
@task_item n=14 part=touches
The role taken from argv before clap: the console has no role of its own at the catch site and falls back to the cli role while it logs under the console role.
@task_item n=15 part=touches
The cli internal error human line contract now also covers the console, written only after the terminal restore.
@task_item n=16 part=touches
The rules on short-lived verbs using stderr for humans and the run being silent on the terminal: the console is a long-lived verb that owns its terminal and draws on stdout.
@task_item n=17 part=touches
The rule escaping control characters in terminal rendering: the console's writer emits only its fixed alternate-screen and cursor sequences and plain ASCII, no SGR, never echoing a key byte.
@task_item n=18 part=touches
The additive-only diagnostic schema rules: both schemas grew by additions only (a process word, two event words, their fields).
@task_item n=19 part=touches
The no-info-in-hot-loops and no-hot-path-instrumentation rules: the console re-reads the size every 100 ms but logs a frame only on a draw, and one console-key line per read.
@task_item n=20 part=touches
Counters derived from home-level role files and the panic count over home-level files now read a fifth role file.
@task_fact n=1 slug=obs-scope-summary
fact: viola console is the human's terminal fleet console, a full-screen interactive surface in the alternate screen reading keys in raw mode; it is the one named exception to the absence of an interactive terminal interface.
@task_fact n=2 slug=1-obs-scope-summary
fact: The obs process roles are five and closed: run, hook, mcp, cli, console.
@task_fact n=3 slug=log-file-location
fact: The console role logs to the home's diagnostics/console.ndjson, home-level, under process console, with no instance.
@task_fact n=4 slug=6-log-coverage
fact: console-frame is logged once per draw with cols and rows (counts) and fits (boolean), all required; an unread size logs 0x0 not fitting.
@task_fact n=5 slug=6-log-coverage
fact: console-key is logged once per key read with key, one of quit, interrupt, unbound, required; the key's bytes never reach a line.
@task_fact n=6 slug=otel-sdk-init
fact: The console's start order is: stdin and stdout terminal check, the home's read rule with an absent home passing, then the diagnostics level read, obs init under the console role, its own process-start without endpoint_kind; a refusal before obs init exits 1 with an unable/hint pair and writes no line.
@task_fact n=7 slug=7-error-capture-reporting
fact: A console dispatch error or caught panic restores the terminal first, then writes error: internal error once on stderr, exit 1, with a process-exit internal-error line in console.ndjson; the console is classed as the cli role at the catch site.
@task_fact n=8 slug=3-observability-harness-contract
fact: The console's stdout carries its screen alone (alternate screen, plain ASCII, no colour); its refusals and internal error go to stderr.
@task_item n=1 part=untouched
The channel, JSON-RPC and peer-credential rows: no channel method, frame, error code or correlation key moved.
@task_item n=2 part=untouched
The MCP server tools, frames, stdin bounds and refusal texts.
@task_item n=3 part=untouched
The hook rows: fail-open, stdout decision body, latency and perf budgets, capture arms.
@task_item n=4 part=untouched
The send, wait, dialog and wheel path rows, their spans, refusal details and log fields.
@task_item n=5 part=untouched
The events.ndjson product stream and its event lines: the console writes no product event.
@task_item n=6 part=untouched
The OTel, exporter, error reporter, network egress and hosted deployment absences: nothing leaves the host.
@task_item n=7 part=untouched
The CI integration gates, artifact retention, secret scan and upload rules; no gate or threshold moved.
@task_item n=8 part=untouched
The heartbeat, liveness and list rows, and the run's start spans and exit-1 detail codes.
