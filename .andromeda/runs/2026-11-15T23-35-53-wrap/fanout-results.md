architecture: add 8 · set 6 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 3 · retire 0 · none 0 · ops-security-plan.md
design-system: add 1 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 1 · set 0 · retire 0 · none 1 · ops-layout-templates.md
test-plan: add 4 · set 4 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 4 · set 1 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 1 · set 0 · retire 0 · none 0 · ops-a11y-plan.md

ops-architecture.md
contract/event-data/turn-ended-last-assistant-message · escalate → Q2 — check 1: Boundary widening (rul-r74hshif, rul-p2zl2kgl): the turn-ended data gains `cause`, the F2 crossing; Expected amendment 1 · resolved: apply (Q2) — delegate the overseer, 2026-11-15: «cause: "api-error" on API-error ends only» — the P4 answer to F2, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
decision/capability-ledger-gate · escalate → Q1 — check 1: Boundary widening: StopFailure admitted on the hook surface with no ledger row (F1); Expected amendment 5 · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
decision/cli-capability-ledger · escalate → Q1 — check 1: Boundary widening (F1): the same named exception at a second statement of the ledger rule, stale without it · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
decision/capability-ledger · escalate → Q1 — check 1: Boundary widening (F1): the same named exception at a third statement of the ledger rule, stale without it · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/hook-tiers/ordering-spine · escalate → Q1 — check 1: Boundary widening (F1): records StopFailure in Stop's tier · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/hook-tiers/sync-spine · escalate → Q1 — check 1: Boundary widening (F1): records StopFailure in the sync spine · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/hook-to-kind-map/stopfailure · escalate → Q1 — check 1: Boundary widening (F1): the hook-to-kind row of the admitted event; Expected amendment 2 · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/turn-end-cause/cause-set · escalate → Q2 — check 1: Boundary widening (F2): the closed set bounding the crossing word; Expected amendment 2 (TurnEndCause's home) · resolved: apply (Q2) — delegate the overseer, 2026-11-15: «cause: "api-error" on API-error ends only» — the P4 answer to F2, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/event-data/turn-ended-cause · escalate → Q2 — check 1: Boundary widening (F2): the `cause` word crossing into events.ndjson, wait --json and MCP wait; Expected amendment 6 · resolved: apply (Q2) — delegate the overseer, 2026-11-15: «cause: "api-error" on API-error ends only» — the P4 answer to F2, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/hook-event/turn-ended-cause · reject — check 2: the fact's home is security-plan contract/channel-frames/hook-event-revalidation
decision/stop-failure-text-never-carried · reject — check 2: the fact's home is security-plan decision/api-error-turn-end-text-dropped
decision/api-error-class-in-agent-crate · reject — check 2: the fact's home is security-plan decision/api-error-classes-adapter-only
name/other/hook-stop-failure · escalate → Q1 — check 1: Boundary widening (F1): the admitted event's occupied hook name, beside the nine `name/other/hook-*` rows · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
decision/api-error-turn-end-clears-turn · escalate → Q1 — check 1: Boundary widening (F1): the behaviour the admitted event gives · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed

ops-security-plan.md
decision/hook-unknown-event-silent-exit · escalate → Q1 — check 1: Boundary widening (F1): the closed set of hook events nine → ten; Expected amendment 3 · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/channel-frames/hook-event-revalidation · escalate → Q2 — check 1: Boundary widening (F2): the wrapper re-validates the crossing word; Expected amendment 3 · resolved: apply (Q2) — delegate the overseer, 2026-11-15: «cause: "api-error" on API-error ends only» — the P4 answer to F2, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
decision/unverified-cli-gate · escalate → Q1 — check 1: Boundary widening (F1): the named exception at the unverified-CLI gate · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
decision/api-error-turn-end-text-dropped · escalate → Q2 — check 1: Boundary widening (F2): the crossing word and the dropped error text, security-plan §Data Protection; Expected amendment 6 · resolved: apply (Q2) — delegate the overseer, 2026-11-15: «cause: "api-error" on API-error ends only» — the P4 answer to F2, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
decision/api-error-classes-adapter-only · apply — check 1: Accurate this-chunk addition (rul-5hynfkq5): `ApiErrorClass` held inside viola-agent-claude, no class crossing (report Symbols / APIs), marked provisional with F2 open

ops-design-system.md
decision/api-error-turn-end-shown-as-turn-end · escalate → Q2 — check 1: Boundary widening (F2): the crossing word as the end's only trace on the cli surface · resolved: apply (Q2) — delegate the overseer, 2026-11-15: «cause: "api-error" on API-error ends only» — the P4 answer to F2, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed

ops-layout-templates.md
decision/api-error-turn-end-reads-turn-ended · reject — check 2: the fact's home is architecture decision/api-error-turn-end-clears-turn and security-plan decision/api-error-turn-end-text-dropped

ops-test-plan.md
contract/fake-script/format · escalate → Q1 — check 1: Boundary widening (F1): the fake agent's registered events nine → ten; Expected amendment 7 · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/event-data/turn-ended · escalate → Q2 — check 1: Boundary widening (F2): the turn-ended data keys 2 → 3 with `cause`; Expected amendment 1 · resolved: apply (Q2) — delegate the overseer, 2026-11-15: «cause: "api-error" on API-error ends only» — the P4 answer to F2, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/viola-send/turn-running · escalate → Q1 — check 1: Boundary widening (F1): the send accepted after the admitted event's turn end · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
decision/hook-payloads-recorded-by-verify · escalate → Q1 — check 1: Boundary widening (F1): StopFailure registered with no recorded payload and no verify probe · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/turn-end-cause/api-error · reject — check 2: the fact's home is architecture contract/turn-end-cause/cause-set
decision/api-error-text-crosses-into-nothing · reject — check 2: the fact's home is security-plan decision/api-error-turn-end-text-dropped
decision/send-cases-copy-recorded-fixtures · apply — check 1: Accurate this-chunk addition: the measured sweep hazard of the report's Deviations and Decisions & corrections, a test-data convention
decision/api-error-classes-held-in-adapter · reject — check 2: the fact's home is security-plan decision/api-error-classes-adapter-only

ops-obs-plan.md
contract/diag-line-fields/hook-events · escalate → Q1 — check 1: Boundary widening (F1): `hook_event` one of nine → ten; Expected amendment 4 · resolved: apply (Q1) — delegate the overseer, 2026-11-15: «no ledger row» — the P4 answer to F1, inputs#I2, taken by relay on inputs#I4 item 2; PROVISIONAL, the founder's own word owed
contract/turn-ended-event-data/cause · reject — check 2: the fact's home is architecture contract/event-data/turn-ended-cause
decision/api-error-text-never-recorded · reject — check 2: the fact's home is security-plan decision/api-error-turn-end-text-dropped
decision/api-error-class-held-in-agent-crate · reject — check 2: the fact's home is security-plan decision/api-error-classes-adapter-only
decision/turn-end-cause-revalidated · reject — check 2: the fact's home is security-plan contract/channel-frames/hook-event-revalidation

ops-a11y-plan.md
decision/api-error-turn-end-closed-cause · reject — check 2: the fact's home is security-plan decision/api-error-turn-end-text-dropped
