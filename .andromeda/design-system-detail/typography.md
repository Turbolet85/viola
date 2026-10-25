# design-system · Typography
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Typography detail.

## Typography

<!-- detail: values · typography -->

| Role | Font | Weight | Size | Tracking | Usage |
|------|------|--------|------|----------|-------|
| Display | Bahnschrift stack, semi-condensed | 600 | 16px / 24px line, uppercase | 0.08em | `VIOLA` as the board's display word (there is no hero; a 0.2.0 frontend's page markup is that version's). The `BAY` after it is Label and `127.0.0.1:47319` is Data, as in the ATIS anatomy |
| Heading | Bahnschrift stack, semi-condensed | 600 | 11px / 16px, uppercase | 0.12em | Rack separator labels: `WRAPPED`, `UNWRAPPED · READ-ONLY`, `TAPE`, each rendered as an `<h2>` (a11y-plan D-A11Y-02); the two rack labels sit in their rack table's `<caption>` |
| Callsign | Bahnschrift stack, normal width | 600 | 15px / 32px (strip height) | 0.02em | The NAME cell. Never text-transformed: a `ViolaName` is lower-case `[a-z0-9-]` and must read exactly as it is typed as a `target`. |
| Body | Cascadia Mono stack | 400 (600 for `human` in WHEEL and in `prompt · human`, and for the strip's `DIALOG` word) | 13px / 20px | 0 | Field values, tape text, transfer markers, phraseology words |
| Label | Bahnschrift stack, semi-condensed | 400 | 11px / 16px, uppercase | 0.06em | Column captions (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), and `BAY`, `5H` / `7D` and `TAPE` in the ATIS header (the tape panel's `TAPE` separator is Heading). The strip's `DIALOG <kind>` value is not Label: it is Body (Cascadia 13px) with `DIALOG` at weight 600, so the 20ch DIALOG track measures it in `ch` like every other field. |
| Code | Cascadia Mono stack | 400 | 12px / 20px | 0 | Commands shown in empty states (`viola run <name> -- claude`), the expanded tape body (`white-space: pre-wrap`) |
| Data | Cascadia Mono stack | 400 (600 for `RB`) | 13px / 20px. The one exception is `RB` inside the readback box: `--fs-label` 11px, line-height 1, `--fw-strong` 600, identical in every place the box appears | `font-variant-numeric: tabular-nums` | Percentages, timestamps `HH:MM:SS.mmm`, `since`, cursors, dialog ids, ages, skipped counts |

<!-- /detail -->

<!-- detail: values · typography -->

- `--font-label: "Bahnschrift", "DIN Alternate", "Avenir Next Condensed", "DejaVu Sans Condensed", sans-serif;`
- `--font-field: "Cascadia Mono", Consolas, "SF Mono", Menlo, "DejaVu Sans Mono", monospace;`

| Role token | Windows (live target) | macOS (CI-tested) | Linux (CI-tested) | Last resort |
|---|---|---|---|---|
| `--font-label` | Bahnschrift (DIN 1451) | "DIN Alternate" (DIN, ships with macOS), then "Avenir Next Condensed" | "DejaVu Sans Condensed" (the condensed system sans) | `sans-serif` |
| `--font-field` | "Cascadia Mono", then Consolas (on every Windows) | "SF Mono", then Menlo | "DejaVu Sans Mono" | `monospace` |

- Semi-condensed labels use `font-stretch: 87.5%`, which Bahnschrift's variable width axis honours. On the fallbacks it is a no-op: DIN Alternate, Avenir Next Condensed and DejaVu Sans Condensed are already narrow cuts.
- Strip and tape columns are sized in `ch` of `--font-field` (`--strip-cols`, `--tape-cols`), so they re-measure to whichever monospace resolves. DejaVu Sans Mono is wider than Cascadia Mono, so the Linux render is the widest case the layout must hold without horizontal scroll.

<!-- /detail -->
