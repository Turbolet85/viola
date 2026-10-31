### brand-identity#2.2 · values · expression-motion · ## Brand Identity · lines 21–21
| cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
### color-palette#5.5 · values · status-states · ## Color Palette › ### Semantic Colors · lines 66–66
| SessionStatus::unknown | #2A2E33 | #9C9278 | #E6D8AE | `unknown` printed in the box (`claude agents --json` missing, failed or not joined) |
### color-palette#5.19 · values · status-states · ## Color Palette › ### Semantic Colors · lines 80–80
| CliVersion::(absent) | #2A2E33 | #9C9278 | #E6D8AE | `unknown`: a failed reading on a wrapped strip |
### color-palette#5.20 · values · status-states · ## Color Palette › ### Semantic Colors · lines 81–81
| CliVersion::(absent, unwrapped) | #1E2124 | #4A5057 (dashed) | #9C9278 | `n/a`: absent by contract, as in the `viola list` sample |
### color-palette#5.25 · values · status-states · ## Color Palette › ### Semantic Colors · lines 86–86
| BudgetEnvelope::unknown | #1E2124 | #4A5057 | #E6D8AE | `5H unknown  7D unknown  read unknown`. Nothing is blocked. |
### color-palette#5.28 · values · status-states · ## Color Palette › ### Semantic Colors · lines 89–89
| BudgetWindow::five-hour | #1E2124 | #4A5057 | #E6D8AE | Label `5H`. Detail word `five-hour` in refusals. |
### color-palette#5.29 · values · status-states · ## Color Palette › ### Semantic Colors · lines 90–90
| BudgetWindow::seven-day | #1E2124 | #4A5057 | #E6D8AE | Label `7D`. Detail word `seven-day` in refusals. |
### color-palette#5.30 · values · status-states · ## Color Palette › ### Semantic Colors · lines 91–91
| Window.used_percentage::unknown | #1E2124 | #4A5057 | #E6D8AE | `5H unknown` |
### color-palette#5.31 · values · status-states · ## Color Palette › ### Semantic Colors · lines 92–92
| Window.resets_at::unknown | #1E2124 | #4A5057 | #9C9278 | `resets unknown` |
### color-palette#5.35 · values · status-states · ## Color Palette › ### Semantic Colors · lines 96–96
| Readback::refused (from CL-1 `send-refused`) | #1E2124 + 1px #E6D8AE `/` strike | #E6D8AE | #E6D8AE | Strike, then `unable`, then `reason · detail` |
### color-palette#5.40 · values · status-states · ## Color Palette › ### Semantic Colors · lines 101–101
| RefusalReason::not-delivered | #1E2124 | #E6D8AE | #E6D8AE | `unable · not-delivered` |
### color-palette#5.41 · values · status-states · ## Color Palette › ### Semantic Colors · lines 102–102
| RefusalReason::unknown | #1E2124 | #E6D8AE | #E6D8AE | `unable · unknown`, with the detail printed as opaque plain text |
### color-palette#5.42 · values · status-states · ## Color Palette › ### Semantic Colors · lines 103–103
| RefusalDetail::null | #1E2124 | #E6D8AE | #E6D8AE | Nothing after the reason (no dangling `·`) |
### color-palette#5.47 · values · status-states · ## Color Palette › ### Semantic Colors · lines 108–108
| RefusalDetail::no-prompt-submitted | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · no-prompt-submitted` |
### color-palette#5.49 · values · status-states · ## Color Palette › ### Semantic Colors · lines 110–110
| RefusalDetail::unknown-dialog | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · unknown-dialog` |
### iconography#1.3 · values · icons · ## Iconography · lines 10–12
- **Separator.** Middle dot `·` on the web. The CLI is ASCII only:
  - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
  - ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.
