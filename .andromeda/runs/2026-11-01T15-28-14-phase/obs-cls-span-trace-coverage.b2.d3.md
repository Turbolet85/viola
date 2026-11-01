# Classified — obs · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · yes · the new connect refusal on the CLI wait/last path opens inside wait.block and the dispatch span, so their attributes bind
span-trace-coverage#7.3 · yes · the wait/last process-exit detail shape at connect (during:"connect", instance pair, no frame, no line) is where the chunk's new connect refusals sit
span-trace-coverage#7.4 · yes · wait.block closes on a server-verification refusal made before any frame, the pre-frame path the chunk's client euid and socket-dir refusals take
span-trace-coverage#8.1 · yes · the answer client's server check before channel.request(answer) and the hook's check before hook.dialog are the checks the chunk's client and hook refusals run
span-trace-coverage#8.2 · yes · hook.decision_emit's decision_emitted is the value the hook's new fail-open refusal writes; the dialog_register and dialog_await attributes sit past the frame a refusal precedes
span-trace-coverage#8.3 · yes · the fail-open hook-decision detail, the dialog-path server-verify-failed shape and the CLI answer refusal lines bind the new refusals
span-trace-coverage#8.4 · yes · the fail-open rule (hook-invoked corr:null then hook-decision with a Section 6 fail-open detail) binds the hook's new refusal codes; the hold's expiry does not
span-trace-coverage#9.1 · yes · pause.client and release.client run the server check before their first frame, the same pre-frame slot the chunk's client euid check fills
span-trace-coverage#9.2 · no · run.wheel_transition is the wheel move's own span, and a refused connection never reaches a wheel change
span-trace-coverage#9.3 · yes · the server-verification refusal on pause/release (no channel pair, process-exit 21 server-verify-failed during connect) is the shape the chunk's new client refusals take
span-trace-coverage#9.4 · no · the wheel's point-in-time span and its events.ndjson audit record are untouched by a pre-frame refusal
span-trace-coverage#10.1 · no · the budget statusline spans sit outside the chunk, which opens no statusline arm and no budget write
span-trace-coverage#10.2 · no · the budget span attributes on state.budget_write, statusline.shell_out and run.budget_eval are not reached
span-trace-coverage#10.3 · no · the statusline hook-decision details and lines belong to the statusline arm, which the chunk does not reach
span-trace-coverage#10.4 · no · the statusline shell-out deadline is outside the chunk
span-trace-coverage#11.1 · no · the R8 env strip fields on process-start claude-child; no child-spawn environment changes
span-trace-coverage#12.1 · yes · a refused call (instance check or the new client refusal ahead of any frame) must write no link line and no pair, per the send/answer link rule
span-trace-coverage#13.1 · no · explicit link/unlink validation and trace shape run after a connection passes; the chunk's refusal is before any frame and writes no pair
span-trace-coverage#14.1 · yes · each live_endpoint and hook connect read writes its own state-recovered line per call, so where the new refusals fall against live_endpoint decides the line
span-trace-coverage#15.1 · no · the torn-append heal in events.ndjson is the appender's, and the chunk appends no product event
span-trace-coverage#16.1 · no · the ledger stamps strict read is run's version gate, outside the reached files and the board home check
span-trace-coverage#17.1 · yes · verify's home-tree check runs on the home that the one-rule change makes under Unix strict modes; binds if that check shares the rule
