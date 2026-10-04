# Session Handoff

**Last Updated:** 2026-10-04T04:24Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the pre-CI commit `60c569b` was pushed in the operator pass; this wrap's commit is pushed after this file is written)
**Status:** clean
**Last Commit:** 2026-10-04-windows-boundary-mutation-workflow — the wrap commit of Windows boundary mutation workflow

## Position
- Done: **2026-10-04-windows-boundary-mutation-workflow**:
  - `windows-mutants.yml`: dispatch-only, six-package `windows-2025` matrix, kept complete by
    `contract_windows_mutation_scope`;
  - the 34 owed coordinates graded on run 37174673472: 30 caught · 4 unviable · 0 missed;
  - the nextest `mutants` profile is now `terminate = "wait"` (a ratified reversal): the leak fell from 25 275 + 62 to
    38 + 0, accepted.
- Next: **Readiness gate and timing constants** (`working-route.md:72`) → `/andromeda-phase`. It now carries the
  `[inferred]` git-maintenance hypothesis for the 21 half-removed fixture repos, with a two-sided acceptance.

## Work done
- New `.github/workflows/windows-mutants.yml` and `tests/contract_windows_mutation_scope.rs`;
  `.config/nextest.toml` fail-fast → `terminate = "wait"`; the dispatch recipe in `.claude/docs/commands.md`.
- Evidence: `evidence/{guard,leak,operator-pass,windows-dispatch}.md`.

## Drift resolved
- **20 amendments:**
  - architecture: CI/CD approach, directory tree, Occupied Resources ×2 (7);
  - test-plan: §3 Bootstrap + Cleanup, §9 ×5, §10 (8);
  - security-plan: CI integration (3);
  - obs-plan: §9 Platform and Mutation row (2).
- **1 escalation resolved:** the `terminate = "wait"` reversal, ratified by the overseer as operator on 2026-10-04.
- **3 leaves re-derived:** `docs/workflow.md`, `rules/verification-harness.md`, `docs/tests-summary.md`.

## Notes
- **Epoch 3 boundary audit:** `working-route.md:82` carries the classification note. Unix-only twins make every
  dispatch read red. 25 survivors were listed, and one, `cleanup.rs:106:35`, needs a Windows test or an equivalence
  argument.
- **Epoch 3 holds 9 entries,** near the ~10 growth valve. A split is the operator's call.
- **`host-win32.md`** still describes the retired Windows host. Its replacement is an `/andromeda-setup-project`
  re-run, on the founder's timing (directive 5).
- **Operator cleanup left on disk** (none of it is needed by a future run):
  - `/tmp`: 21 `cargo-mutants-ws-*.tmp` dirs plus about 4 300 `.tmp*` dirs from the 2026-10-03 run 1;
  - `~/dev/projects/viola-mutants-scratch/`: the old `.tmp*` / `cargo-mutants-ws-*` / `rustdoctest*` residue
    (~8 GB), plus this chunk's `lw/` (the 38 witnessed leftovers) and the empty `leak-witness-2026-10-04/`.
  - Keep the scratch dir itself: it is NOCOW, and the next mutation run needs that attribute. Keep its subdir names
    short (`sun_path`).
- **Installed `claude` is 2.1.287.** Fixtures exist only for 2.1.283, so it stays unverified until `viola verify` runs.
- **Deferred learnings:**
  - `recurrence-despite-learning: host-win32.md 2026-09-28` — a heredoc append to a file was blocked again;
  - carried: the "not measured here" vocabulary; PID 1 as the cleanup-deadline target; the PTY master close needing no
    held clone; let a red CI run finish before folding its fix; the doubled-backslash guard recurrence.
- **Last failed command:** none.
