# Journal — 2026-10-13-owed-ledger-rows implement

One line per long run, written before the wait on it. Times are `date -u`.

Live-start ledger (the founder's cap for this chunk: 10, his dialog 2026-10-13T22:52Z): one record round
(entry 8, 5 starts) and one `run --local-live` round (entry 11, 5 starts). A red round is reported, never
re-fired. Each start is ledgered below with `date -u` before its round is fired.

- 2026-10-13T23:19:08Z — gate block entries 1–6 (lint, unit, the nine row cases), backgrounded through
  `gate.py run --only 1,2,3,4,5,6`. No live start. Read: green but entry 5 (one red, `step_lines_count_to_the_relied_total`:
  the case's own oracle wrongly read every row failing an empty probe; `largest-hook-payload` passes over no capture).
- 2026-10-13T23:20:03Z — entries 1, 2, 5, 6 re-run after the case's fix, foreground: four green. No live start.
- 2026-10-13T23:20:30Z (approx., before the call) — a pre-check outside the block, `agent-run.sh run --integration --filter
  'binary(cli_verify) | binary(hook_fail_open) | binary(hook_statusline)'`, fake agent only: 66 passed, 5 failed, all five
  `hook_statusline` cases red at the `stamped_home` fixture (the committed 2.1.287 set holds no StatusLine variant yet). No
  live start.
- 2026-10-13T23:21:12Z — the live round, fired ONCE through `gate.py run --live-legs` (entries 7–12), backgrounded. Starts
  ledgered here before the call, ten in all, the cap:
  - record round (entry 8), `claude` 2.1.287 by path: start 1 the print probe · start 2 Run A · start 3 Run B · start 4
    Run C · start 5 Run D;
  - live-suite round (entry 11), `run --local-live`, 2.1.287 first on `PATH`: start 6 the print probe · start 7 Run A ·
    start 8 Run B · start 9 Run C · start 10 Run D.
  A red at entry 8 stops the round (entry 11 then never fires); neither round is re-fired.
  Read at 2026-10-13T23:23:15Z: `round: COMPLETE · legs fired 2/2`, entry 8 green in 53.68 s (`stamped 2.1.287  21 pass
  0 fail`), entry 11 green in 55.69 s (`"suite":"local-live","passed":1,"failed":0`), both censuses empty. Ten starts made,
  the cap; no further live start in this chunk.
- 2026-10-13T23:23:54Z — the whole non-leg block (entries 1–6, 13–23, `pre-push` the last), backgrounded through a plain
  `gate.py run`. No live start. Surfaced before it: both recorded StatusLine variants hold `workspace.repo.owner`, the
  GitHub handle, which the scrub (OS user, whole word) does not rewrite; the files stay untracked until the operator's word.
  Read: 16 green, entry 23 (`pre-push`) red on `contract_windows_mutation_scope`: the harness's 18-row `LEDGER_ROWS` was
  gated `cfg(windows)` in product code. Fixed to `cfg(not(unix))` (the complement of `statusline::WRAPPED_HERE`).
- 2026-10-13T23:26:35Z — entries 2, 5, 23 re-run, foreground: three green. Then a Windows-target clippy (`target/wincheck`,
  check only) read one unused `mut` in a harness case; restructured, and the Windows-target clippy read clean.
- 2026-10-13T23:28:16Z — the whole non-leg block once more, backgrounded through a plain `gate.py run`. No live start.
- 2026-10-14T05:59Z — the founder's word on the surfaced handle (inputs#I3): accept and push. The operator pass: entry 24 (hygiene), the pre-CI commit, entry 25 (push), entry 26 (the CI read, waited up to 1800 s, backgrounded). No live start.
- 2026-10-14T05:59:57Z — entry 24 green (hygiene: clean); pre-CI commit 9ee83095ace5; entry 25 green (pushed, history moved origin/build/viola-0.1.0 14e62329→9ee83095). Now entry 26, ci.py conclusion --sha HEAD --wait 1800, backgrounded.
- 2026-10-14T06:11:18Z — entry 26 RED on 9ee83095 (ci#38519633153): test (windows-2025) ledger::tests::verified_needs_every_row_pass_under_that_version (popped ALL's last row, not relied on Windows); test (macos-latest) cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins (exit 0, want 10). Macos cause read: the first-Stop statusline replay ran before the paste-hint clear, leaving the earlier input box on screen; reproduced on Linux with a 2 s probe sleep in the replay (red at cli_send.rs:919), green with the replay moved after the turn screen (sleep kept; 80/80 over cli_send, cli_verify, hook_statusline, the drift contract). Sleep removed. Now the whole non-leg block, backgrounded. No live start.
