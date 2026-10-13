# Curation — 2026-10-13-interrupted-verify-cleanup

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + ci.md: "Between the pre-CI commit and the push, write nothing into the tree — not even a run dir's journal line…"
    + ci.md: "A CI job that reads `cancelled` with no steps and no runner name never ran: read its cause from the check run's annotations…"
  Tier 3 (.claude/docs/session-learnings.md): none
  Extended: T2/testing.md: "2026-10-05: nextest pads a status line's duration…" + "an integration test's status line names its binary and its test with a space between"
  Corrected (in place, cap-exempt): ci.md 2026-10-04 dispatch-checks entry (nine → ten `mutants (…)` checks) · verification-harness.md 2026-09-24 `--in-diff` entry (nine → ten jobs)
  Filters: 3 dup · 1 task-specific · 0 conflict · 0 deferred · 1 below threshold (and 3 not a learning: rulings this chunk recorded)
  No-other-home: "write nothing into the tree between the pre-CI commit and the push" · "a never-acquired CI job's cause is in the check run's annotations" · "an integration test's nextest status line has a space between binary and test"
  CLAUDE.md size: 128/200 · T1 2.6 KB, 0 over 600 B

## Applied
- **ci.md**, the push after the pre-CI commit (confidence 0.8: measured gate failure +0.4, technical detail +0.2,
  no-other-home +0.2).
  Proof: entry 19's first firing read `red · exit 0 ✗ (exit 1) · history: unmoved`. A `journal.md` line written after
  commit `c22161e` had dirtied the tree; the entry was green once the tree was restored
  (`evidence/operator-pass.md` §4).
- **ci.md**, a never-acquired CI job (confidence 0.8: measured red +0.4, technical detail +0.2, no-other-home +0.2).
  Proof: ci#38453057445 attempt 1, job 115488618043, read `steps: 0`, `runner_name: ""`, log 404 `BlobNotFound`, and
  the annotation «The job was not acquired by Runner of type hosted even after multiple attempts». The re-run job
  115496299136 ran 30 steps green (`evidence/operator-pass.md` §5–5b).
- **testing.md, extended** (the additive facet of the 2026-10-05 status-line entry; confidence 0.8: measured false
  absence +0.4, technical detail +0.2, no-other-home +0.2).
  Proof: the first grep over the Ubuntu job log, `viola::(cli_verify|bin/viola )?[^ ]*verify_stopped…`, printed no
  stop-table line. The re-grep `viola::cli_verify verify_stopped…` printed both PASS lines (job 115488617646).
- **Correction, ci.md 2026-10-04 entry.** The sweep row `matrix-count` (`.claude/rules/ci.md:31`) read «nine
  `mutants (…)` checks». The workflow now holds ten items (`.github/workflows/windows-mutants.yml:48-50`).
- **Correction, verification-harness.md 2026-09-24 entry.** «nine of them» is now ten, by the same measurement.

## Decisions & corrections, each disposition
- The operator's ruling at implement (the companion workflow line) → not a learning: a decision this chunk recorded
  in `scope-record.md` and the report.
- The operator's ruling on the CI re-run (no-dispatch covers the mutation workflow) → not a learning: its scope is
  this chunk's re-run. The general form, re-running a job that was never acquired, is carried by the applied ci.md
  entry.
- The operator's wrap direction (entry 9 recorded, the guard not edited) → not a learning: this wrap's own
  instruction.
- The sweep hazard «a new `cfg(windows)` product file needs its `windows-mutants.yml` line; a plan guard freezing
  workflows contradicts it» → rejected, below threshold at exactly 0.6 (measured gate failure +0.4, technical detail
  +0.2). No conditional signal applies: this wrap amended the fact into architecture's CI/CD key («added … because
  the contract test below requires every product file with a `cfg(windows)` gate to be listed»), so the master owns
  it. The pipeline half (the plan template's guard form) is evolve telemetry (the fix-loop's
  `contract.jointly-contradictory-instructions` record), not a session learning.
- The sweep hazard «journal line dirtied the tree» → applied (ci.md).
- The CI reading «cancelled, steps 0, runner never acquired» → applied (ci.md).
- The sweep hazard «`[^ ]*` cannot cross the binary/test space» → applied as an extension (testing.md).
- The process attribution (a `sleep` from another project's session) → rejected, duplicate: host-linux.md
  2026-10-07, «decide whose a process is by `/proc/<pid>/exe` against the repository root», covers it.
- The guard refusal (the Bash guard's `cd` arm) → rejected, duplicate: host-linux.md §Paths already states that the
  Bash guard denies a `cd` that would move the cwd. It cost one re-issue, so it is not a recurrence.
- The remove-the-guard controls as a scripted neutralise / restore with a sha read-back → rejected, duplicate:
  testing.md 2026-09-25 (the remove-the-guard run and re-reading the guard's site before each neutralising edit).
- The operator's invocation directions held (`inputs#I1`) → rejected, task-specific: this chunk's directions,
  already the operator's standing practice in the handoff.
