group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 30 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/a11y-summary.md:53   class: omission
  leaf-text: «`passthrough_with_a_statusline_source_adds_no_viola_bytes` (`tests/tui_passthrough.rs`) is its reading of the zero-viola-bytes clause, green on the three CI runners.»
  source: .andromeda/a11y-plan.md:33 «the boundary still holds after a contained vt100 feed panic (no viola literal on the outer terminal on every OS, the child's own bytes on Unix)»
  replace-with: «`passthrough_with_a_statusline_source_adds_no_viola_bytes` (`tests/tui_passthrough.rs`) is its reading of the zero-viola-bytes clause, green on the three CI runners. The boundary still holds after a contained vt100 feed panic: `chaos_feed_panic_refuses_send_and_keeps_passthrough` (`tests/chaos_feed_panic.rs`) refuses the driver's `send` `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child, and none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal on all three OSes; the child's own bytes reach it on Unix, their Windows reading owed to "Windows-only live measurements".»
