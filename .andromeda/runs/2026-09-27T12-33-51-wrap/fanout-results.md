# Fan-out results — 2026-09-27-wrapper-channel (wrap resumed at P2)

Seven Explore doc-agents, one parallel batch, prompt from `amendment-flow.md` sent verbatim. Returns were clean YAML
(no preamble to strip; entity probe: no `&lt;`/`&gt;`/`&amp;` in any return).

| doc | verdict | raw twin |
|---|---|---|
| architecture | 18 proposals (A1–A18) | `.raw-fanout-architecture.md` (transcribed, fields kept) |
| security-plan | 5 proposals (S1–S5; S3–S5 dependents of S1/S2) | `.raw-fanout-security-plan.md` |
| design-system | `proposals: []` — all three new surfaces `tokens n/a` | — |
| layout-templates | `proposals: []` — squatted-name is a new cause in the existing `viola run` exit-1 region (layout :483-492, :532-533) | — |
| test-plan | 17 proposals (T1–T17); D-tests-framework no drift | `.raw-fanout-test-plan.md` (transcribed, fields kept) |
| obs-plan | `proposals: []` — noted gap: :739 / :1528 name `srv_conn` only for a missing `conn`, not a mis-shaped one | — |
| a11y-plan | `proposals: []` — noted: a11y :220-225 is the §1 verbatim copy of the obs Log Format | — |

## Validate

**Re-derivation tell (rejected before the checks):** A4, A6, A13, A14, A15, A16, A17 cite source or manifest
locations the report does not carry (`crates/viola-channel/src/{endpoint,server,frame}.rs`, `Cargo.toml:153/157`,
`crates/*/Cargo.toml`), and A4/A15 carry facts the report does not (fallback on a relative/empty dir value, socket
removed before the lock is released, lock mode 0600, windows-sys/libc versions). A18 falls with its primary A17 (group
rule). Each fact the REPORT or PLAN carries is re-raised by the orchestrator below (O1–O7), re-derived from the report.

**Check 1 — playbook:**
- Routine, "Accurate this-chunk addition": A1, A2, A3, A5, A7, A8, A9, A10, A11, A12; S1–S5; T1–T12, T15–T17.
- S1/S3/S4/S5 (chmod 0600 after bind replaces `mode(0o600)`): weighed against "Boundary widening" — not that class:
  nothing new crosses; the Unix socket reaches 0600 on every OS where the spec'd mechanism left macOS at the umask
  mode; the 0700 per-user dir stays primary. S2 (DACL via windows-sys conversion; read-back FA, SID alias): same
  two ACEs, GA's pipe mapping is FA — no widening.
- **Escalated — T13 + T14 (E2 fd witness):** no rule matches and main is uneasy. The report calls the new predicate
  "overseer-widened"; directive 1 (founder ruling 2026-09-27) says a widening halts for a live answer and an earlier
  direction never ratifies it.

**Orchestrator raises (check 5 expected amendments · check 6 disproved claims · detector-noted gaps), all routine:**
- O1 (A4 re-raised; expected amendment 1): arch IPC endpoints Unix → the per-user 0700 dir per security-plan arch
  amendment 2, `<dir>/viola-<h12>.sock`, chmod 0600 after bind, arbiter `<dir>/viola-<h12>.lock` (report: exclusive-bind
  start arbiter; plan Goal: "Unix: a held `.lock` sibling"; Reverted bullet).
- O2 (A6 re-raised): arch Wrapper channel frames → the `MAX_FRAME` line bound (`\n` counted), `-32600` + close, `conn`
  stripped before dispatch and never identity, only `hook.event` id-less (report Symbols / APIs; plan acceptance).
- O3 (A13): arch Stack Logging → tracing gains `attributes` (report Dependencies).
- O4 (A14): arch Stack → veil `=0.3.0`, no `toggle` (report Dependencies).
- O5 (A15): arch Stack Wrapper IPC → the server DACL through windows-sys `Win32_Security_Authorization`; libc on Unix
  for `viola-channel` (report Dependencies, Reverted).
- O6 (A16; expected amendment 2): arch Crate dependency direction `viola-channel` → + serde, serde_json, thiserror,
  tracing, veil (plan list), libc Unix (report); sync and tokio-free as landed.
- O7 (A17 + A18; expected amendment 2): `viola-pty` gains tracing — Crate dependency direction and [Error Handling]'s
  exact-set clause.
- O8 (expected amendment 5; disproved claim 1): obs-plan §3 `corr` table (:660-662) + test-plan §3 Log format
  (:702-704, T9) bound pair, and obs :697 "declares `corr` … as optional keys" → optional except the per-event rules the
  schema now requires.
- O9 (expected amendment 6): security-plan §Bootstrap phases :371 amendment 2 → folded by this chunk; :376 (S4) the
  SDDL landed with `viola-channel`, ahead of Epoch 6's admission entry.
- O10 (obs detector gap): obs-plan §3 IPC `conn` → a mis-shaped peer `conn` also logs `srv_conn` (report Symbols).

**Check 2 — cross-contradiction:** T9 and O8 edit the bound test↔obs pair in one direction; S2/T15/T16 and O5 state the
same FA/alias fact. None opposing.
**Check 3 — intent-consistency:** justified divergences (report Deviations): `corr` rules per obs discriminators, chmod
after bind, E2 predicate (escalated), conn strip + id-less rejection, folded extras. No unjustified divergence.
**Check 4 — absence needs evidence:** no proposal claims an absence; the cascade sweep carries the caught-ALL claim.
**Check 5 — expected amendments:** 1 → O1 + A5; 2 → O6 + O7; 3 → A1 + A2 + A3; 4 → T1–T3 + T5–T8 + T15; 5 → O8;
6 → O9 + S4. All covered.
**Check 6 — disproved claims:** (1) → O8 + T9; (2) → T13/T14 (escalated); (3) → S2 + T15 + T16; (4) → T17 (+ O2's
close); (5) → S1 + S3–S5 + O1.
