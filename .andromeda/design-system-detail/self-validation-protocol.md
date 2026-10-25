# design-system · Self-Validation Protocol
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Self-Validation Protocol detail.

## Self-Validation Protocol › ### 4. Token Test

<!-- detail: procedure · validation -->

Every colour traces to one of the eight `--c-*` values (#1E2124, #2A2E33, #4A5057, #E6D8AE, #9C9278, #3B3A36, #D97706, #5B8DB8). `--c-*` is read only where a role alias is assigned or reassigned (`@layer tokens`, and the `stale` swap in `@layer states`). Components use the role aliases (`--surface-*`, `--ink*`, `--rule-*`, `--attention`, `--handoff`, `--focus-ring`, `--rb-*`) and never `--c-*`. Every length traces to a token (`--space-*`, `--fs-*`, `--track-*` (em), `--lh-display`, `--lh-label`, `--strip-h`, `--line-h`, `--band-w`, `--rb-size`, `--rule`, `--rule-strong`, `--radius`, `--cock-offset`, `--focus-w`, `--focus-offset`) or to a `ch` track in `--strip-cols` / `--strip-cols-wrapped` / `--tape-cols`, with the wrapped strip's rows in `--strip-rows-wrapped`. Every duration is `--cock-dur` or `--fade-dur`. The only literal lengths allowed are the two Navigation → Width breakpoints (`760px`, `1024px`), because `@media` conditions cannot read custom properties. A ninth hex value or any other untokened px value signals no system.

<!-- /detail -->
