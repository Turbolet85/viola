# Answer card — wrap 2026-10-26-home-and-code-bearing-file-integrity

## The founder's answers on record

- `inputs#I3` Q1 — «Перевести сейчас (Рекомендую)», option 1, Move them now: the readers (hook connect, hook statusline, run's collision check, `live_endpoint`) take `read_snapshot_or_replay`, so hook, mcp and CLI processes read `events.ndjson` on a new basis. The founder's own word, live by dialog in the overseer session, 2026-10-26T18:11Z, relayed by the overseer; his boundary-widening ratification (the relay's note).
- `inputs#I4` — «yes» on `.andromeda/runs/2026-10-26T16-59-13-phase/p5-card-1.md`, the whole card, 2026-10-26T18:59Z, the founder's own word relayed by the overseer.

## The escalated proposals lying wholly inside `inputs#I3` Q1 (the widening each records is a reader's move onto the replay)

- `fanout-architecture.md 6` · §Standard Contracts (Snapshot envelope) — Append: the replay's product readers are `viola revive`, `viola list`, `liveness::live_endpoint` (the CLI frame verbs and `viola mcp`'s frame tools), 
- `fanout-architecture.md 7` · §Infrastructure Patterns → Project directory structure — viola-state's line «the classified snapshot read, the log replay and the session chain (`replay.rs`; read by `viola revive` and `board::read`)» names 
- `fanout-architecture--cascade-g1.md 2` · .andromeda/architecture-detail/established-decisions.md:79 — - A verified client meets an unreadable or newer snapshot through `replay::read_snapshot_or_replay` (§Standard Contracts, Snapshot envelope), a replay
- `fanout-architecture--cascade-g1.md 3` · .andromeda/architecture-detail/established-decisions.md:79 — - On the replay, `viola_state::liveness::live_endpoint` proceeds only on a real snapshot (a replayed one carries no `endpoint`, `pid` or `started_at`,
- `fanout-architecture--cascade-g1.md 5` · .andromeda/architecture-detail/established-decisions.md:110 — - The event and dialog paths, once their instance check has passed ([Hook Contract]), share one connect: `replay::read_snapshot_or_replay` (a replayed
- `fanout-architecture--cascade-g1.md 8` · .andromeda/architecture-detail/established-decisions.md:203 — After the pid + start time sentence add: `run`'s `collision_check` takes the caller's one read (`Option<&Recovered>`); over a replayed snapshot (no `p
- `fanout-architecture--cascade-g1.md 9` · .andromeda/architecture-detail/established-decisions.md:203 — - `viola revive`'s preflight reads the snapshot once and hands that read to both its collision check and its recorded-directory reading: one replay, o
- `fanout-architecture--cascade-g1.md 11` · .andromeda/architecture-detail/established-decisions.md:207 — - The snapshot's `pid` and `started_at` are also the server identity: `viola_state::liveness::live_endpoint` returns them with the endpoint from one `
- `fanout-architecture--cascade-g1.md 23` · .andromeda/architecture-detail/standard-contracts.md:24 — The fallback's product readers go from two to six, each after a strict-modes check has passed: beside `viola revive`'s preflight (one read handed to i
- `fanout-architecture--cascade-g1.md 24` · .andromeda/architecture-detail/standard-contracts.md:25 — «`read_snapshot` keeps its results, and its three product callers (…) still read every such snapshot as absent» becomes: `read_snapshot` keeps its res
- `fanout-architecture--cascade-g1.md 26` · .andromeda/architecture-detail/standard-contracts.md:51 — Add: before that comparison each client first runs the strict-modes instance check (`client::refuse_untrusted` for the six frame verbs and `link` / `u
- `fanout-architecture--cascade-g1.md 29` · .andromeda/architecture-detail/standard-contracts.md:101 — In the arm's order «reads the snapshot» becomes «reads the snapshot through `replay::read_snapshot_or_replay` (a replayed snapshot carries no `statusl
- `fanout-architecture--cascade-g1.md 30` · .andromeda/architecture-detail/standard-contracts.md:109 — «all from its one `read_snapshot` call, so a frame-writing client takes the liveness-only pre-check first and the server check after it» becomes «all 
- `fanout-architecture--cascade-g2.md 1` · .andromeda/architecture-detail/occupied-resources.md:8 — Replace «It rebuilds the path by the same rule only when the snapshot is missing or unreadable.» with: Each reads the snapshot through the replay (`re
- `fanout-architecture--cascade-g2.md 7` · .andromeda/architecture-detail/occupied-resources.md:105 — Add: a snapshot rebuilt from the log writes one `state-recovered` line from each reader moved onto the replay at chunk 2026-10-26-home-and-code-bearin
- `fanout-security-plan.md 4` · Input Validation — Own state files on read row: drop «except the ratified dated gaps»; every own state file is read only after the home strict-modes check, no dated gap 
- `fanout-security-plan--cascade-g1.md 1` · .andromeda/security-plan-detail/threat-model-summary.md:56 — In the as-built consequence, replace «The snapshot is read without the strict-modes check (owed to «Home and code-bearing file integrity»), except by 
- `fanout-security-plan--cascade-g1.md 2` · .andromeda/security-plan-detail/threat-model-summary.md:87 — Add after the `hook statusline` trust line: every hook path (event, dialog and `hook statusline`) runs the instance's strict-modes check on the canoni
- `fanout-security-plan--cascade-g1.md 5` · .andromeda/security-plan-detail/threat-model-summary.md:124 — Replace «has two product readers, `viola revive`'s preflight and `viola list` (…), each taking it after the strict-modes check on the instance's files
- `fanout-security-plan--cascade-g1.md 10` · .andromeda/security-plan-detail/authentication-authorization.md:27 — In **Collision check**, replace «reads the snapshot through `read_snapshot`» with: takes the caller's one read through `read_snapshot_or_replay` (`run
- `fanout-security-plan--cascade-g1.md 11` · .andromeda/security-plan-detail/authentication-authorization.md:28 — Extend the stack reference: `viola-state` `strict::check_instance` (called by the root bin's `cmd::client::refuse_untrusted` and `cmd::hook`'s `truste
- `fanout-security-plan--cascade-g1.md 29` · .andromeda/security-plan-detail/authentication-authorization.md:63 — Replace «taken with the `endpoint` from one `read_snapshot` call» with: taken with the `endpoint` from one `read_snapshot_or_replay` call made after t
- `fanout-security-plan--cascade-g2.md 8` · .andromeda/security-plan-detail/input-validation.md:53 — Replace «They have two product callers» (revive's preflight and list's board::read) with six: those two and, since chunk 2026-10-26-home-and-code-bear
- `fanout-security-plan--cascade-g2.md 9` · .andromeda/security-plan-detail/input-validation.md:54 — Replace the sentence saying «the three other snapshot readers» and `live_endpoint` stay on `read_snapshot`, with their move a CARRY owed to «Home and 
- `fanout-security-plan--cascade-g2.md 13` · .andromeda/security-plan-detail/input-validation.md:77 — After «the snapshot read» in the arm's order, add: that read is `read_snapshot_or_replay`, and a replayed snapshot carries no `statusline_command`, so
- `cascade-compare-security-plan.md S2` · .andromeda/security-plan-detail/error-handling.md:9 — New item after line 9: «- A `viola hook` event or dialog whose instance snapshot is replayed or absent at `connect` (`read_snapshot_or_replay`: only a
- `cascade-compare-security-plan.md S3` · .andromeda/security-plan-detail/error-handling.md:9 — New item after line 9: «- `viola hook statusline` (without `--capture`) over a replayed snapshot finds no `statusline_command`, so nothing runs and no
- `fanout-test-plan.md 3` · §6 E2E Test Strategy → Scenario: E5 — snapshot corruption → replay recovery (As landed) — **As landed: built.** Steps 1–3 run as `tests/chaos_snapshot_replay.rs` (`a_start_over_an_unreadable_snapshot_replays_the_log`, `a_start_over_a_replay
- `fanout-test-plan.md 4` · §6 E2E Test Strategy → Scenario: E5 — snapshot corruption → replay recovery (Verification signal, third bullet) — The «No refusal of start (exit is not 1)» signal is qualified: it holds once the heartbeat is older than `STALE_AFTER`; over a replayed snapshot the c
- `fanout-test-plan.md 16` · §4 Unit Test Strategy → What unit tests cover: root bin — The root bin bullet adds: the readers over a replayed snapshot (the collision check reading the beat alone, hook `connect` unreachable, statusline `se
- `fanout-test-plan--cascade-g1.md 5` · .andromeda/test-plan-detail/test-scope-summary.md:159 — Append to the statusline signal: «Over a replayed snapshot (unreadable or of an unsupported `v`) the arm finds no `statusline_command`, so it runs and
- `fanout-test-plan--cascade-g1.md 6` · .andromeda/test-plan-detail/test-scope-summary.md:173 — Replace «and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive)» with «`viola revive` is its first product reader (chunk 2026-
- `fanout-test-plan--cascade-g1.md 8` · .andromeda/test-plan-detail/unit-test-strategy.md:19 — Replace the sentences from «The three other snapshot readers» to the end of the line with: «Since chunk 2026-10-26-home-and-code-bearing-file-integrit
- `fanout-test-plan--cascade-g1.md 9` · .andromeda/test-plan-detail/unit-test-strategy.md:19 — Add after the snapshot-envelope item: «The replayed readers'' unit cases (chunk 2026-10-26-home-and-code-bearing-file-integrity): `live_endpoint_of_a_
- `fanout-test-plan--cascade-g1.md 10` · .andromeda/test-plan-detail/unit-test-strategy.md:22 — Replace «`live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call.» with «`live_endpoint` returns `
- `fanout-test-plan--cascade-g2.md 2` · .andromeda/test-plan-detail/e2e-test-strategy.md:174 — Extend the Step 6 bullet: `statusline_command` never runs when strict-modes fails; and since chunk 2026-10-26-home-and-code-bearing-file-integrity a s
- `fanout-test-plan--cascade-g2.md 3` · .andromeda/test-plan-detail/e2e-test-strategy.md:236 — Head `As landed (…): built` (was `landed in part`, the chunk 2026-10-26-home-and-code-bearing-file-integrity added); replace `The replay has two produ
- `fanout-test-plan--cascade-g2.md 4` · .andromeda/test-plan-detail/e2e-test-strategy.md:237 — Replace `Steps 1–3 stay unbuilt while run's collision check calls read_snapshot; the three other snapshot readers (…) stay on read_snapshot, and their
- `fanout-test-plan--cascade-g2.md 19` · .andromeda/test-plan-detail/e2e-test-strategy.md:338 — Add a bullet: snapshot replay (`tests/chaos_snapshot_replay.rs`, chunk 2026-10-26-home-and-code-bearing-file-integrity): the built E5 steps 1–3 and th
- `fanout-obs-plan.md 11` · §4 Span / Trace Coverage → Edge flows (E5) — E5: `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}` at WARN, written by `viola_state::replay::read_snapshot_or_replay`, one lin
- `fanout-obs-plan--cascade-g1.md 2` · .andromeda/obs-plan-detail/obs-scope-summary.md:133 — Add a sub-bullet after the snapshot.json scrub rule: «  - A `snapshot.json` replayed from `events.ndjson` (`read_snapshot_or_replay`) carries no `endp
- `fanout-obs-plan--cascade-g2.md 2` · .andromeda/obs-plan-detail/obs-scope-summary.md:299 — In the must-trace chain insert `state.snapshot_recover` under `run.start` before `run.collision_check` (the one snapshot read the collision check take
- `fanout-obs-plan--cascade-g3.md 8` · .andromeda/obs-plan-detail/span-trace-coverage.md:30 — Replace «(`read_snapshot`, then `connect_by`, then `verify_server`)» with «(`read_snapshot_or_replay`, then `connect_by`, then `verify_server`)» and a
- `fanout-obs-plan--cascade-g3.md 9` · .andromeda/obs-plan-detail/span-trace-coverage.md:39 — Add after the must-trace chain: «The start one snapshot read (`read_snapshot_or_replay`, handed to `run.collision_check`) puts `state.snapshot_recover
- `fanout-obs-plan--cascade-g3.md 10` · .andromeda/obs-plan-detail/span-trace-coverage.md:42 — Extend the `run.collision_check` attribute line: «; over a replayed snapshot (no pid or start time) liveness is judged from the heartbeat alone: `beat
- `fanout-obs-plan--cascade-g3.md 13` · .andromeda/obs-plan-detail/span-trace-coverage.md:54 — Add to the revive sentence: «the preflight makes one snapshot read and hands it to both its collision check and its recorded-directory reading: one re
- `fanout-obs-plan--cascade-g3.md 16` · .andromeda/obs-plan-detail/span-trace-coverage.md:69 — Extend the server-check sub-bullet: «Before it, the instance check refuses a widened instance with no frame and no line: CLI `send` runs `client::refu
- `fanout-obs-plan--cascade-g3.md 24` · .andromeda/obs-plan-detail/span-trace-coverage.md:191 — After the second-caller sentence add: «Since chunk 2026-10-26-home-and-code-bearing-file-integrity four more product readers take `read_snapshot_or_re
- `fanout-obs-plan--cascade-g3.md 25` · .andromeda/obs-plan-detail/span-trace-coverage.md:192 — Replace the first sentence («The three other snapshot readers … still call `read_snapshot` and write no `state-recovered` line.») with «No product rea
- `fanout-obs-plan--cascade-g4.md 5` · .andromeda/obs-plan-detail/log-coverage.md:32 — After the `server-verify-failed` sentence add: `channel-unreachable` is also hook `connect`''s code over a snapshot `read_snapshot_or_replay` returns 
- `cascade-compare-obs-plan.md S3` · .andromeda/obs-plan-detail/obs-scope-summary.md:326 — Line 326's must-trace spans read `hook.handle` › `state.snapshot_recover` › `channel.request(hook.dialog)` › …: hook `connect` reads `read_snapshot_or

## Not on the card — the halt's other questions

Q2 (check-7 contradictions, each against a standing line this same pass rewrites):
- `fanout-architecture--cascade-g1.md 3 (also on the card, Q1)`
- `fanout-architecture--cascade-g2.md 13`
- `fanout-security-plan--cascade-g1.md 12`
- `fanout-security-plan--cascade-g1.md 13`
- `fanout-security-plan--cascade-g1.md 17`
- `fanout-security-plan--cascade-g1.md 18`
- `fanout-security-plan--cascade-g1.md 24`
- `fanout-security-plan--cascade-g1.md 26`
- `fanout-security-plan--cascade-g1.md 32`
- `fanout-security-plan--cascade-g2.md 9 (also on the card, Q1)`
- `fanout-security-plan--cascade-g2.md 11`
- `fanout-security-plan--cascade-g2.md 14`
- `fanout-design-system--cascade.md 1`
- `fanout-test-plan--cascade-g1.md 26`
- `fanout-test-plan--cascade-g2.md 16`
- `fanout-test-plan--cascade-g2.md 19 (also on the card, Q1)`
- `fanout-obs-plan--cascade-g1.md 6`
- `fanout-obs-plan--cascade-g3.md 9 (also on the card, Q1)`
- `fanout-obs-plan--cascade-g3.md 13 (also on the card, Q1)`
- `fanout-obs-plan--cascade-g4.md 2`
- `cascade-compare-obs-plan.md S8`

Q3 (an owner for `viola list`'s home-check inconsistency, report Count 14):
- `fanout-architecture--cascade-g2.md 12`
- `fanout-security-plan--cascade-g1.md 15`
- `fanout-design-system--cascade.md 6`

## Sampler lines (the operator's forced draw, `inputs#I5`)

- architecture: E carried 42 · the cascade lost 3 · beside a placed fact 2
- security-plan: E carried 83 · the cascade lost 5
- design-system: E carried 10 · the cascade lost 0
- test-plan: E carried 47 · the cascade lost 4 · beside a placed fact 1
- obs-plan: E carried 48 · the cascade lost 6 · beside a placed fact 2
- a11y-plan: E carried 6 · the cascade lost 2 · beside a placed fact 2

## The answer as given

Answered in the dialog at 22:43Z by the overseer, founder-delegated, as given verbatim below.

## Q1 — the answer card
Answer: «Confirm all 51 (Recommended)»
Notes: «The overseer's technical answer (founder-delegated; he is asleep, his word «ты за главного» to overseer1 and «продолжают работать автономно»), 22:43Z: I read answer-card.md; the escalated proposals each record a reader's move onto the replay, after the instance check, which his inputs#I3 Q1 named; none adds a crossing beyond it. Written PROVISIONAL as the skill requires; the ratification stays his I3.»

## Q2 — the check-7 contradictions
Answer: «Apply with companions (Recommended)»
Notes: «The overseer's technical answer (founder-delegated; he is asleep, his word «ты за главного» to overseer1 and «продолжают работать автономно»), 22:43Z: the same shape as the last two wraps (each contradicts a line this chunk retires, its companion rewrites it in the same apply).»

## Q3 — the owner of `viola list`'s home-check inconsistency
Answer: «CARRY on Unix hardening (Recommended)»
Notes: «The overseer's technical answer (founder-delegated; he is asleep, his word «ты за главного» to overseer1 and «продолжают работать автономно»), 22:43Z: the home's checks belong to «Unix endpoint and home hardening», which the three proposals already name.»

Snapped: `inputs#I6`.
