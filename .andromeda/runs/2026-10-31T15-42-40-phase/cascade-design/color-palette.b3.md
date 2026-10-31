### color-palette#5.33 · values · status-states · ## Color Palette › ### Semantic Colors · lines 94–94
| Readback::open (from CL-1 `send-issued`) | #1E2124 | #E6D8AE (1px) | #E6D8AE | Empty square, then the word `open` |
### color-palette#5.34 · values · status-states · ## Color Palette › ### Semantic Colors · lines 95–95
| Readback::read (matching `prompt-submitted`, or ledger post-condition met) | #E6D8AE | #E6D8AE | #3B3A36 in the box / #E6D8AE word | Box filled, `RB` inside, word `read back`. Instant, no transition. |
### color-palette#5.35 · values · status-states · ## Color Palette › ### Semantic Colors · lines 96–96
| Readback::refused (from CL-1 `send-refused`) | #1E2124 + 1px #E6D8AE `/` strike | #E6D8AE | #E6D8AE | Strike, then `unable`, then `reason · detail` |
### color-palette#5.36 · values · status-states · ## Color Palette › ### Semantic Colors · lines 97–97
| Readback::unconfirmable (`ok`, `confirmed:false`) | #1E2124 | #E6D8AE | #E6D8AE | The open drawing, never filled, with the word `unconfirmable` |
### color-palette#5.37 · values · status-states · ## Color Palette › ### Semantic Colors · lines 98–98
| RefusalReason::human-typing | #1E2124 | #E6D8AE | #E6D8AE | `unable · human-typing` |
### color-palette#5.38 · values · status-states · ## Color Palette › ### Semantic Colors · lines 99–99
| RefusalReason::budget-paused | #1E2124 | #E6D8AE | #E6D8AE | `unable · budget-paused` |
### color-palette#5.39 · values · status-states · ## Color Palette › ### Semantic Colors · lines 100–100
| RefusalReason::unverified-cli | #1E2124 | #E6D8AE | #E6D8AE | `unable · unverified-cli` |
### color-palette#5.40 · values · status-states · ## Color Palette › ### Semantic Colors · lines 101–101
| RefusalReason::not-delivered | #1E2124 | #E6D8AE | #E6D8AE | `unable · not-delivered` |
### color-palette#5.41 · values · status-states · ## Color Palette › ### Semantic Colors · lines 102–102
| RefusalReason::unknown | #1E2124 | #E6D8AE | #E6D8AE | `unable · unknown`, with the detail, a code word or nothing, printed as plain text |
### color-palette#5.42 · values · status-states · ## Color Palette › ### Semantic Colors · lines 103–103
| RefusalDetail::null | #1E2124 | #E6D8AE | #E6D8AE | Nothing after the reason (no dangling `·`) |
### color-palette#5.43 · values · status-states · ## Color Palette › ### Semantic Colors · lines 104–104
| RefusalDetail::manual-pause | #1E2124 | #E6D8AE | #E6D8AE | `human-typing · manual-pause` ("I have control" spoken via `viola pause`) |
### color-palette#5.44 · values · status-states · ## Color Palette › ### Semantic Colors · lines 105–105
| RefusalDetail::five-hour | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · five-hour` |
### color-palette#5.45 · values · status-states · ## Color Palette › ### Semantic Colors · lines 106–106
| RefusalDetail::seven-day | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · seven-day` |
### color-palette#5.46 · values · status-states · ## Color Palette › ### Semantic Colors · lines 107–107
| RefusalDetail::input-not-ready | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · input-not-ready` |
### color-palette#5.47 · values · status-states · ## Color Palette › ### Semantic Colors · lines 108–108
| RefusalDetail::no-prompt-submitted | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · no-prompt-submitted` |
### color-palette#5.48 · values · status-states · ## Color Palette › ### Semantic Colors · lines 109–109
| RefusalDetail::turn-running | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · turn-running` |
### color-palette#5.49 · values · status-states · ## Color Palette › ### Semantic Colors · lines 110–110
| RefusalDetail::unknown-dialog | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · unknown-dialog` |
### color-palette#5.50 · values · status-states · ## Color Palette › ### Semantic Colors · lines 111–111
| RefusalDetail::control-character (security plan) | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · control-character` |
### color-palette#5.51 · values · status-states · ## Color Palette › ### Semantic Colors · lines 112–112
| RefusalDetail::empty-text | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · empty-text` |
### color-palette#5.52 · values · status-states · ## Color Palette › ### Semantic Colors · lines 113–113
| EventKind::session-start | #1E2124 | none | #E6D8AE | Tape line `session-start` + cause word |
### color-palette#5.53 · values · status-states · ## Color Palette › ### Semantic Colors · lines 114–114
| EventKind::prompt-submitted | #1E2124 | none | #E6D8AE (#9C9278 if harness) | Folded into the open send line when it is the matching driver prompt. Otherwise its own line `prompt · <origin>` + text. |
### color-palette#5.54 · values · status-states · ## Color Palette › ### Semantic Colors · lines 115–115
| EventKind::turn-ended | #1E2124 | none | #E6D8AE | `turn-ended` + first line of `last_assistant_message` as plain text, or `no message` for `null`. Never fills a readback box. |
### color-palette#5.55 · values · status-states · ## Color Palette › ### Semantic Colors · lines 116–116
| EventKind::question | #1E2124 | none | #E6D8AE | `question  dialog 7` + first question text. No amber in the tape: history is not attention. |
### color-palette#5.56 · values · status-states · ## Color Palette › ### Semantic Colors · lines 117–117
| EventKind::permission | #1E2124 | none | #E6D8AE | `permission  dialog 8` + tool name. `input` only in the expanded body, as text. |
### color-palette#5.57 · values · status-states · ## Color Palette › ### Semantic Colors · lines 118–118
| EventKind::plan | #1E2124 | none | #E6D8AE | `plan  dialog 9` + first plan line |
