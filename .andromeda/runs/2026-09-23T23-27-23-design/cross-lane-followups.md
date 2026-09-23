# Cross-lane follow-ups — /andromeda-design run 2026-09-23T23-27-23

Items design found that belong to another specialist's lane. Design does not edit those artifacts; wrap's reconcile folds them in.

## CL-1 → architecture.md: log sends and refusals as events

- **Found in:** Phase 3 (exploration.md, Signature Element implementation notes), confirmed by the orchestrator against architecture.md 2026-09-24.
- **Gap:** `viola ui` is a pure reader of `events.ndjson`, but no event is written when a `send` is issued or refused. A refusal (`result.refusal` + `detail`) returns only to the caller; the only logged trace of a send is the driven session's `prompt-submitted` (origin `driver`) when it lands. The normalised event kinds (Conventions → Naming patterns) have no send-issued / send-refused kind.
- **Why it is arch's gap, not a design stretch (overseer, founder-delegated, 2026-09-24):** brief §7 makes the event log the audit trail, so sends and refusals belong in `events.ndjson` regardless of design.
- **Requested addition:** two event kinds appended by the driven instance's wrapper (`source: wrapper`):
  - `send-issued` — carrying the send's `cursor` (the end offset of `events.ndjson` when the send was accepted, as in the `send` ok payload) and `from` when present;
  - `send-refused` — carrying `refusal` + `detail` (the `RefusalReason` and its closed-set detail, incl. `control-character` from security-plan.md) and `from` when present.
  - Whether a ledger-listed local command's `ok` with `{"confirmed":false,"detail":"unconfirmable"}` also needs its own line is for arch to decide.
  - **Extended by Phase 4 draft (2026-09-24):** the design system's readback box has a fourth printed word, `unconfirmable`, which it can reach only if that `ok`/`confirmed:false` outcome is logged; and the outbound transfer marker needs `from` on `send-issued` to place the send under the driver's strip. The draft's readback pairing rule is: the next `prompt-submitted` with origin `driver` on that instance (at most one send in flight, since a send during a running turn is refused `turn-running`), or `session-start` cause `clear` with a new session id for a `/clear` send — arch should confirm this pairing holds or add an explicit correlation field.
- **Design dependency:** the signature element "The readback box" (quiz-design.md Q5) has three states — `open` (send-issued, no matching `prompt-submitted` yet), `RB` (matching `prompt-submitted` landed), `refused` (send-refused, typed reason printed). Design specifies all three; until arch adds the events, a v1 page can render only `RB`.
- **Owner:** architecture (via wrap reconcile into architecture.md). Downstream readers to re-check after the fold: security (new event content on SSE — refusal detail only, no new user content), obs, tests.
