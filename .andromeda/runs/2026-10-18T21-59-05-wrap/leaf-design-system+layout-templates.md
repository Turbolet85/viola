group: design-system+layout-templates
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:7   class: omission
  leaf-text: «(for `/clear`, until its new-session post-condition does)»
  source: .andromeda/design-system.md:11 «for `/compact` on a verified CLI, once its measured post-condition (a `session-start` of cause `compact` carrying the session's own id) does; on an unverified CLI `/compact` stays `unconfirmable`.»
  replace-with: «(for `/clear`, until its new-session post-condition does; for `/compact` on a verified CLI, until its same-session compact post-condition does, and on an unverified CLI `/compact` stays `unconfirmable`)»
