# Fan-out results — 2026-10-17-mcp-server-for-drivers

Batch snapshot before: `tree 548f141e23e3113b5d81345e8f3677a9799c54cc index 61758f03751df7926d3a99d983ff9d5b3ed7bd9f head
a902e6e4ae470765aa5aa55d7e926c570794af4f`; after: `tree 6cac97a7… index 61758f03… head a902e6e4…`; `git diff-tree` printed
exactly the seven `A …/fanout-{doc}.md` lines. Every receipt count equals its file's `- detector:` items; entity probe 0
in every file; no fix owed, no twin. No proposal rejected for a source the report does not carry: every `basis` line is
the report's own, a row or range of its `## New text, by line`, or names a file without a line.

Validation keys: R = playbook "Accurate this-chunk addition" (routine, apply) · CARD = escalated, inside the founder's own
answer to P4 fork Q1 (`inputs#I2`), on the answer card (`answer-card.md`) · Q{n} = escalated, its own question in the halt.

architecture: 30 proposals · fanout-architecture.md — note: build-system.md's Licence bullet omits viola-mcp (the report
states no licence key: not raised); the eighth gap is security-plan's; ledger rows for the plugin tool-name prefix and the
`ttlMs` / `cacheScope` shape are a question the report does not raise (not raised — `ttlMs` / `cacheScope` is the MCP
protocol `2026-07-28`'s own requirement, recorded as measured).
1. §Occupied Resources → Workspace crates — R, apply; its `fake-agent = ["tokio"]` clause rides Q3.
2. §Infrastructure Patterns → Build system (code-graph member list) — R, apply.
3. §Established Decisions → [Deployment / Distribution] (`.mcp.json` registers `viola`) — R, apply (check 6: arch:96).
4. §Occupied Resources → Claude Code integration names — R, apply.
5. §Conventions → Error handling schema (MCP bullet) — R, apply (expected amendment).
6. §Standard Contracts → Session liveness (`mcp`'s `list` built) — CARD.
7. §Standard Contracts → Channel methods (`list`) — CARD (dependent of 6).
8. §Infrastructure Patterns → Project directory structure (`run/agents.rs` comment) — CARD (dependent of 6).
9. §Conventions → CLI exit codes (`role_of` names `mcp`) — R, apply (check 6: arch:129).
10. §Occupied Resources → Binary, subcommands (`mcp` hidden) — R, apply.
11. §Occupied Resources → Environment variables (`VIOLA_DIR`, `mcp` through `resolved_home`) — Q4.
12. §Occupied Resources → Filesystem (`mcp.ndjson` producer and lines) — R, apply.
13. §Infrastructure Patterns → Crate dependency direction (`viola-mcp` row) — R, apply.
14. §Infrastructure Patterns → Crate dependency direction (`viola-state` `live_endpoint`) — R, apply.
15. §Infrastructure Patterns → Project directory structure (two scripts) — R, apply.
16. §Established Decisions → [Module Boundaries] (tokio listing; channel `fake-agent = ["tokio"]`) — Q3.
17. §Stack and Technologies (Wrapper IPC row) — R, apply.
18. §Infrastructure Patterns → Crate dependency direction (`viola-channel` Tokio client) — R, apply; its `fake-agent` clause rides Q3.
19. §Infrastructure Patterns → Build system (sync list 4, the tree form; premise corrected) — R, apply (check 6: build-system.md:8).
20. §Established Decisions → [Concurrency] KEYSTONE enforcement — R, apply.
21. §Infrastructure Patterns → CI/CD approach (job 3 set) — R, apply.
22. §Infrastructure Patterns → CI/CD approach (supply-chain step) — R, apply.
23. §Infrastructure Patterns → Build system (rmcp release-graph check landed) — R, apply (check 6: build-system.md:12).
24. §Infrastructure Patterns → CI/CD approach (lint job rmcp step) — R, apply.
25. §Infrastructure Patterns → Build system (rmcp `>=3.4.1, <3.5`) — R, apply.
26. §Established Decisions → [MCP] (`wait` cap 50 000 ms) — R, apply (check 6: arch:58).
27. §Established Decisions → [Delivery Confirmation] (client timeout > 50 s) — R, apply (check 6: arch:46).
28. §Stack and Technologies (MCP server row) — R, apply.
29. §Established Decisions → [Error Handling] (`ServeError` beside `McpError`) — Q2.
30. §Conventions → Rust error types (`ServeError`) — Q2 (dependent of 29).
31. (raised, check 5) §Established Decisions → [MCP]: each handler re-validates; a schema violation is `invalid-params`, an internal handler fault `-32603`; `list` built — R for the first two, CARD for `list` built.

security-plan: 13 proposals · fanout-security-plan.md — note: tokio `=1.53.1` and schemars `=1.2.2` exact pins; rmcp's range and features match §Dependency Security; no banned crate.
1. Input Validation (Child executable resolution, `mcp`'s `list` built) — CARD (check 6: :213).
2. Threat Model Summary (`mcp` spawns one per `list` call) — CARD (check 6: :110 ×2).
3. Input Validation (MCP tool inputs) — R, apply (the validation the detector looks for is present).
4. Error Handling (MCP `isError` codes + `-32603`) — R, apply.
5. Input Validation (Paste text, the MCP `send` tool) — R, apply.
6. Authentication & Authorization (the eighth dated gap) — CARD.
7. Security Anti-Patterns → Authentication (dated exceptions) — CARD.
8. Input Validation (CLI arguments gap list: the eighth) — CARD.
9. Authentication & Authorization (strict-modes entry points: `mcp`'s start check; its frame tools under the gap) — CARD.
10. Dependency Security (tokio ban: four sole roots + the channel tree form) — R, apply (the ban still holds; a form change, not a widening).
11. Threat Model Summary (Trust boundary, both tokio forms) — R, apply.
12. Threat Model Summary (CI jobs) — R, apply.
13. API Security (MCP transport, release-graph scope) — R, apply.
14. (raised, check 1) Input Validation → CLI arguments, Home resolution: `viola mcp` resolves through `resolved_home` — Q4.

design-system: 0 proposals · fanout-design-system.md — note: D-design-tokens holds (every new surface `tokens n/a`).
1. (raised, check 5) §Surface: cli · Platform-Specific Notes: `viola mcp` a hidden plugin-run verb — R, apply.

layout-templates: 2 proposals · fanout-layout-templates.md — note: `layout-templates.md:270` stays true.
1. §Output structure — `viola --help` (As built: `mcp` hidden, in no row) — R, apply.
2. §Primary screens (commands) (`viola mcp` the driver's machine view) — R, apply.

test-plan: 23 proposals · fanout-test-plan.md — note: D-tests-obs-harness no drift; the report's `test-plan.md:399` and `:1021` readings look swapped (re-read at apply, raised as 24 if a site is stale); the §5 impostor bullet's MCP half cannot fire under the eighth gap.
1–23. each R, apply (1: check 6 test-plan:601; 2: check 6 test-plan:618; 13 and 16: check 6 owner rows :766, :1021).
24. (raised, check 6) the sync-crates site at `test-plan.md:399` — re-read at apply.

obs-plan: 15 proposals · fanout-obs-plan.md — note: D-obs-pii no drift; D-obs-stack no off-spec dependency; no site claims a live `process-exit`.
1–15. each R, apply (§1 is a verbatim copy kept current — playbook "Verbatim upstream copy kept current"; 1: check 6 obs-plan:184).
16. (raised, check 5) §3 → Logging stack: the board's lines and the `agents-probe` pair in `mcp.ndjson` — R, apply where the key states its writers.

a11y-plan: 1 proposal · fanout-a11y-plan.md — note: D-a11y-obs-schema no drift.
1. §8 Timeout extensions (MCP `wait` cap, a driver-only bound) — R, apply.

Check 3 (intent): scope record — 9 lines (companion 6, in-intent 3, widening 0), each `serves` holds against the report's
Changes. The report's one acceptance read UNMET as written ("the live start count is 1") is a justified divergence: the
founder's own word raised the cap to 2 (`inputs#I4`) — the plan's text is the stale side; no amendment (a plan is a chunk
artifact), the matrix acceptance names no count.
Check 6 (disproved claims): all ten entries matched above.

## Escalations resolved (one halt, `answer-card.md`)
- CARD (architecture 6, 7, 8, 31's `list` clause; security-plan 1, 2, 6, 7, 8, 9) — confirmed; the confirm a delegate's
  (the Viola overseer), PROVISIONAL; the ratification the founder's own answer to Q1 (`inputs#I2`). Applied.
- Q2 (architecture 29, 30) — "Record as open divergence"; applied, plus the same claim at §Inherited Defaults (Errors).
- Q3 (architecture 16, and the `fake-agent` clauses of 1 and 18) — "Write as built"; applied.
- Q4 (architecture 11, security-plan 14) — "As built, NOT ratified"; applied.
- test-plan 24 (raised) — re-read: `test-plan.md:399` says "the sync crates carry no tokio dev-dependency" and names no
  crate of the list, true (`viola-channel` is no sync crate now); the crate-naming site is the Lint row, amended as 15. No change.
- obs-plan 16 (raised) — applied at §3 → Logging stack (`mcp.ndjson`'s lines).
- design-system 1 (raised) — applied.
- architecture 31 (raised) — applied in the [MCP] tool-inputs paragraph.

Applied: 84 proposals + 4 raised (architecture 31, security-plan 14, design-system 1, obs-plan 16; test-plan 24 no
change) · rejected 0 · escalations 4 resolved (the card and Q2–Q4, one halt of four questions).
