# Evidence — a red met at P2: the Windows mutation run read a stale root test binary

Measured 2026-09-27/28 by `/andromeda-implement`, on the uncommitted tree over `273e1ab`. Folded into this chunk (CLAUDE.md
session learning: a red met during a chunk folds into it, even outside its diff).

## The red
- Gate entry 22, `bash scripts/agent-run.sh run --mutants --file src/cmd/hook.rs` (first full block, implement run
  `2026-09-27T21-48-40`): exit 1, `25 mutants tested: 4 missed, 18 caught, 3 unviable`. Missed:
  - `src/cmd/hook.rs:67:5: replace hook -> Result<ExitCode, Failure> with Ok(Default::default())`
  - `src/cmd/hook.rs:142:5: replace reject_stdin with ()`
  - `src/cmd/hook.rs:171:5: replace deliver -> bool with false`
  - `src/cmd/hook.rs:200:5: replace decided with ()`
- The same block's entry 29 (`pre-push`) read green, but its `windows-2025` leg listed the same four as MISSED, plus
  `src/main.rs:49:23: replace match guard role == Role::Hook with false`. The union passed only because the
  `ubuntu-latest` leg caught them.

## The cause (known, not sampled)
- Each missed mutant is killable only by a root integration test that spawns `env!("CARGO_BIN_EXE_viola")`.
- Hand-applying the first mutant in the repository and running `cargo nextest run --test hook_fail_open` failed 4 tests
  at once: the tests do kill it.
- In the mutant's own cargo-mutants log, the `viola` bin was rebuilt mutated (its dead-code warnings name the stubbed
  functions). Yet `hook_fail_open …case_09_no_snapshot` passed. That case needs role lines only a live `hook()` writes.
- The test binary it ran, `target/debug/deps/hook_fail_open-2816cd3b49df6d23.exe`, also exists in the repository's own
  `target/debug`. It was built at 00:04 by an ad-hoc `cargo nextest` run, and its compiled-in paths are
  `D:\dev\projects\viola\target\debug\viola.exe` and `…\viola-fake-agent.exe`.
- `run --mutants` uses `--copy-target=true`: cargo-mutants copied that binary into its scratch tree, and cargo read it as
  fresh there, so the "mutated" run drove the repository's unmutated `viola`.
- The ubuntu leg runs in the WSL clone, whose `target/debug` holds no root test binary. Its baseline builds them in the
  scratch tree, which is why it caught the mutants. CI's legs build fresh too.

## The fix
`crates/viola-e2e/src/harness/run/mutants.rs`: the mutation run gets its own target dir, `target/mutants`.
- The root pre-build writes into the repository's `target/mutants`.
- cargo-mutants gets the relative `CARGO_TARGET_DIR=target/mutants`, so its copied tree builds every test binary in its
  own copy. No ad-hoc `cargo test` writes there.
- New test: `run_mutants_builds_in_its_own_target_dir` pins both values.

## Green after the fix
Entry 22 re-run (`--only 22`): exit 0, `25 mutants tested in 3m: 22 caught, 3 unviable`. All four earlier survivors read
`caught`.
