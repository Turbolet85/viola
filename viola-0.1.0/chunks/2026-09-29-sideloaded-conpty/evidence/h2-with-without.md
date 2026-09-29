# H2 with and without the sideloaded ConPTY — the founder's acceptance measurement

Recorded as measured; no rate is asserted (founder ruling 2026-09-29, plan Acceptance Criteria 1).

- **Run:** ci#36563868040, commit `224efc4`, job `test (windows-2025)` (job id 109390953935), step
  `H2 loop (measurement only)`; the run's verdict: green, 15/15 checks (`ci.py conclusion`).
- **Runner image:** `windows-2025-vs2026`, version `20260828.587`.
- **Test:** the pre-`dce98ad` race shape, measurement-only behind `viola-pty`'s `h2-measure` feature: key `x`, wait
  `byte 78`, resize to 120x40, key `y` at once (no wait for the new size), then `byte 79 size=120x40` and
  `restored=true`. `tests::h2_race_sideload` pre-loads the vendored `conpty.dll` (`pty_backend` `conpty-sideload`,
  `OpenConsole.exe` beside it); `tests::h2_race_inbox` restricts the DLL search to System32 first (`pty_backend`
  `conpty`). Each iteration is one `cargo llvm-cov nextest --no-report --profile ci -p viola-pty --features h2-measure`
  run; a lost iteration is a failed run with a kept `viola-pty-watch` report.

| leg | printed line | iterations | losses |
|---|---|---|---|
| sideloaded ConPTY (OpenConsole.exe 1.24.260710001) | `h2-loop: sideload iterations 200 · losses 0` | 200 | 0 |
| inbox ConPTY (kernel32 / conhost) | `h2-loop: inbox iterations 200 · losses 14` | 200 | 14 |

- The lost inbox iterations: 18, 46, 49, 52, 73, 86, 87, 93, 95, 110, 115, 136, 166, 198.
- Every kept report has one shape (iteration 18 shown): the child reports `start pid=… raw=true size=100x30`,
  `byte 78`, `size 120x40` and never `byte 79`; the test side reports `key-written`, `key-flushed`,
  `resize-returned`, `key-written`, `key-flushed`, `dsr-cpr 0`. That is the key written right after the resize,
  lost below viola while the child read, the same class as the earlier inbox measurement (13/200, ci#36527891850).
- The legs ran one after the other in the same job (sideload first); the real `claude` key-after-resize impact is
  not measured here ("First live test and self-drive" owns it).
- The measurement is removed before the wrap (plan step 9): the removal commit, entry 27's grep, then the final
  `ci.py` read.
