### brand-identity#2.1 · values · expression-motion · ## Brand Identity · lines 18–20
| Surface | Expression | What it permits |
|---|---|---|
| cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners. |
### brand-identity#2.2 · values · expression-motion · ## Brand Identity · lines 21–21
| cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
### iconography#1.3 · values · icons · ## Iconography · lines 10–12
- **Separator.** Middle dot `·` on the web. The CLI is ASCII only:
  - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
  - ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.
