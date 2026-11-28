@row_set n=1 row=decision/cli-tests-as-subprocess cell=except seen=1 detector=none
value: viola console takes no --json; it is tested under its own outer PTY, its signal its exit code and its diagnostics lines.
basis: fct-ooj26e7o, fct-jyhw74pa
because: The new console verb takes no --json and is driven under a PTY, so the subprocess-with---json rule no longer covers every CLI verb.
@row_set n=2 row=decision/cli-json-one-document cell=except seen=1 fact=F3 detector=none
value: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document; viola console takes no --json, its two refusals fixed stderr pairs.
basis: fct-ooj26e7o
because: The console's two refusals are fixed stderr pairs with no document, named among the not-error-bodies, a second exception to one document per verb.
@row_set n=3 row=decision/hint-names-its-cause cell=except seen=2 detector=none
value: boot-time-unread of viola up is held by its unit pair alone, since a host's boot-time source cannot be hidden from a case; viola console's two exit-1 causes print no --json document.
basis: fct-ogupc5er, fct-ooj26e7o
because: The console's no-terminal and strict-modes-failed causes join the exit-cause table with their hints but have no --json document to assert.
@row_set n=4 row=decision/home-another-user-may-read-refused cell=rule seen=1 detector=none
value: A home another user may read refuses run, viola list and viola console, and a start under umask 000 makes every home path owner-only.
basis: fct-xexxpxch, fct-efbgxzuj
because: viola console now refuses a home another user may read before any log, so the list of refusing verbs gains it.
@row_set n=5 row=decision/home-read-rule cell=rule seen=1 detector=none
value: A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, viola list refuses the whole board, opening nothing, and viola console exits 1 with its home pair, writing no line.
basis: fct-xexxpxch, fct-efbgxzuj
because: viola console now applies the home read rule before any log opens, exiting 1 with its own pair and no line.
@row_set n=6 row=decision/home-readable-by-others-refused cell=rule seen=1 detector=none
value: A pre-existing home another user may read refuses viola run with exit 1, viola list with exit 21 and viola console with exit 1.
basis: fct-xexxpxch, fct-efbgxzuj
because: viola console refuses a widened home with exit 1, a refusal this row's verb list did not hold.
@row_set n=7 row=contract/strict-modes-refusal/21 cell=promise seen=1 detector=none
value: Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair; viola console exits 1.
basis: fct-xexxpxch
because: viola console's strict-modes refusal exits 1, so the every-other-verb promise of 21 is false for it.
@row_set n=8 row=contract/strict-modes-refusal/no-line cell=promise seen=4 detector=none
value: A refused check at run, viola revive, viola adopt, viola verify, viola focus, viola console, viola up's board scan, the frame verbs, the hook and the MCP frame tools writes no line.
basis: fct-xexxpxch, fct-efbgxzuj
because: viola console's home refusal writes no line, a new site of the no-line promise.
@row_set n=9 row=decision/strict-modes-before-any-log cell=rule seen=3 detector=none
value: Frame verbs, link verbs, starts, viola focus, viola console and viola up's board scan take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found.
basis: fct-xexxpxch
because: viola console takes the home read-rule check before any log opens, leaving the mode as found, so it joins the verbs this decision binds.
@row_set n=10 row=decision/tui-verdicts-from-records cell=rule seen=1 fact=F2 detector=none
value: TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status; the fleet console's verdicts come from its console.ndjson lines and its terminal's line mode, never from the screen.
basis: fct-jyhw74pa, fct-hjsklcas, fct-ezqophup
because: The console's E2E binary takes its verdicts from console-frame, console-key and process-exit lines and the line mode, a TUI source this row did not name.
@row_set n=11 row=contract/log-line/process cell=promise seen=1 fact=F4 detector=D-tests-obs-harness
value: One of run, hook, mcp, cli, console; cli is a short-lived verb with a resolved instance, writing its own cli role file; console writes console.ndjson with or without an instance.
basis: fct-twjnmpkt, fct-xexxpxch
because: The closed diagnostics process roles moved from four to five with console, so the harness-grepped process field must name it.
@row_set n=12 row=decision/process-logs-via-tracing-subscriber cell=rule seen=1 detector=D-tests-obs-harness
value: Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp, cli and console process in the home's diagnostics.
basis: fct-twjnmpkt, fct-xexxpxch
because: The console gains its own role file console.ndjson, so the role-file list gains console.
@row_set n=13 row=decision/structured-json-process-logs cell=rule seen=1 detector=D-tests-obs-harness
value: Every process (run, hook, mcp, cli, console) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/.
basis: fct-twjnmpkt
because: The diagnostics process roles are now run, hook, mcp, cli and console, and this row's process list named three.
@row_set n=14 row=decision/literal-test-oracles cell=except seen=2 detector=D-tests-count-literals
value: The relied-row count is read from the ledger's own count constant at every test site, held by one literal pin beside the relied list.
basis: fct-twjnmpkt, fct-dbwrolau
because: The relied count is now read from RELIED_COUNT at every test site with one literal pin, which this rule against importing ledger rows as the oracle did not allow.
@row_add n=1 row=contract/strict-modes-refusal/console area=integration-test-strategy detector=none
boundary: Strict-modes refusal
item: viola console
promise: Exit 1 with the home pair, made before any log opens: no line, the mode left as found.
basis: fct-xexxpxch, fct-efbgxzuj
because: viola console is a new verb with its own strict-modes refusal outcome, beside the per-verb items of this table.
@row_add n=2 row=decision/console-restore-read-on-own-terminal area=e2e-test-strategy fact=F1 detector=none
name: Console restore read on its own terminal
rule: The console's terminal restore is asserted by reading line mode on the console's own terminal, opened without a controlling tty, raw while the frame stands and cooked after the quit; equality with the mode before the start is not asserted.
why: The PTY seam exposes no master fd, and a read before the start races the console's own raw-mode entry.
guard: console_restores_the_terminal_on_quit
basis: fct-otdimpgw, fct-ite6x2pn, fct-qdzcb3ca
because: The chunk measured the planned master-fd read unreachable and chose a read on the console's own terminal, a test strategy later console cases follow.
@row_add n=3 row=contract/log-event-fields/console-frame area=log-format fact=F5 detector=D-tests-obs-harness
boundary: log event fields
item: console-frame
promise: Carries cols, rows and fits, all required.
basis: fct-u62b25hk
because: The closed event enum gains console-frame with its own required fields, which the console's E2E cases assert on.
@row_add n=4 row=contract/log-event-fields/console-key area=log-format fact=F5 detector=D-tests-obs-harness
boundary: log event fields
item: console-key
promise: Carries key, one of quit, interrupt or unbound, required, never the key's bytes.
basis: fct-u62b25hk, fct-xexxpxch
because: The closed event enum gains console-key with a closed key word and no bytes, which the console's E2E cases assert on.
@row_add n=5 row=contract/caught-panic-exit-code/console area=e2e-test-strategy fact=F6 detector=none
boundary: exit code on a caught panic
item: console
promise: viola console restores the terminal first on a caught panic or dispatch error, then prints error: internal error once and exits 1, with its process-exit internal-error line.
basis: fct-elardgcb, fct-xexxpxch
because: The console joins the catch site's role-line owners with a restore-then-print outcome the existing run and hook items do not state.
