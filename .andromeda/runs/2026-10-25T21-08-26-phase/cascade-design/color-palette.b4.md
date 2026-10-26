### color-palette#5.58 · values · status-states · ## Color Palette › ### Semantic Colors · lines 119–119
| EventKind::session-end | #1E2124 | #4A5057 (1px rule under the line) | #E6D8AE | `session-end`. The rail rule closes that instance's segment. |
### color-palette#5.59 · values · status-states · ## Color Palette › ### Semantic Colors · lines 120–120
| EventKind::activity | #1E2124 | none | #9C9278 | Whole line lamp-off: `activity  <tool>` (log-only) |
### color-palette#5.60 · values · status-states · ## Color Palette › ### Semantic Colors · lines 121–121
| EventKind::link | #1E2124 | none | #5B8DB8 names / #E6D8AE word | `link  overseer → builder` |
### color-palette#5.61 · values · status-states · ## Color Palette › ### Semantic Colors · lines 122–122
| EventKind::unlink | #1E2124 | none | #5B8DB8 names / #E6D8AE word | `unlink  overseer → builder` |
### color-palette#5.62 · values · status-states · ## Color Palette › ### Semantic Colors · lines 123–123
| EventKind::wheel | #1E2124 | none | #E6D8AE | `wheel  <holder> · <cause>` |
### color-palette#5.63 · values · status-states · ## Color Palette › ### Semantic Colors · lines 124–124
| EventKind::budget-gate | #1E2124 | none | #E6D8AE | `budget-gate  paused · five-hour` / `budget-gate  open` |
### color-palette#5.64 · values · status-states · ## Color Palette › ### Semantic Colors · lines 125–125
| EventKind::send-issued (CL-1) | #1E2124 | none | #E6D8AE | `send  → builder` + text + readback box (`open`) |
### color-palette#5.65 · values · status-states · ## Color Palette › ### Semantic Colors · lines 126–126
| EventKind::send-refused (CL-1) | #1E2124 | none | #E6D8AE | Folded into its send line: strike + reason. Never a separate line. |
### color-palette#5.66 · values · status-states · ## Color Palette › ### Semantic Colors · lines 127–127
| EventKind::(unknown to this page) | #1E2124 | none | #9C9278 | `skipped  1 unknown kind`, printed at the point in the tape where it occurred |
### color-palette#5.67 · values · status-states · ## Color Palette › ### Semantic Colors · lines 128–128
| PromptOrigin::driver | #1E2124 | none | #E6D8AE | Matched: folded into the send's readback. Unmatched: `prompt · driver`. |
### color-palette#5.68 · values · status-states · ## Color Palette › ### Semantic Colors · lines 129–129
| PromptOrigin::human | #1E2124 | none | #E6D8AE | `prompt · human`, with `human` at weight 600 (the human took the wheel) |
### color-palette#5.69 · values · status-states · ## Color Palette › ### Semantic Colors · lines 130–130
| PromptOrigin::harness | #1E2124 | none | #9C9278 | Whole line lamp-off: `prompt · harness`. Neither party's traffic. |
### color-palette#5.70 · values · status-states · ## Color Palette › ### Semantic Colors · lines 131–131
| SessionStartCause::startup | #1E2124 | none | #E6D8AE | `startup` |
### color-palette#5.71 · values · status-states · ## Color Palette › ### Semantic Colors · lines 132–132
| SessionStartCause::clear | #1E2124 | none | #E6D8AE | `clear`. Also the post-condition that fills the readback box of a `/clear` send. |
### color-palette#5.72 · values · status-states · ## Color Palette › ### Semantic Colors · lines 133–133
| SessionStartCause::resume | #1E2124 | none | #E6D8AE | `resume` |
### color-palette#5.73 · values · status-states · ## Color Palette › ### Semantic Colors · lines 134–134
| SessionStartCause::compact | #1E2124 | none | #E6D8AE | `compact`. Also the post-condition that fills the readback box of a `/compact` send on a verified CLI, when it carries the session's own id. |
### color-palette#5.74 · values · status-states · ## Color Palette › ### Semantic Colors · lines 135–135
| SessionStartCause::unknown | #1E2124 | none | #E6D8AE | `unknown` |
### color-palette#5.75 · values · status-states · ## Color Palette › ### Semantic Colors · lines 136–136
| WheelCause::start | #1E2124 | none | #E6D8AE | `driver · start` |
### color-palette#5.76 · values · status-states · ## Color Palette › ### Semantic Colors · lines 137–137
| WheelCause::human-input | #1E2124 | none | #E6D8AE | `human · human-input` |
### color-palette#5.77 · values · status-states · ## Color Palette › ### Semantic Colors · lines 138–138
| WheelCause::manual-pause | #1E2124 | none | #E6D8AE | `human · manual-pause`, i.e. "I have control" |
### color-palette#5.78 · values · status-states · ## Color Palette › ### Semantic Colors · lines 139–139
| WheelCause::release | #1E2124 | none | #E6D8AE | `driver · release`, i.e. "you have control" |
### color-palette#5.79 · values · status-states · ## Color Palette › ### Semantic Colors · lines 140–140
| EventSource::hook / wrapper / cli | #1E2124 | none | #9C9278 | `source hook` / `source wrapper` / `source cli`, only inside an expanded tape line |
### color-palette#5.80 · values · status-states · ## Color Palette › ### Semantic Colors · lines 141–141
| Skipped::zero (unknown_kinds / unknown_fields / torn_lines) | #1E2124 | none | #9C9278 | `skipped 0 · 0 · 0` in the ATIS line |
### color-palette#5.81 · values · status-states · ## Color Palette › ### Semantic Colors · lines 142–142
| Skipped::nonzero | #1E2124 | #E6D8AE (1px box around the count) | #E6D8AE | `skipped 2 unknown kinds · 0 · 1 torn line`, kept visible, never auto-cleared |
### color-palette#5.82 · values · status-states · ## Color Palette › ### Semantic Colors · lines 143–143
| TapeConnection::connecting (derived, EventSource) | #1E2124 | #4A5057 | #9C9278 | `TAPE connecting` |
