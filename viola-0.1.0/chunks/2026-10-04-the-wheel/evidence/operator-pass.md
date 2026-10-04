# Operator pass — 2026-10-04-the-wheel

Fired by hand in the implement session, on the operator's word ("Run the operator pass with the ci.py
conclusion read (leg=operator) as usual").

- Entry 17 (`bash scripts/agent-run.sh pre-push`) on the uncommitted tree, in the implement run's final block:
  green, exit 0, `"ok":true`, `"stage":"linux-tests"`.
- Entry 18 (`gate.py hygiene`): exit 0 · `hygiene: clean — read 38 (runs 37 · evidence 1) · trails 14 not read ·
  binary 0 not read by P1`.
- Entry 19: pushed `7f42d6f` (the pre-CI commit) to `origin/build/viola-0.1.0`.
- Entry 20, CI run ci#37225394452 on `7f42d6f`: `verdict: red`, 13/15 jobs green. Let finish before folding.
  - `test (macos-latest)`: `tui_wheel::path5_human_takes_the_wheel_and_release_returns_it` — `release --json`
    answered `{"wheel":"human"}`. Cause (product): the wrapper appended the human's `prompt-submitted` line and only
    then moved the wheel, so the test saw the line, `release` returned the wheel, and the late move took it back.
    Fold: the move is made before the line is appended (`src/run/send.rs`), witnessed by the unit case
    `send_an_unsent_human_prompt_takes_the_wheel_before_its_line_and_a_harness_one_does_not`.
  - `test (windows-2025)`, CARRY §8 (`^Z`): `tui_ctrl_z_reaches_the_child_and_later_keys_still_do` timed out on the
    first key. Measured in the kept home `viola-test-wvRdGs` (artifact `diag-windows-2025`): the child started at
    18:44:06.336Z, sent one `hook.event`, and exited `child_exit_status 0` / `handle-wait` at 18:44:06.388Z with no
    Ctrl-C and no channel call. The fake agent returns 0 only on `0x03` or a 0-byte read, and the wrapper keeps the
    child's writer open past its own input's end, so the `0x1A` crossed viola's console reader and the fake agent's
    own std console read (rust-lang/rust#38274) turned the lone `^Z` into end of input. Fold: the fake agent reads
    its console through `viola_pty::host_stdin()` (test-side; the real CLI does not read through std).
  - `test (windows-2025)`, CARRY §9 (focus reports): `tui_focus_mouse_and_resize_never_take_the_wheel` timed out
    waiting for 16 key receipts with some `key` receipts present (home `viola-test-9LTjdV`); which bytes arrived is
    not in the artifacts. Fold: the wait now names each key that arrived and the `wheel` record count in its report,
    so the next `windows-2025` read is the measurement.
- Local red during the fold (pre-push, coverage stage): 1450 tests passed, then `llvm-profdata merge` refused
  `viola-2660688-…_16.profraw` (77 496 B, header corrupt; the common size is 80 848 B). Not reproduced in 3 further
  pre-push runs after the fold, nor in the final block. Cause NOT proven; hypothesis: the harness `cleanup` presses
  Ctrl-C every 500 ms, and the wrapper's new exit-time wheel flush ran after the terminal left raw mode, so a
  repeated Ctrl-C was a SIGINT killing the wrapper during its profile write. The flush now runs before the terminal
  is restored (`src/cmd/run.rs`). Open for the wrap with this basis.
