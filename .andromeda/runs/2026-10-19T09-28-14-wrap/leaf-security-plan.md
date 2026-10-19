group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:28   class: omission
  leaf-text: «and no value switches the gate off.»
  source: .andromeda/security-plan.md:205 «no value makes the hold zero or unbounded, and no env var or flag sets it (a boundary widening the founder answered himself, chunk 2026-10-19-dialog-hold-for-the-driver, `inputs#I2` Q1)»
  replace-with: «and no value switches the gate off. The dialog hold `dialog_hold_s` is read once at the wrapper's start right after the thresholds (through `take(MAX_FRAME)`) as a whole number of seconds into `DialogHold` (60–3600, default 60); a bad value takes 60 with one `parse-rejected{parser:"config-json"}` line, no value makes the hold zero or unbounded, and no env var or flag sets it (a boundary widening the founder answered himself, chunk 2026-10-19-dialog-hold-for-the-driver).»

- leaf: .claude/docs/security-summary.md:66   class: omission
  leaf-text: «Thresholds are `Percent` with a per-key fallback; no config value switches the gate off; the gate is fail-open.»
  source: .andromeda/security-plan.md:205 «no value makes the hold zero or unbounded, and no env var or flag sets it (a boundary widening the founder answered himself, chunk 2026-10-19-dialog-hold-for-the-driver, `inputs#I2` Q1)»
  replace-with: «Thresholds are `Percent` with a per-key fallback; no config value switches the gate off; the gate is fail-open. The dialog hold `config.json` `dialog_hold_s` is read once at the wrapper's start into `DialogHold` (60–3600 s, default 60; a bad value takes 60 with one `parse-rejected` line); no value makes the hold zero or unbounded and no env var or flag sets it (a boundary widening the founder answered himself, chunk 2026-10-19-dialog-hold-for-the-driver).»
