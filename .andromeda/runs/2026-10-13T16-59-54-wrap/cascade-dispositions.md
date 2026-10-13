# Cascade dispositions — 2026-10-13-epoch-4-survivors

The search: `cascade-patterns.toml` — four patterns derived from every amendment of this pass, each over the seven
masters, the registry files, the curation homes, the judgment bases and the leaves:
- `oldwatch` — the retired claim "the report file is named after the test alone": its tokens (`<test name>.report`,
  `<test>.report`, the `.test.report` siblings), its phrasing ("named after the test") and the directory name.
- `snapwriter` — the [Snapshot writer] loop: `replace_private_with`, `retry_replace`, `REPLACE_ATTEMPTS`,
  `replace_private_retrying`, "retried every 10 ms".
- `notmeasured` — the §10 Mutation gate's record form.
- `linuxtmpdir` — the mutation scratch / Linux `TMPDIR` the new §10 sentence sits beside.

Listing: `total (4 patterns) · 20 rows over 13 files` · `per class · new 0/0 · standing 9/5 · leaf 11/8 · curation
0/0 · base 0/0`. Every control fired (architecture.md:415, :51, :452; test-plan.md:1226).

## Rows
- `architecture.md:415` oldwatch (edited) — **amended**: the names now carry `<pid>`, so no `<test name>.report` form
  stands on the line (re-read: the one remaining hit is the directory name `viola-pty-watch`, a true claim).
- `.claude/docs/gotchas.md:111` oldwatch leaf ×3 — **re-derived** (step 3): it names `<test>.report` /
  `<test>.test.report`, stale.
- `.claude/docs/services/viola-pty.md:33` oldwatch leaf ×3 — **re-derived** (step 3): same stale names.
- `architecture.md:51` snapwriter (edited, ×6, @c496…1466) — **amended**: the added sentence names
  `replace_private_retrying` and the passed-in decision. Each other match (`REPLACE_ATTEMPTS` = 100, retried every
  10 ms, `retry_open` under the same bound) is a true claim, re-read in place, unchanged.
- `.claude/docs/services/viola-state.md:23`, `:24` snapwriter leaf — **re-derived** (step 3, the arch group): `:23`
  states the 100-attempt bound (true); whether the leaf names the restated loop is the comparer's.
- `test-plan.md:1226` notmeasured (edited, ×2) — **amended**: the Epoch 4 survivors' record was added; the two
  "not measured here" matches are the standing record form, true.
- `.claude/docs/tests-summary.md:42` notmeasured leaf — **re-derived** (step 3, the test-plan group).
- linuxtmpdir standing rows — `architecture.md:453`, `security-plan.md:607`, `test-plan.md:441`, `test-plan.md:1226`
  (edited: the new sentence sits beside the `TMPDIR` sentence, which stands), `obs-plan.md:973`,
  `registries/contracts/test-plan/5-command-implementation.md:45` — **no change**: each states the scratch or the
  Linux `TMPDIR` as it stands; none claims the report-file name or the two-job sharing.
- linuxtmpdir leaf rows — `.claude/rules/testing.md:48`, `.claude/rules/verification-harness.md:44`,
  `.claude/docs/commands.md:45`, `:47`, `.claude/docs/security-summary.md:73`, `.claude/docs/tests-summary.md:42` —
  **no change from this pattern** (true claims about the scratch). `tests-summary.md` and the rules re-derive in step 3
  as their groups hold them.

Not looked for: a restatement of the per-process report name in other words outside these tokens. The leaf comparers'
read is the control for reach.

## Leaves
`cascade.py leaves` (pass mode; `--add` gotchas.md, services/viola-pty.md, services/viola-state.md, tests-summary.md
from the rows above): `total · pass · 5 group(s) · 21 leaves · 225570 B · unmapped 2`. Snapshot pair: before `tree
f8042a3e…`, after `tree 1c2ecf50…`, exactly the five `A` lines, index and head unchanged.
- architecture-1 · pass · findings 2 · applied 2 · by hand 0 · declined 0 — `gotchas.md:111` and
  `services/viola-pty.md:33`: the per-process report names.
- architecture-2 · pass · findings 1 · applied 1 · by hand 0 · declined 0 — `services/viola-state.md:23`: the
  restated loop named (`replace_private_retrying`, the passed-in decision).
- architecture-3 · pass · findings 0.
- architecture+test-plan · pass · findings 0.
- test-plan · pass · findings 1 · applied 1 · by hand 0 · declined 0 — `tests-summary.md:42`: the Epoch 4 survivors'
  disposition.
- Unmapped: `.claude/rules/host-linux.md` (the host leaf, no source) and `.claude/rules/testing-src.md` (a pointer
  list to testing.md, no master statement): no comparer.
- `cascade.py leaves --apply`: `total · blocks 4 · applied 4`, each leaf written and read back.
- The verification-harness rule's 2026-09-27 Session Additions entry (the filter form) is a preserve-verbatim
  curation home: P3's, not this step's.
