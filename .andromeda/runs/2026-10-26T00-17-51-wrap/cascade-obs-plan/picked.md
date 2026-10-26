### span-trace-coverage#8.1 · values · scenario-budget-governor · ## 4. Span / Trace Coverage › #### Scenario: Budget governor · lines 133–133 · stale
- **Must-trace spans:** `hook.statusline` (SERVER) › `state.budget_write` › `statusline.shell_out` (CLIENT). In `run`: `run.budget_eval` (INTERNAL).
### span-trace-coverage#8.2 · values · scenario-budget-governor · ## 4. Span / Trace Coverage › #### Scenario: Budget governor · lines 134–139 · stale
- **Required span attributes:**
  - `state.budget_write`: `five_hour_pct`, `seven_day_pct`;
  - `statusline.shell_out`: `shell_exit_status`, `duration_ms`;
  - `run.budget_eval`: `trigger` (`start|send|turn-ended`), `paused`, and `window`, recorded only when paused. As landed (chunk 2026-10-10-budget-governor) it is declared once, on `BudgetGate::decide_at`, `skip_all`, and opened once per evaluation: at start, at a `send`'s arrival, and after a `turn-ended` hook line is appended.
  - Two operations on this path carry no span and no line of their own, as landed: `viola_state::budget::read_budget` (a refused, unreadable, over-cap or other-`v` `budget.json` reads as no reading) and `BudgetGate::lift` (`release` with `budget: true`), whose trace is the `channel-request` / `channel-response` pair of method `release`, the `budget-gate` record and the snapshot's two budget fields. A `budget-gate` record is appended only when the triple (paused, window, override end) changes.
  - Never `statusline_command`.
### span-trace-coverage#8.4 · values · scenario-budget-governor · ## 4. Span / Trace Coverage › #### Scenario: Budget governor · lines 147–147 · stale
- **Cleanup:** `statusline.shell_out` closes on child exit or at the bound `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`); the bound ends in `hook-decision{detail:"deadline", deadline_hit:true}`. The user's stdout passes through unchanged and is never logged. Oversize or malformed stdin prints nothing and runs no shell.
### span-trace-coverage#13.1 · values · paths · ## 4. Span / Trace Coverage › #### Scenario: Budget governor · lines 179–182 · stale
- **Capability ledger** (`ledger/stamps.json`, obs-scope §1 persistent stores):
  - Read inside `run.version_gate` as the child span `state.ledger_read`. Its outcome is the existing `cli_verified` on `process-start{subject:"claude-child"}`. A missing stamp gives `cli_verified:false` and the Scenario 4 `unverified-cli` branch.
  - An unreadable or unparsable ledger, or one the strict-modes check refuses (`run`'s version gate reads it through `viola_state::stamps::read_stamps_strict`), gives `parse-rejected{parser:"ledger-stamps", detail:"unreadable"|"malformed"|"strict-modes-failed", count}` in `run-<name>.ndjson` and is treated as unverified (`cli_verified:false`).
  - The path and stamp contents never appear.
### ci-integration#2.2 · values · ci · ## 9. CI Integration · lines 23–23 · stale
| Unit tests | In-memory `Vec<u8>` writer tests parse emitted lines with serde_json against `schemas/diag-line.v1.json` (tests-owned bodies) | nextest JUnit |
