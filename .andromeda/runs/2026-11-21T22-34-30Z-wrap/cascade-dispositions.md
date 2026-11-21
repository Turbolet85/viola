# Cascade dispositions — 2026-11-21-mcp-calls-bounded (wrap 2026-11-21T22-34-30Z-wrap)

## Step 2 — the sweep
No `rows-land` dry run wrote a `cascade-patterns-{doc}-{k}.toml`: every one printed `patterns: none: {n} set(s) and
retire(s) changed prose alone — no file` (or, for the security-plan add alone, `patterns: none — the call sets and
retires no row`). The search is main's own: `cascade-patterns.toml` — OLDWAIT (`stdio tool-call default|smallest
default|times out on the client first`), OLDSEND (`total stays below|tool-call timeout`), OLDTOK (`runs no tokio
timer|no paused test clock|no tokio timer`), OLDSET (`closed cause set|cause set \(|written by both clients`), OLDLIST
(the six-member list's old ending), each control fired on the baseline 53669b68 masters. Not looked for: other
phrasings of the six-member count than those listed; the old figure «60 s» as a bare number.

`cascade.py sweep --patterns-file cascade-patterns.toml` (trail `sweep-pass-1.1.txt`): `total (5 patterns) · 15 rows
over 8 files` · `per class · new 2/2 · standing 6/2 · leaf 7/6 · curation 0/0 · base 0/0`.

- architecture.md:133 OLDSEND new dec-eljum6qo (decision/mcp-call-deadline) — amended this pass; «tool-call timeout»
  its new why, a true claim.
- architecture.md:455 OLDSET standing ctr-4lunasqf (turn-end-cause/cause-set) — no change: the turn-end set, a true
  claim sharing the token.
- architecture.md:456 OLDSET standing ctr-x7v6ouwh `edited` — amended this pass; «written by both clients» kept for
  the six by design (the qualifier the proposer dropped).
- architecture.md:413 OLDLIST standing ctr-wywpxiwa `edited` — amended this pass; the list goes on to call-deadline.
- obs-plan.md:244 OLDLIST new ctr-7mzj6yuo — amended this pass; the list goes on to call-deadline.
- obs-plan.md:653 ctr-adhcpufc (hook-decision-detail/fail-open-codes) — no change: the hook's fail-open codes, no
  MCP cause.
- obs-plan.md:670 ctr-aekt6zl2 (process-exit-detail/exit-21) — no change: the CLI's exit-21 process-exit detail; the
  CLI set stays six.
- obs-plan.md:734 ctr-ajkxdthd (exit-21-document/detail) — no change: the CLI exit-21 document, six words, true.
- leaf rows (7 over 6 files): `.claude/rules/api.md`:35 (OLDWAIT, OLDSEND), `.claude/docs/services/viola-mcp.md`:24
  (OLDWAIT), `.claude/rules/testing.md`:28 (OLDTOK ×2), `.claude/rules/security.md`:25 (OLDSET, the turn-end set — no
  change), `.claude/docs/services/viola-core.md`:6 (OLDSET ×2, TurnEndCause — the UnreachableCause list on that line
  is step 3's), `.claude/rules/observability.md`:28 (OLDLIST, the CLI exit-21 list — no change) → step 3's set.

No hit in the curation homes or the judgment bases.

## Leaves
`cascade.py leaves` (pass mode, baseline 53669b68; `epoch · «Epoch 4 — Driver reliability: ledger rows and bounds» ·
markerless 1 · gated 0 · open`): `total · pass · 13 group(s) · 26 leaves · 446387 B · unmapped 2`. One comparer a group,
`--batch leaf`: 13 of 13 pass. `leaves --apply` over the 13 files: `total · blocks 13 · applied 13`, every finding
applied by the tool (none declined: each finding's source row read at its cited line, each a stale statement or an
omission a reader would act against).

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan | pass | 1 | 1 | 1 (figures to names) | 0 |
| architecture-1 | pass | 4 | 4 | 1 (viola-mcp.md, figures to names) | 0 |
| architecture-2 | pass | 0 | 0 | 0 | 0 |
| architecture-3 | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan | pass | 3 | 3 | 1 (api.md, figures to names) | 0 |
| architecture+security-plan+test-plan+obs-plan+a11y-plan | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| obs-plan | pass | 0 | 0 | 0 | 0 |
| security-plan | pass | 2 | 2 | 0 | 0 |
| security-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| test-plan-1 | pass | 2 | 2 | 2 (testing.md, tests-summary.md, figures to names) | 0 |
| test-plan-2 | pass | 0 | 0 | 0 | 0 |
| test-plan+obs-plan | pass | 1 | 1 | 1 (observability.md, figures to names) | 0 |

**By hand (the wrap directive, `inputs#I7` item 4: «a leaf holds no figure the code states — the documentation pages
that already state bounds are corrected to names»):** after the tool's writes, six leaves drafted whole by a scripted
exact-once replacement and landed by `rec.py harness land` (tx 3792–3797): the comparers' «55 s» to
`MCP_CALL_DEADLINE`, and the standing `WAIT_CAP` figures (`min(given, 50 000)`, «50 000 ms when none is given», «after
50 s», «`WAIT_CAP` 50 000 ms») to `WAIT_CAP` — `.claude/docs/a11y-summary.md`:53, `.claude/docs/services/viola-mcp.md`:22
and :24, `.claude/rules/api.md`:35, `.claude/rules/testing.md`:28, `.claude/docs/tests-summary.md`:27,
`.claude/rules/observability.md`:40. Left as found: `.claude/docs/services/viola-core.md`:6's dialog-hold figures (60 s ·
60 s · 3600 s) — no MCP bound, outside the directive's subject.

**Unmapped:** `.claude/rules/host-linux.md` (host) — the host's own leaf, no source; `.claude/rules/testing-src.md` — no
source group maps it, not compared; it names no MCP bound (not searched beyond the sweep, which held no row in it).
