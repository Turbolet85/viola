
## 2026-10-10-self-healing-state — where `state-recovered` is written and how often; tracing-subscriber's dev-dependents named
**Section:** §4 Span / Trace Coverage (Edge flows) · §6 Log Coverage (Additive field catalog) · §3 → Bootstrap phases (derive for route / setup-project) · §3 → OTel SDK init
**Change:**
- §4 Edge flows, E5: the line is written by `viola_state::replay::read_snapshot_or_replay` at WARN with no `corr`: `snapshot-replayed` for an unreadable snapshot, `snapshot-unsupported-v` with `v_seen` for a newer one, one line per call, none for a present or absent snapshot and none when the replay fails. Spans `state.snapshot_recover` and `state.replay`, both `skip_all` with neither `err` nor `ret`. Library code: no product run writes these two details until a reader takes the replay (first "viola revive").
- §4 Edge flows, a new Torn append line: `state-recovered{detail:"torn-line-healed", file:"events.ndjson", offset}` at WARN with no `corr`, written by the process whose append healed, `offset` the log's length before the append; one line per heal, none for an append that did not heal, none from `try_append_event` under a held lock. The reader's per-read counts have no process-log carrier.
- §6 Additive field catalog, the `state-recovered` row: `file` is `events.ndjson` on `torn-line-healed` and `snapshot.json` on the two snapshot details; `offset` on `torn-line-healed` only; `v_seen` on `snapshot-unsupported-v` only (was the four fields unscoped).
- §3 Bootstrap phases and OTel SDK init: tracing-subscriber is a product dependency of the root bin only; `viola-channel` and `viola-state` name the workspace pin under dev-dependencies for their unit tests' line capture (was "the root bin only" and "the only crate that depends on tracing-subscriber").
**Why:** the event had no product emitter before this chunk and now has two. The enum, the schema and the allowed fields are unchanged.
**Kept:** the reader's counts stay unlogged: the catalog names `list --json` and `/api/sessions` as their only carriers.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/
