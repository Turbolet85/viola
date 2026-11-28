@leaf_group group=test-plan-3
@leaf_read leaf=.claude/rules/testing.md lines=134
last: - 2026-11-25: A case whose start must end before its session
@leaf_read leaf=.claude/rules/verification-harness.md lines=81
last: - 2026-11-24: A live-reading script prints verdict words fro
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/rules/testing.md line=19 class=omission
leaf-text: libtest `#[cfg(test)] mod tests` inline in every crate + root `src/`, run through cargo-nextest
source: dec-dyjhqqkw «decision/unit-tests-inline · Inline unit tests — Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary. · except: A unit-test module too large for one file is split»
replace-with: libtest `#[cfg(test)] mod tests` inline in every crate + root `src/` (a unit-test module too large for one file is split into sibling path files that keep its module path, its cases grouped into child modules, each helper standing once beside its only users), run through cargo-nextest
