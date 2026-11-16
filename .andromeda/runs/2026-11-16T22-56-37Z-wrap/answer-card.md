# Answer card — Q1 · 2026-11-16-scheduled-wake-count wrap

## The answer cited
- The founder's ruling B, his own word «Ратифицировать + добавить session_crons записью (Recommended)», 2026-11-16,
  relayed verbatim by the Viola overseer — CARRY cry-ljmfpqi3 on «Scheduled-wake count on a turn end» (relay
  `.andromeda/runs/2026-11-16T06-05-33-wrap/inputs/I1-e3-mid-route-adaptation.md.txt`): Stop's `session_crons`
  carried as a count beside `background_tasks` on `turn-ended` (one number, no content); `wait`'s wake rule itself
  does not change.

## The operations read as lying wholly inside it (9 lines)
- ops-architecture.md · contract/event-data/turn-ended-last-assistant-message · standard-contracts · fit: fits type 3 — «carries no scheduled-wake count» ends; data reads four keys
- ops-architecture.md · contract/hook-to-kind-map/stopfailure · conventions · fit: fits type 3 — the API-error end's data gains `scheduled_wakes` null
- ops-architecture.md · contract/channel-methods/wait-wake-kinds · standard-contracts · fit: fits type 3 — every `turn-ended` wakes whatever its `background_tasks` or `scheduled_wakes` (rule unchanged)
- ops-architecture.md · contract/event-data/turn-ended-scheduled-wakes · standard-contracts · fit: fits type 3 — the count, its null cases, its three hops
- ops-security-plan.md · decision/api-error-turn-end-text-dropped · data-protection · fit: fits type 1 — null enumeration gains `scheduled_wakes`
- ops-main-security-plan.md · decision/stop-session-crons-count-only · input-validation · fit: fits type 1 — `session_crons` a length only, no element read
- ops-test-plan.md · contract/event-data/turn-ended · integration-test-strategy · fit: fits type 3 — four keys
- ops-test-plan.md · contract/dialog-data/scheduled-wakes · unit-test-strategy · fit: fits type 3 — the unit sibling of null-details
- ops-test-plan.md · contract/turn-ended/scheduled-wakes · e2e-test-strategy · fit: fits type 3 — the e2e sibling of background-tasks

Named beside the card, not asked (rejected at check 1 / check 2, inside ruling B's subject but no row of their own):
ops-security-plan.md · decision/stop-session-crons-count-only (its `state: provisional`; main's add carries it) ·
decision/scheduled-wake-count-reach · ops-layout-templates.md · contract/wait-turn-ended-json/scheduled-wakes ·
name/other/scheduled-wakes. No proposal outside ruling B reached the card.

## The answer
The operator AWAY; no dialog raised (`inputs#I5`). The wrap directive (`inputs#I4`, item 1), verbatim: «the rows that
lie wholly inside ruling B (the count beside `background_tasks` on `turn-ended`, one number, no content; `wait`'s wake
rule unchanged) are applied with the delegate's confirm: «resolved: apply, PROVISIONAL: delegate overseer,
2026-11-16» — the ratification his ruling B, the confirm a delegate's, to be replaced by his own word in the epoch
boundary's batch.»
Confirm: all 9 — delegate overseer, 2026-11-16, PROVISIONAL; the founder's own confirm owed at the epoch boundary
(`founder-forks.md` F1).
