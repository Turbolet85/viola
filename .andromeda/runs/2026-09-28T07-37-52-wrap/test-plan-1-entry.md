## 2026-09-28-hook-perf-gate — `run --perf` built, `gate` requires four rows, the per-OS `perf` job
**Section:** §2 pyramid (Performance); §3 `run` (`--perf`), `gate` (perf breach, detail codes), Bootstrap `ci-tool-install` (G2 / jq); §9 Pipeline Perf row, tools paragraph, Test report format; §10 Perf run rules (Binary under test, Perf session, Status) and the perf table (`session-end`, `pre-tool-use` rows)
**Change:**
- `--perf` is named-only: probe → `target/perf` release build (`--features viola/fake-agent`) → ONE session `perf-<harness pid>` through the `PerfSession` seam (was two sessions, stamped + unstamped) → four rows (`session-start`, `user-prompt-submit`, `stop`, `session-end`) with synthetic `--input` payloads (was fixture payloads) → zero-panic check → cleanup; suite `perf`, 6 passed on green. The verdict stays with `gate`.
- `gate --require perf` requires each row by name (`artifact-missing` per absent `perf-<hook>.json`; was the single `perf-*.json` breach).
- `pre-tool-use` untimed until the dialog-tier chunk; no async-tier row. Status: built (was "not built yet").
- CI: its own `perf` job (was "not yet in `ci.yml`"); G2 is `scripts/g2-zero-panics.sh` in the `test` and `perf` jobs; the `perf` job's scan-gated uploads join the report inventory, and no perf export lands in `harness-<os>`.
**Why:** the chunk built the arm and the job (operator P4 fork 1); measured: a kept-home run took the channel path for all 132 samples; host max 72.8–73.7 ms.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/
