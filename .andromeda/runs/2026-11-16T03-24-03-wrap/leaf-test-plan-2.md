@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: stamped by `viola verify` against the fake agent at 2.1.287 over `fixtures/claude`
source: dec-apmz5kn6 «Stamped test homes copy one viola verify stamp taken per pair of built binaries, kept under target/e2e-home keyed by the binaries' hashes and taken under an exclusive lock.»
replace-with: a copy, never a link, of one `viola verify` stamp per build (taken against the fake agent at 2.1.287 over `fixtures/claude` once per pair of built binaries, under an exclusive lock, kept under `target/e2e-home` keyed by the binaries' hashes)
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: never a dir without a record (a runner-killed test never drops its home)
source: dec-e7dpfnbo «A build's verify stamp dir carries no owner record; another build's stamp is swept only when its lock is taken without waiting.»
replace-with: never a dir without a record, save a build's verify stamp dir, which carries none: another build's stamp is swept only when its lock is taken without waiting (a runner-killed test never drops its home)
