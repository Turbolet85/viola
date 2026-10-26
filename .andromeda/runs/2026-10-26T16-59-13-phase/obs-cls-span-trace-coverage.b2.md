# Classified — obs · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · no · the dialog-path spans (run.dialog_register, run.dialog_await, hook.decision_emit) run after the check on a dialog flow the chunk does not change; its fail-open lines are bound by 7.3 and 7.4 instead
span-trace-coverage#7.3 · yes · the CLI answer refusal (process-exit 21 at WARN, no channel line) is the template for the new strict-modes refusal, MCP answer writes no line, and a refused hook is a fail-open path before dialog_id
span-trace-coverage#7.4 · yes · the fail-open sentence binds the hook's new refused-check path (hook-invoked corr null, then hook-decision with a detail); the dialog-expired and backstop lines do not move
span-trace-coverage#8.1 · yes · pause and release run the server check before channel.request, so the new strict-modes check on CLI pause and release must sit at that same point, before any frame
span-trace-coverage#8.2 · no · run.wheel_transition's attributes belong to the wrapper's wheel span, which the chunk leaves untouched beyond the pre-frame check
span-trace-coverage#8.3 · yes · the server-verification refusal on pause and release (no channel line, process-exit 21 at WARN) is the template for the new strict-modes refusal on those two verbs
span-trace-coverage#8.4 · no · the wheel point-in-time span and its events.ndjson audit record are the wrapper's, and no wheel transition changes
span-trace-coverage#9.1 · no · the budget governor's must-trace spans are Not needed: the statusline arm already checks before its open and no shell-out or budget write changes
span-trace-coverage#9.2 · no · the budget span attributes (state.budget_write, statusline.shell_out, run.budget_eval) are Not needed and no budget code is reached
span-trace-coverage#9.3 · no · the statusline arm's lines and the budget-paused send refusal are Not needed; the arm's refused-check silence is held by its own early return
span-trace-coverage#9.4 · no · the STATUSLINE_DEADLINE shell-out and stdout pass-through are another unit of hook.rs; the chunk edits only the VIOLA_DIR snapshot read there
span-trace-coverage#10.1 · no · the E2 R8 strip fields are Not needed; the run start edits are the home, bin, plugin, ui and trusted-file checks, not the child's env strip
span-trace-coverage#11.1 · no · the E3 link and unlink traces are Not needed (link and unlink already run check_instance first) and the implicit link is the wrapper's, which the chunk does not change
span-trace-coverage#12.1 · no · the E3 explicit link and unlink methods are Not needed (they already run check_instance first) and the chunk reaches no link code
span-trace-coverage#13.1 · yes · E5 binds the readers the chunk edits (live_endpoint, hook's VIOLA_DIR snapshot read, run's collision check): none may write state-recovered unless the P4 fork moves them
span-trace-coverage#14.1 · no · the events.ndjson torn-line heal is Not needed (unchanged); the chunk's fs.rs reach is the owner-only mode work, not the one-write append
span-trace-coverage#15.1 · yes · a strict-modes-refused ledger/stamps.json binds run's version gate (parse-rejected ledger-stamps, cli_verified false) and the update_stamps read the chunk edits
span-trace-coverage#16.1 · yes · plugin install's strict-modes tree check, its refusal with no log, and the marketplace/ integrity work bind, as does verify's stamps read under the new check
