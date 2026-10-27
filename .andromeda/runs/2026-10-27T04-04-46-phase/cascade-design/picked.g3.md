### color-palette#5.31 · values · status-states · ## Color Palette › ### Semantic Colors · lines 92–92
| Window.resets_at::unknown | #1E2124 | #4A5057 | #9C9278 | `resets unknown` |
### color-palette#5.32 · values · status-states · ## Color Palette › ### Semantic Colors · lines 93–93
| Window::expired (derived: `resets_at` has passed) | #1E2124 | #9C9278 | #E6D8AE | `5H 62 % expired`: old information shown as old |
### color-palette#5.35 · values · status-states · ## Color Palette › ### Semantic Colors · lines 96–96
| Readback::refused (from CL-1 `send-refused`) | #1E2124 + 1px #E6D8AE `/` strike | #E6D8AE | #E6D8AE | Strike, then `unable`, then `reason · detail` |
### color-palette#5.40 · values · status-states · ## Color Palette › ### Semantic Colors · lines 101–101
| RefusalReason::not-delivered | #1E2124 | #E6D8AE | #E6D8AE | `unable · not-delivered` |
### color-palette#5.41 · values · status-states · ## Color Palette › ### Semantic Colors · lines 102–102
| RefusalReason::unknown | #1E2124 | #E6D8AE | #E6D8AE | `unable · unknown`, with the detail printed as opaque plain text |
### color-palette#5.42 · values · status-states · ## Color Palette › ### Semantic Colors · lines 103–103
| RefusalDetail::null | #1E2124 | #E6D8AE | #E6D8AE | Nothing after the reason (no dangling `·`) |
### color-palette#5.46 · values · status-states · ## Color Palette › ### Semantic Colors · lines 107–107
| RefusalDetail::input-not-ready | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · input-not-ready` |
### color-palette#5.49 · values · status-states · ## Color Palette › ### Semantic Colors · lines 110–110
| RefusalDetail::unknown-dialog | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · unknown-dialog` |
### color-palette#5.50 · values · status-states · ## Color Palette › ### Semantic Colors · lines 111–111
| RefusalDetail::control-character (security plan) | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · control-character` |
### color-palette#5.51 · values · status-states · ## Color Palette › ### Semantic Colors · lines 112–112
| RefusalDetail::empty-text | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · empty-text` |
### color-palette#5.52 · values · status-states · ## Color Palette › ### Semantic Colors · lines 113–113
| EventKind::session-start | #1E2124 | none | #E6D8AE | Tape line `session-start` + cause word |
### color-palette#5.54 · values · status-states · ## Color Palette › ### Semantic Colors · lines 115–115
| EventKind::turn-ended | #1E2124 | none | #E6D8AE | `turn-ended` + first line of `last_assistant_message` as plain text, or `no message` for `null`. Never fills a readback box. |
### color-palette#5.55 · values · status-states · ## Color Palette › ### Semantic Colors · lines 116–116
| EventKind::question | #1E2124 | none | #E6D8AE | `question  dialog 7` + first question text. No amber in the tape: history is not attention. |
### color-palette#5.56 · values · status-states · ## Color Palette › ### Semantic Colors · lines 117–117
| EventKind::permission | #1E2124 | none | #E6D8AE | `permission  dialog 8` + tool name. `input` only in the expanded body, as text. |
### color-palette#5.57 · values · status-states · ## Color Palette › ### Semantic Colors · lines 118–118
| EventKind::plan | #1E2124 | none | #E6D8AE | `plan  dialog 9` + first plan line |
### color-palette#5.66 · values · status-states · ## Color Palette › ### Semantic Colors · lines 127–127
| EventKind::(unknown to this page) | #1E2124 | none | #9C9278 | `skipped  1 unknown kind`, printed at the point in the tape where it occurred |
### color-palette#5.80 · values · status-states · ## Color Palette › ### Semantic Colors · lines 141–141
| Skipped::zero (unknown_kinds / unknown_fields / torn_lines) | #1E2124 | none | #9C9278 | `skipped 0 · 0 · 0` in the ATIS line |
### color-palette#5.81 · values · status-states · ## Color Palette › ### Semantic Colors · lines 142–142
| Skipped::nonzero | #1E2124 | #E6D8AE (1px box around the count) | #E6D8AE | `skipped 2 unknown kinds · 0 · 1 torn line`, kept visible, never auto-cleared |
