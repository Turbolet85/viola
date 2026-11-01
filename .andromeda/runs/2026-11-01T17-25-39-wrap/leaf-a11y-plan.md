group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 30 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/docs/a11y-summary.md:18   class: omission
  leaf-text: «output-discipline checks on all three OSes»
  source: .andromeda/a11y-plan.md:261 «server-verify peer pid witness on windows-2025 and ubuntu-latest only: macOS reports no peer pid, the word there an euid mismatch alone; the socket-directory refusal on ubuntu-latest and macos-latest»
  replace-with: «output-discipline checks on all three OSes (the frame verbs' server-verify peer pid witness on windows-2025 and ubuntu-latest only: macOS reports no peer pid, the word there an euid mismatch alone; the socket-directory refusal on ubuntu-latest and macos-latest)»
- leaf: .claude/rules/a11y.md:19   class: omission
  leaf-text: «viola writes nothing past it)»
  source: .andromeda/a11y-plan-detail/a11y-scope-summary.md:67 «a peer the wrapper refuses is dropped in `accept_loop`, before its worker thread and any frame: it writes nothing to the outer terminal and moves no wheel»
  replace-with: «viola writes nothing past it, and a peer the wrapper refuses is dropped in `accept_loop`, before its worker thread and any frame: it writes nothing to the outer terminal and moves no wheel)»
