# Cascade dispositions — 2026-10-13-epoch-4-survivors-ii

## Step 2 — the sweep

`cascade.py sweep --patterns-file cascade-patterns.toml` over the seven masters, `.andromeda/registries/**`, the
three curation homes, the two judgment bases and the leaf bodies. Baseline: `64d7e5f7`, the pre-CI parent.

The amendment: test-plan §10 Mutation gate. Was "the 16 in `src/cmd/verify/typed.rs` … are owed to the route entry
"Epoch 4 survivors II". The Epoch 5 boundary audit's own run is the reader that confirms the 29 kills". Now the 16
were disposed by this chunk, by restatement and cases in `typed.rs`, none owed, and the audit confirms 45 kills
(29 + 16).

The patterns, each spanning the claim's own words and its verbs:
- `e4s2`: the route entry's name;
- `owedto`: the retired verb, "owed to (the route entry) Epoch 4 survivors";
- `confirms29`: the retired count, "confirms the 29 kills" / "29 kills";
- `runbsyms`: the restated or removed symbols `framing_turns` · `SharedPty` · `wait_pump` · `end_by_ctrl_c`.

Every control fired at `test-plan.md:1226`.

total (4 patterns) · 3 rows over 2 files · per class · new 0/0 · standing 1/1 · leaf 2/1 · curation 0/0 · base 0/0

- `.andromeda/test-plan.md:1226` runbsyms standing edited mixed ×3 (@c4168, 4187, 4604) · **no change**. @4168
  (`framing_turns` 5) and @4187 (`SharedPty` 5) are the audit's labels for the 16 survivor sites, as the audit
  recorded them at `dffad3e`, and stay true as the names of those sites. @4604 is this pass's new text,
  "`SharedPty` generic over its `Pty`", which is current truth. Re-read whole: the line holds no duplicate of the
  retired "owed" or the "29 kills" count.
- `.claude/docs/tests-summary.md:42` e4s2 leaf @c1964 · **re-derived** in step 3. The leaf states the 16 as "owed to
  the route entry "Epoch 4 survivors II"".
- `.claude/docs/tests-summary.md:42` owedto leaf @c1939 · **re-derived** in step 3. This is the same clause.

Masters other than test-plan: 0 rows for every pattern. Registries: 0. Curation homes: 0. Judgment bases: 0.
`confirms29`: 0 rows with its control fired, so the retired count stands nowhere after the apply.

## Leaves

`cascade.py leaves` (pass mode, `--add .claude/docs/tests-summary.md`): total · pass · 2 group(s) · 6 leaves · 87814 B
· unmapped 2. Comparers: one per group, in one batch inside one snapshot pair. The diff-tree printed exactly the two
`A …/leaf-test-plan-{1,2}.md` lines; index and head unchanged.

- test-plan-1 · pass · leaves `.claude/docs/commands.md` · `.claude/docs/tests-summary.md` · `.claude/rules/observability.md`
  · `.claude/rules/testing.md` · findings 1 · applied 1 · by hand 0 · declined 0. The one finding is
  `tests-summary.md:42`, stale: the 16 "owed to the route entry "Epoch 4 survivors II"". It now reads disposed at this
  chunk, none owed, with the audit confirming the 45 kills. Its source was checked at `test-plan.md:1226`.
- test-plan-2 · pass · leaves `.claude/rules/verification-harness.md` · `CLAUDE.md` · findings 0.
- unmapped `.claude/rules/host-linux.md` (host): the host leaf, whose statements no master states, so no comparer.
- unmapped `.claude/rules/testing-src.md`: it holds pointers into `testing.md` only, states no master claim, and the
  amendment touches none of its pointers, so no comparer.

total · blocks 1 · applied 1 (`cascade.py leaves --apply`).

Lateral binds: test-plan §3 ↔ obs-plan §3. §3 was not amended (only §10), so the bind is untouched. The a11y ↔ obs
schema: untouched.
