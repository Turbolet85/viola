
## 2026-10-04-dialog-answers-by-dialog-id — strict-modes-failed on the stamps read; five perf rows
**Section:** §4 Span / Trace Coverage → Edge flows (capability ledger) · §6 Log Coverage → `parse-rejected` detail catalog · Filesystem refusals · §9 CI artifacts (hyperfine row) · §10 Performance budgets (status, the spine / `pre-tool-use` row)
**Change:**
- `parse-rejected{parser:"ledger-stamps"}` details were `unreadable` / `malformed`; now also `strict-modes-failed` (`run`'s version gate refused the stamps file through `read_stamps_strict`; read as unverified, `cli_verified:false`, at `warn`, no path). The §6 closed list and the Filesystem refusals line carry it, matching `diag-line.v1.json`'s enum.
- CI `viola verify` runs at the recorded 2.1.287 (was 2.1.283).
- Perf: was four hyperfine rows with `pre-tool-use` untimed until the dialog-tier chunk; now five, `pre-tool-use` included, on an unstamped session, each required by name by `gate --require perf`.
**Why:** the chunk's strict stamps read emits the new code (the plan's `strict-modes` spelling has no place in the closed schema); the dialog hook joined the perf gate.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/
