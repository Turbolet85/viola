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
- Entry 19: pushed `5c6101c` (the first fix commit).
- Entry 20, CI run ci#37226294797 on `5c6101c`: `verdict: red`, 14/15 jobs green; macOS and Ubuntu green.
  - CARRY §8 CLOSED: `tui_ctrl_z_reaches_the_child_and_later_keys_still_do` passed on `windows-2025`: `^Z` reaches
    the child through viola's console reader, and the key after it does too.
  - CARRY §9 MEASURED: `tui_focus_mouse_and_resize_never_take_the_wheel` timed out on `windows-2025` with the watch
    report `keys [1b 5b 3c 30 3b 31 30 3b 35 4d] wheel records 2 receipt lines 16`. The child received the SGR mouse
    report alone, so the inbox ConPTY outer terminal swallows `ESC[I` / `ESC[O`. And a `wheel` record beyond the
    start one was appended, so one of the three inputs took the wheel on Windows; the reading cannot say which.
  - Fold (test-side, a measurement): the case asks the wheel after each step — a mouse report alone; the focus
    reports followed by a second mouse report as the read barrier; the resize — and expects the focus reports
    swallowed on Windows. Pre-push green before the push.
- Entry 19: pushed `126e921` (the second fix commit).
- Entry 20, CI run ci#37226763168 on `126e921` (re-read with `ci.py conclusion --sha 126e921` after the host's
  19:09Z reboot ended the background wait): `verdict: red`, 14/15 jobs green.
  - `test (windows-2025)`: `tui_focus_mouse_and_resize_never_take_the_wheel` — `a mouse report took the wheel;
    wheel records [start, {human, human-input}]`: the first step, an SGR mouse report alone, whose bytes reach the
    child unchanged, moved the wheel. Two mechanisms fit and are not yet told apart: (a) the inbox ConPTY hands the
    report to the wrapper's `ReadConsoleW` in pieces, a lone `ESC` first, which the classifier's lone-trailing-`ESC`
    rule counts as the Esc key; (b) the sideloaded ConPTY's win32-input-mode request (`ESC[?9001h`, through the
    wrapper's stdout) makes the outer console send each typed character as a `CSI … _` key sequence.
  - Fold (a measurement, test-side): `crates/viola-pty/src/lib.rs` gains two real-PTY cases that report each
    `host_stdin()` read of an SGR mouse report written into the platform PTY, plain and after `ESC[?9001h`, each
    asserting one whole read; on `windows-2025` the reading (or the failure's child report) tells (a) from (b).
    Pre-push green before the push.
- Entry 19: pushed `849588b` (the measurement commit).
- Entry 20, CI run ci#37227518624 on `849588b`: `verdict: red`, 14/15 jobs green.
  - MEASURED (`windows-2025`, the inbox ConPTY): `console_read_of_a_mouse_report_is_one_whole_read` PASSED — plain,
    the SGR mouse report reaches the reader as one whole read, so mechanism (a), a lone `ESC` read, is false.
    `console_read_of_a_mouse_report_under_win32_input_mode_is_one_whole_read` FAILED with the reading: after
    `ESC[?9001h` every character arrives as its own win32-input-mode key event `ESC[0;0;<Uc>;1;0;1_` (Vk 0, Sc 0,
    Uc = the character, key-down, no modifiers, repeat 1), first ones read `27 91 60 48 59 49 48 59` (`ESC [ < 0 ; 1 0
    ;`). Mechanism (b) holds: the sideloaded ConPTY's win32-input-mode request turns a mouse report written into the
    outer ConPTY's input into ten typed keys before the wrapper reads it; F-W2's classifier rightly reads them as
    typing, and the inner ConPTY decodes them back, so the child still sees the original bytes.
  - `tui_focus_mouse_and_resize_never_take_the_wheel` red as before (`a mouse report took the wheel`).
  - Not folded: the case's Windows acceptance meets a platform fact the plan did not foresee; the fold needs the
    operator's word (a classifier that decodes win32-input-mode is new behaviour; an acceptance change is a spec
    amendment).
- RULING F-W3 — the founder's live ruling, 2026-10-04 (answered 20:34Z through the overseer's AskUserQuestion, the
  measured win32-input-mode mechanism shown, and that the error only ever favours the human), relayed by the
  overseer: "Pin the platform fact." Authority form: word: "Pin the platform fact" — the founder, 2026-10-04,
  relayed by the overseer.
  - Test-side fold (this pass): on Windows the outer-PTY case expects an injected mouse report to take the wheel
    (`human` / `human-input`) and the focus reports to reach neither the child nor the wrapper; Unix keeps the full
    assertion that focus and mouse reports and a host resize never take the wheel. `crates/viola-pty/src/lib.rs`
    pins the encoding: plain, one whole read of the report; under `ESC[?9001h` on Windows, only win32 key-down
    records (Vk 0, Sc 0) whose characters spell the report, the focus reports absent; elsewhere the bytes unchanged.
  - For the wrap: a11y-plan §4 P4 tui case (3) and the v1-32 acceptance gain a Windows clause (an injected mouse
    report arrives as typed keys under the sideloaded ConPTY and takes the wheel; focus reports are swallowed by the
    inbox ConPTY); route pin: a mouse report from a real Windows terminal is measured at `:82` live.
  - Local before the push: lint, the viola-pty and full unit entries, the outer-PTY entry, the default suite and
    pre-push green; the windows-target clippy clean.
- Entry 19: pushed `79ec57c` (the F-W3 fold).
- Entry 20, CI run ci#37232840791 on `79ec57c`: `verdict: green · checks 15/15`.

## After the wrap's light gate (2026-10-04, wrap run 2026-10-04T20-44-01)
- The wrap's light gate read entry 16 RED. The guard `! (git diff eb53a582c8dc -- '*.rs' … std::env::var …)` hit the
  `+ … std::env::var(CHILD_MODE)` line `849588b` added to viola-pty's cfg(test) self-exec child for the `reads` /
  `reads-win32` modes. This pass had re-run pre-push and CI after `849588b`, but not that probe.
- Direction (the overseer, founder-delegated, through the operator): halt the wrap. Then move the new test-child modes
  off the env read into argv, so the guard stays as written and uncorrected; keep every measurement assertion
  unchanged; commit only the source fix; push; read CI.
- Fix `ba36659`: `spawn_child_entry` passes the mode as one more `--exact` filter (it names no test), and the child
  finds it in `std::env::args()`. 5 lines, in the test module only.
- Local before the push: `cargo fmt --all --check` and the workspace clippy (`-D warnings`) green; the
  `x86_64-pc-windows-msvc` viola-pty clippy clean; `run --unit --filter 'package(viola-pty)'` 38 passed, 0 failed —
  `console_read_of_a_mouse_report_is_one_whole_read` and
  `console_reads_under_win32_input_mode_are_the_platform_encoding` among them, so the argv mode reaches the child.
  Entry 16 bare: exit 0, no output. `gate.py hygiene` clean (after two wrap run-dir files were cleaned). Pre-push
  `"ok":true` at `linux-tests` (coverage 1453 passed, 0 failed; gate breaches none).
- Entry 19: pushed `ba36659` (`git push origin HEAD`). The entry's `git diff --quiet && git diff --cached --quiet`
  pre-condition was not run: by the direction, the wrap's uncommitted tree work stays out of the fix commit and rides
  the wrap commit.
- Entry 20, CI run ci#37235841342 on `ba36659`: `verdict: green · checks 15/15` (wall 308 s) — the final HEAD's run.
