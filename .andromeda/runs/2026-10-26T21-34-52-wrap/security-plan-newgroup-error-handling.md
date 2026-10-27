# New groups — security-plan · error-handling

## Verdicts
- fanout-security-plan--cascade-new-error-handling.md 1 · distinct · no group names `StampsError`, `PinError::StrictModes` or their fixed messages and mappings
- cascade-compare-security-plan.md 2 · distinct · lines 8–9 name `channel-unreachable` and the server-check fail-open, but no group states the hook connect over a replayed or absent snapshot
- cascade-compare-security-plan.md 3 · distinct · line 8 holds only the statusline `--capture` form; no group states the uncaptured statusline over a replayed snapshot
- cascade-compare-security-plan.md 4 · distinct · line 10 states the same unreachable result for a failed server check only; the strict-modes check before the liveness pre-check is not stated
- cascade-compare-security-plan.md 5 · distinct · no group states the six CLI frame verbs' strict-modes refusal (line 11 holds only `link` / `unlink`'s wrapper replies)
- cascade-compare-security-plan.md 6 · distinct · no group states the run / revive / verify home-tree refusals
- cascade-compare-security-plan.md 7 · twin · fanout-security-plan--cascade-new-error-handling.md 1 · its `StampsError` and `PinError::StrictModes` messages restate proposal 1's; only the `Refused::Readable` / `Symlink` messages are new
- cascade-compare-security-plan.md 8 · distinct · no group states a refused `diagnostics/` path as a failed open
