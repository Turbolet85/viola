group: architecture+design-system+layout-templates
read: .claude/rules/frontend.md · 33 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:3   class: tree
  leaf-text: «  - "crates/viola-ui/assets/**"»
  source: .andromeda/architecture.md:108 «Members: `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, plus the test-only `viola-e2e`»
  replace-with: «  - "src/human/**"»

- leaf: .claude/rules/frontend.md:27   class: omission
  leaf-text: «## CLI surface (`src/cmd/**`)»
  source: .andromeda/architecture.md:351 «Read by every `viola` build's human output (the colour decision of `viola list`, `src/human/style.rs` `read`; release builds included)»
  replace-with: «## CLI surface (`src/cmd/**`, `src/human.rs`, `src/human/**`)»

- leaf: .claude/rules/frontend.md:28   class: omission
  leaf-text: «the depth is truecolor / 256 / 16 from `COLORTERM` / `TERM`»
  source: .andromeda/design-system.md:411 «- 256-colour if `TERM` contains `256color`, or on Windows once VT enabling has succeeded;»
  replace-with: «the depth is truecolor / 256 / 16 from `COLORTERM` / `TERM` (256 on Windows once VT enabling has succeeded)»
