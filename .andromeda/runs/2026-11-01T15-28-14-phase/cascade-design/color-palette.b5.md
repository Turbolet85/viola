### color-palette#5.83 · values · status-states · ## Color Palette › ### Semantic Colors · lines 144–144
| TapeConnection::open | #1E2124 | #4A5057 | #E6D8AE | `TAPE live since 19:40:02Z` |
### color-palette#5.84 · values · status-states · ## Color Palette › ### Semantic Colors · lines 145–145
| TapeConnection::closed | #1E2124 | #E6D8AE | #E6D8AE | `TAPE stopped · not answering`. Strips keep the last values, and the ATIS age keeps counting. |
### color-palette#5.85 · values · status-states · ## Color Palette › ### Semantic Colors · lines 146–146
| Problem::unauthorized (401) | #1E2124 | #E6D8AE | #E6D8AE | An access refusal. Never shows the token, the URL or the file path. |
### color-palette#5.86 · values · status-states · ## Color Palette › ### Semantic Colors · lines 147–147
| Problem::state-unreadable (503) | #1E2124 | #E6D8AE | #E6D8AE | Rack strip `unable · state-unreadable` |
### color-palette#5.87 · values · status-states · ## Color Palette › ### Semantic Colors · lines 148–148
| Problem::not-found (404) | #1E2124 | #E6D8AE | #E6D8AE | Rack strip `unable · not-found`. The page only requests its own routes, so this is defensive. |
### color-palette#5.88 · values · status-states · ## Color Palette › ### Semantic Colors · lines 149–149
| Problem::method-not-allowed (405) | #1E2124 | #E6D8AE | #E6D8AE | Never produced by the page (GET only). If seen: `unable · method-not-allowed`. |
### color-palette#5.89 · values · status-states · ## Color Palette › ### Semantic Colors · lines 150–150
| Problem::host-not-allowed (403) | #1E2124 | #E6D8AE | #E6D8AE | The page cannot load at all, so there is no in-page visual. The browser shows the Problem JSON. |
### color-palette#5.90 · values · status-states · ## Color Palette › ### Semantic Colors · lines 151–151
| Problem::cross-origin-forbidden (403, v1.x) | #1E2124 | #E6D8AE | #E6D8AE | Reserved for the brake: `unable · cross-origin-forbidden` |
### color-palette#6.1 · values · palette · ## Color Palette › ### Border Progression · lines 159–161
| Intensity | Value | Usage |
|-----------|-------|-------|
| Subtle | #4A5057 | Rack-rail aluminium: rack separators, the strip outer edge, the band slot of a non-cocked strip, the dashed outline of bare strips, the in-strip field grid of a `stale` strip (the `--rule-field` swap), the tape minute rule, the `session-end` rule and the rule under the ATIS header. Decorative, 2.0:1. In the domain status rows, #4A5057 on an ATIS-line value (`BudgetEnvelope`, `BudgetWindow`, `Window` other than `expired`, `gate open`, `TapeConnection::connecting` / `open`) names that header rule: those cells are never boxed. |
### color-palette#6.2 · values · palette · ## Color Palette › ### Border Progression · lines 162–162
| Standard | #9C9278 | The printed field-grid rule between cells inside a lit strip: the rule printed on the paper, quieter than the ink. Also the 1px box around an ATIS `expired` word (`Window::expired`, on anthracite): old information is marked as old, quieter than the buff boxes of declared states. |
### color-palette#6.3 · values · palette · ## Color Palette › ### Border Progression · lines 163–163
| Emphasis | #E6D8AE | Borders whose presence is the state: the readback box outline and strike, the `budget-paused` box, nonzero `skipped` boxes, the ATIS `TAPE stopped` box, error strips |
### color-palette#6.4 · values · palette · ## Color Palette › ### Border Progression · lines 164–164
| Focus | #E6D8AE | 2px solid focus ring, offset 2px, radius 0 (11.4:1 on anthracite, 9.6:1 on holder) |
### color-palette#6.5 · values · palette · ## Color Palette › ### Border Progression · lines 165–165
| Handoff | #5B8DB8 | The 2px (`--rule-strong`) left tick of the outbound and inbound transfer markers only, in the rack gap on anthracite (4.6:1). The only blue rule. Never on holder, never a strip or cell border. |
