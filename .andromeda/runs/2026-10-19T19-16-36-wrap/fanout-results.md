# Fan-out results — 2026-10-19-overseer-switch-to-viola

Before snapshot: `tree c5fae7c396b68a40135d3450c8d4872b3c0a1851 index ecdc77e92f51d1dd205e59e5d424d9cf4d05ee76 head aa965127d51eb4512ff5c5ef29be8bf89e70f474`.
After snapshot: `tree 15b82d8fa774f14e952e5a4731727cd623bbb081`, index and head unchanged. `git diff-tree` printed exactly the seven
`A .andromeda/runs/2026-10-19T19-16-36-wrap/fanout-{doc}.md` lines. No file needed a fix, so there is no raw twin. Each
receipt's count equals its file's.

Rejected for a source the report does not carry: 0. Every `basis` coordinate is the report's (Symbols / APIs) or a row of its
`## New text, by line`, or a line of the detector's own master.

## architecture: 3 proposals · fanout-architecture.md
note: no D-arch-decisions drift; `architecture.md:77` is a stale placement outside both detectors (raised below as 4).
1. D-arch-resources · §Standard Contracts, `turn-ended` data row (`architecture.md:255`) → **apply**. Check 1, the playbook's
   "Accurate this-chunk addition": `background_tasks` is a count, and no upstream text crosses. The text is re-derived; the
   founder's "count only" (`inputs#I14` Q4) is named in the sidecar's Why, not the body.
2. D-arch-resources · §Standard Contracts, `question` data row (`architecture.md:256`) → **escalate: answer card**. Check 1,
   "Boundary widening the founder answered himself to a P4 fork": the option description and preview text now cross into
   `events.ndjson`, `wait --json` and the MCP `wait`.
3. D-arch-resources (dependent-of 1) · §Standard Contracts, example `hook.event` frame (`architecture.md:230`) → **apply**
   with 1. A stale example of the same shape.
4. (raised by main; checks 5 and 6) · §Established Decisions, the dialog-hold bullet (`architecture.md:77`) → **apply**. The
   first live near-ceiling reading moves from "Overseer switch to viola" to "Version done-check"; the ceiling stays
   PROVISIONAL. No hold near the ceiling arose (27 345 ms). The P5 card's row 2 names this exact re-pin, approved by the
   operator's word (`inputs#I15`): a recorded direction settles it (check 1, no-match settled by direction).
5. (raised by main; check 6, Spec claims disproved 3) · §Established Decisions [Plugin Scope] rationale
   (`architecture.md:110`), the print-mode probe's residual → **escalate: Q3** (a new residual entering an accepted-risk
   class needs the operator's word).

## security-plan: 4 proposals · fanout-security-plan.md
note: D-security-auth and D-security-deps found no drift; the verify residual omission is at `security-plan.md:261`
(raised below as 5).
1. D-security-input · §Input Validation, Hook stdin row → **escalate: answer card** (the widening; check 1).
2. D-security-input (dependent-of 1) · Threat Model Summary, data classification → **escalate: answer card**. Its section is
   a verbatim upstream copy, which the playbook rule "Verbatim upstream copy kept current" judges as any body amendment; its
   subject is the widening.
3. D-security-input (dependent-of 1) · Bootstrap phases → logging-redaction-wire, the floor's Scope (`security-plan.md:390`)
   → **escalate: answer card**.
4. D-security-input (dependent-of 1) · Security Anti-Patterns → Code Patterns, R1 (`security-plan.md:527`) → **escalate:
   answer card**. It names option text as upstream text, a tightening that lies inside the same answer.
5. (raised by main; check 6, Spec claims disproved 3) · §Data Protection, the verify accepted-risk paragraph
   (`security-plan.md:261`): each real-CLI verify also leaves the CLI's empty project dir for the print probe's cwd →
   **escalate: Q3**.

## design-system: 0 proposals · fanout-design-system.md
note: D-design-tokens unfired; no new UI element.

## layout-templates: 0 proposals · fanout-layout-templates.md
note: D-layout-surface holds; human line forms unchanged.

## test-plan: 4 proposals · fanout-test-plan.md
note: D-tests-framework and D-tests-obs-harness found no drift.
1. D-tests-coverage · §6 Path 3 (`test-plan.md:673`) → **apply**. Check 1, "Accurate this-chunk addition"; check 5 carries
   the plan's test-plan §6 Paths 3–4 entry, the live reading written as a local measurement.
2. D-tests-coverage · §6 Path 4 (`test-plan.md:693`) → **apply** (as 1).
3. D-tests-coverage · §4 viola-agent-claude unit cases → **apply** (as 1). It names the functions; the `#[case]` rows are the
   count (Tier 1: a master states no test-case count).
4. D-tests-coverage · §5 On-disk `state_events.rs` → **apply** (as 1).

## obs-plan: 1 proposal · fanout-obs-plan.md
note: no new hot path; D-obs-pii holds (the option text reaches content payloads only). The expected §4 readback amendment
finds no stale text.
1. D-obs-instrumentation · §4 Scenario Dialog → `answer`, wrapper line (`obs-plan.md:610`) → **apply**. Check 6 disposes of
   Spec claims disproved 1: `from` / `from_trust` are present only when the answer carried a `from`. Check 1, "Accurate
   this-chunk addition": a reconciliation to the measured behaviour.
- Expected amendment "obs-plan §4 Scenario: `wait` / `last` event-driven readback — the two additive keys" (check 5) →
  **not carried, no change**. The detector read `obs-plan.md:282`, `:587` and `:597`: they state log fields only, and that no
  result body reaches a line, so no site states the event data keys. The report's `readback` sweep found no data-key site in
  obs-plan.

## a11y-plan: 0 proposals · fanout-a11y-plan.md
note: neither detector fires; the takeover clause's count goes stale (raised below as 1).
1. (raised by main; check 5) · §3 → Keyboard test harness (key file `keyboard-test-harness.md:3`) → **apply**. The takeover
   clause gains its second live reading on Linux: the founder's own key on claude 2.1.296 under the product binary,
   `wheel{human,human-input}` @26032, the next driver send exiting 10 `human-typing` with no `send-issued` (@26181), and his
   own `release` → `wheel{driver,release}` @26337, as measured at `evidence/switch-readings.ndjson` (takeover). "one live
   reading" becomes two.

## Disproved claims (check 6)
1. `obs-plan.md:610`: obs-plan 1.
2. v1-33's `send-confirmed{confirmed:true}`, a matrix entry: routed to P7.3 (premise correction), together with v1-33's
   "`dialog-answered` with `from`".
3. The verify residual omission: architecture 5 and security-plan 5, Q3.
4. The plan's step-8 mode premise: no master states it (pattern `homenarrow`, 4 standing, all consistent). Disposed: no
   master change.

## Escalations resolved (`inputs#I21`, the P2 halt, 2026-10-19T19:28Z)
- **Answer card:** confirmed in full, the founder's own word relayed by the overseer, so not provisional. architecture 2 and
  security-plan 1–4 → **apply**. The ratification is his P4 Q4 answer (`inputs#I14`).
- **Q2, inputs/obs:** "Inputs exempt, recorded". This is the overseer's technical answer, a delegate's, so PROVISIONAL until
  the founder's word at the Epoch 5 boundary batch. No master change: the criterion is read as covering `evidence/` and the
  report.
- **Q3, residual:** "State it as accepted", the founder's own word. architecture 5 and security-plan 5 → **apply**: the
  print-mode probe's empty project dir joins the accepted class of the Run B–D transcripts.

## Unlinked acceptance UNMET (check 3)
- The obs criterion "no prompt … text anywhere under the chunk folder" against the inputs rule (`inputs/I1`, `I15`, `I17`):
  **escalate: Q2**.
