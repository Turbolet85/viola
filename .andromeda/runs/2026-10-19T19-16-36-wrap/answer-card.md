# Answer card — 2026-10-19-overseer-switch-to-viola

## The founder's answer it rests on
- **P4 fork Q4, "New gaps (playbook pattern "Boundary widening")"** (`inputs#I14`, his own word given live in the overseer's
  dialog, relayed by the overseer, 2026-10-19T10:41Z). The answer: "Build both, count only (Recommended)". The notes: «Строить
  оба» — option description and preview in the question event, `background_tasks` as a count only; the widening ratified by
  him. The question named the widening: "Carrying option text into events.ndjson and drivers' `wait --json` and MCP results
  is a boundary widening (a validated surface admits more upstream content)."

## The escalated proposals lying wholly inside it (numbered as `fanout-results.md`)
- architecture 2 · §Standard Contracts, `question` data row: each question gains `option_details` (`{description, preview}`
  per option, a string or `null`), reaching `events.ndjson`, `hook.dialog`, `wait --json` and the MCP `wait`.
- security-plan 1 · §Input Validation, Hook stdin row: the two new keys with their validation (string or `null`; an array
  length only); the option-text crossing recorded as his P4 Q4 widening.
- security-plan 2 · Threat Model Summary, data classification: `question` content includes option description and preview
  text; `turn-ended` adds a count, no text.
- security-plan 3 · logging-redaction-wire, the floor's Scope: option text named as user content, carried only in the
  by-design content payloads, otherwise only in `diagnostics/`.
- security-plan 4 · Security Anti-Patterns R1: option text named among upstream text never interpreted.

None of these widens beyond his answer: no task text crosses, `options` keeps its shape, and no log line carries option text.

## The confirm, as given (verbatim, relayed by the overseer; `inputs#I21`)
"Confirm all five (Recommended)" — notes: The founder's own word, live at his keyboard, in the overseer's dialog at 2026-10-19T19:28Z: all five confirmed inside his P4 Q4 answer.
