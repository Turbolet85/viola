
## 2026-09-27-hooks-to-normalised-events — session-start is record three, D-28 half carried, perf rows to the "Hook perf gate" tail
**Section:** §3 Logging stack (D-28 shared detail files) · §4 Scenario 1 · §10 Performance budgets (status line; spine row)
**Change:**
- §4: `session-start{source:"hook"}` is record three, sent through `hook.event` (was "joins with Hooks to normalised events"); the harness `boot` check of it joins with "Capability ledger and viola verify".
- D-28: the concurrent-append check landed for the hook files (8 processes: 16 lines in `hook-<name>.ndjson`, 8 in `detail-hook.ndjson`, 3 OSes) without a line over 4 KiB; the >4 KiB half lands with "Hook perf gate", whose ratified panic seam writes the only hook detail line that large.
- §10: the hook rows are not built yet; `--perf`, hyperfine and the per-OS job land with "Hook perf gate". The spine gate stays tests-owned and provisionally 1.0 s; the hook's own provisional 750 ms connect deadline sits below it.
**Why:** the hooks chunk as built (P4 split; P5 review: no seam in the head).
**Kept:** §1's "provisionally 1.0 s" (:419) stands: §1 is the verbatim scope copy (playbook "Verbatim scope copy"); §10 carries the 750 ms fact.
**Ref:** .andromeda/runs/2026-09-27T23-42-19-wrap/
