# Cascade dispositions — 2026-10-27-bounded-inputs-at-every-boundary

## Step 2 — the sweep
The call: `cascade.py sweep` over the apply writers' `## Patterns` — `apply-architecture.md` (arch-01…05),
`.raw-apply-security-plan.md` (sec-01; the writer's first file — its ruled re-spawn wrote `(none)`), `apply-obs-plan.md`
(obs-01…03), `apply-a11y-plan.md` (none), `apply-test-plan.p1.md` (tp1-01), `apply-test-plan.p2.md` (tp2-01…03);
baseline `4b0bf776` (the parent of the oldest pre-CI commit `607792cb`). Trail `cascade-2026-10-27-bounded-inputs-at-every-boundary.json`.

Totals, copied: `total (13 patterns) · 6 rows over 2 files` · `per class · new 0/0 · standing 1/1 · leaf 5/1 · curation 0/0 · base 0/0`.

What was searched: the retired `oversize`-for-a-caught-panic wording (arch-01), the old `13/13` / «twelve bans» count
(arch-05, sec-01, tp1-01, tp2-01, tp2-02), the `mcp.ndjson` line list, the `mcp` session-end and build-system ban list
wordings (arch-02…04), the `parse-rejected` parser list and per-parser count list wordings (obs-01…03), and the
chaos-suite passthrough wording (tp2-03). Seven patterns read 0 rows with their control fired: a statement about
those patterns.

Rows:
- `.andromeda/security-plan-detail/input-validation.md:69` arch-01 standing — no change: the clause is the DROPPED
  copy's (`a full queue drops the copy … a dropped copy poisons the model like a panic (one
  parse-rejected{parser:"vt100-feed", detail:"oversize"} per poisoning episode`), true per `src/run/gate.rs:72`; the
  caught panic's `panicked` stands earlier on the same line.
- `.claude/docs/commands.md:60` arch-05 · sec-01 · tp1-01 · tp2-01 · tp2-02 leaf (×5, one line) — the stale
  `deny-probes: 13/13 banned, control clean` and «the twelve bans one probe each»: re-derived in step 3 (`--add`).

No curation home and no judgment base holds a swept wording.

## Leaves
Set: `cascade.py leaves --add .claude/docs/commands.md` (pass mode: this chunk closes no epoch, Epoch 6 keeping two
markerless entries) — `total · pass · 13 group(s) · 26 leaves · 384911 B · unmapped 2`. Unmapped:
`.claude/rules/host-linux.md` (the host's own leaf: no comparer's) and `.claude/rules/testing-src.md` (no provenance
header names a master: not re-derived). The comparers' apply: `total · blocks 16 · applied 6 · reshaped 4 · held 6`.
An omission is taken only where a reader of the leaf would otherwise act against the source.

- a11y-plan (pass) · 1 finding · applied 0 · by hand 0 · declined 1: `a11y-summary.md:53` — the feed-panic case's detail stays in a11y-plan §1; the summary's reading line stands true.
- architecture-1 (pass) · 1 · 0 · 0 · declined 1: `viola-agent-claude.md:21` — that crate holds no own-executable read and no MCP stdin; its «bounded by `Read::take(MAX_FRAME)`» stands true for it.
- architecture-2 (pass) · 2 · applied 1 (`viola-mcp.md:23`, the one `mcp-stdin` line) · by hand 1 (`viola-mcp.md:6`, reshaped: the comparer's text carried a `\n` escape the line lacks; written shorter, naming `CappedLines`, the cap and the session end) · declined 0.
- architecture-3 (pass) · 2 · applied 1 (`stack.md:19`, stale: the Serialization row gains the `unbounded_depth` ban) · by hand 1 (`viola.md:45`, reshaped: written as the session end and its exits, without the escape) · declined 0.
- architecture-4 (pass) · 1 · 0 · 0 · declined 1: `frontend.md:25` — a stdout rule; the session end writes nothing on stdout, so the line stands true.
- architecture+security-plan (pass) · 1 · 0 · by hand 1 (`api.md:35`, reshaped: written shorter, `CappedLines` at `MAX_FRAME` and the refusal) · 0.
- architecture+security-plan+test-plan+obs-plan+a11y-plan (pass) · 2 · 0 · by hand 1 (`CLAUDE.md:38`, the GENERATED warnings line: the MCP stdin and the census named in one parenthesis, shorter than the comparer's) · declined 1: `CLAUDE.md:25` — the modules line names the crate's surface; the stdin bound is detail its master and service doc hold.
- architecture+test-plan+obs-plan (pass) · 2 · applied 2 (`commands.md:22`, the `mcp` verb's session end; `commands.md:60`, stale: `13/13` → `14/14`, the thirteen bans — the step-2 sweep's leaf row) · 0 · 0.
- obs-plan (pass) · 0 findings.
- security-plan (pass) · 2 · applied 1 (`security.md:25#2`, the `deny.toml` ban beside «never `unbounded_depth`») · by hand 1 (`security.md:25#1`, reshaped: the MCP stdin line cap in one clause) · 0.
- security-plan+obs-plan (pass) · 0 findings.
- test-plan (pass) · 1 · applied 1 (`testing.md:38`, the outer-terminal byte search of the feed-panic case) · 0 · 0.
- test-plan+obs-plan (pass) · 1 · 0 · 0 · declined 1: `observability.md:19` — the `mcp-stdin` line's detail stays in obs-plan §6; the rule's `mcp.ndjson` statement stands true.
