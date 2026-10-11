
## 2026-10-11-the-board-viola-list — viola list's lines, its no-line refusal and the replay's second caller
**Section:** §1 (Vector 7 trigger, refusal telemetry); §2 (Agent-readable invariants, the panic exemptions); §4 (the cli surface row; Edge flows E5); §6 (`detail` catalog, Filesystem refusals); §10 (Bounded exemptions 1 and 7); §3 → OTel SDK init (steps 5 and 6)
**Change:**
- Four strict-modes refusals leave no line (was three): `list`'s scan of the home, `instances/` and each instance is added; the `budget.json` read that leaves none is named `read_budget_stamped`, taken by the gate (through `read_budget`) and by `list`.
- `list` (span `cli.list`) opens `cli-<name>.ndjson` only after its scan passed and only under a `VIOLA_NAME`; a refused scan (exit 21) writes no `process-start`, `process-exit` or `strict-modes-failed`, so no arm of this build writes that word on `process-exit` exit 21.
- §10: exemption 1's pre-init window names `list`'s scan; exemption 7 adds `list`'s refused scan; §2 restates both.
- E5: the replay's second product caller is `board::read`; under a name each unreadable or newer snapshot writes one `state-recovered` into `cli-<name>.ndjson`, carrying the caller's instance, never the scanned one (owed to "CLI machine contract"). The "one product caller" clause is retired.
- The `state-entry` line's call site is `board::read`: one `parse-rejected{parser:"state-entry", detail:"symlink-ignored"|"malformed", count}` per nonzero scan count, naming no entry; `board::scan` logs nothing.
- OTel init: `list` reads `config.json`'s thresholds through `read_thresholds` under `cli.list`, each rejection one per-key `config-json` line under a name (was: `run` alone reads them a second time).
**Why:** the board as built. The no-line refusal is the operator's answer at that chunk's planning, set against the founder's direction for a structured log from every process; he has not ruled on it.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/
