# Cascade dispositions — 2026-11-17-trust-record-ledger-row (wrap 2026-11-17T21-00-47Z-wrap)

## Step 2 — the sweep
The search: `cascade.py sweep` over `cascade-patterns-architecture-1.toml` (arch1-4, arch1-5: `viola-e2e`, the term the
two claude-shapes excepts dropped), `cascade-patterns-architecture-2.toml` (arch2-1: `22nd`) and main's own
`cascade-patterns.toml` (m-runs5 «five interactive runs / Run A to E» · m-pairs8 «eight spawn pairs» · m-rows22 «22nd
row / 22 rows» · m-writesnone «writes nothing under/into/in … claude» · m-namesonly «never a value / values never
read»), each spanning all seven masters, the key files, the curation homes, the bases and the leaves. The listing:
`sweep-pass-2.1.txt` (the second pass sweep, after the last row TX 2511; `sweep-pass-1.1.txt` the first, run before the
re-raised set landed — partial by construction, superseded). Its counts: «total (8 patterns) · 161 rows over 18 files»
· «per class · new 0/0 · standing 50/3 · leaf 107/15 · curation 4/1 · base 0/0».

The other masters' dry runs wrote no pattern file (`patterns: none: … changed prose alone`), so their changed rows
reach step 3 by their deltas.

- arch1-4 / arch1-5 (`viola-e2e`) · standing 19 each (architecture 7 · test-plan 12) — no change: each a true claim
  sharing the crate's name; the term left the two excepts only because they were reworded. leaf 44 each and curation 2
  each (`.claude/rules/testing.md:85`, `:112`) — no change, same reason; the leaves are re-read whole in step 3.
- arch2-1 (`22nd`) · 0 rows, control fired — the stale count is gone from the masters.
- m-runs5 · standing 0 — every master row now reads six; leaf 10 (security.md:17, verification-harness.md:28,
  commands.md:23, services/viola-agent-claude.md:11, services/viola-pty.md:14, services/viola.md:6 and :35,
  stack.md:13, tests-summary.md:21, workflow.md:49) → step 3, stale.
- m-pairs8 · 0 rows, control fired — no master or leaf says eight pairs.
- m-rows22 · standing 0; leaf 2 (services/viola-agent-claude.md:26 ×4, workflow.md:49) → step 3, stale.
- m-writesnone · standing 3 (security-plan decision/nothing-written-under-claude-home, /no-writes-under-claude-home,
  /viola-writes-nothing-in-claude-dir) — amended (their except carries verify's own record, TX 2504); leaf 2
  (security.md:17, security-summary.md:73) → step 3, each to carry the bounded exception.
- m-namesonly · standing 9 — amended where an except now names the one read (architecture env-strip-names-only,
  env-strip-reads-names-only, and the two claude-shapes rows' «never a value» which is the rig's UPS payload, true);
  no change, true claims of another subject: security-plan decision on the UPS payload (raynvazb), the two
  environment-names log contracts (qsdgfift, qstz7thh) and test-plan ctr-c7gl7p62 (names in a log field); security-plan
  credential-strip-by-name-only (rlatwaem) amended. leaf 5 (CLAUDE.md:25, :43, conventions.md:54, security-summary.md:73,
  services/viola-agent-claude.md:6) → step 3 (the UPS-payload wording is true; the env wording is the comparers').
- Curation homes: no stale hit (the two testing.md lines name the crate truly). Bases: 0.

## Leaves
The set: `cascade.py leaves` (pass mode; «epoch · «Epoch 4 — Driver reliability: ledger rows and bounds» · markerless 7
· gated 0 · open»): «total · pass · 14 group(s) · 25 leaves · 412446 B · unmapped 2» — unmapped
`.claude/rules/host-linux.md` (the host leaf, no source) and `.claude/rules/testing-src.md` (no changed source states
it) — no comparison owed. The batch: 14 comparers, `fanout.py check` «VERDICT: 14 of 14 pass».
The decision: `leaves --apply` «total · blocks 51 · applied 49 · held 2».
- Groups with no finding: architecture+design-system · architecture+security-plan · obs-plan · security-plan+obs-plan.
- Applied 49, each read against its row: architecture-1 9 · architecture-2 7 · architecture-3 3 · the five-master group
  (CLAUDE.md) 3 · architecture+test-plan+obs-plan 5 · design-system 1 · security-plan 10 · test-plan-1 5 · test-plan-2
  5 · test-plan+obs-plan 1.
- Held and declined 2: commands.md:23#3 and :24#1 — each moved the leaf's already-stale «twenty-four» (the carried
  leaf-count drift, CARRY cry-2snj2igc on «Version done-check») to «twenty-five»; the code's relied count is 23.
- Applied in error 1, corrected by hand: commands.md:24#2 wrote «a twenty-four-row stamp written before chunk
  2026-11-17-trust-record-ledger-row» (should read twenty-two) — the operator of this wrap meant to hold it.
- By hand (drafted whole, `rec.py harness land`): commands.md (TX 2527: «all twenty-three», «`[23/23] trust-record`»,
  «a twenty-two-row stamp … before chunk 2026-11-17-trust-record-ledger-row»); and five spots the comparers did not
  name, found by a grep for the old counts after the apply: rules/security.md:17 «five interactive PTY children» → six
  (TX 2528) · rules/verification-harness.md:28 «all five interactive runs» → six and «all twenty-four» → twenty-three
  (TX 2529) · docs/security-summary.md:72 «five interactive `claude` children» → six (TX 2530) ·
  docs/tests-summary.md:21 «verify's five interactive runs» → six (TX 2531) · docs/services/viola.md:47 «Runs A–E» →
  A–F (TX 2532).
- Left standing, named: commands.md:24's earlier stamp-history row counts and services/viola-agent-claude.md:26's
  «twenty-two ids every stamp written before chunk 2026-10-16…» are history the carry cry-2snj2igc owns.
- No leaf states the trust-record row verified or the CLI honouring a record as a measured fact: each finding names
  the row's probe and what it passes on, never its live standing.
