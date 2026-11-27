# Light gate — wrap 2026-11-27T21-41-23Z-wrap (chunk 2026-11-27-epoch-5-cleanup)

`gate.py run --chunk 2026-11-27-epoch-5-cleanup --run-dir .andromeda/runs/2026-11-27T21-41-23Z-wrap` (no skip, no
void), before it `hostwatch.py read --last 15`: `verdict QUIET - 0 s stalled on IO`, `df -m /tmp` 13% used.

Summary: `entries 33 · green 24 · red 0 · recorded 1 · timeout 0 · not-run 8`.

- 1–21, 23–25 green by their `expect` (18 and 19 exit 1 by design, their precondition lines).
- 22 `recorded` (the host read; nothing asserted).
- 26–32 `leg = 'operator'`, re-verified by `evidence/`: 26 `evidence/operator-26.txt` (`Baseline Success 372`, run
  `2026-11-27T20-43-40Z-implement`); 27–32 `evidence/operator-pass.md` (each green, HEAD `328c22d8` = upstream).
- 33 `ci.py conclusion --sha HEAD` (operator leg, not fired by the pass): read here by `gh run view 46071784912` —
  `success`, `completed`, headSha `328c22d85f0e03bdd363f166c3d1c918044305a0`, jobs test · lint · perf · msrv ·
  fuzz-replay · supply-chain · release each `success` (also the overseer's reading, `inputs#I8`).

Every entry holds by one ASSERT arm; no red.
