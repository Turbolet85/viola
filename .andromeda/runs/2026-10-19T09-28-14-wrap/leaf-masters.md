group: masters
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:20   class: retired
  leaf-text: «`SPINE_DEADLINE`, `DIALOG_DEADLINE`»
  source: .andromeda/architecture.md:66 «(`viola_core::DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX`, PROVISIONAL, beside `SPINE_DEADLINE`)»
  replace-with: «`SPINE_DEADLINE`, `DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX`»
- leaf: CLAUDE.md:20   class: omission
  leaf-text: «`Percent`, `v` constants»
  source: .andromeda/security-plan.md:205 «`Percent` and `DialogHold` are `viola-core`'s»
  replace-with: «`Percent`, `DialogHold`, `v` constants»
