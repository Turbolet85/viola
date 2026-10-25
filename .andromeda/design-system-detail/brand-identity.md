# design-system · Brand Identity
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Brand Identity detail.

## Brand Identity

<!-- detail: values · expression-motion -->

- `0.0-0.2`: static, grid-locked, zero animation, dense
- `0.3-0.4`: subtle hover states, fade transitions, quiet focus rings
- `0.5-0.6`: smooth transitions, micro-interactions, loading skeletons
- `0.7-0.8`: staggered entrances, parallax, spring physics
- `0.9-1.0`: scroll-driven, 3D, particles, canvas/WebGL

<!-- /detail -->

<!-- detail: values · expression-motion -->

| Surface | Expression | What it permits |
|---|---|---|
| cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners. |
| cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |

<!-- /detail -->
