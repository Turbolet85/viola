### brand-identity#1.1 · values · expression-motion · ## Brand Identity · lines 8–8
- `0.0-0.2`: static, grid-locked, zero animation, dense
### brand-identity#1.2 · values · expression-motion · ## Brand Identity · lines 9–9
- `0.3-0.4`: subtle hover states, fade transitions, quiet focus rings
### brand-identity#1.3 · values · expression-motion · ## Brand Identity · lines 10–10
- `0.5-0.6`: smooth transitions, micro-interactions, loading skeletons
### brand-identity#1.4 · values · expression-motion · ## Brand Identity · lines 11–11
- `0.7-0.8`: staggered entrances, parallax, spring physics
### brand-identity#1.5 · values · expression-motion · ## Brand Identity · lines 12–12
- `0.9-1.0`: scroll-driven, 3D, particles, canvas/WebGL
### brand-identity#2.1 · values · expression-motion · ## Brand Identity · lines 18–20
| Surface | Expression | What it permits |
|---|---|---|
| cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners. |
### brand-identity#2.2 · values · expression-motion · ## Brand Identity · lines 21–21
| cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
