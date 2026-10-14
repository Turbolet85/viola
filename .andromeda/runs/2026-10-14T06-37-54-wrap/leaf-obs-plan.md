group: obs-plan
read: .claude/docs/obs-summary.md · 77 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
- leaf: .claude/docs/obs-summary.md:70   class: omission
  leaf-text: «`deadline_hit` only when true.»
  source: .andromeda/obs-plan.md:706 «and none under the hidden `hook statusline --capture` arm, which runs no instance check, no shell-out and no `budget.json` write and opens no role file (§4 Edge flows);»
  replace-with: «`deadline_hit` only when true. The hidden `hook statusline --capture` arm (`viola verify`'s Run B probe capture, Unix only) does no obs init and runs no instance check, no shell-out and no `budget.json` write: it opens no role file, writes no line and prints nothing (panic exemption 5, with `hook <event> --capture`).»
