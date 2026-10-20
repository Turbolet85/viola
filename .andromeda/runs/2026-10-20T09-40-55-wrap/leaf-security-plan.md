group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/docs/security-summary.md:73   class: omission
  leaf-text: «The two keepers follow a linked base only when its target is absolute and one `lstat` reads a real directory with no group or other bit; they create it 0700 and remove nothing.»
  source: .andromeda/security-plan.md:536 «a target that is itself a link is refused, so is a backing whose filesystem reads less than the 9 GiB headroom floor free (`statvfs`), and a refusal is a fixed message that names no path»
  replace-with: «The two keepers follow a linked base only when its target is absolute and one `lstat` reads a real directory with no group or other bit, and refuse a backing whose filesystem reads less than the 9 GiB headroom floor free (`statvfs`), each refusal a fixed message that names no path (the headroom one holds the two counts); they create it 0700 and remove nothing.»
