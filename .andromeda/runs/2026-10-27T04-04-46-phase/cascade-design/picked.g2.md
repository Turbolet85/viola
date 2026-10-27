### color-palette#4.2 · values · status-states · ## Color Palette › ### Semantic Colors · lines 52–52
| Warning, degraded but not refused (`unverified-cli`, `unconfirmable`, expired budget window) | #2A2E33 for `unverified-cli` (a word on a lit strip); #1E2124 for `unconfirmable` and `expired` | #9C9278 for `unverified-cli` (field grid) and `expired`; #E6D8AE for `unconfirmable` (the readback-box outline is a state border) | #E6D8AE |
### color-palette#4.3 · values · status-states · ## Color Palette › ### Semantic Colors · lines 53–53
| Error, "unable" plus a reason (refusal, HTTP problem) | #1E2124 | #E6D8AE (plus a 1px `/` strike where a readback box exists) | #E6D8AE |
### color-palette#5.3 · values · status-states · ## Color Palette › ### Semantic Colors · lines 64–64
| SessionStatus::idle | #2A2E33 | #9C9278 | #E6D8AE | `idle` |
### color-palette#5.4 · values · status-states · ## Color Palette › ### Semantic Colors · lines 65–65
| SessionStatus::busy | #2A2E33 | #9C9278 | #E6D8AE | `busy`. No spinner, pulse or animation. |
### color-palette#5.5 · values · status-states · ## Color Palette › ### Semantic Colors · lines 66–66
| SessionStatus::unknown | #2A2E33 | #9C9278 | #E6D8AE | `unknown` printed in the box (`claude agents --json` missing, failed or not joined) |
### color-palette#5.9 · values · status-states · ## Color Palette › ### Semantic Colors · lines 70–70
| Wrapped::true | #2A2E33 | #9C9278 grid; 4px band slot #4A5057 | #E6D8AE | Strip in a holder boot, in the WRAPPED rack |
### color-palette#5.10 · values · status-states · ## Color Palette › ### Semantic Colors · lines 71–71
| Wrapped::false | #1E2124 | #4A5057 (1px dashed outline, no band slot) | #E6D8AE (name, live, status) / #9C9278 (`n/a`) | Bare strip, another facility's traffic, in the UNWRAPPED · READ-ONLY rack. The name is shown as text, never as a target. |
### color-palette#5.14 · values · status-states · ## Color Palette › ### Semantic Colors · lines 75–75
| PendingDialogKind::question | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG question` |
### color-palette#5.15 · values · status-states · ## Color Palette › ### Semantic Colors · lines 76–76
| PendingDialogKind::permission | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG permission` |
### color-palette#5.16 · values · status-states · ## Color Palette › ### Semantic Colors · lines 77–77
| PendingDialogKind::plan | #1E2124 | #D97706 | #D97706 / #E6D8AE | `DIALOG plan` |
### color-palette#5.18 · values · status-states · ## Color Palette › ### Semantic Colors · lines 79–79
| CliVerified::false | #2A2E33 | #9C9278 | #E6D8AE | `2.1.281 unverified-cli`: transport only, dialog answers held back. No colour. |
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
