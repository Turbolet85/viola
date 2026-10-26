### color-palette#5.8 · values · status-states · ## Color Palette › ### Semantic Colors · lines 69–69
| WheelHolder::(absent, unwrapped) | #1E2124 | #4A5057 (dashed) | #9C9278 | `n/a`: absent by contract, not a failed reading |
### color-palette#5.9 · values · status-states · ## Color Palette › ### Semantic Colors · lines 70–70
| Wrapped::true | #2A2E33 | #9C9278 grid; 4px band slot #4A5057 | #E6D8AE | Strip in a holder boot, in the WRAPPED rack |
### color-palette#5.10 · values · status-states · ## Color Palette › ### Semantic Colors · lines 71–71
| Wrapped::false | #1E2124 | #4A5057 (1px dashed outline, no band slot) | #E6D8AE (name, live, status) / #9C9278 (`n/a`) | Bare strip, another facility's traffic, in the UNWRAPPED · READ-ONLY rack. The name is shown as text, never as a target. |
### color-palette#5.11 · values · status-states · ## Color Palette › ### Semantic Colors · lines 72–72
| DialogPending::false | #2A2E33 | #4A5057 band | #E6D8AE | DIALOG prints `none` |
### color-palette#5.12 · values · status-states · ## Color Palette › ### Semantic Colors · lines 73–73
| DialogPending::(absent, unwrapped) | #1E2124 | #4A5057 (dashed) | #9C9278 | `n/a`: absent by contract on a bare strip, which is never cocked |
### color-palette#5.13 · values · status-states · ## Color Palette › ### Semantic Colors · lines 74–74
| DialogPending::true | #1E2124 (DIALOG cell inset) | #D97706 (4px holder band) | #D97706 (`DIALOG`) + #E6D8AE (kind; #9C9278 when the strip is also `stale`, per the `stale` `--ink` swap) | Cocked: `translateX(12px)` once over 160ms, amber band, `DIALOG <kind>`. Slot kept. Announced politely. |
### color-palette#5.14 · values · status-states · ## Color Palette › ### Semantic Colors · lines 75–75
| PendingDialogKind::question | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG question` |
### color-palette#5.15 · values · status-states · ## Color Palette › ### Semantic Colors · lines 76–76
| PendingDialogKind::permission | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG permission` |
### color-palette#5.16 · values · status-states · ## Color Palette › ### Semantic Colors · lines 77–77
| PendingDialogKind::plan | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG plan` |
### color-palette#5.17 · values · status-states · ## Color Palette › ### Semantic Colors · lines 78–78
| CliVerified::true | #2A2E33 | #9C9278 | #E6D8AE | `2.1.280 verified` (known aircraft type) |
### color-palette#5.18 · values · status-states · ## Color Palette › ### Semantic Colors · lines 79–79
| CliVerified::false | #2A2E33 | #9C9278 | #E6D8AE | `2.1.281 unverified-cli`: transport only, dialog answers held back. No colour. |
### color-palette#5.19 · values · status-states · ## Color Palette › ### Semantic Colors · lines 80–80
| CliVersion::(absent) | #2A2E33 | #9C9278 | #E6D8AE | `unknown`: a failed reading on a wrapped strip |
### color-palette#5.20 · values · status-states · ## Color Palette › ### Semantic Colors · lines 81–81
| CliVersion::(absent, unwrapped) | #1E2124 | #4A5057 (dashed) | #9C9278 | `n/a`: absent by contract, as in the `viola list` sample |
### color-palette#5.21 · values · status-states · ## Color Palette › ### Semantic Colors · lines 82–82
| InstanceBudgetPaused::false | #2A2E33 | #9C9278 | #E6D8AE | The WHEEL cell prints the holder only |
### color-palette#5.22 · values · status-states · ## Color Palette › ### Semantic Colors · lines 83–83
| InstanceBudgetPaused::true | #2A2E33 | #9C9278 | #E6D8AE | The WHEEL cell prints `driver · budget-paused`: minimum fuel declared, new sends stop, the running turn finishes |
### color-palette#5.23 · values · status-states · ## Color Palette › ### Semantic Colors · lines 84–84
| InstanceBudgetPaused::(absent, unwrapped) | #1E2124 | #4A5057 | #9C9278 | Covered by `n/a` in WHEEL |
### color-palette#5.24 · values · status-states · ## Color Palette › ### Semantic Colors · lines 85–85
| BudgetEnvelope::reading | #1E2124 | #4A5057 | #E6D8AE figures / #9C9278 age | ATIS line `5H 62 %  7D 41 %  read 4m ago` |
### color-palette#5.25 · values · status-states · ## Color Palette › ### Semantic Colors · lines 86–86
| BudgetEnvelope::unknown | #1E2124 | #4A5057 | #E6D8AE | `5H unknown  7D unknown  read unknown`. Nothing is blocked. |
### color-palette#5.26 · values · status-states · ## Color Palette › ### Semantic Colors · lines 87–87
| BudgetEnvelope.paused::false | #1E2124 | #4A5057 | #9C9278 | `gate open` |
### color-palette#5.27 · values · status-states · ## Color Palette › ### Semantic Colors · lines 88–88
| BudgetEnvelope.paused::true | #1E2124 | #E6D8AE (1px box around the word) | #E6D8AE | `budget-paused`, boxed like a declared state. No amber, no red. |
### color-palette#5.28 · values · status-states · ## Color Palette › ### Semantic Colors · lines 89–89
| BudgetWindow::five-hour | #1E2124 | #4A5057 | #E6D8AE | Label `5H`. Detail word `five-hour` in refusals. |
### color-palette#5.29 · values · status-states · ## Color Palette › ### Semantic Colors · lines 90–90
| BudgetWindow::seven-day | #1E2124 | #4A5057 | #E6D8AE | Label `7D`. Detail word `seven-day` in refusals. |
### color-palette#5.30 · values · status-states · ## Color Palette › ### Semantic Colors · lines 91–91
| Window.used_percentage::unknown | #1E2124 | #4A5057 | #E6D8AE | `5H unknown` |
### color-palette#5.31 · values · status-states · ## Color Palette › ### Semantic Colors · lines 92–92
| Window.resets_at::unknown | #1E2124 | #4A5057 | #9C9278 | `resets unknown` |
### color-palette#5.32 · values · status-states · ## Color Palette › ### Semantic Colors · lines 93–93
| Window::expired (derived: `resets_at` has passed) | #1E2124 | #9C9278 | #E6D8AE | `5H 62 % expired`: old information shown as old |
