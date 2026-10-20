# P5 controls — 2026-10-20-epoch-5-cleanup (validation check 4 (9))

The clone: `git clone -q --shared` of the project at HEAD `d35644a58909aec12c4898e5a066bcfca5a8ebd2`, at
`$TMPDIR/{scratchpad}/p5-clone`, the chunk folder copied in (the guard reads its plan). Inputs planted by
`$TMPDIR/{scratchpad}/p5_controls.py` (the clone only). Each entry fired AS ITSELF through `gate.py run --plan
{clone plan} --entry {n}` / `--only 12,13,15 --project-root {clone}`, no run dir.

## Unmodified (must read what the tree read)
- 12 · `red · exit 1 ✗ (exit 0) · exit 0 · 2 B` — the literal line present, as on the tree
- 13 · `red · exit 1 ✗ (exit 0) · exit 0 · 431 B` — the eight web `paths:` lines, as on the tree
- 15 · `green · exit 0 · 316 B` — an empty change, as on the tree (vacuous by itself)

## Must-pass inputs (`p5_controls.py {clone} pass`)
Input: `tests/support/home.rs`'s `let passed = if cfg!(unix) { 20 } else { 17 };` replaced by `let passed = 0;`; the
`paths:` list items naming `e2e-web/` or `crates/viola-ui/` dropped from `a11y.md` (5), `api.md` (1), `frontend.md` (1),
`testing.md` (1).
- 12 · `green · exit 1 · 2 B` — count 0
- 13 · `green · exit 1 · 0 B` — no output
- 15 · `green · exit 0 · 342 B` — an allowed file changed (`tests/support/home.rs`), `guard: clean`

## Must-fail input (`p5_controls.py {clone} fail`, on top of the pass inputs)
Input: `src/main.rs` edited (one comment line appended), `src/new_file.rs` created untracked.
- 15 · `red · last line 'guard: clean' ✗ (last: 'guard: outside 2 files')` — both counted

## The by-name filter entries (9, 11): red on the tree, the form controlled
Typed beside the entry, differing by the pattern operand alone, as the tool types an entry (`/usr/bin/bash -o pipefail
-c '{command}'`), on the real tree:
- `bash scripts/agent-run.sh run --integration --filter 'test(/home_base_backing_plain_base_is_created_and_kept/)'` →
  exit 0, `"ok":true`, nextest-integration passed 1 (1 test run: 1 passed, 397 skipped)
- `bash scripts/agent-run.sh run --e2e --filter
  'test(/mutants_verify_window_override_stands_before_the_harness_package_override/)'` → exit 0, `"ok":true`,
  nextest-e2e passed 1 (1 test run: 1 passed, 171 skipped)

## A deviation, recorded
The unmodified firing of 12 / 13 / 15 in the clone was read through a `| grep` stage over the tool's stdout, against
the letter's run-bare rule; every later firing ran bare. The three lines it showed are the tool's own entry lines,
copied above unedited.
