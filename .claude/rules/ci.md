---
paths:
  - ".github/**"
---

# CI and Operator-Pass Rules

Path-scoped rules for the workflows directory and for the operator pass that takes a chunk's tree to CI. This file points to its masters and restates none of them: a workflow rule that is not here is read there.

**Authoritative sources (bound):**
- `.andromeda/security-plan.md` §Dependency Security, the **CI integration** paragraph — the pins, the permissions, the audits and what no workflow may do
- `.andromeda/obs-plan.md` §9 CI Integration, **Step order and conditions** — the gate steps and their conditions, copied into `ci.yml`
- `.andromeda/test-plan.md` §9 CI Integration — the jobs, the per-OS gates and the uploads

A workflow edit keeps all three true. A change to a pin, a permission, a step's order or a job's gate is an amendment of its master, made at the wrap, never an edit of the workflow alone.

## The operator pass
The pass runs after implement's report and before the wrap, in this order. The operator fires it, or the implementer does on the operator's word given for that chunk.
1. **The native gate:** `bash scripts/agent-run.sh pre-push` on the uncommitted tree (`.claude/rules/verification-harness.md`, Shims). A red stops the pass.
2. **Hygiene:** the plan's seeded `gate.py hygiene` entry reads `hygiene: clean`, and is read again right before the commit.
3. **The pre-CI commit:** the whole tree under the subject `chore(<marker>): operator pre-CI commit`, the prefix the gate tool finds the chunk's base by.
4. **The push:** the plan's guarded entry, `git diff --quiet && git diff --cached --quiet && git push origin HEAD`. Never a force push.
5. **The CI read:** `ci.py conclusion --sha HEAD --wait 1800` reads `verdict: green`; the run's `run_attempt` is read from the run itself, and the report names the run id.
6. **Fix commits on top:** a red is fixed by a new commit on the pre-CI commit, never an amend, and the pass repeats from step 1 for it.

Each entry's command, exit and atoms are recorded in the chunk's `evidence/operator-pass.md`. CI's run on the pushed sha is the verdict of record (`.claude/docs/workflow.md`, Before the push).

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run._
- 2026-10-04: Reading a `windows-mutants` dispatch while it runs: `gh run view --job <id> --log` refuses until EVERY job of the run has finished, so a finished job's log reads through `gh api repos/<owner>/<repo>/actions/jobs/<id>/logs --allow-escape-sequences` (without the flag it refuses the ANSI output), with the escapes stripped before grepping. Each job's harness document is its `{"v":1,"cmd":"run",…}` line, and the outcome lines carry every mutant's grade: read a MISSED line's cfg from source before calling it a survivor.
- 2026-10-04: A dispatched run joins its sha's checks: `ci.py conclusion --sha <sha>` after a `windows-mutants` dispatch reads the dispatch's six `mutants (…)` checks too (15 → 21 on the measured commit), so the push's CI verdict is read before the dispatch, and the dispatch's own verdict row with `--name mutants`.
- 2026-10-09: `gh run list --commit` takes the full 40-character sha: given an abbreviated one it prints an empty list with exit 0 while a run exists on that commit, which reads as "no run". Pass `$(git rev-parse HEAD)`, and never read an empty list from a short sha as an absence.
