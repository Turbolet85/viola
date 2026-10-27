# a11y · Visual Design Verification — category read

## Carried
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The pin race fix sits on `run`'s start (`pin_bytes`) and `verify` (`pin_exe`), so whatever the absent-copy write does under a concurrent first start (a wait, a retry, a refusal) must write no SGR of viola's own to the `viola run` terminal or a passed `viola revive`'s start.
  > viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`, and the revive refusal pairs and `--list` rows carry no ESC byte.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) Any refusal or failed-step line the chunk adds or changes on the pin path (`run`'s start refusal pair, a `verify` step line) must carry no colour.
  > `unable` / `fail` are never coloured.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The Windows half of the race (the exe's arm, beside `pin_companions`' `ERROR_SHARING_VIOLATION` wait) is read on the windows-2025 runner, so the existing windows-2025 `viola run` check (viola's literals absent from the outer-PTY stream, on both ConPTY backends) must still pass after the change.
  > Under `viola run` on Windows the outer-PTY stream also carries the ConPTY host's own SGR, cursor and query bytes (not viola's: the inbox host's as measured at chunk 2026-09-25-pty-wrapper-on-windows research fact 4, the sideloaded `OpenConsole.exe`'s `ESC[1t ESC[c ESC[?1004h ESC[?9001h` as measured at chunk 2026-09-29-sideloaded-conpty), so the windows-2025 `viola run` check is that viola's literals are absent, on both backends.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) Those checks must run on all three CI legs, the same matrix this chunk's Windows reading happens on.
  > Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`.
- (.andromeda/a11y-plan.md:340) The anti-pattern guard on the same surfaces applies to any output the pin path writes.
  > NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.

## Bindings
- **zero viola SGR under `viola run` ↔ the body's tui surface (boundary-only)**: the category's zero-SGR rule under `viola run` (.andromeda/a11y-plan-detail/visual-design-verification.md:9) is the visual side of the body's rule that viola writes no terminal bytes of its own in the passthrough (.andromeda/a11y-plan.md:55); a pin-path wait or refusal in `run`'s start must meet both.
- **`unable` / `fail` never coloured ↔ the body's §11 Visual anti-pattern**: the category's rule (.andromeda/a11y-plan-detail/visual-design-verification.md:9) and the body's guard (.andromeda/a11y-plan.md:340) are one rule, held at both places for a refusal line or `verify` step line the chunk touches.
- **windows-2025 `viola run` literal check ↔ the body's §9 CI matrix**: the Windows outer-PTY check (.andromeda/a11y-plan-detail/visual-design-verification.md:9) runs inside the per-OS `test` job's `coverage` suite on the matrix (.andromeda/a11y-plan.md:289), where this chunk reads Windows.
