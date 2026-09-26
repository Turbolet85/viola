
## 2026-09-26-ci-chunk-base-and-union-verdict — derived whole-chunk mutation base, compiling-leg union, syn/proc-macro2 in viola-e2e
**Section:** Stack and Technologies (Code quality row) · Occupied Resources → Environment variables (`AGENT_RUN_CHUNK_BASE`) · Occupied Resources → Repository (`target/agent-run/`) · Infrastructure Patterns → Crate dependency direction · Infrastructure Patterns → CI/CD approach (concurrency rationale; `test` job `harness-<os>` upload; `mutants` legs; `mutants-verdict` union)
**Change:**
- `AGENT_RUN_CHUNK_BASE` is an explicit override of the derived base; CI does not set it.
- `mutants` legs: step "Mutation leg (whole chunk)", only `LEG` through `env:`, base derived by the harness (last master flip before the oldest pre-CI commit, `HEAD^` on the wrap push), the run document names `base`.
- `mutants-verdict`: the union judges each mutant only by the legs whose `#[cfg]`s compile its line (`harness::cfg_legs`), every leg when none does.
- `harness-<os>` uploads `target/agent-run/` minus `target/agent-run/chunk.diff`; the Repository row records that `chunk.diff` is the one file `secret-scan` skips.
- The no-concurrency reason keeps the `always()` gate/upload chain and retires the "drop a push's `--in-diff` mutation diff" half (every run now covers the whole chunk).
- syn 2.0.119 + proc-macro2 1.0.107 (`span-locations`) registered in the Code quality row and as `viola-e2e`'s dependency-direction entry.
**Why:** chunk 2026-09-26-ci-chunk-base-and-union-verdict report Dependencies, Symbols/APIs, Harness / gate surface, Expected amendments (architecture ×4).
**Sweep:** the test-plan entry's 12 patterns; architecture rows :364 (edited, true: "an explicit override of the mutation gate's diff base"), :519, :522 amended; no other architecture hit. Cross-master: security-plan :331 restated the retired concurrency reason — amended in this pass. Leaves re-derived: `docs/stack.md:33` (Code quality row), `docs/workflow.md:10`, `docs/commands.md:31/:41`; CLAUDE.md `GENERATED:setup:*` blocks recomputed against the amended sections — no block states the base, the union or the scan scope (0 hits of the 12 patterns in CLAUDE.md), no change. Fanned 8 proposals (6 D-arch-resources, 2 D-arch-decisions; 4 `dependent-of`), all applied with text re-derived from the report.
