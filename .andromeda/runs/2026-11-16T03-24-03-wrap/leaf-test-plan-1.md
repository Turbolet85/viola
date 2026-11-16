@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_read leaf=.claude/rules/testing.md lines=118
last: - 2026-11-16: A shared case table whose assertions pin one r
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: (the root `stamped_home` and harness `boot` step 4
source: dec-apmz5kn6 «decision/one-verify-stamp-per-build · One verify stamp per build — Stamped test homes copy one viola verify stamp taken per pair of built binaries, kept under target/e2e-home keyed by the binaries' hashes»
replace-with: (the root `stamped_home`, each stamped test home a copy, never a link, of one `viola verify` stamp taken per pair of built binaries, kept under `target/e2e-home` keyed by the binaries' hashes and taken under an exclusive lock, and harness `boot` step 4
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=20 class=omission
leaf-text: a root home's leftover removed only once its recorded owner process is gone (removal deletes `owner.json` last, `remove_owned`)
source: dec-e7dpfnbo «decision/test-home-owner-record-sweep · Owner-record sweep of test homes — Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose»
replace-with: a root home's leftover removed only once its recorded owner process is gone (removal deletes `owner.json` last, `remove_owned`; a build's verify stamp dir carries no owner record, and another build's stamp is swept only when its lock is taken without waiting)
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=27 class=omission
leaf-text: wakes only on driver-relevant kinds
source: ctr-ic6eivw6 «contract/viola-wait/non-waking-records · viola wait · activity and wheel records — Activity and wheel records, and a harness-origin prompt-submitted, never wake a parked wait.»
replace-with: wakes only on driver-relevant kinds (activity and wheel records, and a harness-origin `prompt-submitted`, never wake a parked `wait`)
@leaf_edit n=4 leaf=.claude/rules/testing.md line=33 class=omission
leaf-text: A case that needs a home viola created starts from `stamped_home`.
source: dec-dvrimbrq «decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link;»
replace-with: A case that needs a home viola created starts from `stamped_home`, a copy (never a link) of its build's one `viola verify` stamp, taken per pair of built binaries, kept under `target/e2e-home` keyed by the binaries' hashes and taken under an exclusive lock.
