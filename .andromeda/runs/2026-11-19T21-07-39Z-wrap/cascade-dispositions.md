# Cascade dispositions — 2026-11-19-context-as-a-local-command (wrap 2026-11-19T21-07-39Z)

## Step 2 — the sweep
- Tool pattern files: none. Every `rows-land` dry run of this pass printed `patterns: none: {n} set(s) and retire(s)
  changed prose alone — no file` or `patterns: none — the call sets and retires no row` (architecture ×2, security-plan,
  design-system, layout-templates, test-plan, obs-plan, a11y-plan); step 3's leaves read each changed row's delta.
- Main's own patterns, `cascade-patterns.toml` (the OLD wording of the amended cells and their mechanism's phrasing):
  `m-except` (rig seeds config) · `m-turn-end` (after a turn end, the box row's cell attributes) · `m-echo-probe` (the
  exact verify echo probe command; /mcp only by a new stdout tag) · `m-verified` (stamped by viola verify prints;
  unstamped CLI build) · `m-unconfirm` (the ok-payload unconfirmable list). One call, `cascade.py sweep --patterns-file
  cascade-patterns.toml`, exit 0, part `sweep-pass-1.1.txt` read whole.
- Counts (its `total` and `per class` lines): `total (5 patterns) · 16 rows over 7 files` · `per class · new 0/0 ·
  standing 10/3 · leaf 6/4 · curation 0/0 · base 0/0`. `m-verified`: 0 rows, its control fired — no master or leaf
  holds the old verified / unverified-cli wording.
- Rows:
  - architecture.md:71, :969, :1039 `m-except` — amended (the three excepts, tx 3191; `edited`: the swept token stands
    inside the new text by design, «Test-only rig seeds private config»).
  - test-plan.md:38, :148, :271, :1093 `m-turn-end` — no change: each states «the box row's cell attributes, never text»
    with no timing, true for the hint cells read after the second send.
  - a11y-plan.md:233 `m-turn-end` — amended (tx 3197).
  - test-plan.md:856 `m-echo-probe` — amended (tx 3195).
  - architecture.md:429 `m-unconfirm` — amended (tx 3190).
  - leaf rows (step 3's set): `.claude/docs/a11y-summary.md:46` (stale: «after a turn end» alone) ·
    `.claude/rules/testing.md:38`, `.claude/rules/verification-harness.md:13`, `.claude/docs/tests-summary.md:50` (true
    as worded) · `.claude/rules/verification-harness.md:41`, `.claude/docs/tests-summary.md:21` (the fake's echo probe —
    the comparers read them against the new promise).
- Not looked for: the new rows' own wording (adds bring no old wording); a count — no leaf gains one (`inputs#I6`
  item 8).

## Leaves
- The set: `cascade.py leaves --add .claude/docs/services/viola-agent-claude.md`, pass mode, baseline afa8699c — `total · pass · 15 group(s) · 27 leaves · 439876 B · unmapped 2`; epoch open. `unmapped .claude/rules/host-linux.md (host)` — the host leaf, no comparer's; `unmapped .claude/rules/testing-src.md` — names no master in its provenance, no source changed for it: declined.
- The batch: 15 comparers, `check` 15 of 15 pass. The apply: `total · blocks 26 · applied 18 · held 8`.
- Per group (findings · applied · by hand · declined):
  - a11y-plan · 1 · 1 · 0 · 0
  - architecture-1 · 5 · 1 (#2, `local-command-context` beside its sibling rows) · 4 (#1, #3, #4, #5 written by hand without a count: «its closed rows», «in its order», «every one on every build», «Run B's compiled paste texts (the eighth `/context` …») · 0 — `inputs#I6` item 8: no leaf gains a count; the finding's chunk-marker change in #4 declined (the standing marker names when the shorter row set began)
  - architecture-2 · 2 · 1 (#2) · 1 (#1, «with compiled pastes») · 0
  - architecture-3 · 0
  - architecture+design-system+layout-templates · 0
  - architecture+security-plan · 1 · 1 · 0 · 0
  - architecture+test-plan+obs-plan · 1 · 0 · 1 (#1: the `/context` paste added, «with compiled pastes» without the count) · 0
  - design-system+layout-templates · 2 · 2 · 0 · 0
  - masters · 2 · 2 · 0 · 0
  - obs-plan · 0
  - security-plan · 7 · 5 (36#1, 72#2, 73#1, 73#2, security.md 17#2) · 2 (72#1 «compiled-literal pastes», security.md 17#1 «Run B's pastes are compiled literals») · 0
  - security-plan+obs-plan · 0
  - test-plan-1 · 3 · 3 · 0 · 0
  - test-plan-2 · 2 · 2 · 0 · 0
  - test-plan+obs-plan · 0
- Hand writes: drafted whole by a script (exact-string replacements, one hit each) and landed by `rec.py harness land` (tx 3210–3214): viola-agent-claude.md, viola.md, commands.md, security-summary.md, rules/security.md.
