group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 97 · «- 2026-10-17: A new test binary's nextest layer is proven by»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/rules/testing.md:40   class: omission
  leaf-text: «compare kebab-case serde values and integer exits, never `Debug` strings.»
  source: .andromeda/test-plan.md:596 «A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` (`tests/support/home.rs` `UNWRAPPED`) and a path oracle compares `resolved(home)`, never `TestHome::path()`.»
  replace-with: «compare kebab-case serde values and integer exits, never `Debug` strings. A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` (`tests/support/home.rs` `UNWRAPPED`), and a path oracle compares `resolved(home)`, never `TestHome::path()`.»
