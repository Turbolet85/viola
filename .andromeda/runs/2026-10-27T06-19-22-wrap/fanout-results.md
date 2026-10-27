# Fan-out results — 2026-10-27-bounded-inputs-at-every-boundary

Not a sampled run (`cascade-sample.txt`: `sample: no · wrap · draw 3 of 10`). Batches: stage1 (13: 7 doc, 6 task; the
architecture and design-system task writers re-spawned once in stage1-r1, `[NEW_FACTS]` and `[UNTOUCHED_REACHED]`,
both passed) · classify (66 of 66 pass, yes 57 of 879) · strong (5; test-plan re-spawned once in strong-r1,
`[PICKED_EVERY_ID]` missing `ci-integration#1.2`, passed) · newdetail (3 of 3 pass). Every check's `[PATH_CHECK]`
PASS, index and head unchanged. No fix was owed to any result file (no `[PREAMBLE]`, `[COMMENTARY]` or escape row), so
no `.raw-fanout-*` twin of a result file exists beyond the re-spawned outputs' move-asides.

## Verdicts
- architecture: 2 proposals · fanout-architecture.md · 8 cascade proposals · fanout-architecture--cascade.md · 1 new-detail proposal · fanout-architecture--cascade-new-cross-cutting-patterns.md · 6 facts · 6 placed · 0 no home
  - note (doc): the [Screen Model] disproved claim and the [MCP] stdin-bound detail are the cascade's; no D-arch-resources drift (no IPC method, tool, endpoint, env var, exit code, crate or socket added).
  - note (new-detail): the body's Cross-cutting Patterns section holds no broad line for bounded readers.
- security-plan: 6 proposals · fanout-security-plan.md · 14 cascade proposals · fanout-security-plan--cascade.md · 9 facts · 9 placed · 0 no home
  - note (doc): no D-security-auth drift; the detail moves (input-validation.md:90, dependency-security.md:14, the new stdin entity) are the cascade's; security-plan.md:136 and :160 stay true.
  - note (cascade): body :452 now a `deny.toml` ban with a firing probe; body :174 Parser surfaces may name the MCP stdin line reader.
- design-system: 0 proposals · fanout-design-system.md · 0 cascade proposals (pick: yes 0) · 0 facts
  - note: no drift — no new UI, no cli output wording changed.
- layout-templates: 0 proposals · fanout-layout-templates.md (whole master, no cascade)
  - note: no cli output literal, surface or region added.
- test-plan: 11 proposals · fanout-test-plan.md · 14 cascade proposals · fanout-test-plan--cascade.md · 1 new-detail proposal · fanout-test-plan--cascade-new-unit-test-strategy.md · 9 facts · 9 placed · 0 no home
  - note (doc): D-tests-framework and D-tests-obs-harness no drift; deviation 1's §4 note retires if P5 gives a route owner.
  - note (new-detail): the body's §4 viola-mcp broad bullet names only handler re-validation and `tools/list`.
- obs-plan: 1 proposal · fanout-obs-plan.md · 10 cascade proposals · fanout-obs-plan--cascade.md · 0 new-detail proposals · fanout-obs-plan--cascade-new-slo-invariants-telemetry-budgets.md · 6 facts · 5 placed · 1 no home
  - note (doc): D-obs-stack and D-obs-pii no drift; the `mcp-stdin` parser value's §12 Decisions Log entry is left to the orchestrator.
  - note (new-detail, F5 no home): the serde_json `unbounded_depth` ban's home is obs-plan.md §10's broad `cargo deny` failure list (:669), not the slo file.
- a11y-plan: 1 proposal · fanout-a11y-plan.md · 4 cascade proposals · fanout-a11y-plan--cascade.md · 3 facts · 3 placed · 0 no home
  - note (doc): D-a11y-obs-schema holds (the a11y violation schema carries no diag-line kind).

## Dispositions
The two test gaps of inputs#I4 item 4 are each stated ONCE in the masters, in test-plan, with their owner: the next
route entry «Sanitised error surfaces», which takes both as a PREREQ at this wrap's P5 (Tier 1: a missing test for a
clause a chunk landed is folded into the next chunk). Every other master proposal that restates a gap drops that clause
(`· as:`), or is rejected as the test-plan note's twin.

fanout-architecture.md
1. apply — check 1: Accurate this-chunk addition (the report's Symbols / APIs, `serve` / `CappedLines`); plan Expected amendment, architecture [MCP]
2. apply — check 1: Accurate this-chunk addition, and "A count in a master that carries no rule": every site of `13/13` is amended in this apply (this key; security `dependency-security.md:14`; test-plan `test-plan.md:794`, `ci-integration.md:15`), the rule named beside it — the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe `channel-tokio-edge`
3. apply — check 5 (raised): §Cross-cutting Patterns (body, `.andromeda/architecture.md`) — add one broad bullet after «Untrusted upstream text»: «**Bounded readers.** Every product reader of external bytes is bounded in its own statement (`take`) or named as an exception (the own-executable reads); the contract binary `tests/contract_bounded_readers.rs` enumerates the boundaries and censuses the product sources.» — the body line of fanout-architecture--cascade-new-cross-cutting-patterns.md 1, whose group would otherwise stand under no broad line

fanout-architecture--cascade.md
1. apply — check 6: the report's Spec claims disproved (`established-decisions.md:23`, `panicked` for a caught panic, `oversize` for a dropped message); plan Expected amendment, architecture [Screen Model]
2. apply — check 7: architecture-newgroup-established-decisions.md · distinct
3. apply — check 7: architecture-newgroup-established-decisions.md · distinct · as: drop the closing clause «; that second path is driven by no test, its owner given at the chunk's P5» — the gap is stated once, in test-plan §4, with its owner
4. apply — check 7: architecture-newgroup-established-decisions.md · distinct
5. apply — check 7: architecture-newgroup-stack-and-technologies.md · distinct
6. apply — check 7: architecture-newgroup-established-decisions.md · distinct (OPEN, owner «Windows-only live measurements», the P5 card's CARRY, PROVISIONAL)
7. apply — check 1: Accurate this-chunk addition (`serve`'s session end; no exit code added)
8. apply — check 1: Accurate this-chunk addition (the `mcp.ndjson` line list gains the one `mcp-stdin` line)

fanout-architecture--cascade-new-cross-cutting-patterns.md
1. apply — check 7: architecture-newgroup-cross-cutting-patterns.md · distinct · as: drop the closing «Owed: five `BOUNDARIES` rows … whose owner was given at P5.» sentence — that gap is stated once, in test-plan §6's Contract suite note, with its owner

fanout-security-plan.md
1. apply — check 1: Accurate this-chunk addition; plan Expected amendment, security-plan §Input Validation (a `viola mcp` stdin row)
2. apply — check 1: plan Expected amendment (the Constants list gains the MCP stdin and the compare read; the own-executable reads named no external input)
3. apply — check 1: Accurate this-chunk addition (dependent of 1)
4. apply — check 1: Accurate this-chunk addition (dependent of 1)
5. apply — check 1: plan Expected amendment, security-plan §Dependencies (the serde_json `unbounded_depth` feature ban)
6. apply — check 1: the report's `depth-ban` site `security-plan.md:452` (change)

fanout-security-plan--cascade.md
1. apply — check 7: security-plan-newgroup-threat-model-summary.md · distinct
2. apply — check 1: the body broad line of 1 (dependent-of)
3. apply — check 7: security-plan-newgroup-input-validation.md · distinct
4. apply — check 7: security-plan-newgroup-input-validation.md · distinct · as: drop the closing «; no test drives that pre-initialize path» — the gap is stated once, in test-plan §4, with its owner
5. apply — check 7: security-plan-newgroup-input-validation.md · distinct
6. reject — twin of fanout-security-plan.md 1: both add the body `viola mcp` stdin row after the MCP tool inputs line; its fact rides that proposal
7. apply — check 7: security-plan-newgroup-input-validation.md · distinct
8. apply — check 1: plan Expected amendment (the detail Constants list, `input-validation.md:90`)
9. apply — check 7: security-plan-newgroup-input-validation.md · distinct
10. apply — check 7: security-plan-newgroup-input-validation.md · distinct · as: drop the clause «; five rows (…) name a witness that never drives past the cap, their cap held by the census alone» — that gap is stated once, in test-plan §6, with its owner
11. apply — check 7: security-plan-newgroup-input-validation.md · distinct
12. apply-merged → fanout-security-plan.md 2 — the same body Constants line (`security-plan.md:176`), written as one edit
13. apply — check 1: "A count in a master that carries no rule" (the rule named; every site amended, see fanout-architecture.md 2)
14. apply — check 7: security-plan-newgroup-dependency-security.md · distinct

fanout-test-plan.md
1. apply — check 1: the count literal `13/13` → `14/14` (every site amended, see fanout-architecture.md 2)
2. apply — check 1: Accurate this-chunk addition (the passthrough half, `tests/chaos_feed_panic.rs:109-130`)
3. apply — check 1: Accurate this-chunk addition (dependent of 2)
4. apply — check 1: Accurate this-chunk addition
5. apply — check 1: Accurate this-chunk addition (dependent of 4)
6. apply — check 1: Accurate this-chunk addition (the MCP stdin negative)
7. apply — check 1: Accurate this-chunk addition (the `capped_lines_*` and `serve` child cases)
8. apply — check 5 (inputs#I4 item 4, the pre-initialize stdin cut: a test-plan note) · as: the note's owner reads «owner: the route entry «Sanitised error surfaces», as a PREREQ», in place of «its owner the one this wrap gives»
9. apply — check 1: Accurate this-chunk addition (the bounded compare read)
10. apply — check 1: Accurate this-chunk addition (the enumeration)
11. apply — check 5 (inputs#I4 item 4, the five rows whose witness never drives past the cap: a test-plan note) · as: the note ends «; owner: the route entry «Sanitised error surfaces», as a PREREQ (a cap-driving case for each row)»

fanout-test-plan--cascade.md
1. apply — check 1: Accurate this-chunk addition (dependent-of)
2. apply — check 1: Accurate this-chunk addition
3. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct
4. apply — check 1: Accurate this-chunk addition
5. apply — check 1: Accurate this-chunk addition (the raw stdin writer, `mcp_tools.rs` new text 455-510)
6. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
7. reject — twin of fanout-test-plan.md 8: the pre-initialize gap is stated once, in §4, with its owner (check 7: test-plan-newgroup-e2e-test-strategy.md · distinct against the file, not against the list)
8. apply — check 1: Accurate this-chunk addition
9. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
10. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
11. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
12. reject — twin of fanout-test-plan.md 11: the five-row gap is stated once, in the body's §6 Contract suite note, with its owner (check 7: test-plan-newgroup-e2e-test-strategy.md · distinct against the file, not against the list)
13. apply — check 1: the count literal and «the twelve bans» → thirteen (every site amended, see fanout-architecture.md 2)
14. escalate → Q1 — check 7: test-plan-newgroup-ci-integration.md · contradiction · line 15 «last line `deny-probes: 13/13 banned, control clean`: the twelve bans one probe each» — escalated as written (inputs#I4 item 3), though fanout-test-plan--cascade.md 13 rewrites that line's count in the same apply · resolved: apply (Q1) · as: written together with fanout-test-plan--cascade.md 13 as ONE edit of `ci-integration.md:15` — the count `14/14` and «thirteen bans», and the Runs clause naming the serde_json `unbounded_depth` ban and its probe row (inputs#I5: the overseer as operator, founder-delegated, 07:05Z)

fanout-test-plan--cascade-new-unit-test-strategy.md
1. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct

fanout-obs-plan.md
1. apply-merged → fanout-obs-plan--cascade.md 3 — the same `viola mcp` Exporter line of §1 Telemetry surfaces, written as one edit
  (the `mcp-stdin` parser value's §12 Decisions Log record rides the obs-plan sidecar entry: the log lives in `obs-plan-amendments.md`, §12 of the body pointing there)
  (obs new-detail F5, no home: the serde_json `unbounded_depth` ban is security's subject — carried by security-plan 5, 6 and cascade 14, architecture 2 and cascade 5, test-plan cascade 13 and 14 — and obs-plan states no serde_json ban, so nothing is raised into obs)

fanout-obs-plan--cascade.md
1. apply — check 1: "Verbatim upstream copy kept current" (§1 is judged like any body amendment); the report's site `obs-plan.md:125`
2. apply — check 1: "Verbatim upstream copy kept current"; Accurate this-chunk addition
3. apply — check 7: obs-plan-newgroup-obs-scope-summary.md · distinct
4. apply — check 1: the report's `parse-counts` site `obs-scope-summary.md:500` (change)
5. apply — check 1: the report's `parser-catalog` site `log-coverage.md:20` (9 → 10 values, the rule: the schema's `parse-rejected.parser` enum)
6. apply — check 1: the report's `parser-catalog` site `log-coverage.md:34` (change)
7. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct
8. apply — check 7: obs-plan-newgroup-error-capture-reporting.md · distinct
9. apply — check 7: obs-plan-newgroup-error-capture-reporting.md · distinct · as: drop «, a path no test drives» — the gap is stated once, in test-plan §4, with its owner
10. apply — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct

fanout-a11y-plan.md
1. apply-merged → fanout-a11y-plan--cascade.md 3 — the same body tui Entity line (`a11y-plan.md:33`), its detail in cascade 1 and 2

fanout-a11y-plan--cascade.md
1. apply — check 7: a11y-plan-newgroup-a11y-scope-summary.md · distinct
2. apply — check 7: a11y-plan-newgroup-a11y-scope-summary.md · distinct
3. apply — check 1: plan Expected amendment, a11y-plan §1 (the report's site `a11y-plan.md:33`)
4. reject — check 1: "Not this chunk's drift": the feed-panic case's `send` refusal and typed key landed at chunk 2026-10-04 (the report's Changes add only the outer-terminal half); check 7 (a11y-plan-newgroup-aria-patterns-roles.md · distinct) notwithstanding

## Checks 2–6
- check 2 (cross-contradiction): none — every pair touching one line is merged (security 12 → 2; obs 1 → cascade 3; a11y 1 → cascade 3) or a twin rejected (security cascade 6; test-plan cascade 7, 12); no two proposals edit one section in opposing directions.
- check 3 (intent-consistency): the report's Deviations 1–6 each carry a justification; scope record none (`gate.py scope` clean, 0 recorded); no proposal diverges from the entry «Bounded inputs at every boundary» or the plan's acceptance.
- check 4 (absence needs evidence): the "no master names the MCP stdin" claims rest on the report's `mcp-stdin` sweep (control unfired: 0 sites); check 7's readers read each target file whole.
- check 5 (Expected amendments): security §Input Validation → security 1, 2, cascade 3–5, 8–11 · security §Dependencies → security 5, 6, cascade 13, 14 · obs §6 → obs cascade 5–7 (the Decisions Log record → the obs sidecar entry) · obs §1/§4 → obs cascade 1–3 (§4: no MCP surface row states the stdin — the report's `mcp-surface` sweep found obs-plan.md:125 the one site; superseded for §4) · architecture [MCP] / [Screen Model] → architecture 1, cascade 1–4 · test-plan §5/§6/§9 → test-plan 1–11, cascade 1–14 · a11y §1 → a11y cascade 1–3. Every entry carried. inputs#I4 item 4's two owners: test-plan 8 and 11 (raised by the directive, their owner the PREREQ P5 lands).
- check 6 (disproved claims): `established-decisions.md:23` → architecture cascade 1 (apply).
