# Classified — obs · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · no · wait.block's after/timeout_ms and run.wait_dispatch's woken_kind are set on the successful wait path, which the chunk does not change
span-trace-coverage#7.3 · yes · the wait/last client process-exit detail list at WARN and the instance-check refusal shape (exit 21 pair, --json document, no frame, no line) sit where the new connect-time euid and socket-directory refusals land
span-trace-coverage#7.4 · yes · wait.block closes on a server-verification refusal before any frame, which the new client-side euid and socket-directory refusals before the first frame reach
span-trace-coverage#8.1 · yes · the dialog hook's and CLI answer's server check before each frame is where the new client-side euid check runs, and the instance check order before the frame applies
span-trace-coverage#8.2 · no · dialog_register, dialog_await and decision_emit attributes belong to the dialog spans, which a pre-frame refusal does not open
span-trace-coverage#8.3 · yes · the hook's fail-open line shape (hook-invoked corr:null, hook-decision with a fail-open detail, no frame) and the answer refusal lines are where the new refusals are written
span-trace-coverage#8.4 · yes · the fail-open clause before dialog_id (hook-invoked corr:null plus hook-decision detail from the Section 6 fail-open codes) is the path the new hook refusals take
span-trace-coverage#9.1 · yes · pause and release run the server check before channel.request, and the chunk's client-side euid check on every Unix client sits in that same slot
span-trace-coverage#9.2 · no · run.wheel_transition's wheel_from, wheel_to and cause attributes are not touched by a connect refusal
span-trace-coverage#9.3 · yes · a server-verification refusal on pause/release before any frame writes no channel line and exits 21 at WARN, the rule the new pre-frame refusals on those verbs must meet
span-trace-coverage#9.4 · no · run.wheel_transition is a point-in-time span and the wheel product event is the audit record, neither of which the chunk changes
span-trace-coverage#10.1 · no · the budget governor spans are out of scope (no budget behaviour moves) and statusline does not connect
span-trace-coverage#10.2 · no · budget span attributes (budget_write, shell_out, budget_eval) are not reached by the chunk
span-trace-coverage#10.3 · no · statusline hook lines are not reached: the arm opens no socket connection the chunk changes
span-trace-coverage#10.4 · no · statusline.shell_out cleanup and the STATUSLINE_DEADLINE bound are out of scope
span-trace-coverage#11.1 · no · the R8 strip's env_stripped_* and env_kept fields are out of scope (no child-spawn environment changes)
span-trace-coverage#12.1 · yes · link and unlink are among the six frame verbs and the implicit link record: a refused call (refused ahead of any frame by the new checks) records nothing and writes no send/answer channel pair
span-trace-coverage#13.1 · yes · the driven wrapper's link/unlink trace is the channel pair with method link|unlink, and the wrapper-side euid refusal on a link connection must not write that pair or a link line
span-trace-coverage#14.1 · no · the state-recovered replay and its live_endpoint callers are out of scope (the chunk does not change snapshot replay), and the new refusals sit after the liveness pre-check without altering its line
span-trace-coverage#15.1 · no · torn-line healing is out of scope
span-trace-coverage#16.1 · no · the capability ledger (stamps.json) is not reached by the chunk
span-trace-coverage#17.1 · no · verify and plugin install's home-tree check and their process-exit codes are not reached: the chunk edits only board::scan's home check and the socket-directory checks
