
## 2026-09-28-hook-perf-gate — G2's exact-path seam exemption, the D-28 over-4 KiB half, the perf rows built
**Section:** §3 Logging stack (the detail files, D-28); §9 G2 and its snippet; §10 Always-required invariant (Counting rule), Performance budgets (status, `session-end` and spine rows), error budget Definition
**Change:**
- G2 runs as `scripts/g2-zero-panics.sh` (`--probe` first, `test` and `perf` jobs) and does not count a panic line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>`; the snippet carries the exemption; the invariant still holds for the seam's line (written exactly once), only its count is exempt; the error budget's "0 panic lines" reads as G2 counts them.
- D-28: the over-4 KiB half landed (8 concurrent forced panics, 3 OSes).
- §10: status built; rows judged by `gate --require perf` on `target/agent-run/artifacts/perf-<hook>.json` (was `jq -e … perf/hook-<event>.json`); `pre-tool-use` untimed until the dialog tier.
**Why:** the G2 exemption is the one the founder ratified live on 2026-09-28 at 09:52:07 (relay the Viola overseer; security-plan Decisions Log); the rest was measured at implement and in CI.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/
