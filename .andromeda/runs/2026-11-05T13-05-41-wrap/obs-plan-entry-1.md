@amendment marker=2026-11-05-linux-only-code-and-ci
title: the perf job on its one leg, PID reuse without its OS
section: §8 (pii-scrubbing-compliance.md: the scanned artifacts); §9 CI Integration (body: the perf gates; ci-integration.md: the perf rows); §10 SLO Invariants & Telemetry Budgets (body: the spine-hook rows); §3 (contracts/trace-context-propagation)
change:
  - The hyperfine perf gates, the spine-hook rows and the `diag-perf-<os>` artifact are read in the `perf` job; was the per-OS `perf` job.
  - Trace context: «PIDs are recycled»; was «PIDs are recycled, quickly on Windows».
why: the CI matrix is `[ubuntu-latest]` (the founder's own rulings of 2026-11-04, inputs#I1); doc-agent remarks and a residual census after the cascade sweep found this wording no sweep pattern named (`cascade-dispositions.md` §Residual census).
kept: `exit_source:"kill-fallback"` (`ExitSource::KillFallback` stands in `viola-pty`) and the `junit-<os>` upload (`junit-${{ matrix.os }}` in `ci.yml`), both true.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
