# Cascade dispositions — 2026-10-17-mcp-server-for-drivers

**The search.** `cascade.py sweep --patterns-file .andromeda/runs/2026-10-17T14-19-45-wrap/cascade-patterns.toml`
(cascade v1.10, baseline 574ad969 — the pre-CI parent), 24 patterns derived from every amendment of this pass, each the
RETIRED claim's own wording and mechanism (not the name it was known by): `mcp`'s `list` unbuilt / spawns none · the empty
`.mcp.json` · the `mcp` verb unbuilt · "only faults on the MCP connection" are JSON-RPC errors · `TokioChildProcess` /
`transport-child-process` · the sync channel client inside the MCP handler · `12/12` / twelve bans · "has no subject
until" · the MCP steps "owed to" · `mcp_schema_violations` · the `mcp` stderr sink without `VIOLA_DIR` · the seventh /
six-earlier / seven dated gaps · `viola-channel` inside a sync-crate list · "covered through the sync roots" · rmcp `~3.4`
· "only `viola-mcp` enables / lists tokio" and "Tokio flavour only in `viola-mcp`" · "twenty verify-driven" · the one open
divergence · the MCP `wait` open item · `start_paused = true` · an `instance-unreachable` body without `detail` and the
two-code closed list · `stamp:true` · "MCP when it lands" · the tests/arch misalignment note. Every control fired.
Sections read whole besides the rows: architecture [MCP], [Error Handling], [Module Boundaries], [Concurrency], Session
liveness, Channel methods `list`, Binary/subcommands, Environment variables, Filesystem `diagnostics/`; security-plan
rows 110, 182, 184, 198, 202, 203, 213, 273, 295, 451, 476; obs-plan §1 mcp surface, §6, §7, §10; the touched key files.

**Counts (copied):** `total (24 patterns) · 33 rows over 13 files` · `per class · new 3/3 · standing 14/6 · leaf 16/7 ·
curation 0/0 · base 0/0`. Zero-row patterns (`mcp-verb-unbuilt`, `sync-client-mcp`, `no-subject`, `sync-roots`,
`rmcp-tilde`, `kill-twenty`, `unreach-nodetail`, `stamp-true`, `misalign`): control fired, no row stands.

## Master and registry rows
- `architecture.md:96` mcp-json-empty, edited — no change: the clause states what `.mcp.json` shipped until the verb existed (history, marked as such).
- `architecture.md:30` child-transport, new — no change: "never `transport-child-process`", this pass's truth.
- `security-plan.md:273` child-transport, new — no change: same.
- `test-plan.md:618` child-transport, edited ×2 — no change: "never rmcp's `TokioChildProcess`" and "no `transport-child-process`", this pass's truth.
- `registries/…/test-plan/bootstrap-phases-derive-for-route-setup-project.md:24` child-transport, edited — no change: "no `transport-child-process`".
- `test-plan.md:1022` probes-12, new — no change: "the twelve bans one probe each, plus probe `channel-tokio-edge`" (13/13).
- `test-plan.md:840` schema-file, edited — no change: "no `mcp_schema_violations.rs` binary".
- `obs-plan.md:45` mcp-stderr, edited — no change: the item now reads reconciled by D-09.
- `security-plan.md:182` gap-count, edited ×2 (@c4797, @c5480) — no change: "the seventh dated gap overall" labels the fifth interim gap (pause / release), true; "none of the six earlier gaps" is that gap's own borrowing rule, true; the eighth follows them.
- `architecture.md:95` channel-sync, edited — a true claim sharing the token: the workspace member list.
- `architecture.md:314` channel-sync — a true claim: the product crate list.
- `architecture.md:316` channel-sync, edited — no change: "every member — …", the member list.
- `test-plan.md:130` channel-sync — a true claim: the unit-level crate list.
- `registries/…/architecture/build-system.md:14` channel-sync, edited — a true claim: the code-graph plane's member list.
- `registries/…/architecture/build-system.md:18` channel-sync — a true claim as far as the report carries: the licence inheritors (whether `viola-mcp` inherits `license.workspace` the report does not state; not amended).
- `test-plan.md:935` paused-tokio, edited — no change: the planned mechanism now carries its qualifier (no manifest names `test-util`, no `start_paused` test exists), the mock_instant form.
- `test-plan.md:987` paused-tokio — a true claim: conditional, "for any timer the MCP adapter itself owns" (it owns none).

## Leaf rows (each joins cascade step 3's set — re-derived, never patched)
- `.claude/rules/api.md:21` list-unbuilt — leaf, re-derived.
- `.claude/docs/services/viola-agent-claude.md:14` list-unbuilt; `:26` one-divergence — leaf, re-derived.
- `.claude/docs/services/viola-mcp.md:22` conn-only-rpc; `:24` wait-open; `:35` paused-tokio; `:36` schema-file — leaf, re-derived.
- `.claude/docs/stack.md:14` only-mcp-tokio; `:23` one-divergence; `:47` child-transport — leaf, re-derived.
- `.claude/docs/commands.md:60` probes-12 — leaf, re-derived.
- `.claude/docs/tests-summary.md:26`, `:27`, `:28`, `:29` owed-mcp — leaf, re-derived.
- `.claude/docs/services/viola-core.md:21` mcp-lands — leaf, re-derived.

No curation-home row and no judgment-base row.

## Leaves

Mode: pass, every group (Epoch 5 is not closed — `route.py epoch`: 2 markerless entries). Set: `cascade.py leaves` with
`--add` of the seven leaves the sweep's leaf rows name; 14 groups, 28 leaves; batch path check clean (14 `A` lines, index
and head unchanged). Counts copied from the apply's `total` line: `total · blocks 68 · applied 63 · reshaped 3 · held 2`.

| group | findings | applied (tool) | by hand | declined |
|---|---|---|---|---|
| architecture-1 | 18 | 18 | 0 | 0 |
| architecture-2 | 10 | 10 | 0 | 0 |
| architecture-3 | 0 | 0 | 0 | 0 |
| architecture+security-plan | 8 | 6 | 2 (`api.md:35`, `api.md:36#1`: `reshaped`, the replace-with's `|` characters read as table cells; written by hand, shortened, without them) | 0 |
| masters (CLAUDE.md) | 3 | 2 | 1 (`CLAUDE.md:27#2`, held: the comparer's clause was a full description of the verb in always-loaded text; a short pointer written by hand instead) | 0 |
| architecture+design-system+layout-templates | 2 | 2 | 0 | 0 |
| architecture+test-plan+obs-plan | 4 | 4 | 0 | 0 |
| security-plan | 7 | 6 | 1 (`security.md:40`, `reshaped` for the same `|` reason; written by hand, shortened) | 0 |
| security-plan+obs-plan | 1 | 0 | 0 | 1 (`ci.md:11`: a pointer line re-listing gate steps its cited paragraph does not hold — a detail that stays in its master; the leaf's pointers stay true) |
| design-system+layout-templates | 0 | 0 | 0 | 0 |
| test-plan | 8 | 8 | 0 | 0 |
| test-plan+obs-plan | 3 | 3 | 0 | 0 |
| obs-plan | 3 | 3 | 0 | 0 |
| a11y-plan | 1 | 1 | 0 | 0 |

The write was one `cascade.py leaves --apply {the 14 files} --only {every applied row of the dry run but CLAUDE.md:27#2 and
ci.md:11#1}` call (the script that carried the id list moved to the session scratchpad at hygiene: it held a host path).
Every finding's source side was read at its cited master line in this pass's own edits (the sources are this wrap's
bodies). Unmapped: `.claude/rules/host-linux.md` (the host leaf — the host's statements, no comparer) and
`.claude/rules/testing-src.md` (setup's path-scoped template; it states none of the amended claims: no `mcp`, tokio,
sync-crates, deny-probes or `start_paused` text) — no change.
