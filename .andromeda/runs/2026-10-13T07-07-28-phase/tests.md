# tests extract

## Relevance
relevant — three of the four parts are test-strength or test-gate work: §1 disposes mutation survivors, §2 is a red Windows mutation baseline on a unit test, and §4 is a lint gate. §3 is a pure move that must keep every named test.

## Constraints
- A mutant is killed only by the tests of the package that owns the mutated file. cargo-mutants' default scope is kept (`--test-workspace` unset), so the `viola-state` (10), `viola-agent-claude` (2), `viola-pty` (1) and `viola-e2e` (2) survivors each need a test inside their own crate. A root `tests/` case or a `viola-e2e` scenario cannot kill them. (per test-plan §3 `run` step 4 Test scope)
- The next audit reads a survivor as disposed only if it is caught, timed-out-free and not missed. The harness leaves a missed mutant out only when its whole span sits under a `cfg` predicate proved false for the host (`mutants.host_excluded`). A mutant behind a const (for example `cfg!(windows)`) is never left out. The plan has no "accepted equivalent" outcome for a reachable site, so the scope's third disposal form has no home in the test plan. Which record the next audit reads is research's and P4's question. (per test-plan §3 `run` step 4 Verdict)
- A mutant this host cannot compile or reach is recorded "not measured here; owed to {route entry}" by coordinate. It is never called "equivalent" and never counted as caught. The plan already places `scratch.rs` `prepare`'s two missed mutants (`:48`, `:54`) there: reachable only on a Windows host, read caught on `windows-2025` in run 38036448183, and still missed on a Linux whole-member run. (per test-plan §10 Mutation gate)
- A mutant caught only by a doctest counts as missed under nextest. Every disposing test must be a nextest unit or integration test. (per test-plan §10 Mutation gate)
- No mutation run inside a chunk, pre-push or CI. Mutation runs at the epoch boundary through `/andromeda-code-audit`, and on demand through `run --mutants` [`--file`]. The operator's take-up direction also rules out the on-demand form in this chunk, so the survivor proof is a hand-applied mutant, not `run --mutants --file`. (per test-plan §2 Mutation row)
- OS-branch code is verified only on its own runner. The `statusline_an_instance_viola_laid_down_passes_its_check` red (Windows owner + DACL) and any `cfg(windows)` survivor cannot be closed by a Linux run. A check-only `x86_64-pc-windows-msvc` clippy on Linux lints Windows code but does not test it. (per test-plan §11 Test Strategy)
- Coverage floors hold per OS job: lines 85, functions 95, regions 80, with the `COVERAGE_IGNORE` regex unwidened. A pure split of `src/run/send.rs` / `src/human.rs` must not move a test out of the instrumented run. (per test-plan §10 Coverage thresholds)

## Patterns to follow
- Disk-touching survivors (`fs.rs` `replace_private_with`, `board.rs` `check_if_present` / `scan` NotFound guards, `SkippedTotal::add`): use a real tempfile home and real file faults, never mocked on-disk state. A malformed state file may be planted only by altering the writer's own bytes. (per test-plan §11 Integration; §11 Test Data)
- PTY-seam survivors (`viola-pty` `resize`, `typed.rs` `SharedPty` `resize` / `close` / `child_pid`): consumers are tested through the existing mockall double of the six-method `Pty` seam. A disposing test asserts what the seam call does, not a new failpoint. (per test-plan §4 viola-pty; §11 Mocking)
- Name tests `<subject>_<condition>_<expected>`, as inline `#[cfg(test)] mod tests` at the bottom of the source file. rstest `#[case]` tables suit the boundary flips (`>`→`>=`, `<`→`<=`). (per test-plan §2 Test directory + naming conventions)
- Clock-bound survivors (`dialog.rs` `millis`, `typed.rs` `end_by_ctrl_c` / `finish` arithmetic, wheel `Classifier`): assert through deterministic clock injection or pure functions given the instant. No sleep and no elapsed-time verdict on the test side. (per test-plan §11 Universal)
- A cfg-gated test is compiled out with `#[cfg(...)]`, never `#[ignore]`, so the `gate` `suite-skipped` breach stays meaningful. (per test-plan §3 `run` Output format, `skipped`)

## Anti-patterns to avoid
- Never trust the cargo-mutants exit code or its summary line alone. A survivor counts as disposed only on the harness's `outcomes.json` reading, at the next boundary. (per test-plan §11 Quality)
- Never test implementation details (private methods or internal state). A disposing test for a private site (`max_gap`, `Classifier::step`, `is_reply`) asserts observable behaviour of the function or API that reaches it, not internal fields. (per test-plan §11 Unit)
- Never widen the coverage ignore regex, add a retry, or skip a gate to turn the Windows baseline or a split green. (per test-plan §11 CI; §11 Quality)

## Contract bindings
- tests ↔ CI (`windows-mutants.yml`): the root package is four `matrix.include` items, split by file, each running `run --mutants --package viola --file …` over its Windows-gated files. Whether `src/run/send.rs` / `src/human.rs` appear in a `--file` list, and so must follow the split, is research's question. `tests/contract_windows_mutation_scope.rs` pins the workflow's tool versions to the `test` job's. (per test-plan §9 Mutation row; §9 CI security)
- tests ↔ security (strict-modes): the Windows baseline red is `check_instance`'s owner + DACL read on a test-laid instance inside the cargo-mutants scratch, which on a Windows host follows `TMP`/`TEMP` into `<repo parent>/viola-mutants-scratch`. The test home and tempdir rules are tests-owned. The DACL verdict is security-plan's. (per test-plan §3 `run` step 4 Command, Windows host scratch arm; §3 Test data bootstrap)
- tests ↔ masters: test-plan §4 names tests by file ("six of the seven, in `src/run/send.rs`", the two `inner_cr_is_one_lf` tests in `src/run/send.rs`). The split keeps every test name, and the wrap reconciles the file citations. (per test-plan §4 root bin)
- tests ↔ harness `pre-push`: its stages are fixed (`host` · `tools` · `linux-tests`). A cross-target clippy added as a pre-push stage, not only as a chunk `[[gate]]`, is a §3 `pre-push` contract amendment. (per test-plan §3 `pre-push`)

## Acceptance criteria contributions
- Each in-scope survivor has a nextest test in the package that owns the mutated file, shown failing under its hand-applied mutant and passing on the unmutated tree. Any survivor this host cannot reach is recorded "not measured here; owed to {entry}" by coordinate, never "equivalent". (per test-plan §3 `run` step 4 Test scope; §10 Mutation gate)
- `scripts/agent-run.sh run` (default `--all`) and `scripts/agent-run.sh run --coverage` exit 0, at lines ≥ 85, functions ≥ 95 and regions ≥ 80, with no test skipped. `scripts/agent-run.sh pre-push` reads `ok:true`. (per test-plan §10 Coverage thresholds; §3 `pre-push`)
- After the `send.rs` / `human.rs` split, every test name that test-plan §4 cites still exists and passes, and no test is lost: the nextest test count before and after the move is equal. (per test-plan §4 root bin)
- The Windows statusline baseline fix is shown on a `windows-2025` leg of `ci.yml`, not by a Linux run. The next boundary's `windows-mutants.yml` dispatch reads the root package's verdict. (per test-plan §11 Test Strategy; §9 Mutation row)
