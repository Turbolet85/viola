
## 2026-10-04-running-turn-refusal — turn-running's two causes in the confirmed-send scenario
**Section:** §4 Span / Trace Coverage → Scenario: Confirmed `send` (CL-1) from driver to readback
**Change:** the wrapper `send-refused` bullet keeps `detail` ∈ `input-not-ready|no-prompt-submitted|turn-running` and now states `turn-running`'s two causes under the one closed detail, with no new field: a running turn, or another `send` in flight. The running turn is in-memory wrapper state (marked by a `prompt-submitted` of any origin, ended by `turn-ended` / `session-start` / `session-end`, cleared by a wheel-returning `release`) that writes no event, span or log line of its own; either cause's refusal is the existing `send-refused` with no `cursor` (`corr` the end offset, D-28), no `send-issued`, client exit 13.
**Why:** the chunk widened the cause behind an existing detail and reused the one emitter, so the must-trace scenario names the cause without changing the schema.
**Ref:** .andromeda/runs/2026-10-04T22-27-20-wrap/
