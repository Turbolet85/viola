group: design-system+layout-templates
read: .claude/docs/design-summary.md · 78 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/la»

- leaf: .claude/docs/design-summary.md:55   class: omission
  leaf-text: «the `viola send` readback mirror, phraseology verb output, `viola verify` step counter,»
  source: .andromeda/design-system.md:811 «A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary: stderr carries exactly `unable: verify was stopped before it finished`»
  replace-with: «the `viola send` readback mirror, phraseology verb output, `viola verify` step counter (a run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` on stderr, exit 1, no stamp written),»
