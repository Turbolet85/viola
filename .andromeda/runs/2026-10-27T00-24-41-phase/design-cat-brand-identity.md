# design · Brand Identity — category read

## Carried
- (.andromeda/design-system-detail/brand-identity.md:21) Any wait-out, retry or new failure the pin fix adds to `run`'s start (`pin_bytes`), or to a `viola revive` start that passed its preflight, must write no colour, glyph, cursor control or spinner, and nothing at all once the wrapped `claude` TUI runs.
  > `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
- (.andromeda/design-system-detail/brand-identity.md:20) A wait the fix adds to `viola verify`'s `pin_exe` on a human TTY must add no redrawn line and no spinner beside verify's static step lines.
  > Nothing redraws in place and there are no spinners.
- (.andromeda/design-system.md:38) The expression level bounds the fix's human output at zero motion: a concurrent-pin wait is never shown as progress on the cli.
  > At 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.

## Bindings
- **The 0.0 passthrough row ↔ Surface: cli colour decision order and `run`'s start refusal**: the row's silence (.andromeda/design-system-detail/brand-identity.md:21) is the body's decision-order step 2 (.andromeda/design-system.md:246) and `run`'s "prints nothing once the child starts", a start refusal taking only its fixed `unable:` line and hint (.andromeda/design-system.md:332); a pin failure the fix surfaces before the child starts follows that fixed form, no path or pid.
- **The TTY 0.2 row ↔ Surface: cli `verify` step counter**: no spinner or redraw (.andromeda/design-system-detail/brand-identity.md:20) keeps `verify` to its static `pass` / `fail` lines and summary (.andromeda/design-system.md:330), unchanged by a pin wait.
- **The expression level ↔ Motion**: the single source for motion (.andromeda/design-system.md:38) is what the Motion section calibrates to (.andromeda/design-system.md:195); the chunk adds no motion.
