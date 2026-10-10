# Cascade dispositions — 2026-10-10-windows-mutation-grade

The search: `cascade.py sweep` over `cascade-patterns.toml`, fired once after every body amendment of the pass but
one (O3, below). From the listing: `total (15 patterns) · 32 rows over 13 files` and `per class · new 2/2 · standing
16/7 · leaf 12/6 · curation 2/2 · base 0/0`. Every pattern's control fired on the pre-pass masters (baseline
`781563cd`).

What was looked for: the retired job shape (`six-package`, `mutants (<package>)`, "per package", "each package's"),
the retired "jobs read red until / while" claim and its `cfg(unix)` twins, the retired `survived` formula
(`missed + timeout`), the bare "missed or timed-out" and `missed == 0` conditions, "surviving mutant", the first
run's reading (`9 of 9 caught`, `37174673472`), "union verdict", the owed entry's name, the dispatch-timing wording,
and "exit code alone". What was not looked for: a restatement of the job shape that uses none of these words (the
seven detectors read their masters whole and named none), and any claim about mutation in a sidecar, an archive or a
chunk folder, which the cascade never opens.

Zero-row patterns after the pass, each with its control fired: `six-package`, `read-red`, `survived-sum`,
`missed-or`, `nine-of-nine`, `union-verdict`. Their wording stands in no master, key file, leaf or curation home.

## Rows

Masters and key files (new 2, standing 16):

- `.andromeda/test-plan.md:1212` `twins` standing edited · `first-run-id` new · `missed-zero` ×3 standing edited ·
  `owed-entry` standing edited → amended (T7, T8, T9, T13, O1). Re-read at offsets 368, 1958, 2294, 3195, 3730 and
  3767: the three `missed == 0` are the amended requirement, the boundary tier's "the same", and the overseer's
  ruling, which is not reworded; the twins and the run id stand in the dated history of the first run.
- `.andromeda/obs-plan.md:1013` `twins` new → this pass's own text (B1).
- `.andromeda/obs-plan.md:1105` `surviving` · `dispatch-timing` standing edited → amended (B3).
- `.andromeda/test-plan.md:182`, `:413`, `:414` `surviving` standing → no change: Founder Direction 1's words
  ("surviving mutants are red") and the §1 trigger; true under the harness document's `survived`.
- `.andromeda/architecture.md:443` `first-run-id` standing → no change: a dated reading of `scratch_bytes` in the
  first run.
- `.andromeda/security-plan.md:351` `dispatch-timing` standing edited → amended (O2: the exception named after the
  ruling's sentence, which is not reworded).
- `.andromeda/registries/contracts/architecture/ci-cd-approach.md:2` `dispatch-timing` standing edited → amended
  (A1, A3, O2), read at offset 1051.
- `.andromeda/test-plan.md:1247` `dispatch-timing` standing edited → amended (T10).
- `.andromeda/test-plan.md:1136` · `.andromeda/obs-plan.md:993` `dispatch-timing` standing → no change: each names
  the workflow as the boundary audit's Windows leg, which it is.
- `.andromeda/registries/contracts/architecture/project-directory-structure.md:113` `dispatch-timing` standing →
  no change on that line (the ruling's timing); the line below it is amended (A2).
- `.andromeda/registries/contracts/test-plan/5-command-implementation.md:47` `exit-code-alone` standing edited →
  amended (T1); "The exit code alone is not trusted" stands and is true.

Leaves (12 rows over 6 files):

- `.claude/rules/verification-harness.md:44` `per-package` → re-derived: per matrix item, nine jobs, the labels; and
  the host exclusion with `host_excluded` stated in the `run --mutants` paragraph.
- `.claude/docs/commands.md:47` `per-package` ×2 · `dispatch-timing` ×2 → re-derived: nine `mutants (<label>)`
  jobs, the harness document judges a job. "Dispatched only during the epoch-boundary audit" stands.
- `.claude/docs/commands.md:30`, `:44` `dispatch-timing` → no change: true.
- `.claude/docs/tests-summary.md:42` `per-package` · `missed-zero` · `owed-entry` · `dispatch-timing` → re-derived:
  the gate reads after the exclusion, the two `prepare` mutants measured, the nine jobs and run 38036448183.
- `.claude/rules/testing.md:48` `exit-code-alone` → re-derived: "zero missed" reads beyond the host-excluded
  mutants. "Never the exit code alone" stands.
- `.claude/docs/gotchas.md:123` · `.claude/docs/services/viola.md:29` `exit-code-alone` → no change: another claim
  sharing the words (`Wrapper::stop` waits for the endpoint, not the exit code).

Leaves by provenance, beyond the rows: `.claude/docs/stack.md` (the verbatim §Stack mirror, the syn and proc-macro2
text, and its Dev-deps line) re-derived. `.claude/docs/security-summary.md`, `.claude/rules/security.md`,
`.claude/docs/obs-summary.md`, `.claude/rules/observability.md`, `.claude/docs/a11y-summary.md`, `.claude/docs/workflow.md`
and CLAUDE.md's generated blocks were read for mutation statements (`grep -n -i 'mutation\|mutants'`): none states
the job shape, the old verdict rule or the first run's reading, so none changes.

Curation homes (2 rows):

- `.claude/rules/verification-harness.md:56` `per-package-job` (the 2026-09-24 entry's 2026-10-04 correction names
  `mutants (<package>)` jobs) → curation home — P3, an in-place extension.
- `.claude/rules/testing.md:74` `dispatch-timing` → no change: true.

Judgment bases: 0 rows.

## After the sweep

- O3, authored after the sweep and adding text only: `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md`,
  the `viola-e2e` dependency line gains syn and proc-macro2. It retires no wording, so it adds no pattern;
  `proc-macro2|proc_macro2|\bsyn\b` read 0 hits over the masters and key files before the pass.
- `.claude/rules/ci.md:31` (a Session Additions entry: "the dispatch's six `mutants (…)` checks too (15 → 21 on the
  measured commit)") carries the old job count under none of the swept words. It was found by a read of the leaf
  lines naming the workflow, and goes to P3 as an in-place extension.
