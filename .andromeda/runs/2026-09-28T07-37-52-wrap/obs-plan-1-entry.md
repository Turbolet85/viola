## 2026-09-28-hook-perf-gate — the `perf` job's uploads: the existing scan-gated, synthetic-input class
**Section:** §8 PII Scrubbing item 6 (Detail-file upload, Scan failure); §9 Telemetry artifact handling (the hyperfine row); §9 Step order, step 1
**Change:**
- The detail-file upload covers `diag-<os>` and the `perf` job's `diag-perf-<os>`; the `perf` job's inputs are synthetic (string-field payloads with the tests-owned canary, the seam's fixed text in `detail-hook.ndjson` only).
- The hyperfine exports are `target/agent-run/artifacts/perf-<hook>.json` (was `perf/*.json`), uploaded as `perf-<os>` only on a successful scan (was a bare `if: always()`); a failed scan withholds both and uploads `secret-scan-perf-<os>`.
- Step 1: perf no longer runs in the `test` job but in its own per-OS `perf` job with its own G2, G4 and scan (was "same per-OS job; a separate job would upload no diagnostics or collide").
**Why:** escalated (D-obs-pii) and resolved by the operator (overseer): the same channel, scan gate and synthetic-input basis as the ratified `diag-<os>`, only a second artifact name — no new crossing.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/
