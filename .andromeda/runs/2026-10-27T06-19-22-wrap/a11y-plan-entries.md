## 2026-10-27-bounded-inputs-at-every-boundary — the tui passthrough boundary after a contained vt100 feed panic
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new scope Entity after the `viola run` TUI passthrough Entity; a11y-plan.md: the `viola run` TUI passthrough Entity line)
**Change:**
- a11y-scope-summary.md gains the Entity `viola run` TUI passthrough after a contained vt100 feed panic (the boundary of the entity above), boundary-only, its Source `tests/chaos_feed_panic.rs` (`chaos_feed_panic_refuses_send_and_keeps_passthrough`). Its Reason: the driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, on all three OSes; on Unix the child's own bytes (`e4 b8 ad`) reach it.
- The body's tui passthrough Entity line ended at "do not count as human editing"; it now goes on: the boundary still holds after a contained vt100 feed panic (no viola literal on the outer terminal on every OS, the child's own bytes on Unix).
**Why:** the report records the case as new coverage: the outer-terminal half of the feed-panic case is witnessed, the bytes on Unix and the literal absence on every OS. Its (a11y) outcome is met, green on ubuntu, macOS and windows-2025. The report's Expected amendments name a11y-plan §1's two tui Entity sites for the witness.
**Kept:** §4 ARIA Patterns & Roles (aria-patterns-roles.md, the P4 tui bullet's clause (2)) is unchanged. The case's `send` refusal and its typed key reaching the child landed at chunk 2026-10-04, and this chunk adds only the outer-terminal half. §3's keyboard test harness key is unchanged too, as the report names it no-change.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the Windows byte reading after a feed panic, owed
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new scope-entities Owed line after the feed-panic Entity; a11y-plan.md: the `viola run` TUI passthrough Entity line)
**Change:**
- a11y-scope-summary.md gains an Owed line: the Windows outer-terminal reading of the child's own bytes (`e4 b8 ad`) after a contained vt100 feed panic. ConPTY re-renders a wide character on a 1-column screen, so windows-2025 reads only viola's literal absence. The reading is owed to "Windows-only live measurements".
- The body's tui passthrough Entity line now ends with the same debt: its Windows byte reading is owed to "Windows-only live measurements".
**Why:** the report finds no Windows byte oracle for the panicking wide character assertable here: ConPTY re-renders the child's output, and a wide character on a 1-column screen has no guaranteed rendering. The case's byte reading is `unrunnable-here` on Windows. The reading is the CARRY on «Windows-only live measurements» in the P5 card's Beyond-the-plan row. That card was approved by the word of delegate overseer1 (founder-delegated), relayed by the operator, and it stays PROVISIONAL until the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/
