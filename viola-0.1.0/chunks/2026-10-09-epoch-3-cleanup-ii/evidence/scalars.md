# The two audit scalars, before and after — 2026-10-09-epoch-3-cleanup-ii

Instrument: `evidence/scalars.py` (this chunk's), which runs the Epoch 3 audit record's own commands
(`.andromeda/runs/2026-10-08T10-08-51-code-audit/record.json`, `commands.duplication` and `commands.complexity`,
the population rule of `commands.sizes`) into a temporary directory it makes and removes, and summarises them as the
audit's `a13.py` and `cx.py` do (closures included). Tools on the host: `jscpd` 5.0.16, `rust-code-analysis-cli`
0.0.25, the audit record's versions.

One reading differs from the audit's letter and is stated here: the population is every `*.rs` file git lists on the
working tree, tracked or not yet tracked, outside the audit's excluded top-level directories. The audit read
`git ls-files "*.rs"`, which leaves a file out until it is committed; this chunk adds two files, and they are in the
population from the edit on. On the untouched tree both rules read the same 124 files.

## The three earlier records and this chunk's readings

| Reading | Tree | `duplication.pct` | `complexity.over_ceiling` |
|---|---|---|---|
| Epoch 2 boundary record | — | 1.29 | 0 |
| Epoch 2b boundary record | — | 1.35 | 1 |
| Epoch 3 boundary record (2026-10-08, `e304994`) | — | 3.13 | 4 |
| **before** (this chunk, 2026-10-09T22:04Z) | HEAD `14f1fb5`, no chunk edit | 3.36 | 4 |
| **after** (this chunk, 2026-10-10T01:07Z) | HEAD `14f1fb5` plus every edit of the chunk | 2.84 | 1 |

Both scalars moved in the asked direction and below the audit's own readings: duplication under the bound the plan
set (below 3.13), `over_ceiling` at the one function left standing.

## After — the three lines as printed (2026-10-10T01:07:35Z, the tree the three witness runs measured)

```
duplication: pct 2.84 · duplicated 1533 of 53987 · clones 187 · named fragments 0
complexity: over_ceiling 1 · dialog_variants 10 · record 2 · submit 11 · files 126 of 126
selftest: 0 mismatches · 9 duplication cases · 6 complexity cases
```

- All three exit 0. The same two graded lines were read once before the witness runs (2026-10-09T22:16:47Z), with
  the same values.
- Duplication: 1814 → 1533 duplicated lines (−281), 205 → 187 clones, of 53968 → 53987 lines; named fragments
  7 → 0. The plan predicted about 2.7 to 3.0.
- Complexity: `dialog_variants` 19 → 10, `record` 17 → 2, `submit` 16 → 11; no function of `ledger.rs`,
  `src/cmd/verify.rs` or `viola-fake-agent.rs`, the split-out helpers included, reads over 15; 126 files (124 and
  the two new support files).

## Before — the three lines as printed (2026-10-09T22:04Z, HEAD `14f1fb5`, no chunk edit in the tree)

```
selftest: 0 mismatches · 9 duplication cases · 6 complexity cases
duplication: pct 3.36 · duplicated 1814 of 53968 · clones 205 · named fragments 7
complexity: over_ceiling 4 · dialog_variants 19 · record 17 · submit 16 · files 124 of 124
```

- `selftest` exit 0. `duplication` exit 1 and `complexity` exit 1: both graded verbs read red on the untouched
  tree, which is each one's must-fail reading on real input.
- The values are research.md's (3.36 %, 1814 of 53968, 205 clones; 4 over, at 19, 17, 16 and `sgr_attributes` 23).
- A named fragment is a clone pair of 20 lines or more either between two different files of the eight lifted
  test files or inside `tests/cli_verify.rs`: 7 at the base commit (five cross-file, two in `cli_verify.rs`).
- These readings were taken while the `viola-e2e` score (the run journal's run 1) was in flight; the two tools
  read source text only.

## Standing

`sgr_attributes` (`tests/cli_output_plain.rs:36`, cognitive 23) is not split by this chunk. It stood over the
ceiling at the Epoch 2b baseline too, and it is the one function `over_ceiling` is expected to count after this
chunk.

## Test counts (acceptance: none dropped)

| Reading | Tree | unit | integration |
|---|---|---|---|
| before | `f0a0dd1`, the last wrap's light gate (`run`, 2026-10-09T21:09Z) | 1471 | 353 |
| after | HEAD `14f1fb5` plus every edit of the chunk (`run`, 2026-10-09T23:34Z) | 1484 | 354 |

After: `run --unit` read 1484 of 1484 and `run` read unit 1484, integration 354, both `"ok":true`, exit 0, before
the first witness run. The difference is the fourteen new cases and nothing else: +13 unit (viola-state 5,
viola-agent-claude 4 of which two are added rstest cases, the fake agent 3, `src/cmd/hook.rs` 1) and +1 integration
(`tests/cli_wait_last.rs`). No test was renamed, merged, split or removed. The gate block's own `run` entry re-reads
both counts; its reading is in the run dir's gate trail.

Basis for "before": the base commit `14f1fb5` differs from `f0a0dd1` in no `.rs`, manifest, fixture or nextest
file (`git diff --stat f0a0dd1 14f1fb5`: a setup run dir, `.claude/rules/host-linux.md`, `.claude/settings.json`,
`CLAUDE.md`), so the last wrap's count is the base commit's. It was not re-run on the base commit by this chunk: a
full suite may not run beside the mutation score.
