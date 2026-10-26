# obs-plan · picked accounting

## Picked
span-trace-coverage#8.1 · not touched · the report moves only the test capture call at src/run/budget.rs:859 and src/cmd/hook/statusline.rs:375, so the must-trace spans `hook.statusline`, `state.budget_write`, `statusline.shell_out` and `run.budget_eval` still hold on both reached files (no product file changed)
span-trace-coverage#8.2 · not touched · no asserted event, field or level changed in the budget gate's tests (Touches), so the span attributes, `decide_at`'s one declaration and the span-less `read_budget` / `lift` still hold on src/run/budget.rs
span-trace-coverage#8.4 · not touched · src/cmd/hook/statusline.rs gained one test line (375) and no product line, so the `STATUSLINE_DEADLINE` bound, its `hook-decision{detail:"deadline"}` ending and the unlogged stdout still hold
span-trace-coverage#13.1 · not touched · src/run/gate.rs moved only its test capture at :465 (the `set_global_default` at :429 untouched), so the `run.version_gate` / `state.ledger_read` read and its `parse-rejected{parser:"ledger-stamps"}` outcomes still hold
ci-integration#2.2 · not touched · the unit-test row's in-memory writer parse against `schemas/diag-line.v1.json` and its nextest JUnit artifact still hold: only the capture call moved, no asserted line changed, and the new Windows cases ride the existing `junit-windows-2025` artifact with no CI step changed
