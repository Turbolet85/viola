@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=109
last: - 2026-11-06: A sweep for a `cfg` predicate reads the predic
@leaf_read leaf=.claude/rules/verification-harness.md lines=72
last: - 2026-10-20: `target/debug/viola` is a hard link cargo re-p
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=39 class=stale
leaf-text: ≥ 85 % per OS
source: fct-ryyndnld «Gates are enforced in the `test` job on `ubuntu-latest` with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`.»
replace-with: ≥ 85 % (the `test` job on `ubuntu-latest`)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=42 class=stale
leaf-text: the mutation audit reads it at the route entry "Version done-check"
source: fct-ocdr3xdn «That survivor stands as version 0.1.0's known limit, read at each epoch-boundary audit (matrix#v1-44).»
replace-with: it stands as version 0.1.0's known limit, read at each epoch-boundary audit (matrix#v1-44)
@leaf_edit n=3 leaf=.claude/rules/testing.md line=46 class=stale
leaf-text: regions 80 per OS
source: fct-ryyndnld «Gates are enforced in the `test` job on `ubuntu-latest` with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`.»
replace-with: regions 80, enforced in the `test` job on `ubuntu-latest`
@leaf_edit n=4 leaf=.claude/rules/testing.md line=46 class=stale
leaf-text: the mutation audit reads it at the route entry "Version done-check"
source: fct-ocdr3xdn «That survivor stands as version 0.1.0's known limit, read at each epoch-boundary audit (matrix#v1-44).»
replace-with: it stands as version 0.1.0's known limit, read at each epoch-boundary audit (matrix#v1-44)
@leaf_edit n=5 leaf=.claude/rules/verification-harness.md line=43 class=retired
leaf-text: A booted-session test must stop its session in a `Drop` guard: under a mutated `cleanup` a leaked supervisor locks its `.exe` on Windows, and every later mutant build fails.
source: fct-x3qmgvn2 «viola-e2e's booted lifecycle tests (`crates/viola-e2e/tests/harness_lifecycle.rs`) hold a `Booted` guard instead: on drop, even after a failed assertion, it runs `cleanup(.., keep_homes = true)`»
replace-with: A booted-session test must stop its session in a `Drop` guard: viola-e2e's `Booted` guard runs `cleanup` on drop, even after a failed assertion.
