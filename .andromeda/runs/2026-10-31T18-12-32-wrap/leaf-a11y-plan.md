group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 30 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/docs/a11y-summary.md:20   class: omission
  leaf-text: «cli / tui output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1 with no conformance claim»
  source: .andromeda/a11y-plan.md:196 «SC 2.1.1 (boundary stand-in on tui)»
  replace-with: «cli / tui output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1 and the tui boundary for SC 2.1.1, with no conformance claim»
- leaf: .claude/docs/a11y-summary.md:53   class: omission
  leaf-text: «The MCP `wait` tool (chunk 2026-10-17-mcp-server-for-drivers)»
  source: .andromeda/a11y-plan-detail/cognitive-accessibility.md:8 «`viola wait` returns `{timed_out:true}` only for a timeout the caller passes in.»
  replace-with: «`viola wait` returns `{timed_out:true}` only for a timeout the caller passes in. The MCP `wait` tool (chunk 2026-10-17-mcp-server-for-drivers)»
- leaf: .claude/docs/a11y-summary.md:53   class: omission
  leaf-text: «a key pressed during the wait goes to the child and takes the wheel at once.»
  source: .andromeda/a11y-plan-detail/cognitive-accessibility.md:8 «a key pressed during the wait goes to the child and takes the wheel at once, and the waiting send is then refused `human-typing`.»
  replace-with: «a key pressed during the wait goes to the child and takes the wheel at once, and the waiting send is then refused `human-typing`.»
- leaf: .claude/rules/a11y.md:19   class: omission
  leaf-text: «human keystrokes are never blocked or delayed past the atomic paste.»
  source: .andromeda/a11y-plan.md:334 «never count focus / mouse / resize sequences or terminal replies as editing (P4) — a mouse report the Windows ConPTY already turned into win32 key-down records is typing (F-W3).»
  replace-with: «human keystrokes are never blocked, refused or delayed past the atomic paste, and focus / mouse / resize sequences and terminal replies are never counted as editing — a mouse report the Windows ConPTY already turned into win32 key-down records is typing (F-W3).»
- leaf: .claude/rules/a11y.md:19   class: omission
  leaf-text: «tui: the terminal belongs to the child,»
  source: .andromeda/a11y-plan-detail/a11y-scope-summary.md:50 «under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.»
  replace-with: «tui: the terminal belongs to the child (under `viola revive --json` a `pump_child` error prints no document: viola writes nothing past it),»
- leaf: .claude/rules/a11y.md:22   class: omission
  leaf-text: «`viola link` / `viola unlink` print one plain-ASCII result line in one write with no SGR in any mode.»
  source: .andromeda/a11y-plan.md:53 «C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.»
  replace-with: «`viola link` / `viola unlink` print one plain-ASCII result line in one write with no SGR in any mode. C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.»
- leaf: .claude/rules/a11y.md:22   class: omission
  leaf-text: «The page's contrast pairs are deferred with the frontend to 0.2.0.»
  source: .andromeda/a11y-plan.md:257 «on the CLI, `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line»
  replace-with: «A CLI refusal is `unable  <reason>  <detail>` on stderr (`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: one `unable: <text>` line), then `hint:` as the last stderr line, with a typed exit code; the one refusal with no hint is `unknown` (exit 14), faults are not refusals and carry no hint, and `--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths (a fixed stderr line, no document). The page's contrast pairs are deferred with the frontend to 0.2.0.»
