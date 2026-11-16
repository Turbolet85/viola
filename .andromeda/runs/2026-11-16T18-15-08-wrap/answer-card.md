# Answer card — Q1 · 2026-11-16-granular-api-error-cause

Rule: «Boundary widening the founder answered himself to a P4 fork» (rul-bznresq5).

The answer cited: the founder's own answer to the P4 fork on the turn-end cause, ruling A «Подробный список
(Recommended)», 2026-11-16, relayed verbatim by the Viola overseer — on record in the plan's Implementation notes
(Expected amendments, security-plan §Data Protection), the scope's CARRY on the route entry, and re-stated by the
overseer's wrap directive `inputs#I3` item 1. What it covers: the CLI's StopFailure error word classified inside the
Claude adapter into nine closed turn-end cause words (`rate-limited` · `overloaded` · `auth-failed` · `billing` ·
`invalid-request` · `model-not-found` · `server-error` · `output-limit` · `api-error`), only the word crossing into
`events.ndjson`, `wait --json` and MCP `wait`, the error text crossing nothing.

The operations lying wholly inside it (11):
1. ops-architecture.md · contract/turn-end-cause/cause-set · conventions · fit: fits type 3
2. ops-architecture.md · contract/event-data/turn-ended-cause · standard-contracts · fit: fits type 3
3. ops-main-architecture.md · contract/hook-to-kind-map/stopfailure · conventions · fit: fits type 3
4. ops-security-plan.md · contract/channel-frames/hook-event-revalidation · input-validation · fit: fits type 3
5. ops-security-plan.md · decision/api-error-turn-end-text-dropped (rule) · data-protection · fit: fits type 1
6. ops-main-security-plan.md · decision/api-error-classes-adapter-only (rule) · data-protection · fit: fits type 1
7. ops-main-security-plan.2.md · decision/api-error-classes-adapter-only (state → standing) · data-protection · fit: —
8. ops-main-security-plan.2.md · decision/api-error-turn-end-text-dropped (why) · data-protection · fit: fits type 1
9. ops-main-security-plan.3.md · decision/api-error-turn-end-text-dropped (state → standing) · data-protection · fit: —
10. ops-design-system.md · decision/api-error-turn-end-shown-as-turn-end · surface-cli · fit: fits type 1
11. ops-test-plan.md · contract/event-data/turn-ended · integration-test-strategy · fit: fits type 3

Not on the card (not a widening, applied as routine): the `parse-rejected{hook-stdin, unknown-value}` notice —
obs-plan contract/parse-rejected-detail/unknown-value, contract/parse-rejected/hook-stdin-unknown-value,
decision/parse-failures-unknown-counter, decision/parse-failures-surface-as-unknown; architecture
decision/unparseable-external-values-unknown — a code and a count, no value crossing, an application of obs's standing
parse-failure rule (the overseer's technical answer, `inputs#I2`).

## The answer
Given in the session's question dialog, read at 2026-11-16T18:39:23Z — the option «Confirm (Recommended)», with the
note, verbatim:

> Confirmed by the delegate overseer, 2026-11-16T18:39:15Z, after reading answer-card.md's 11 operations against
> ruling A's text on the route carry: each lies inside it. The confirmation is the delegate's, not the founder's live
> word on this card; mark it as the wrap's rule requires for a delegate's confirm.

Reading: the ratification stays the founder's ruling A (his own answer to the P4 fork, relayed verbatim). The card's
confirm is the delegate overseer's, so it is PROVISIONAL: each line it resolves is written `delegate overseer,
2026-11-16`, and each owning sidecar entry names the confirm as the delegate's, to be superseded by the founder's own
word in the batch at the epoch boundary.
