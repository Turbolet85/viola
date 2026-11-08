@fact id=fct-k5ssxpy7 expect_rev=1
old: | radius-sm | 0px | Readback box, field cells, focus ring, the reserved v1.x brake buttons |
new: | — | — | N/A: viola renders no page, so no radius tokens |

@fact id=fct-hdk4heck expect_rev=1
old: | 0.0-0.2 | 0-100ms | instant/none | none | none |
new: | — | — | — | — | N/A: viola renders no page, so no motion; the cli has none |

@fact id=fct-rcoxrota expect_rev=1
old: | space-micro | 4px | Gap between the readback box and its word cell, and between `→` and a name. Never cell padding: field-cell inline padding is `space-xs` (8px), which `--strip-cols` is sized for |
new: | — | — | N/A: viola renders no page, so no spacing tokens; the cli's spacing is the terminal grid |

@fact id=fct-73hf7cl5 expect_rev=1
old: | Display | Bahnschrift stack, semi-condensed | 600 | 16px / 24px line, uppercase | 0.08em | `VIOLA` as the board's display word (there is no hero; a 0.2.0 frontend's page markup is that version's). The `BAY` after it is Label and `127.0.0.1:47319` is Data, as in the ATIS anatomy |
new: | — | — | — | — | — | N/A: viola renders no page; the cli's type is the terminal's |

@fact id=fct-5lo2eq3p expect_rev=1
old: Every colour traces to one of the eight `--c-*` values (#1E2124, #2A2E33, #4A5057, #E6D8AE, #9C9278, #3B3A36, #D97706, #5B8DB8). `--c-*` is read only where a role alias is assigned or reassigned (`@layer tokens`, and the `stale` swap in `@layer states`). Components use the role aliases (`--surface-*`, `--ink*`, `--rule-*`, `--attention`, `--handoff`, `--focus-ring`, `--rb-*`) and never `--c-*`. Every length traces to a token (`--space-*`, `--fs-*`, `--track-*` (em), `--lh-display`, `--lh-label`, `--strip-h`, `--line-h`, `--band-w`, `--rb-size`, `--rule`, `--rule-strong`, `--radius`, `--cock-offset`, `--focus-w`, `--focus-offset`) or to a `ch` track in `--strip-cols` / `--strip-cols-wrapped` / `--tape-cols`, with the wrapped strip's rows in `--strip-rows-wrapped`. Every duration is `--cock-dur` or `--fade-dur`. The only literal lengths allowed are the two Navigation → Width breakpoints (`760px`, `1024px`), because `@media` conditions cannot read custom properties. A ninth hex value or any other untokened px value signals no system.
new: N/A — viola renders no page, so no stylesheet token to trace.
