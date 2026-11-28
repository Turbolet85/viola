@task_prose part=session
The session added a new CLI verb, viola console: the human's terminal fleet console scaffold, which shows a title line and a q quit line at or above an 80x24 floor and one too-small line below it, with no fleet data yet.
The console first requires stdin and stdout to be terminals, then holds the home to the strict-modes read rule before any log opens (an absent home passes). Either refusal writes two fixed lines, exits 1, writes no log line and leaves the modes as it found them.
It reads the human's keys from stdin in raw mode on a reader thread. Each read is bounded at 64 bytes and decoded to one of three closed classes (quit, interrupt, unbound). Each key is logged by its class word, never by its bytes, and the terminal is restored by a drop guard before any error line, a panic included.
The diagnostics catalog gained a fifth process role, console, with its own role file console.ndjson, and two events, console-frame and console-key, so the closed event list went from 16 to 18. At the catch site the console counts as a cli role and writes error: internal error, exit 1.
Two refactors changed no behaviour: verify's typed turns now paste through one private step function, with each guard and end kept, and the verify ledger exposes a relied-row count constant. The chrono advisory was re-read: no patched version exists, and the RUSTSEC-2026-0543 ignore stands with the deny config unchanged.
@task_file n=1 path=src/cmd/console.rs
what: edited (new): the viola console command, which runs the terminal check and the home strict-modes read-rule check before any log opens, holds the two fixed refusal pairs, and writes the console's own process-start line
@task_file n=2 path=src/console/mod.rs
what: edited (new): the console's loop and its screen guard, which enters raw mode and the alternate screen and restores the terminal on drop before any error is printed
@task_file n=3 path=src/console/frame.rs
what: edited (new): the console's 80x24 floor check on the terminal size
@task_file n=4 path=src/console/keys.rs
what: edited (new): the console's key reader, a bounded 64-byte stdin read decoded to the closed key classes quit, interrupt and unbound
@task_file n=5 path=src/human/console.rs
what: edited (new): the console's one terminal writer, which writes plain ASCII with no SGR and no colour
@task_file n=6 path=tests/tui_console.rs
what: edited (new): the console's end-to-end tests under their own pseudo-terminal and test home, covering key logging by class alone, terminal restore, and the no-terminal and widened-home refusals
@task_file n=7 path=crates/viola-core/src/obs.rs
what: edited: the closed process-role and event catalog of the diagnostics logs, now with the console role and the console-frame and console-key events
@task_file n=8 path=schemas/diag-line.v1.json
what: edited: the diagnostics line schema, which gains the console process, the two console events and their closed fields
@task_file n=9 path=schemas/diag-detail.v1.json
what: edited: the diagnostics detail schema's process and event enums
@task_file n=10 path=src/obs.rs
what: edited: the root diagnostics setup, which maps each role to its role file and writes the catch-site internal-error process-exit line for the run, cli and console roles
@task_file n=11 path=src/main.rs
what: edited: the CLI entry and its catch site for internal errors and panics
@task_file n=12 path=src/cmd/mod.rs
what: edited: the CLI verb list, its dispatch and the help's verb groups
@task_file n=13 path=src/human.rs
what: edited: the human-mode output writers module
@task_file n=14 path=tests/cli_output_plain.rs
what: edited: the plain human output check over the served verbs and the help
@task_file n=15 path=tests/contract_error_bodies.rs
what: edited: the census of every product error writer, where the console's two refusal writers are named as fixed stderr pairs
@task_file n=16 path=tests/cross_exit_causes.rs
what: edited: the exit-cause table of refusals, which gains the console's no-terminal and strict-modes-failed causes
@task_file n=17 path=tests/contract_diag_schema.rs
what: edited: the contract binding the diagnostics schemas' event enums to the log-format event list
@task_file n=18 path=Cargo.toml
what: edited: the root package's test targets, which gain the console end-to-end binary gated on the fake-agent test feature
@task_file n=19 path=crates/viola-agent-claude/src/ledger.rs
what: edited: the Claude adapter's verify ledger of relied rows, which gains a public relied-row count
@task_file n=20 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited: the verify stamp ledger's test cases
@task_file n=21 path=src/cmd/verify.rs
what: edited: the viola verify command and its relied-row tests
@task_file n=22 path=src/cmd/verify/typed/turns.rs
what: edited: verify's typed probe turns, which paste probe text as one bracketed paste under the human-typing and turn guards and wait for the answer
@task_item n=1 part=touches
The bounded readers census and the bounded external readers rule: a new product reader exists, the console's raw stdin key reader, bounded at 64 bytes a read. The census reads it as bounded unchanged.
@task_item n=2 part=touches
The rules over the strict-modes read rule and checking the home tree before any log opens: viola console is a new checker that holds the home to the read rule before its log opens. Its refusal leaves no trace (no line written, modes as found).
@task_item n=3 part=touches
The strict-modes refusal contracts, one item per verb: viola console has its own refusal pair (unable: viola console could not use its home, with its hint), exit 1, and no item for it stands yet.
@task_item n=4 part=touches
Every product error writer named: two new refusal writers (no terminal, home refused) are named as fixed stderr pairs with no document. The console takes no --json.
@task_item n=5 part=touches
Fixed error messages that interpolate nothing, and refusals that hold no paths, pids or ids: the console's two refusal pairs are fixed text with no path. Its too-small screen line names only the two terminal sizes.
@task_item n=6 part=touches
The catch-site internal error output and the rule holding internal errors at the unit layer: the console is a cli role at the catch site. It restores the terminal first, then writes error: internal error once, exit 1, with its process-exit internal-error line.
@task_item n=7 part=touches
User content only in diagnostics, the never-log floors and stderr's never-log floor: a key's bytes never reach any line. console-key carries its closed class word only, and console-frame carries the size and whether it fits.
@task_item n=8 part=touches
Owner-only and private diagnostics files: a new role file console.ndjson under the home's diagnostics, written by the new console role.
@task_item n=9 part=touches
A new seam or environment read needs a log entry: the console's stdin key reader is a new input seam, logged by console-key. The console's size poll is logged by console-frame.
@task_item n=10 part=touches
The CLI input trust boundary and the threat model's local surfaces: a new CLI verb reads raw keystrokes from the human's own terminal. It binds no endpoint, opens no socket and sends no channel frame.
@task_item n=11 part=touches
Closed types for protocol fields and closed code words: the key classes quit, interrupt and unbound are a closed set in the schema, and the console's process role word console joins the closed role enum.
@task_item n=12 part=touches
Escaping controls in human output and colour facts choosing SGR only: the console's writer emits fixed plain ASCII and terminal-mode sequences (alternate screen, cursor show and hide), with no SGR and no colour, and echoes no input.
@task_item n=13 part=touches
Send text typed as one bracketed paste, human typing taking the wheel, and verify never typing into a dialog: verify's typed turns now paste through one shared step. Each turn keeps its human-typing guard before the paste and its end as before.
@task_item n=14 part=touches
Verify's echo, context and cross-session probe residuals: the echo and context latencies are now read at the poll that answered the wait. The socket address is read before the cross-session guard, and a guard miss on the local paste ends with Kill when an address stood, Ctrl-C otherwise.
@task_item n=15 part=touches
The advisory ignore list and its grounds: the chrono advisory RUSTSEC-2026-0543 was re-read. It still has no patched version, crates.io's newest is the pin 0.4.45, and the ignore stands to the Epoch 7 boundary with the deny config unchanged.
@task_item n=16 part=touches
Test seams only under the fake-agent feature and tests using their own home: the new console end-to-end binary requires the fake-agent feature and runs under a test-owned home and its own pseudo-terminal.
@task_item n=17 part=touches
The diagnostics catalog's counts where a row holds them: there are now 5 process roles (run, hook, mcp, cli, console) and 18 events. The catch-site role-line owners are now run, cli and console.
@task_fact n=1 slug=input-validation
fact: viola console reads the human's keys from its own terminal in raw mode in reads of at most 64 bytes, each decoded to one closed class (0x03 anywhere is interrupt, exactly q is quit, anything else is unbound); no key byte reaches any log line or output.
@task_fact n=2 slug=authentication-authorization
fact: viola console refuses unless stdin and stdout are both terminals, then holds the home to the strict-modes read rule (an absent home passing) before any log opens; either refusal writes no line, leaves the modes as found and exits 1.
@task_fact n=3 slug=authentication-authorization
fact: The strict-modes refusal of viola console is the fixed pair unable: viola console could not use its home, then hint: make the viola home readable by you alone, then run viola console again, on stderr, exit 1, with no line written.
@task_fact n=4 slug=error-handling
fact: The console's screen guard restores the terminal (cursor shown, alternate screen left) before any error is printed. A dispatch error or a caught panic then writes error: internal error once, exit 1, with a process-exit internal-error line; the console owns a catch-site role line beside run and cli.
@task_fact n=5 slug=data-protection
fact: The console role writes its own diagnostics file console.ndjson. Its lines carry only process-start without an endpoint kind, console-frame with columns, rows and fits, console-key with a closed class word, and process-exit, and hold no key byte and no path.
@task_fact n=6 slug=threat-model-summary
fact: viola console is a local surface on the human's own terminal. It reads keys only from its own stdin, binds no endpoint, connects to no instance and sends no channel frame.
@task_item n=1 part=untouched
Secret management: credential stripping, the hermetic child environment, canary scans and the live-rig credential rows. The console spawns no child and reads no environment variable of its own.
@task_item n=2 part=untouched
The IPC channel: the socket directory, peer euid checks, server verification, frame caps and the wrapper channel method contracts. The console opens no socket and sends no frame.
@task_item n=3 part=untouched
The MCP server, its tool list, its inputs and errors, and the rmcp feature rows. No MCP tool or rmcp feature changed.
@task_item n=4 part=untouched
The hook contract, its stdin caps and its decision details, and the hook fail-open rows. No hook behaviour changed.
@task_item n=5 part=untouched
revive, run, up, list, link and adopt: their refusal codes, JSON documents and snapshot rules. None of these verbs changed.
@task_item n=6 part=untouched
CI workflows, action SHA pinning, zizmor, the release and publish rows, and the toolchain pins. No workflow or toolchain changed.
@task_item n=7 part=untouched
Fixture hygiene and the fixture refusal codes, the pinned binary and the plugin file rewrites, and the Claude-home write bans. No fixture, pinned artefact or plugin file was touched.
@task_item n=8 part=untouched
The wrapper's stdin classifier and its closed non-editing list, and the paste text character set. The console's key reader is separate from the wrapper's stdin path, and no paste validation changed.
