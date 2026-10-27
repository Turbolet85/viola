# Apply — a11y-plan

## Verdicts
- fanout-a11y-plan--cascade.md 3 · applied · .andromeda/a11y-plan.md:33#1
- fanout-a11y-plan.md 1 · folded into fanout-a11y-plan--cascade.md 3
- fanout-a11y-plan--cascade.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:48+#1
- fanout-a11y-plan--cascade.md 2 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:49+#1

## Edits
- line: .andromeda/a11y-plan.md:33
  old: «and do not count as human editing.»
  new: «and do not count as human editing; the boundary still holds after a contained vt100 feed panic (no viola literal on the outer terminal on every OS, the child's own bytes on Unix), its Windows byte reading owed to "Windows-only live measurements".»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:48
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** `viola run` TUI passthrough after a contained vt100 feed panic (the boundary of the entity above)»
  add: «  - **Source:** `tests/chaos_feed_panic.rs` (`chaos_feed_panic_refuses_send_and_keeps_passthrough`).»
  add: «  - **Assertability:** boundary-only»
  add: «  - **Reason:** the driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, on all three OSes; on Unix the child's own bytes (`e4 b8 ad`) reach it.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:49
  anchor: «»
  next: «<!-- detail: values · scope-entities -->»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Owed:** the Windows outer-terminal reading of the child's own bytes (`e4 b8 ad`) after a contained vt100 feed panic — ConPTY re-renders a wide character on a 1-column screen, so windows-2025 reads only viola's literal absence; owed to "Windows-only live measurements".»
  add: «»
  add: «<!-- /detail -->»
  add: «»

## Patterns
(none)
