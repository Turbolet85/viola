# Review feedback 1 — Phase 3.5 (overseer, founder-delegated), 2026-09-24

- Tier: Standard (1) with the Minimal exporter carve-outs — ACCEPTED.
- A: keep the four extra events as real events (`liveness-changed`, `state-recovered`, `sse-opened`/`sse-closed`, `parse-rejected`); log a tests enum amendment in the Decisions Log. The overseer applies it to test-plan.md right after this run.
- B: accept `process="cli"` + `cli-<name>.ndjson`; same tests amendment (process enum + harness `--process` filter).
- C: key absence accepted; the tests amendment states "absent = null" for `corr` / `instance`.
- D-08: accepted; log it as an arch amendment request (arch names only `instances/<name>/diagnostics/`).
- D-18: the workspace floor rises to 1.96 (security already requires toolchain >= 1.96), so sysinfo 0.39.6 MSRV 1.95 fits; log as an arch amendment request.
- Then proceed.
