# Classified — test-plan · unit-test-strategy.b1

## Answers
unit-test-strategy#1.1 · yes · add-to · F7: the viola-channel scoped captures in `test_support.rs` and `lib.rs` stay on a bare `with_default`, never observed red, with no owner
unit-test-strategy#1.2 · yes · add-to · F7: the viola-state `#[cfg(test)]` capture this entity cites (`crates/viola-state/src/lib.rs`) stays on a bare `with_default`, never observed red, with no owner
unit-test-strategy#1.3 · no · no `crates/` file is reached and no product file changed, so the readiness verdict, the vt100 feed's `catch_unwind` and its screen and gate unit cases stand
unit-test-strategy#2.1 · yes · add-to · F5, F6: the root bin's scoped log captures now go through `test_support::with_scoped_default`, and its witness test is a new root-bin unit case
