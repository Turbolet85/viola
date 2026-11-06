@amendment marker=2026-11-05-linux-only-code-and-ci
title: residual Windows reasons and the one-leg perf job
section: §2 Test Strategy (body: Perf; the perf runner line); §3 (5-command implementation: the harness target dir, the coverage ignore class, the counted tests)
change:
  - The perf run is in its own `perf` job; was its own per-OS job.
  - The warm-up absorbs first-execution costs; was that, «for example Windows Defender scanning a freshly built exe».
  - The coverage ignore class is separator-agnostic, a form kept from the Windows target, where llvm-cov reports paths with `\`; was that reason stated as current.
  - The harness target dir: on the retired Windows host a running `target/debug/viola-harness.exe` could not be relinked; was stated as current.
  - Tests compiled out by an OS `#[cfg(…)]` gate are never counted; was by `#[cfg(windows)]` / `#[cfg(unix)]`.
why: the Windows target and the per-OS CI legs are retired (the founder's own rulings of 2026-11-04, inputs#I1); a residual census after the cascade sweep found this wording no sweep pattern named (`cascade-dispositions.md` §Residual census).
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
