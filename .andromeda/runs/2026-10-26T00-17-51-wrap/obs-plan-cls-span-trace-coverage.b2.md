# Classified — obs-plan · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · no · the wheel code is not among the reached files and no touch names run.wheel_transition or the wheel
span-trace-coverage#7.3 · no · the report clears the send-refused product line and its fields, the channel-request/response shape with its corr and conn fields, and the budget-gate record's asserted events
span-trace-coverage#7.4 · no · the wheel code and its events.ndjson audit record are not among the reached files and no touch names either
span-trace-coverage#8.1 · yes · stale · the test-capture change reaches src/cmd/hook/statusline.rs and src/run/budget.rs whole, where hook.statusline, statusline.shell_out and run.budget_eval open, and the report clears only their asserted log lines, not the spans
span-trace-coverage#8.2 · yes · stale · the change reaches src/cmd/hook/statusline.rs and src/run/budget.rs whole, where these span attributes and the no-span statement for BudgetGate::lift sit, and the report clears only asserted log lines
span-trace-coverage#8.3 · no · the report clears the statusline and budget-gate asserted events, fields and levels and the send-refused budget-paused line, and the hook-stdin parse-rejected line lives in src/cmd/hook.rs, which the change did not reach
span-trace-coverage#8.4 · yes · stale · the change reaches src/cmd/hook/statusline.rs whole, where the statusline.shell_out span closes, and the report clears the hook-decision line and the hook timing but not the span close or the stdout pass-through
span-trace-coverage#9.1 · no · the R8 strip lives in viola-agent-claude, which the change did not reach, and the run-start asserted events, fields and levels are unmoved
span-trace-coverage#10.1 · no · E3 is an untouched edge flow (no viola-state or links code changed) and its channel-request/response shape is unmoved
span-trace-coverage#11.1 · no · E5 is an untouched edge flow (no viola-state or links code changed) and the one new obs site is a test-only witness, not a product emitter
span-trace-coverage#12.1 · no · the torn-append rule is unmoved (the healing appender writes one line per heal), the appender lives in viola-state, which no code changed, and the witness site is test-only
span-trace-coverage#13.1 · yes · stale · the run-start gate that opens run.version_gate is code the change reaches whole, and the report clears its asserted lines but not this span
span-trace-coverage#14.1 · no · the verify and plugin install edge flow and its uninstrumented capture arms are untouched, neither verb's code changed, and no CI step or gate changed
