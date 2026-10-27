proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 46
    section: "## 1. A11y Scope Summary"
    change: "Add a Reason sub-bullet after line 46: `- the boundary holds after a contained vt100 feed panic, witnessed by `chaos_feed_panic_refuses_send_and_keeps_passthrough` (`tests/chaos_feed_panic.rs`): on every OS none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and on Unix the child's `e4 b8 ad` reach it within `WITHIN`; green on ubuntu, macOS and windows-2025 (chunk 2026-10-27-bounded-inputs-at-every-boundary).`"
    sidecar: "2026-10-27 · a11y-scope-summary · tui Entity: the boundary after a contained vt100 feed panic witnessed on the outer terminal (every-OS literal absence, Unix bytes) — chunk 2026-10-27-bounded-inputs-at-every-boundary"
    rationale: "Report Coverage of new surfaces (the tui passthrough after a feed panic: tests e2e, a11y literal-absence on every OS) and Outcome (a11y) met green on the three CI OSes; Expected amendments name a11y-scope-summary.md:40, the tui Entity line, as the site for the witness."
    basis: tests/chaos_feed_panic.rs:109
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 46
    section: "## 1. A11y Scope Summary"
    change: "Add after the tui Entity (line 46) an Owed line in the scope-entities Owed shape: `- **Owed:** the Windows outer-terminal byte reading of the child's wide character (`e4 b8 ad`) after a contained vt100 feed panic — unrunnable here (ConPTY re-renders a wide character on a 1-column screen; windows-2025 reads only viola's literal absence); owed to \"Windows-only live measurements\" by the P5 card's CARRY, PROVISIONAL until the founder's own word (chunk 2026-10-27-bounded-inputs-at-every-boundary).`"
    sidecar: "2026-10-27 · a11y-scope-summary · Owed: the Windows byte oracle of the tui passthrough after a feed panic, to \"Windows-only live measurements\" (P5 CARRY, PROVISIONAL)"
    rationale: "Report Coverage of new surfaces (byte reading `unrunnable-here` on Windows, owed by the P5 card's CARRY on «Windows-only live measurements») and Decisions & corrections (no Windows byte oracle assertable; PROVISIONAL, inputs#I3)."
    basis: tests/chaos_feed_panic.rs:110
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 33
    section: "## 1. A11y Scope Summary"
    change: "Append to the tui Entity broad line: `; the boundary is witnessed after a contained vt100 feed panic too (every OS: no viola literal on the outer terminal; Unix: the child's own bytes), its Windows byte reading owed to \"Windows-only live measurements\".`"
    sidecar: "2026-10-27 · a11y-plan §1 · tui Entity broad line names the feed-panic witness and its owed Windows byte reading"
    rationale: "Report Expected amendments (a11y-plan §1) name a11y-plan.md:33 beside a11y-scope-summary.md:40 as the tui Entity sites for the witness; Coverage of new surfaces and Outcome (a11y) give the every-OS literal absence, the Unix bytes and the owed Windows reading."
    basis: .andromeda/a11y-plan.md:33
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 19
    section: "## 4. ARIA Patterns & Roles"
    change: "Append to the P4 tui bullet, after the readiness-gate case under clause (2): `A second case sits under clause (2): after a forced vt100 feed panic a typed key still reaches the child and an automation `send` is refused `not-delivered` / `input-not-ready` (exit 13) — the poisoned screen blocks no human keystroke (`chaos_feed_panic_refuses_send_and_keeps_passthrough`, `tests/chaos_feed_panic.rs`; green on all three OS legs).`"
    sidecar: "2026-10-27 · aria-patterns-roles · P4 tui clause (2) gains the feed-panic case: typed key reaches the child, `send` refused `input-not-ready` exit 13"
    rationale: "Report Outcome (a11y): after a forced feed panic `send` refused `input-not-ready` (exit 13) and a typed key reaching the child, met green on ubuntu, macOS and windows-2025; Coverage of new surfaces names the case."
    basis: tests/chaos_feed_panic.rs:109
    fact: F3
note: body §4's tui tool-reach row (a11y-plan.md:187), §9 and §10's suite claims and the P4 broad line (:196) re-read true with the chaos case inside the coverage suite; the CARRY itself is the working route's «Windows-only live measurements».
