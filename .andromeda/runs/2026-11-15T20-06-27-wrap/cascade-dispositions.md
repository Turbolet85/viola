# Cascade dispositions — 2026-11-15-mutation-and-coverage-baselines-green (wrap 2026-11-15T20-06-27-wrap)

## Step 2 — the sweep
patterns: none — each `rows-land` dry run printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no file`
(architecture 1 · test-plan 3 · test-plan main 1); no file holds a pattern, so the pass sweep is skipped and step 3's
leaves read each changed row's delta.

The search that stands in for it: Phase 1's pre-pass sweep (`report-patterns.toml`, trail
`cascade-2026-11-15-mutation-and-coverage-baselines-green-pre-pass.json`, part `sweep-pre-pass-1.1.txt`, read whole):
`total (6 patterns) · 84 rows over 14 files` · `per class · new 0/0 · standing 38/4 · leaf 21/8 · curation 23/3 · base 2/1`.
The patterns: `mutants-kill` · `within-bound` · `mutants-floor` · `socket-root` · `tmpdir-length` · `record-home`
(`root-wait-count` refused unfired: no master holds it). What was NOT looked for: wordings of the kill or the bound that
name neither the figure with «kill»/«wait bound» nor the tokens `WITHIN`, `slow-timeout`, `terminate-after`.

Standing (master) rows, 38 over 4 masters:
- amended (tx 1608–1610): `dec-wc3cidpf` architecture decision/root-waits-share-one-bound · `dec-fbchz5ku` test-plan
  decision/root-wait-bound · `dec-emlxz4li` decision/within-7s-wait-bound · `dec-emfw6k27`
  decision/mutants-kill-below-cargo-mutants-timeout · `dec-elfq6uya` decision/send-window-paste-hint-30s-kill.
- added beside: architecture decision/test-sockets-under-short-tmp-root (the `socket-root` add-to on `nam-zt67rtxw`,
  which stays true as a name); test-plan decision/record-home-names-no-home (the `record-home` fact).
- no change, true claim sharing the token: `dec-emh2empl` · `dec-enijixl3` · `dec-elxnaqiu` (mutants-kill: other
  profile settings and overrides, unchanged) · `ctr-gmpn3ck7` (within-bound: the MCP wait tool's product bound) · 19
  `mutants-floor` rows other than the two amended (the cargo-mutants tool, its pins, runs and outcomes; the 20 s floor
  unchanged) · `nam-zt67rtxw` (the socket name, unchanged) · `dec-w6hzw6ly`, `dec-re6on572` (tmpdir-length: the
  mutation scratch dir, the product socket) · `dec-x23lxz5g`, `dec-udvyz5rj`, `dec-shf3zo3r`, `dec-nvdmm3x2`
  (record-home: the product's `verify --record`).
- curation rows (23 over 3 files) → P3: `.claude/rules/testing.md:55` (the `mutants` profile's 10 s kill) and `:61`
  (a CI runner's 10 s kill, «never raise») are stale against the 15 s default; `.claude/rules/verification-harness.md:62`
  (the `TMPDIR` length rule) binds no test socket now. The other curation rows: true, no change.
- base rows (2 over 1): `rows:rule:43` (the «9 root waits» provenance note of rule rul-yzz4mbxd — history, no claim
  of the bound) · `rows:rule:35` (a `TMPDIR` carve-out named in a rule's provenance) — no change.
- leaf rows (21 over 8 files) → step 3's set.

## Leaves
`cascade.py leaves` (pass mode, epoch open, baseline 73b06303; `--add` gotchas.md · verification-harness.md ·
testing.md · tests-summary.md · commands.md from the pre-pass leaf rows): `total · pass · 7 group(s) · 20 leaves ·
314419 B · unmapped 2`. Unmapped: `.claude/rules/host-linux.md` (the host's leaf — no source states it, no comparer's)
· `.claude/rules/testing-src.md` (names no amended master) — no comparer, no change.
Apply dry run: `total · blocks 3 · applied 3 — dry-run, nothing written`.
- architecture-1 · pass · 0 findings · 0 applied · 1 by hand: `.claude/docs/gotchas.md:139-141` — «kills a root test at
  10 s», «(5 s × 2), which equals `CONFIRM_WINDOW_FALLBACK`», «against the 10 s kill» stale against test-plan
  decision/mutants-kill-below-cargo-mutants-timeout (15 s); the comparer was silent (the leaf maps to architecture
  alone, the change is test-plan's); drafted whole, landed by `rec.py harness land` tx 1613.
- architecture-2 · pass · 1 finding · 1 applied (`viola-channel.md:41#1`, omission: the test socket's short /tmp root).
- architecture-3 · pass · 0 findings.
- architecture-4 · pass · 0 findings.
- architecture+test-plan · pass · 0 findings.
- test-plan-1 · pass · 1 finding · 1 applied (`testing.md:34#1`, omission: the record-home rule).
- test-plan-2 · pass · 1 finding · 0 applied · 1 declined: `verification-harness.md:42#1` — the same record-home rule
  already lands in `testing.md`, its one leaf home; a second copy is detail the master holds.
Applied by tool: `total · blocks 2 · applied 2`. Also stale and not a leaf's: `.claude/rules/testing.md:55,61` and
`.claude/rules/verification-harness.md:62` sit in `## Session Additions` (preserve-verbatim) → P3.
