# Cascade dispositions — 2026-11-20-harness-prefix-and-identity-floor-ledger-rows (wrap 2026-11-21T08-50-56Z-wrap)

## Step 2 — the sweep
The search: `cascade.py sweep` over `cascade-patterns-architecture-1.toml` (arch1-1: the ten floor names the retired
`decision/identity-floor-eleven-names` held and no new cell holds), `cascade-patterns-test-plan-1.toml` (tests1-1:
`11-name`) and main's own `cascade-patterns.toml` (M-ELEVEN: the floor's old count in every spelling and the retired
handle; M-NOLIST: «no compile-time list», obs-plan's retired claim; M-MEASURED: «measured CLAUDE», the r8 floor's
retired qualifier; M-TWELVE: the canary row's old count). Dry runs with no file: architecture's ops file and `.2`,
security-plan's three files, obs-plan's two files — `patterns: none` (prose changes alone); none superseded.
Counts (the tool's lines): `total (6 patterns) · 15 rows over 8 files`; `per class · new 0/0 · standing 5/3 · leaf
10/5 · curation 0/0 · base 0/0`. M-NOLIST, M-MEASURED, M-TWELVE: 0 rows, each control fired (a statement about the
pattern).

Master rows (5) — each read in the part:
- security-plan `dec-rqiyphcq` (:485), `dec-rhjxynpo` (:517), `dec-qyk7bnwk` (:623) · arch1-1 — no change: true
  claims sharing the token (the secret names never logged / never output), not the retired floor list.
- test-plan `dec-lv7ik2go` (:52, edited) · arch1-1 — no change: this pass's own new text, which keeps
  CLAUDE_CODE_MESSAGING_TOKEN as an example; no duplicate of the retired count stands on the line.
- obs-plan `dec-6qg5hoi5` (:155) · arch1-1 — no change: the secrets-never-logged list, a true claim.

Leaf rows (10 over 5 files) — to step 3's set (`cascade.py leaves` reads them): `CLAUDE.md:36`,
`.claude/rules/security.md:6`, `.claude/docs/security-summary.md:26` (arch1-1: the NEVER-log secrets, true claims);
`.claude/docs/gotchas.md:93-94` (the 11-name floor), `.claude/docs/security-summary.md:71` (the 11-name floor),
`.claude/rules/testing.md:39` (the 11 identity-floor names) — stale figures the code states.

## Leaves
`cascade.py leaves` (pass mode, baseline b3eb184c; epoch «Epoch 4 — Driver reliability: ledger rows and bounds»
open, markerless 2): `total · pass · 12 group(s) · 24 leaves · 427439 B · unmapped 2`. One comparer a group, batch
`leaf`, `check` 12 of 12 pass. Apply: `total · blocks 17 · applied 17`, by hand 0, declined 0.
- architecture-1 · pass · 2 findings · applied 2 (`services/viola-agent-claude.md:28` the «(11, measured)» figure
  gone; `:26` the identity-floor relied row added).
- architecture-2 · pass · 2 · applied 2 (`services/viola.md:36` the capture arm's env-names file; `:6` the row).
- architecture-3 · pass · 0. architecture+security-plan · pass · 0. security-plan+obs-plan · pass · 0.
  test-plan+obs-plan · pass · 0.
- architecture+security-plan+test-plan+obs-plan · pass · 2 · applied 2 (`gotchas.md:93-94` the 11-name figures gone;
  CLAUDE.md read, no finding — its NEVER-log line holds).
- architecture+test-plan+obs-plan · pass · 2 · applied 2 (`commands.md:24-25` the env fixture and `env_probe`).
- obs-plan · pass · 1 · applied 1 (`obs-summary.md:69` names only for the env-names file).
- security-plan · pass · 4 · applied 4 (`security-summary.md:73` HF, HG; `:71` the 11-name floor gone, the four
  non-identity names; `rules/security.md:13`, `:6` the capture arm's env read and the stamp's env probe).
- test-plan-1 · pass · 3 · applied 3 (`rules/testing.md:39` the 11 gone; `tests-summary.md:21` ×2 the fake agent's
  env replay and the env fixture's hygiene class).
- test-plan-2 · pass · 1 · applied 1 (`rules/verification-harness.md:38` the fake agent's SessionStart env).
- unmapped: `.claude/rules/host-linux.md` (host — no master states it, no comparer's); `.claude/rules/testing-src.md`
  (no provenance header naming a master; its statements are the test suite's own, none on the floor, the capture arm
  or the stamp — read as not reached).
- not compared: none.
