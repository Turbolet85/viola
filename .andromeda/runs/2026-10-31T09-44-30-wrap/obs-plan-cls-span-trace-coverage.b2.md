# Classified — obs-plan · span-trace-coverage.b2

## Answers
span-trace-coverage#7.4 · yes · stale · the wait.block cleanup sits in the reached client verbs (src/cmd/client.rs), whose wait-path -32602 now carries a closed channel-response detail, and the task gives no verdict that the closure holds there
span-trace-coverage#8.1 · yes · stale · the answer.client and dialog spans sit in reached src/cmd/client.rs and src/run/dialog.rs, whose answer-path -32602 now carries a closed detail, and the task gives no verdict that they hold
span-trace-coverage#8.2 · yes · stale · the run.dialog_register and run.dialog_await attributes sit in reached src/run/dialog.rs, and the task gives no verdict that they hold there
span-trace-coverage#8.3 · yes · contradict · the driver-side channel-response refusal carries detail unknown-dialog, which the closed three-word detail no longer allows, since detail is absent on every other channel-response line
span-trace-coverage#8.4 · yes · stale · the dialog-expired and dialog_await cleanup sits in reached src/run/dialog.rs and the wait wake in reached src/cmd/client.rs, and the task gives no verdict that they hold
span-trace-coverage#9.1 · yes · stale · the pause and release spans and their server check sit in reached src/cmd/client.rs and the reached channel client, and the task gives no verdict that they hold
span-trace-coverage#9.2 · yes · stale · the run.wheel_transition attributes and WheelCause::as_str sit in reached crates/viola-core/src/lib.rs and the run wrapper, and the task gives no verdict that they hold
span-trace-coverage#9.3 · yes · add-to · F2 places the channel-response detail on this -32602 reply, release-from-driver on data reason and invalid-params on data null
span-trace-coverage#9.4 · yes · stale · the point-in-time wheel span and its events.ndjson audit record sit in the reached run wrapper (src/run/mod.rs, src/cmd/run.rs), and the task gives no verdict that they hold
span-trace-coverage#10.1 · yes · stale · hook.statusline and statusline.shell_out sit in reached src/cmd/hook/statusline.rs and run.budget_eval sits in reached src/run/budget.rs, and the task gives no verdict that they hold
span-trace-coverage#10.2 · yes · stale · the budget span attributes and the BudgetGate::lift trace sit in reached src/run/budget.rs, and the task gives no verdict that they hold
span-trace-coverage#10.3 · yes · stale · the statusline hook lines sit in reached src/cmd/hook/statusline.rs and the budget-paused refusal in reached src/run/budget.rs, budget-paused exit 11 stays true, and the task gives no verdict that the rest holds
span-trace-coverage#10.4 · yes · stale · the statusline.shell_out cleanup sits in reached src/cmd/hook/statusline.rs, and the task gives no verdict that it holds there
span-trace-coverage#11.1 · yes · stale · the E2 env strip sits in reached src/run/env.rs, and the task gives no verdict that its claim holds on the changed file
span-trace-coverage#12.1 · yes · add-to · the E3 channel-response for method link or unlink now carries detail invalid-params on a bad name or pair, while the no-line-of-own rule is unmoved
span-trace-coverage#13.1 · yes · add-to · each -32602 of the explicit link and unlink methods now carries channel-response detail invalid-params, while the no-line-of-own rule is unmoved
span-trace-coverage#14.1 · no · its state-recovered line and snapshot replay live in crates/viola-state, which the session did not reach and the task marks untouched
span-trace-coverage#15.1 · no · its torn-line heal lives in crates/viola-state, which the session did not reach and the task marks untouched
span-trace-coverage#16.1 · yes · stale · the run.version_gate ledger read and its parse-rejected reading sit in reached src/run/version_gate.rs, and the task gives no verdict that they hold
span-trace-coverage#17.1 · yes · stale · the verify dispatch, its home-tree check and its StampsError mapping sit in reached src/cmd/verify.rs, and the task gives no verdict that they hold
