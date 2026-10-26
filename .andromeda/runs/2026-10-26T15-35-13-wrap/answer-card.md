# Answer card — 2026-10-26-explicit-session-links

## The answer it rests on
- inputs#I3 — the founder's own answer to this chunk's P4 fork (the widening's form), live by dialog in the overseer session
  at 2026-10-26T14:41Z, relayed verbatim by the operator: «Сначала instance check (Рекомендую)» — option 1 of three: the
  wrapper serves `link` / `unlink`; `viola link` / `viola unlink` run `viola_state::strict::check_instance` on the home
  and the driven instance before any log or snapshot read, then the liveness pre-check and the server check like `send`;
  a refused check exits 21 with no log and no line; no dated gap. Recorded in the plan's Metadata (`plan.md:8-11`) and
  the chunk's `inputs/andromeda-inputs.json` I3.

## The proposals lying wholly inside it (`fanout-results.md` numbering)
- fanout-architecture.md 1 — §Established Decisions [Links]: the explicit verbs built in the instance-check form
- fanout-architecture.md 2 — §Standard Contracts Channel methods: `link` / `unlink` served, their answers and `-32602` causes
- architecture cascade 3 — established-decisions:150: the hold retired, the founder's form ratifying the widening, no dated gap
- architecture cascade 4 — established-decisions [Links]: the verbs' order (instance check, liveness, server check)
- architecture cascade 18 — standard-contracts channel methods: `link` / `unlink` take no identity from `from` / `conn`
- fanout-security-plan.md 1 — Authentication & Authorization, IPC server verification row: `link` / `unlink` with no dated gap
- fanout-security-plan.md 2 (raised by main) — the Security Decisions Log entry mapping the served `link` / `unlink` to the IPC controls (in the security-plan sidecar entry)
- security cascade 1 — threat-model-summary:55: eight frame verbs, the two reading the snapshot after `check_instance`
- security cascade 2 — threat-model-summary: a same-user connector can add or remove a link (a record only)
- security cascade 3 — authentication-authorization RBAC row: nine served methods, `link` / `unlink` under inputs#I3
- security cascade 4 — authentication-authorization server-verification row: the no-gap record for the two verbs
- security cascade 5 — authentication-authorization:34: the strict-modes entry-point list gains `link` / `unlink`
- security cascade 6 — authentication-authorization CLI refusal row: eight frame verbs, the strict-modes refusal first
- security cascade 9 — input-validation channel frames: the wrapper's `link` / `unlink` params rule (three `-32602` causes)
- security cascade 13 — security-anti-patterns frame ban: `link` / `unlink` meet it in full, no dated exception
- (sampler) security S1 — bootstrap-phases:15: the owed strict-modes snapshot read names `link` / `unlink` as its exception

Check 6 groups no disproved claim with them.

## The answer
inputs#I6 — the overseer (founder-delegated), the P2 halt dialog, 2026-10-26T16:27Z, verbatim: "Confirm (Recommended)" —
"The overseer's technical answer (founder-delegated), 16:27Z: read answer-card.md; all 16 restate the as-built form his
inputs#I3 named (served methods, the instance-check-first order, no dated gap, the -32602 causes, the frame-ban and
strict-modes lists); none adds a crossing beyond it. Security cascade 2 (a same-user connector can add or remove a link, a
record only) restates the self-reported `from` already in the entry's hold text."

The confirm is a delegate's: PROVISIONAL (`delegate the overseer`), superseded by the founder's own recorded word in the
batch at the epoch boundary. The ratification stays his own answer, inputs#I3.
