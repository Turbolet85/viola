# Fan-out results — 2026-10-26-server-verification-before-any-frame

Report: `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md`. Sampled run, FORCED by the operator
(`cascade-sample.txt`: `sample: yes · wrap · … · draw 2 of 10 · FORCED by the operator`; inputs#I7 item 1, inputs#I8).
Citation sweep: 0 rows owed a read, 0 re-pointed (`citation-dispositions.md`).

## Verdicts
- architecture: 6 proposals · fanout-architecture.md · 18 cascade proposals · fanout-architecture--cascade.md · fanout-architecture--cascade-new-standard-contracts.md · 9 facts · 9 placed · 0 no home · (sampler) 2 lost
- security-plan: 3 proposals · fanout-security-plan.md · 25 cascade proposals · fanout-security-plan--cascade.md · fanout-security-plan--cascade-new-error-handling.md · 8 facts · 8 placed · 0 no home · (sampler) 0 lost
- design-system: 0 proposals · fanout-design-system.md · 4 cascade proposals · fanout-design-system--cascade.md · fanout-design-system--cascade-new-surface.md (0) · 4 facts · 3 placed · 1 no home · (sampler) 0 lost
- layout-templates: 1 proposal · fanout-layout-templates.md (whole master, no cascade)
- test-plan: 4 proposals · fanout-test-plan.md · 35 cascade proposals · fanout-test-plan--cascade-g1.md · fanout-test-plan--cascade-g2.md · 13 facts · 13 placed · 0 no home · (sampler) 4 lost
- obs-plan: 2 proposals · fanout-obs-plan.md · 26 cascade proposals · fanout-obs-plan--cascade-g1.md · fanout-obs-plan--cascade-g2.md · 8 facts · 8 placed · 0 no home · (sampler) 4 lost
- a11y-plan: 0 proposals · fanout-a11y-plan.md · 6 cascade proposals · fanout-a11y-plan--cascade.md · fanout-a11y-plan--cascade-new-a11y-scope-summary.md · 4 facts · 4 placed · 0 no home · (sampler) 0 lost

Notes carried from the files (no fix removed anything; no twin made):
- architecture: the crate-dependency-direction key's viola-channel line omits `server_pid` / `SERVER_PID_REPORTED` (registry over-reach, not raised); `crates/viola-mcp/tests/handlers.rs`'s `viola-test-mcp-` endpoints unnamed in the Test-only IPC row (pre-existing, not this chunk's).
- security-plan: the Linux client euid owner checked against the working-route line of "Unix endpoint and home hardening" — its title «peer euid both sides» and its CARRY «the directory verification … and the peer euid check stay this entry's»: the Unix (Linux included) client euid check is that entry's, so proposals stating it owed there stand.
- obs-plan: proposal 2's MCP no-line qualifier flagged by its agent as a judgement call.
- design-system: F4 (hook and MCP write no human-facing byte) has no home in surface.md; design-system.md:353 Platform-Specific Notes stays true.

## Dispositions

fanout-architecture.md
1. reject — opening rule: the new binders' endpoint name form (`viola-test-chan-<pid>-<label>`) and their place outside the `viola-<h12>` namespace are not in the report (its rows name `served` / `Server::bind` / `Impostor::serve`, never the name); counted, enters through a wrap whose report carries it
2. reject — opening rule: the variable name `MCP_HANDLERS_TEST_HELD` is not in the report (the new-text row shows only `std::env::var_os(HELD)`); counted, as 1
3. apply — check 1: Accurate this-chunk addition (Symbols: `verify_server` compares the snapshot's `pid` / `started_at`; every frame-writing client)
4. apply — check 1: D-retired, the report's Retired bullet names the liveness-only pre-check's server-identity half (key file project-directory-structure.md:67)
5. apply — check 1: D-retired dependent of 4 (key file crate-dependency-direction.md:5); no dependency added
6. apply — check 1: Accurate this-chunk addition (`cmd::client::connect`, `Unreachable`, `CallError`)

fanout-security-plan.md
1. apply — check 1 + check 5: expected amendment (security-plan.md:114); the client euid owner per the route line of "Unix endpoint and home hardening"
2. apply — check 1 + check 5: expected amendment (security-plan.md:439), dependent of 1
3. apply — check 1: Accurate this-chunk addition (security-plan.md:100; `peer_creds().pid()` is the method the build calls)

fanout-design-system.md
(no proposal)

fanout-layout-templates.md
1. apply — check 5: expected amendment (layout-templates §Component — Primary content block 2, the server-verification hint built)

fanout-test-plan.md
1. apply — check 6: disproved claim (test-plan.md:590, the reader move's owner re-pinned, fork 1 PROVISIONAL) · as: fork 1 is the founder's own ruling, «Вариант 1 — утвердить (Рекомендую)», given by dialog at his wake 2026-10-26 (inputs#I10, relayed by the overseer, inputs#I11): option 1, every reader kept on `read_snapshot`; the readers' move onto `read_snapshot_or_replay` (v1-41, E5 steps 1–3) stays re-pinned as a CARRY on "Home and code-bearing file integrity" — write it as his ruling, never PROVISIONAL or a delegate's answer
2. apply — check 6: disproved claim (test-plan.md:604, the case's home; `cross_exit_causes.rs` absent)
3. apply — check 1: Counts (matrix11, test-plan.md:598)
4. apply — check 1: Accurate this-chunk addition (test-plan.md:606, the new security negative)
5. apply — main raised at the 2a resume (inputs#I17, the overseer's founder-delegated answer): §7 Test Data & Fixtures, test-plan.md:670 — the fake agent's exit ignores SIGHUP from its first step (the folded fix `7948850a`, the report's addendum, inputs#I14); written by main (amendment-flow §Apply, a text no writer wrote), its sweep and leaves in cascade-dispositions.md §The resume pass

fanout-obs-plan.md
1. apply — check 6 + check 5: disproved claim (obs-plan.md:361), expected amendment
2. apply — check 1: Accurate this-chunk addition (§7: the mismatch is an MCP reject with no line, as the report's Schema / config bullet states)

fanout-architecture--cascade.md
1. apply — check 1: Accurate this-chunk addition (stack Wrapper IPC row)
2. apply — check 1: Accurate this-chunk addition (stack Process liveness row)
3. apply — check 7: architecture-newgroup-established-decisions.md · distinct
4. apply — check 7: architecture-newgroup-established-decisions.md · distinct
5. apply — check 7: architecture-newgroup-established-decisions.md · distinct; fork 1 recorded PROVISIONAL (no playbook rule matches: no plan step held a landing; option 1 changes nothing) · as: fork 1 is the founder's own ruling, «Вариант 1 — утвердить (Рекомендую)», given by dialog at his wake 2026-10-26 (inputs#I10, relayed by the overseer, inputs#I11): option 1, every reader kept on `read_snapshot`; the readers' move onto `read_snapshot_or_replay` (v1-41, E5 steps 1–3) stays re-pinned as a CARRY on "Home and code-bearing file integrity" — write it as his ruling, never PROVISIONAL or a delegate's answer
6. apply — check 7: architecture-newgroup-established-decisions.md · distinct
7. apply — check 1: Accurate this-chunk addition (SessionEnd direct append, Outcome (arch) MET)
8. apply — check 1: Accurate this-chunk addition (dialog hook fail-open on a failed check)
9. reject — check 1: the held link verbs are not built, so the crossing the line lists is the unbuilt verbs' and unchanged; the report's site sweep dispositions established-decisions.md:125 no-change
10. apply — check 1: Accurate this-chunk addition (Session Liveness: `LiveEndpoint`, `verify_server`)
11. apply — check 7: architecture-newgroup-conventions.md · distinct
12. apply — check 7: architecture-newgroup-conventions.md · distinct
13. apply — check 6: disproved claim + Counts (readers, standard-contracts.md:25)
14. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
15. apply — check 1: Accurate this-chunk addition (`hook.event` sent once the check passed)
16. apply — check 7: architecture-newgroup-occupied-resources.md · distinct; check 5 expected amendment (Occupied Resources, `cli-<name>.ndjson`)
17. apply — check 5: expected amendment (Occupied Resources, `hook-<name>.ndjson`)

fanout-architecture--cascade-new-standard-contracts.md
1. apply — check 7: architecture-newgroup-standard-contracts.md · distinct

(sampler) cascade-compare-architecture.md
S1. apply (sampler) — check 7: architecture-newgroup-occupied-resources.md · distinct
S2. reject (sampler) — check 7: architecture-newgroup-standard-contracts.md · twin · fanout-architecture--cascade.md 14

fanout-security-plan--cascade.md
1. apply — check 1: Verbatim upstream copy kept current (Threat Model Summary)
2. apply — check 1: Accurate this-chunk addition (the Windows pid read as built; the Unix client euid owner per the route line)
3. apply — check 1: D-retired (`hook.event` gap, server-identity half)
4. apply — check 1: D-retired (CLI `send` gap)
5. apply — check 1: D-retired (`wait` / `last` gap)
6. apply — check 1: D-retired (`answer` + `hook.dialog` gap)
7. apply — check 1: D-retired (`pause` / `release` gap)
8. apply — check 1: D-retired (`viola mcp` gap)
9. apply — check 1: Accurate this-chunk addition (On mismatch, as built)
10. escalate → Q2 — check 6 + check 1: the disproved «Collision check: `run` uses the same verification» — the correction leaves a stated requirement with no owner (Sequencing deferral's caution: a gap with no resolving chunk escalates) · resolved: apply (Q2) · as: replace the clause with «**Collision check:** `run`'s collision check (`src/cmd/run.rs:379`) reads the snapshot through `read_snapshot` and judges the recorded wrapper by its `pid` + OS start time and the heartbeat; it opens no connection and writes no frame, so there is no serving process to verify, and the peer-pid server check does not apply to it (retired as moot, chunk 2026-10-26-server-verification-before-any-frame)» — the overseer's founder-delegated answer, 2026-10-26 10:21Z (inputs#I9), consistent with the founder's own ruling of fork CARRY 7/8 that keeps the collision check unchanged (inputs#I10)
11. apply — check 7: security-plan-newgroup-authentication-authorization.md · distinct
12. apply — check 7: security-plan-newgroup-authentication-authorization.md · distinct
13. escalate → Q1 — check 7: security-plan-newgroup-authentication-authorization.md · contradiction · lines 25–27 · resolved: apply (Q1)
14. apply — check 7: security-plan-newgroup-authentication-authorization.md · distinct
15. apply — check 7: security-plan-newgroup-authentication-authorization.md · distinct
16. apply — check 1: Accurate this-chunk addition (stack reference)
17. apply — check 1: D-retired (input-validation.md:30)
18. apply — check 6: disproved claim + Counts (readers, input-validation.md:52)
19. apply — check 7: security-plan-newgroup-input-validation.md · distinct; fork 1 recorded · as: fork 1 is the founder's own ruling, «Вариант 1 — утвердить (Рекомендую)», given by dialog at his wake 2026-10-26 (inputs#I10, relayed by the overseer, inputs#I11): option 1, every reader kept on `read_snapshot`; the readers' move onto `read_snapshot_or_replay` (v1-41, E5 steps 1–3) stays re-pinned as a CARRY on "Home and code-bearing file integrity" — write it as his ruling, never PROVISIONAL or a delegate's answer
20. apply — check 1: D-retired (bootstrap-phases.md:15)
21. apply — check 1 + check 5: D-retired, expected amendment (error-handling.md:8, `hookjoin`)
22. escalate → Q1 — check 7: security-plan-newgroup-error-handling.md · contradiction · lines 8–8 · resolved: apply (Q1)
23. apply — check 1: Accurate this-chunk addition (security-anti-patterns.md:8, the Windows pid named as built)
24. apply — check 1: D-retired (security-anti-patterns.md:8)

fanout-security-plan--cascade-new-error-handling.md
1. apply — check 7: security-plan-newgroup-error-handling.md · distinct

fanout-design-system--cascade.md
1. apply — check 7: design-system-newgroup-surface.md · distinct
2. apply — check 7: design-system-newgroup-surface.md · distinct
3. apply — check 7: design-system-newgroup-surface.md · distinct
4. apply — check 1: Accurate this-chunk addition (surface.md:73, written by this build)

fanout-design-system--cascade-new-surface.md
(no proposal; F4 no home)

fanout-test-plan--cascade-g1.md
1. apply — check 1: Accurate this-chunk addition (the captured-span cases compile out on macOS) · as: name the cases compiled out (the six verbs' `request_refuses_an_unverified_server_inside_its_dispatch_span`, `hook_refuses_an_unverified_server_inside_hook_handle`, the `refused_inside` helper, the `last.rs` / `pause.rs` test modules), not «the server-verification unit cases» — the decision table and the macOS `server_pid` case run on macOS
2. apply — check 1: dependent of 1 (test-plan.md:285) · as: the same narrowing as 1
3. apply — check 1: Accurate this-chunk addition (the root sync-suite prefixes gain `security_negatives`)
4. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct
5. apply — check 1: dependent of 4 (test-plan.md:376)
6. apply — check 6: disproved claim + Counts (readers, unit-test-strategy.md:18)
7. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct · as: drop «(seven rows)» — a master states no test-case count (Tier 1); name the function only
8. apply — check 1: dependent of 7 (test-plan.md:378)
9. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct
10. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct
11. apply — check 1: dependent of 9 (test-plan.md:386)
12. apply — check 6: disproved claim (integration-test-strategy.md:47)
13. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
14. apply — check 6: disproved claim + Counts (readers, owner; e2e-test-strategy.md:235) · as: fork 1 is the founder's own ruling, «Вариант 1 — утвердить (Рекомендую)», given by dialog at his wake 2026-10-26 (inputs#I10, relayed by the overseer, inputs#I11): option 1, every reader kept on `read_snapshot`; the readers' move onto `read_snapshot_or_replay` (v1-41, E5 steps 1–3) stays re-pinned as a CARRY on "Home and code-bearing file integrity" — write it as his ruling, never PROVISIONAL or a delegate's answer
15. apply-merged → fanout-test-plan.md 1 — the same line test-plan.md:590
16. apply — check 1: Counts (matrix11, the count dropped, the functions named)
17. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
18. apply — check 1: Counts (matrix11, e2e-test-strategy.md:244)
19. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
20. apply-merged → fanout-test-plan.md 3 — the same line test-plan.md:598
21. apply — check 6: disproved claim (e2e-test-strategy.md:261)
22. apply — check 6: disproved claim (e2e-test-strategy.md:264)
23. escalate → Q1 — check 7: test-plan-newgroup-e2e-test-strategy.md · contradiction · lines 264–264 · resolved: apply (Q1)
24. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
25. apply-merged → fanout-test-plan.md 2 — the same line test-plan.md:604
26. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
27. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
28. apply-merged → fanout-test-plan.md 4 — the same line test-plan.md:606

fanout-test-plan--cascade-g2.md
1. apply — check 1: Accurate this-chunk addition (the impostors' test-built snapshot, a third named exception)
2. apply — check 1: dependent of 1 (test-plan.md:631)
3. apply — check 1: Accurate this-chunk addition (the holder mode named, report Deviation 4)
4. apply — check 1: Accurate this-chunk addition (Process mocking row: the held recorded-pid process)
5. apply — check 7: test-plan-newgroup-ci-integration.md · distinct
6. apply — check 1: dependent of 5 (test-plan.md:741)
7. apply — check 7: test-plan-newgroup-quality-gates-coverage-targets.md · distinct

(sampler) cascade-compare-test-plan.md
S1. apply (sampler) — check 7: test-plan-newgroup-mocking-stubbing-discipline.md · distinct
S2. apply (sampler) — check 7: test-plan-newgroup-test-data-fixtures.md · distinct; check 3: the scope record's in-intent line (inputs#I5)
S3. apply (sampler) — check 7: test-plan-newgroup-test-scope-summary.md · distinct
S4. apply (sampler) — check 7: test-plan-newgroup-test-strategy.md · distinct

fanout-obs-plan--cascade-g1.md
1. apply — check 1: Verbatim upstream copy kept current (obs-scope-summary.md:123)
2. reject — check 1: Not this chunk's drift — a dialog hook already logs `corr` null on every fail-open path before a `dialog_id` (span-trace-coverage's D-07 rule); the mirror binds test-plan §3 Log format, which no proposal moves
3. apply — check 1: Accurate this-chunk addition (Dialog → answer fields)
4. apply — check 7: obs-plan-newgroup-obs-scope-summary.md · distinct
5. apply — check 1: Accurate this-chunk addition (exit-21 trigger, as landed)
6. apply — check 1: Accurate this-chunk addition (Vector 4 row)

fanout-obs-plan--cascade-g2.md
1. escalate → Q1 — check 7: obs-plan-newgroup-span-trace-coverage.md · contradiction · lines 90–90 · resolved: apply (Q1)
2. apply — check 7: obs-plan-newgroup-span-trace-coverage.md · distinct
3. apply — check 7: obs-plan-newgroup-span-trace-coverage.md · distinct
4. apply — check 7: obs-plan-newgroup-span-trace-coverage.md · distinct
5. apply — check 1: dependent of 1 (Confirmed send must-trace)
6. apply — check 1: Accurate this-chunk addition (Confirmed send log fields)
7. apply — check 1: Accurate this-chunk addition (Confirmed send cleanup)
8. apply — check 1: dependent of 1 (wait / last must-trace)
9. apply — check 6 + check 5: disproved claim (span-trace-coverage.md:90)
10. apply-merged → fanout-obs-plan.md 1 — the same line obs-plan.md:361
11. apply — check 1: Accurate this-chunk addition (wait / last cleanup)
12. apply — check 1: Accurate this-chunk addition (Dialog → answer must-trace)
13. apply — check 1: Accurate this-chunk addition (D-07 fail-open named case)
14. apply — check 1: Accurate this-chunk addition (driver-side answer refusal)
15. apply — check 1: Accurate this-chunk addition (wheel must-trace)
16. apply — check 1: Accurate this-chunk addition (wheel log fields)
17. apply — check 6: disproved claim + Counts (readers, span-trace-coverage.md:167)
18. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct
19. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct
20. apply — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct

(sampler) cascade-compare-obs-plan.md
S1. apply (sampler) — check 7: obs-plan-newgroup-metric-coverage.md · distinct
S2. apply (sampler) — check 7: obs-plan-newgroup-obs-scope-summary.md · distinct
S3. apply (sampler) — check 7: obs-plan-newgroup-slo-invariants-telemetry-budgets.md · distinct
S4. reject (sampler) — check 7: obs-plan-newgroup-span-trace-coverage.md · contradiction, and its CLI, hook and MCP halves twin fanout-obs-plan--cascade-g2.md 1, 3 and 4; its fact is theirs

fanout-a11y-plan--cascade.md
1. apply — check 1: Verbatim upstream copy kept current (a11y-scope-summary.md:8)
2. apply — check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct
3. apply — check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct
4. apply — check 7: a11y-plan-newgroup-cognitive-accessibility.md · distinct
5. apply — check 1: Accurate this-chunk addition (the hint pinned by literal, not trycmd)

fanout-a11y-plan--cascade-new-a11y-scope-summary.md
1. apply — check 7: a11y-plan-newgroup-a11y-scope-summary.md · distinct

## Checks 3, 5, 6 (whole-set)
- check 3 intent: the scope record's two lines hold their `serves` (companion `lib.rs` → step 1; in-intent `tests/contract_fake_agent_drift.rs` → the pre-push gate's ETXTBSY red, the delegate's word inputs#I5); no widening line.
- check 5 expected amendments: every plan entry has proposals — security-plan (fanout-security-plan.md 1–3, cascade 3–8, 17, 20, 21, 23, 24), the readers list (architecture cascade 13, security cascade 18, test g1 6 / 14, obs g2 17; key files architecture 4 / 5), architecture Occupied Resources (cascade 16, 17), obs-plan §4 / §6 (fanout-obs-plan.md 1, g2 9, 18, 19), test-plan §5 / §6 (fanout-test-plan.md 2–4, g1 12, 21–27), design-system / layout-templates (design cascade 1–4, layout 1). No ledger-note entry.
- check 6 disproved claims: obs-plan §4 (obs 1, g2 9) · readers (arch c13, sec c18, test g1 6 / 14, obs g2 17) · integration:47 (g1 12) · the exit-cause case's home (test 2, g1 21 / 22) · the reader move's owner (test 1, g1 14) · the collision check (sec c10 → Q2). All disposed.

## Escalations
- Q1 — four proposals the check-7 readers read `contradiction` against a standing line, where that standing line is the claim this chunk retires and a companion proposal of this set rewrites it: security cascade 13 (auth-authorization 25–27, rewritten by cascade 4–8), security cascade 22 (error-handling 8, rewritten by cascade 21), test g1 23 (e2e 264, rewritten by g1 22), obs g2 1 (span-trace-coverage 90, rewritten by g2 9). Proposed: apply each beside its companion.
- Q2 — security cascade 10, the collision-check clause.
- Resolved at the halt (inputs#I9, the overseer's founder-delegated answers, 2026-10-26 10:21Z): Q1 apply with companion;
  Q2 rewrite as built (retired as moot); the proposed playbook rule (a check-7 contradiction against a line the set retires
  → routine) NOT appended — n=1, recorded in the friction log for overseer1, such cases keep escalating.
- Fork 1 (inputs#I10, inputs#I11): the founder's own ruling «Вариант 1 — утвердить (Рекомендую)» (rulings.md:699)
  supersedes directive item 6 (inputs#I7); every fork-1 text is written as his ruling (`· as:` on architecture cascade 5,
  security cascade 19, test-plan 1, test g1 14).

## Summary counts
proposals 140 (doc 16 · cascade 114 · sampler 10) · apply 124 · apply-merged 5 · reject 6 (2 for a source the report does not carry) · escalate 5 (Q1 ×4, Q2 ×1)
At the 2a resume: +1 item main raised (fanout-test-plan.md 5), applied on inputs#I17 — 130 amendments in all.
