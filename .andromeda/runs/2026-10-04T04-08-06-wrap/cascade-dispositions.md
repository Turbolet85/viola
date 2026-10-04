# Cascade dispositions — 2026-10-04-windows-boundary-mutation-workflow

## The search
- Pattern set: `cascade-patterns.toml` (7 patterns), run by `cascade.py sweep` after every body amendment of the pass
  (A1–A7, T1–T8, S1–S3, O1–O2). Baseline: `7aca5587`, the pre-CI commit's parent. Every control fired. The
  first attempt's `immediate-kill` pattern never fired on the pre-pass masters, was dropped, and was replaced by
  `timeout-misgrade`, the retired reason's own phrasing.
  - `two-workflows` (`two workflows`) — the retired count;
  - `neither-wf` (`Neither (workflow|\`ci.yml\`)`) — the two-workflow restatement;
  - `no-mut-job` (`run(s) no mutation job | no CI (mutation )?job | none in CI | no CI job runs mutation`) — the
    retired absolute claim, in all its spellings;
  - `ci-nightly-pair` — a two-file workflow enumeration closing a parenthesis;
  - `two-low` (`(2 low`) — the retired zizmor count;
  - `term-immediate` (`terminate = .immediate`) — the retired profile value;
  - `timeout-misgrade` (`waits for the running tests | graded? Timeout`) — the retired reason's mechanism.
- Sections also read by hand: architecture §Occupied Resources rows :379 / :433 · test-plan §9 (:1133–1170) and §10
  Mutation gate (:1209) · security-plan §Dependency Security, CI integration (:341–361) · obs-plan §9 (:993, :1013) ·
  the two test-plan key files · the CI/CD and directory-structure key files.
- Leaves by provenance: every `.claude/docs/*` and `.claude/rules/*` line naming `nightly.yml`, mutation, mutants,
  terminate, concurrency or workflow (`grep -nE` over CLAUDE.md, rules, docs).

## Rows (9) and their disposition
| row | pattern | disposition |
|---|---|---|
| `architecture.md:379` (new) | no-mut-job | amended text: "`ci.yml` and `nightly.yml` run no mutation job" — a true, scoped claim. No change. |
| `security-plan.md:361` (standing, edited) | no-mut-job | amended line: "`ci.yml` runs no mutation job" — true (ci.yml carries none). No change. |
| `registries/contracts/architecture/ci-cd-approach.md:5` @c2268 (standing, edited) | no-mut-job | amended: "`ci.yml` runs no mutation job …, whose Windows leg is the dispatch-only `windows-mutants.yml`" — true. No change. |
| `registries/contracts/test-plan/5-command-implementation.md:152` (standing) | no-mut-job | "No CI job gates `mutants`" (window read at its offset): true — the workflow runs no `gate` step and is never a required check. No change. |
| `.claude/docs/workflow.md:10` (leaf) | no-mut-job | stale ("CI runs no mutation job"). **Re-derived** from arch §CI/CD: no push/PR mutation job; the dispatch-only Windows leg named; "No workflow has a `concurrency:` block". |
| `bootstrap-phases-derive-for-route-setup-project.md:14` (standing, edited) | term-immediate | amended line quoting `immediate` as the witness's control side ("170 … under `terminate = \"immediate\"` against 0 under `wait`") — a measurement citation, not a claim of the current value. No change. |
| `bootstrap-phases-…:12` (standing, edited) | timeout-misgrade | amended: "is never graded Timeout" — the new truth. No change. |
| `bootstrap-phases-…:17` (new) | timeout-misgrade | amended history sentence ("`immediate` had replaced a plain `fail-fast = true` … graded Timeout"). Re-read: its first draft misattributed what was replaced and was corrected in this pass. No change after the fix. |
| `.claude/rules/testing.md:57` (curation) | timeout-misgrade | Session Addition 2026-09-24 (test waits and the nextest kill below cargo-mutants' 20 s floor): still true under `wait`. Preserve-verbatim; no curation action. |

Zero-row patterns (`two-workflows`, `neither-wf`, `ci-nightly-pair`, `two-low`): each control fired at its pre-pass
site (ci-cd-approach.md:2, test-plan.md:1154 / :1153, security-plan.md:344), and the post-pass masters carry no
residue.

## Leaves re-derived (step 3)
- `.claude/docs/workflow.md:10` (arch-extracted) — above.
- `.claude/rules/verification-harness.md` `run --mutants` paragraph (rule body, test-plan §3) — "no … CI job runs it" →
  no push/PR CI job; the dispatch-only Windows leg; the `terminate = "wait"` profile.
- `.claude/docs/tests-summary.md` §Mutation row (test-plan §10) — the Windows leg and `terminate = "wait"` added.
- Read, no change:
  - `.claude/rules/testing.md` body (:48 "no chunk, pre-push or CI mutation gate" is still true);
  - `.claude/rules/security.md` (:33 workflow permissions — true for all three);
  - `.claude/docs/obs-summary.md:52` (`mutants.out/` never uploaded — true);
  - `.claude/docs/security-summary.md`, `stack.md` and `gotchas.md` (no workflow-count or mutation-CI claim);
  - CLAUDE.md `GENERATED:setup:*`: recomputed from arch §Infrastructure Patterns. The overview's key directories
    and the pointer table carry no workflow listing; the pointer-table rows are unchanged (17).
- `.claude/docs/commands.md` already carries the dispatch recipe (this chunk's own edit).
- Judgment bases: `playbook.md` / `drift-base.md` — 0 rows.
