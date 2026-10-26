# Classified — obs · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · no · the dialog spans sit in the wrapper, outside the reached files, and hook.decision_emit rides a decision the refused hook check never makes
span-trace-coverage#7.3 · yes · the CLI answer and the hook's server-verify refusal lines keep their order after the new strict-modes check that now precedes them
span-trace-coverage#7.4 · yes · the hook's fail-open lines before dialog_id bind the refused-check path, which may log nothing and writes hook-decision only where a role file may still be opened
span-trace-coverage#8.1 · yes · pause and release run the server check before channel.request, and the new strict-modes check sits ahead of it with no channel.request on a refusal
span-trace-coverage#8.2 · no · the run.wheel_transition span belongs to the wheel worker, and the wheel is untouched beyond the pre-frame check
span-trace-coverage#8.3 · yes · the server-verification refusal on pause and release (no channel line, process-exit 21 at WARN) must hold after the new check, which adds its own exit-21 refusal beside it
span-trace-coverage#8.4 · no · the point-in-time wheel span and the wheel audit event are untouched by the chunk
span-trace-coverage#9.1 · no · the budget governor's must-trace spans are unchanged and the statusline arm already checks before its open (Not needed: budget governor shell-out)
span-trace-coverage#9.2 · no · the budget span attributes, triggers and read_budget's no-reading rule are unchanged (Not needed: budget governor)
span-trace-coverage#9.3 · yes · the statusline arm's silence on a refused instance rests on check_instance, which the chunk edits with its Windows DACL cases, so the refused-check no-line rule binds it; its budget lines do not
span-trace-coverage#9.4 · no · the statusline shell-out cleanup and its stdin rejection are unchanged (Not needed: budget governor shell-out)
span-trace-coverage#10.1 · no · the E2 R8 strip fields on process-start{subject:"claude-child"} are unchanged (Not needed: E2 R8 strip fields)
span-trace-coverage#11.1 · no · the implicit-link trace is recorded wrapper-side inside send or answer, which the chunk does not touch (Not needed: E3 link/unlink traces)
span-trace-coverage#12.1 · no · the explicit link and unlink served methods are untouched and already run check_instance first (Not needed: E3 link/unlink traces)
span-trace-coverage#13.1 · yes · E5 binds the snapshot readers the chunk edits or reaches (run's collision check, hook's VIOLA_DIR read, live_endpoint, replay, snapshot): none may write state-recovered through the new checks
span-trace-coverage#14.1 · no · the torn-append heal and its state-recovered line are the appender's and unchanged (Not needed: torn-line healing)
span-trace-coverage#15.1 · yes · the ledger/stamps.json reads bind: run's version gate through read_stamps_strict and verify's update_stamps, now strict-checked, keep cli_verified:false on a refusal with no path or stamp contents logged
span-trace-coverage#16.1 · yes · verify's update_stamps read, plugin install's marketplace tree check (cli.plugin_install opened after its log, no line on a refusal) and the uninstrumented capture arms bind
