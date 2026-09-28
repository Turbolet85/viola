
## 2026-09-28-hook-perf-gate — CI integration: the per-OS `perf` job and its scan-gated uploads
**Section:** §Dependency Security → CI integration
**Change:** the `perf` job (`contents: read`) installs hyperfine with `cargo install --locked hyperfine@1.20.0` as its own step (no new action, no `github.event` value), runs `agent-run run --perf`, then G2, G4 and its own `secret-scan`; `perf-<os>` and `diag-perf-<os>` upload only on a successful scan, `secret-scan-perf-<os>` only on a failed one; every input is synthetic.
**Why:** an expected amendment no detector raised (Validate check 5); the uploads are the existing scan-gated, synthetic-input class (the operator's ruling, recorded in the obs sidecar).
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/
