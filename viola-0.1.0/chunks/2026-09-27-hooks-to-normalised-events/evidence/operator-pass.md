# Operator pass — 2026-09-27/28 (run by the session on the overseer's word, founder asleep)

Direction as given at `/andromeda-implement`: fold every red the operator pass finds into this chunk, and read CI through
`ci.py conclusion`. Order: entry 31 `pre-push` on the uncommitted tree (a red stops the pass) → entry 30 `gate.py hygiene` →
the pre-CI commit → entry 32's guarded push (fast-forward, never forced) → entry 33's CI read.

## Entry 31 — `bash scripts/agent-run.sh pre-push` (uncommitted tree) — GREEN
- 2026-09-27T23:09:24Z → 23:23:14Z (830 s), exit 0. Stdout was redirected to a file, never piped. `op-31.{out,start,end,rc}`
  sit in the implement run dir `.andromeda/runs/2026-09-27T21-48-40-implement/`; the document is one line with no host path.
- `ok:true`, `stage:"union"`, gate `ok:true`, breaches `[]`.
  - sync: 80 files, tree `0d39d1b61812a728ca437b4090237fa064229cc2` on HEAD `273e1ab`.
  - Linux: `coverage` 745/745, `playwright` 1/1, and a gate with `ok:true`.
  - windows-tests: `coverage` 759/759, gate ok.
  - Legs: `ubuntu-latest` and `windows-2025` each on base `273e1ab`, `counted`, 115 tested.
- The P2 fold is in this tree: the mutation run's own `target/mutants`
  (`evidence/red-mutants-stale-test-binary.md`). On the Windows leg the root-integration-only mutants of `src/cmd/hook.rs`
  now read `caught`. The first P2 pre-push had graded them MISSED on that leg, and only the union's ubuntu leg hid them.

## Entry 30 — `gate.py hygiene` — refused once, then CLEAN
- First read: `hygiene: refused 1 files — P1 0 · P2 0 · P3 1`. The row was `P3
  .andromeda/runs/2026-09-27T21-14-58-phase/control-seam/hook.rs (rust)`: the phase's known-positive control for entry 13,
  a one-line `.rs` file in a committed run dir.
- Folded on the overseer's word: the control keeps its content under `control-seam/hook.rs.txt`, so it is no longer a
  plane source file. The plan's entry-13 `baseline` still cites the old name (`control-seam/hook.rs`), and this line maps
  the two. The rename touches no file a test or gate reads; entry 13 greps `src crates tests`.
- Re-run: `hygiene: clean — read 40 (runs 37 · evidence 3) · trails 11 not read · binary 0 not read by P1`, exit 0.

## Pre-CI commit, entry 32 (push), entry 33 (CI read)
- Pre-CI commit **`bb35d6e`**: `chore(2026-09-27-hooks-to-normalised-events): operator pre-CI commit, for the run this chunk's
  verdict reads`. It was made with `git add -A` (83 files, 8275+/66−), and the tree was clean after it. New files read
  `i/lf w/lf`.
- Entry 32: `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` → exit 0 at
  2026-09-27T23:24:34Z. It was a fast-forward `273e1ab..bb35d6e`, never forced, and HEAD equals `origin/build/viola-0.1.0`.
- Entry 33: `ci.py conclusion --sha HEAD --wait 5400`, 23:24:41Z → 23:41:13Z, exit 0:
  `bb35d6e49327 verdict: green · checks 15/15 · wall 972 s · runs ci#36358593772 completed/success` (33 polls over 992 s).
- Jobs (`gh run view 36358593772 --json jobs`), every one `success`:
  - `test` on ubuntu-latest, macos-latest and windows-2025: the three-OS concurrent append
    (`hook_processes_append_whole_lines_side_by_side`), the hook suites and the coverage floors;
  - `mutants` on ubuntu-latest and windows-2025, plus `mutants-verdict` (the union);
  - `fuzz-replay` over the new `hook_stdin` corpus;
  - `supply-chain` (the fuzz lockfile audit), `msrv`, `lint` ×3 and `release` ×3.
- CI red-to-first-action: **not applicable** (no CI red). No fix commit was needed.
