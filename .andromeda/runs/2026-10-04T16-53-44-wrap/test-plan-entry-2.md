
## 2026-10-04-dialog-answers-by-dialog-id — five perf rows on an unstamped session; the mutants build bound
**Section:** §2 Test Strategy → Performance / Load · §10 Perf run rules (perf session, status) · Performance budgets (`pre-tool-use` row) · §3 → 5-command implementation (`run --perf`, `gate --require perf`, `run --mutants` and its `--package` arm)
**Change:**
- Perf was four timed rows with `pre-tool-use` "untimed until the dialog-tier chunk" on a stamped session (`stamp: true`); now five rows (`session-start`, `user-prompt-submit`, `stop`, `session-end`, `pre-tool-use`) on one unstamped session (`stamp: false`), so `pre-tool-use` times the unverified-CLI path; `gate --require perf` requires all five by name (`perf::ROWS`, re-exported as `run::PERF_ROWS`); the suite reads 7 passed on green (was 6).
- `run --mutants` (both arms) was `--build-timeout-multiplier=5`; now `--build-timeout=400` through the shared `MUTANTS_PROGRESS`, a fixed floor of 5 × the largest measured 78 s baseline, because the multiplier conflicts with it and derives a sub-second bound from a sub-second baseline.
**Why:** the dialog hook is now a registered event, so its latency row joins the gate; the build bound is the chunk's Red B fix.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/
