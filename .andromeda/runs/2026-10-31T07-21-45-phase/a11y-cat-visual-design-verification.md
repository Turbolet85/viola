# a11y · Visual Design Verification — category read

## Carried
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) Every CLI `--json` error document this chunk reshapes, and any revive refusal pair a sanitised message reaches, must hold zero SGR of viola's own and no ESC byte.
  > viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`, and the revive refusal pairs and `--list` rows carry no ESC byte.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:9) A fixed message or code this chunk writes must leave the `unable` / `fail` words uncoloured, and that check must keep passing on all three OS legs.
  > `unable` / `fail` are never coloured. Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`.
- (.andromeda/a11y-plan.md:340) The ban stands over every error path the chunk changes: no colour on `unable` / `fail`, and no SGR in the machine or plain modes.
  > - NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.

## Bindings
- **Zero SGR under `--json` ↔ the body's §8 Error recovery**: the sanitised `--json` error is one document free of SGR (.andromeda/a11y-plan-detail/visual-design-verification.md:9) and free of any hint (.andromeda/a11y-plan.md:257); both hold on the same document.
- **The three-leg check ↔ the body's §9 CI Integration**: the SGR and uncoloured-word checks (.andromeda/a11y-plan-detail/visual-design-verification.md:9) run inside the per-OS `test` job's single `coverage` suite with no second driver and no new `suite` value (.andromeda/a11y-plan.md:300).
