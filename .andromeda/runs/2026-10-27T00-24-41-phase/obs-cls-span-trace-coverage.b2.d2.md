# Classified — obs · span-trace-coverage.b2

## Answers
span-trace-coverage#7.1 · yes · the dialog hook's refused or absent-instance silent no-op (exit 0, no line, no frame, no decision) is the hook fail-open the Needs keep (the dialog flow itself is Not needed)
span-trace-coverage#7.2 · no · the dialog span attributes (run.dialog_register, run.dialog_await, hook.decision_emit) belong to the Dialog → answer scenario the scope marks Not needed, with no dialog path change
span-trace-coverage#7.3 · yes · the hook's refused or absent-instance silent no-op (exit 0, empty streams, no hook-invoked, hook-decision or line, no frame) is the fail-open the Needs keep (the dialog and server-verify lines are Not needed)
span-trace-coverage#7.4 · no · the dialog hold expiry and its dialog-expired and deadline_hit lines belong to the Not-needed dialog path, and the fail-open pair names no refused or absent instance
span-trace-coverage#8.1 · no · the wheel and release scenario is Not needed (no wheel change), and its pause and release server checks are not named by the Needs
span-trace-coverage#8.2 · no · run.wheel_transition's attributes belong to the Not-needed wheel scenario (no wheel change)
span-trace-coverage#8.3 · no · the send-refused human-typing, release-from-driver and budget-gate lines belong to the Not-needed wheel and release scenario (no wheel change)
span-trace-coverage#8.4 · no · the point span run.wheel_transition and the wheel audit event belong to the Not-needed wheel scenario (no wheel change)
span-trace-coverage#9.1 · no · the budget governor scenario and the statusline shell-out are Not needed (the chunk touches no budget gate or status line)
span-trace-coverage#9.2 · no · the budget span attributes (state.budget_write, statusline.shell_out, run.budget_eval) belong to the Not-needed budget governor
span-trace-coverage#9.3 · yes · the statusline arm's silent no-op without a valid VIOLA_NAME or on a refused instance (exit 0, no role file, no line) is the hook fail-open the Needs keep (its budget and shell-out lines are Not needed)
span-trace-coverage#9.4 · no · the statusline shell-out bound and the oversize or malformed stdin no-op belong to the Not-needed budget path, and the named silent no-op covers refused or absent instances only
span-trace-coverage#10.1 · no · the R8 strip's env_stripped and env_kept fields on claude-child process-start are not reached (the chunk changes no child spawn environment, and the Needs name only the sideload_fallback value on that line)
span-trace-coverage#11.1 · no · the implicit link record in send and answer is E3, Not needed (no run::links or state appender change)
span-trace-coverage#12.1 · no · the explicit link and unlink methods are E3, Not needed (no channel method or state appender change)
span-trace-coverage#13.1 · no · the state-recovered lines (snapshot-replayed, snapshot-unsupported-v) are E5, Not needed (no state reader or appender change)
span-trace-coverage#14.1 · no · the torn-line heal is the events.ndjson appender's, Not needed (no state appender change)
span-trace-coverage#15.1 · no · the ledger read and cli_verified are Not needed (stamps untouched), and the Needs' witness reads hook events and events.ndjson offsets, not this line (the unstamped boot runs the existing branch)
span-trace-coverage#16.1 · yes · verify's pin-step logging binds: process-start and process-exit go to cli-<name>.ndjson only with VIOLA_NAME, and the home-tree check runs before the log with a refused tree writing no line (src/cmd/verify.rs edits the pin_exe call)
