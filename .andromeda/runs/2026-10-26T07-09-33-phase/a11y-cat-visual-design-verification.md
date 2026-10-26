# a11y · Visual Design Verification — category read

## Carried
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The refusal a CLI verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) prints when the server verification fails or cannot be made must carry no colour on its `unable` word, the same as every other refusal.
  > `unable` / `fail` are never coloured.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) The new mismatch path in every client, its `--json` document and its stderr pair included, must write no SGR byte of viola's own in any of the named modes.
  > viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`
- (.andromeda/a11y-plan.md:221) The body's broad line for this category: the verification's refusal output stays inside the CLI's SGR rule, colour only in `viola list` and never on `unable`.
  > no SGR of viola's own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`; `unable` / `fail` never coloured
- (.andromeda/a11y-plan.md:340) The Visual anti-pattern the new refusal path must not break.
  > NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.

## Bindings
- **No-colour `unable` on the verification refusal ↔ the body's §8 Error recovery**: the mismatch refusal is an `unable  <reason>  <detail>` line on stderr, then `hint:`, with a typed exit code, or one `--json` document with no hint. The Visual rule keeps that line free of colour, and §8 fixes its layout, (.andromeda/a11y-plan-detail/visual-design-verification.md:9) ↔ (.andromeda/a11y-plan.md:257).
- **Zero SGR under `--json` ↔ the body's §1 cli surface**: the verification's `--json` refusal document carries no SGR, glyph or cursor control, (.andromeda/a11y-plan-detail/visual-design-verification.md:9) ↔ (.andromeda/a11y-plan.md:53).
