
## 2026-10-04-the-wheel — the wheel scenario as landed: human-input, pause spans, release-from-driver's trigger
**Section:** §4 Scenario "Human takes the wheel…" · §4 Span kinds · §4 Instrumentation per surface (cli) · §2 Naming conventions · §1 Critical paths (Path 5) · §3 → Log format JSON schema · §6 Additive field catalog
**Change:**
- `run.wheel_transition.cause`: was `human-key|manual-pause|release`; now `human-input|manual-pause|release` (`WheelCause::as_str`), a point-in-time span with a static name, opened once per change on the wheel's worker thread.
- `pause` is traced like `release`: `pause.client` / `release.client` (CLIENT) › `channel.request(pause|release)` → dispatch, answered after the `wheel` record lands; the span kinds, the cli surface row and the `<area>` list add `pause`, `release`.
- `release-from-driver`: was "when `release` arrives from a driver rather than the CLI"; now when `release` carries a string `from` (the CLI forwards `VIOLA_NAME`), one line at INFO, `from` only when a valid name, beside the `-32602` reply; another `from` type or a non-bool `budget` is a plain `-32602` with no line.
- §1 Path 5: was "pending an enum extension"; `release-from-driver` is admitted by `schemas/diag-line.v1.json`.
**Why:** the chunk served `pause` / `release` and the `release-from-driver` refusal; `human-input` is the wheel's cause vocabulary.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/
