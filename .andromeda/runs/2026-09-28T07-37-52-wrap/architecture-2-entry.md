## 2026-09-28-hook-perf-gate — perf arm registry, G2 script, the `perf` CI job, hyperfine in §Stack
**Section:** §Stack and Technologies Code quality; §Occupied Resources Repository (`target/agent-run/`, `target/perf/`, `target/g2-probe/`); §Infrastructure Patterns directory tree (`scripts/`, `ci.yml`); §CI/CD Setup steps and Jobs wired today
**Change:**
- `target/perf/` builds with `--features viola/fake-agent` (was `fake-agent`); its hyperfine exports are `target/agent-run/artifacts/perf-<hook>.json`, four rows, all required by `gate --require perf` (was the separate `perf/*.json`); the perf session's `<session>/` holds its synthetic `payload-<hook>.json`.
- New `target/g2-probe/` (the `--probe` scope) and `scripts/g2-zero-panics.sh` (G2, fail-closed, exempting only `src/cmd/hook/seam.rs:<digits>`, + `--probe`).
- CI: 9 jobs / 18 check-runs (was 8 / 15, as measured at ci#36390764600); the per-OS `perf` job (hyperfine via `cargo install --locked`, `run --perf`, G2 probe then check, G4, its own scan, scan-gated `perf-<os>` / `diag-perf-<os>`, `gate --require perf`); the `test` job's G2 step runs the script; `test`, pre-push and WSL carry no perf step.
- §Stack lists hyperfine 1.20.0 (as measured on the Windows dev host).
**Why:** the perf gate lands in its own per-OS job (operator P4 fork 1), so CARRY 3 does not fire; the export path is the one `gate.rs` already read.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/
