@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: `
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project prese
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=54 class=retired
leaf-text: - The budget pause (as landed, chunk 2026-10-10-budget-governor) is a bound on the driver only: an automated `send` is refused `budget-paused` (exit 11) until the window's reset or `viola release <name> --budget`, and no key is blocked, refused or delayed. Its reading: `path6_a_reading_over_the_threshold_refuses_the_send` (`tests/cli_budget.rs`) — a human key reaches the child under a closed gate and the outer PTY holds none of viola's literals, green in CI (run 38091081413). `answer`, `wait`, `last` and `pause` are not gated: read in the diff, and held since chunk 2026-10-10-session-links by `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (each gives under a closed gate the exit code and document it gives under an open one).
source: ctr-2go66yec «Retired (was budget-paused) and never reused; an older wrapper's budget-paused refusal reads as unknown, exit 14.»
replace-with: - The budget gate on automated sends is removed whole (its thresholds, the `budget-paused` refusal, `release --budget` and its Path 6 guards); exit 11 is retired (was `budget-paused`) and never reused, and an older wrapper's `budget-paused` refusal reads as unknown, exit 14.
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=22 class=stale
leaf-text: `expired`, `budget-paused`, `unverified-cli`
source: dec-2ivfnsdr «Each board state is carried by its printed word (stale, DIALOG <kind>, expired, unverified-cli, idle, busy, n/a), never by colour alone.»
replace-with: `expired`, `unverified-cli`
