# CARRY 4 — the half-removed fixture repos, two-sided witness

Measured 2026-10-04 at /implement on the Linux dev host (git 2.55.0, btrfs), under the plan's step 12 economy clause
(overseer at P5): a smaller witness that shows ≥ 1 half-removed repo on the CONTROL side stands in for the two 78-min
`run --mutants --package viola-e2e` runs, run identically on the FIX side.

## Why the smaller witness stands in
The leftovers in `chunks/2026-10-04-windows-boundary-mutation-workflow/evidence/leak.md:74-101` are `Pass`-fixture
repos (`crates/viola-e2e/src/harness/run/mutants/base.rs`). The `Pass` fixture's own tests,
`harness::run::mutants::base::tests` (14 tests, each committing into one or more throwaway repos), drive exactly that
fixture's `git add` / `git commit` and its `TempDir` drop — the code path the hypothesis names — without cargo-mutants'
copy and rebuild around it. The control side reproduced the leak at 4 and 5 per 200 rounds on an idle host, so the
two-sided shape holds on this witness (control ≥ 1, fix 0, same witness both sides).

## The two trees
Both are copies of this chunk's working tree (`rsync` without `target/`, `.git`, `node_modules`), each built in its own
`target/`, under the NOCOW host scratch `../viola-mutants-scratch/` (`lsattr` reads `C`):
- `c4fix` — the chunk's tree as is: `maintenance.auto=false` in `GIT_CONFIG` (`run.rs`, the three throwaway-repo git
  calls) and in `Pass::commit` (`base.rs`);
- `c4ctl` — the same tree with step 11 reverted: `diff -r` of the two `crates/` trees shows only the two
  `"-c", "maintenance.auto=false"` pairs (and `GIT_CONFIG`'s length 6 → 4).

## The witness
Per side, a fresh short NOCOW `TMPDIR` (`../viola-mutants-scratch/w{side}[2]`, under 60 B as an absolute path), then
`rounds` × `cargo nextest run -p viola-e2e --lib -E 'test(/harness::run::mutants::base::tests::/)' --test-threads 64
--no-fail-fast` with `TMPDIR` and `CARGO_TARGET_DIR` set per call; then, in that `TMPDIR`, count the `.tmp*` dirs and
those holding `a.rs` + `.git` without `.git/HEAD` (the leak.md:76-81 signature).

| order | side | rounds | red rounds | seconds | leftover `.tmp*` dirs | half-removed (signature) |
|---|---|---|---|---|---|---|
| 1 | ctl | 200 | 0 | 57 | 6 | **4** |
| 2 | fix | 200 | 0 | 55 | 0 | **0** |
| 3 | fix | 200 | 0 | 55 | 0 | **0** |
| 4 | ctl | 200 | 0 | 57 | 5 | **5** |

A 3-round control trial before row 1 left 0 (its empty dir removed before row 1).

The control leftovers carry the leak.md content: `.git/objects/pack/tmp_idx_*` and `tmp_rev_*` (one repo), or
`COMMIT_EDITMSG`, `index`, `logs/`, `MERGE_RR` and `objects/pack/` with no `HEAD`; one further leftover held only
`.git/objects/pack/` (not counted: no `a.rs`).

## Verdict
**Pass** — fix 0 and 0, control 4 and 5, in reverse-order pairs. The `[inferred]` mechanism (a detached
`git maintenance run --auto` writing into the repo while the `TempDir` drop walks it) is now supported two-sided: the
one change between the trees is `maintenance.auto=false`, and it removes the leak on this witness. What is still not
measured: the full 78-min mutation run under the fix (the economy clause's stand-in replaces it), and the
Windows host (the `windows-2025` runner deletes read-only git objects too, testing.md 2026-10-03, but no run there
counted leftovers).

The four witness dirs and the two tree copies are left in the scratch (`w{ctl,fix}`, `w{ctl,fix}2`, `c4ctl`, `c4fix`)
for the operator's cleanup; none is needed by a later run.
