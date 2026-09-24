
## 2026-09-24-diagnostics-plane — obs_event! field set, schema default-deny keyword
**Section:** §3 Observability Harness Contract (intro quote, Logging stack) · §8 PII Scrubbing & Compliance → Default-deny posture (+ Detail-file scope) · §11 Obs Anti-Patterns → Logs
**Change:**
- **§3 Logging stack:** `obs_event!` attaches `event`, plus `process` and `instance` from `ProcessCtx`. `corr` is a caller-supplied typed field; the macro has no dedicated arm, because such an arm is ambiguous with the generic `key = value` arm in `macro_rules`. As measured at the chunk.
- **§3 intro:** the arch harness pattern is re-stated from arch §Diagnostic output channels instead of the retired upstream quote.
- **§8 keyword:** default-deny is enforced by a top-level `unevaluatedProperties: false`, not per-event `additionalProperties: false`, which cannot see `allOf`/`if-then` properties. `diag-detail.v1.json` is self-contained, with the event enum inlined.
- **§11:** the raw-tracing ban's justification drops `corr` from what the macro guarantees.
**Why:** chunk 2026-09-24-diagnostics-plane, report Spec claims disproved #1 and Symbols/Reverted bullets.
- The §8 pair was applied as routine: accurate mechanism reconcile, invariant held.
- The §3/§11 `corr` pair was escalated and resolved WITH the operator: amend to shipped. The trade-off is closed by a CARRY on the wrapper-channel chunk: make `corr` `required` in `diag-line` for every corr-bearing event, with a negative test.

Sweep over all seven masters and the leaves:
- **Patterns:** `additionalProperties`; `attaches .event.|always attaches|attaches .{0,20}.corr.|guarantees .event., .corr.`.
- **Amended:** obs :605, :1201, :1202, :1366.
- **No change:** obs :1203 is the new text.
- **Leaf re-derived:** `.claude/rules/observability.md:24`.
- **Result:** 0 retired-claim hits remain in the masters.
- **Control:** `unevaluatedProperties` fired.
