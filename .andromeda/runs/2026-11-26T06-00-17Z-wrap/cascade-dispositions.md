# Cascade dispositions — gated wrap 2026-11-26T06-00-17Z (chunk 2026-11-25-placing-without-moving-the-view)

## Step 1 — apply
No operation: the three proposers (security-plan, test-plan, obs-plan) each wrote `@none`; architecture, design-system,
layout-templates and a11y-plan had no proposer (`rows-union`: picked 0). No `rows-land` call, no masters TX.

## Step 2 — the sweep
No pattern file: no `rows-land` dry run ran, and main wrote no `cascade-patterns.toml` (no master row changed). The
sweep is skipped with that line. The lateral binds (test-plan §3 ↔ obs-plan §3 / §6, a11y schema ↔ the log schema) are
untouched: no harness command or log field moved.

## Leaves
Mode: WHOLE (`--all`). `cascade.py leaves` in the pass mode printed `epoch · «Epoch 5 — The fleet: lineage, launch and
windows» · markerless 0 · gated 1 · open` and `pass: no source changed — no group`: the epoch reads `open` only because
this chunk's own `gated` record stands until this wrap's P7 flip; this gated wrap closes Epoch 5, so the letter's
whole mode (a chunk closing its epoch) was run. `total · whole · 17 group(s) · 27 leaves · 513923 B · unmapped 2`.
Unmapped: `.claude/rules/host-linux.md` (host — the host's own statements, no comparer's) · `.claude/rules/testing-src.md`
(no master names it; not this chunk's subject, left).

Comparers: 17, `check` `VERDICT: 17 of 17 pass`. Findings by group (each read whole, each source row read at its cited
id by `rec.py get`):
- a11y-plan — 0
- architecture-1 — 1 · applied 1 (viola-agent-claude: an automatic verify refused `untrusted-folder` records nothing, dec-ixmg7xtr)
- architecture-2 — 2 · applied 2 (viola-state: `adopt`'s start among `check_home_tree`'s callers, dec-twgnxsmy; `check_diagnostics` holds the folder and file to the read rule, dec-yngykanm)
- architecture-3 — 2 · applied 2 (viola.md: `adopt`'s start writes the trust record, dec-wjypwipg; `open` among the `cli`-role verbs — `role_of` files every first word but run, revive, adopt, mcp and hook under `cli`)
- architecture-4 — 0
- architecture+design-system+layout-templates — 1 · applied 1 (frontend.md: `viola open --json`'s seven refusal codes, ctr-jayaxyks; the leaf had attributed `focus`'s six codes to `open` too) · 7 `@leaf_stands` read, none false
- architecture+security-plan — 1 · applied 1 (api.md: `untrusted-folder` in verify's `Ended::Refused` codes, ctr-si7jvqtu)
- architecture+test-plan+obs-plan — 0
- design-system+layout-templates — 1 · applied 1 (design-summary: the untrusted-folder refusal, ctr-xj7ijl6e)
- masters — 0
- obs-plan — 1 · applied 1 (obs-summary: the view restore adds no placement word, span field or log line, dec-kkpxti4a)
- security-plan-1 — 1 · applied 1 (security-summary: a send refused while the cut mark stands, ctr-fo4qskxo)
- security-plan-2 — 1 · applied 1 (security.md: verify's untrusted-folder refusal, dec-pv6qgtem) · 1 `@leaf_stands` read, true (the window step's switch back, dec-acn5sof6 r2)
- security-plan+obs-plan — 0 findings · 2 `@leaf_stands` read, none false
- test-plan-1 — 1 · applied 1 (testing.md: a verify test's failing row from a spine set, never a moved trusted root, dec-shiemenq)
- test-plan-2 — 0
- test-plan+obs-plan — 2 · applied 2 (observability.md: the view restore uninstrumented, dec-kkpxti4a; `viola up`'s other exit-1 details, ctr-pecbg5rc)
By hand: 0 · declined: 0. Tool line: `total · blocks 14 · applied 14`.
