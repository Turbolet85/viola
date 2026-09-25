
## 2026-09-25-security-prerequisites — the `test-only-rust-delta` mutation verdict; the SQOS open and content hash pinned
**Section:** §3 `run` step 4 Classification; §3 `run` Output format (`mutants` object, leg file); §3 Closed enums; §6 Security control negatives → Windows client SQOS; §6 Contract suite; §10 Mutation gate; §12 (new `2026-09-25` entry).
**Change:**
- Every place that listed the verdicts now carries the third closed value `test-only-rust-delta`. It applies to a diff whose `.rs` paths are all test targets (`tests/`, `benches/`, `examples/`, root or `crates/<member>/`), which never builds or runs cargo-mutants and names the diff, its file count and `rust_files`. A mixed diff stays `counted` and red `outcomes-missing` without a fresh `outcomes.json`.
- §6 records `tests/channel_sqos_open.rs` (recipe + no-SQOS control) ahead of the viola-client negative, and `tests/contract_content_hash.rs` in the Contract suite.
- §12 carries the decision, refining the chunk-2 ruling (:1781/:1790 history left as written).
**Why:**
- The chunk's report: Spec claims disproved #1, Harness / gate surface, Counts moved (the closed set goes from 2 to 3), Symbols.
- Operator ruling "option A" (fold the fix, 4 conditions).
- cargo-mutants 27.1.0 measurements: `No mutants to filter` over a tests-only diff; `--list-files` lists `src/` only.
- Witness: CI run 36138441784, both mutants legs `9 caught`.
**Sweep** (cascade step 2):
- `test files included|naming any \`\.rs\` path` → 0 after the apply. Control: 1 in `git show HEAD:.andromeda/test-plan.md` (:1496), amended.
- Two-value verdict lists (`` `counted`, `no-rust-delta` `` / `"counted"|"no-rust-delta"`):
  - :566 and :663 amended;
  - :1781 no change (the dated 2026-09-24 §12 entry, history, superseded by the new entry).
- 0 hits in the other six masters: obs-plan names no verdict value, as its detector confirmed.
- Leaves re-derived, all above `## Session Additions`:
  - `.claude/rules/testing.md:48`;
  - `.claude/rules/verification-harness.md:43`;
  - `.claude/docs/tests-summary.md:42`.
- 0 hits in CLAUDE.md, the curation homes, playbook and drift-base.
- Fanned 7 proposals (5 D-tests-obs-harness, 4 of them `dependent-of`; 2 D-tests-coverage), all applied with text re-derived from the report.
