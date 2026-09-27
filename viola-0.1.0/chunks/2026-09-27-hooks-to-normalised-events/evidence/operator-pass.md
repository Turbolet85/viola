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
