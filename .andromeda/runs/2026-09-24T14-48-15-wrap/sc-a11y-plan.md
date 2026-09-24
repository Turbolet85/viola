
## 2026-09-24-quality-gates — CLI output-discipline evidence reports under the CI `coverage` suite
**Section:** §1 (CLI output discipline bullet) · §1 cli surface (Automated tool reach) · §9 Per-pipeline-stage table (Unit / integration row) · §10 Standard+ invariants
**Change:** the CLI output-discipline tests still run on all three OS legs. Locally they report under `nextest-integration` / `nextest-e2e`; in CI they report inside the per-OS `test` job's single instrumented `coverage` suite, gated by `gate --require coverage,doctest`. The run JSON `suites[]` enum in the table row gains `coverage`.
**Why:** chunk 2026-09-24-quality-gates replaced the CI unit and integration runs with one `run --coverage` (report Harness / gate surface). Raised by the orchestrator at Validate: the a11y detector flagged it as outside both D-a11y invariants.
**Sweep:** `nextest-integration|nextest-e2e` 4 hits, all amended (`:280, :499, :1115, :1153`). `:604` ("failures surface only as nextest failures in `suites[].failures[]`") stays true under `coverage`, no change. `.claude/docs/a11y-summary.md` and `.claude/rules/a11y.md`: 0 hits, no change.
