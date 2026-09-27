# Host mutation scratch guard — remove-the-guard pair (plan step 7, overseer condition at P4)

## The one-time removal of the repo-root leftovers (plan step 7, operator step)

- Before: `mutants.out/` at the repo root held 7 560 311 bytes (`du -sb`, measured at implement 2026-09-27 ~15:05Z): `caught.txt`,
  `debug.log`, `diff`, `lock.json`, `log`, `missed.txt`, `mutants.json`, `outcomes.json`, `timeout.txt`, `unviable.txt`.
  `mutants.out.old/` held 7 156 549 bytes. Both are gitignored (`.gitignore:24` `mutants.out/`, `:25` `mutants.out.old/`; `git
  check-ignore -v`), 0 tracked files. They are residue of the pre-change runs, the latest being the 15:15 light gate.
- The implement session's `rm -rf mutants.out` was denied by the permission layer, so it was not retried.
- **Operator step:** the founder removed both, `! rm -rf`, at 17:19 local (15:19Z), relayed by the overseer.
- Verified after (implement session): `test ! -e mutants.out` exit 0; `test ! -e mutants.out.old` exit 0; `git ls-files mutants.out
  mutants.out.old` lists 0 files.
- From here on the gate entry `test ! -e mutants.out` proves that no run recreates it: after the scoped `run --mutants` entry,
  cargo-mutants' output lives in the sibling scratch.

Guard: `crates/viola-e2e/src/harness/run/mutants/scratch.rs` `scratch_allowed(root, scratch)`. It passes only when the
scratch's final component is exactly `viola-mutants-scratch` AND the repo root does not start with the scratch path (so the scratch
is neither the repo nor one of its ancestors). `prepare` refuses with `scratch-refused`, and never falls back to `%TEMP%` or the
repo. A drive or filesystem root has no final component, so the name check refuses it. A separate root check would repeat what the
name check already guarantees, so it would be an unkillable mutant (testing.md 2026-09-24, extended 2026-09-27). The refusal is
instead witnessed by its own cases: `/` in the case table, and `C:\` in `scratch_allowed_refuses_a_drive_root` (Windows).

Instrument: `bash scripts/agent-run.sh run --unit --filter 'package(viola-e2e) & test(/scratch/)'` (13 tests), this host, 2026-09-27.

## Guard neutralised (red)

`scratch_allowed` body replaced by `let _ = (root, scratch, OsStr::new(SCRATCH_NAME)); true`. The edit was confirmed on disk
(`grep -n -A3`) before the run.

- exit 1, `ok:false`, nextest-unit 10 passed / 3 failed:
  - `harness::run::mutants::scratch::tests::scratch_allowed_takes_only_the_named_sibling` FAIL
  - `harness::run::mutants::scratch::tests::scratch_allowed_refuses_a_drive_root` FAIL
  - `harness::run::mutants::scratch::tests::prepare_refuses_a_repo_whose_scratch_is_an_ancestor` FAIL

## Guard restored (green)

The original expression was restored and confirmed on disk. `git diff` of the file after the pair matches the pre-pair edit.

- exit 0, `ok:true`, nextest-unit 13 passed / 0 failed.

## What each refusal case covers

`scratch_allowed_takes_only_the_named_sibling` runs a labelled table: the accepted sibling `/w/viola-mutants-scratch`; refused
`/w/viola-scratch` (another name), `/w/viola-mutants-scratch-2` (a longer name), the repo itself, an ancestor of the repo, `/` (a
filesystem root), and `/w/viola-mutants-scratch/..` (a parent step, which has no final component). `rstest` is not a viola-e2e
dev-dependency, so the table replaces the plan's rstest cases; each case names itself in its assertion message.
