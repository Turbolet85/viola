---
paths:
  - "src/**/*.rs"
  - "crates/*/src/**/*.rs"
---

# Testing Rules, src-scoped pointers

A `#[cfg(test)]` module in product code is bound by `.claude/rules/testing.md`, which does not load here. Each line names the part to read there before writing the test; none of its text is repeated.

- §Framework: the unit tier
- §Naming
- §Determinism
- §What to assert
- 2026-09-24: an observable effect per function
- 2026-09-24: the awaited line
- 2026-09-24: `proptest!` bodies
- 2026-09-24: a process-global `OnceLock`
- 2026-09-24: I/O errors per OS
- 2026-09-24: test waits and the kill line
- 2026-09-24: `cfg(unix)`-only bodies
- 2026-09-24: operator-token mutants
- 2026-09-25: the remove-the-guard run
- 2026-09-25: inline test modules
- 2026-09-27: forcing a timing window
- 2026-09-27: a crate's own security test
- 2026-09-27: `const` flag mutants
- 2026-09-27: log lines off the test thread
- 2026-09-28: a feature-gated test module
- 2026-09-28: no reshaping for mutants
- 2026-10-03: cfg-only imports
- 2026-10-07: a moved compiled bound
- 2026-10-07: test clocks and locks

## Session Additions
_Owned by `/wrap-session`; kept on a setup re-run._
- 2026-10-10: a kill only at a process the test started (`testing.md`, the entry of that date).
