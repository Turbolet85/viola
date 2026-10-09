# The planted-hang control of the moved kill (plan.md step 5), and the order case's two readings

One-shot readings taken at implement on 2026-10-09 (UTC). The plant was a scratch edit and is gone: its marker
string counts 0 in the file.

## The planted hang (testing.md 2026-09-28, extended 2026-10-05)

The subject is `viola-e2e::harness_lifecycle verify_window_boot_with_an_unknown_cli_version_is_verify_failed`, the
test step 4 renamed into the `verify_window_` class. Its designed wait is 34.5 s (`e2e-floor.md`). In the `mutants`
profile the class's override now stands first, so the test's kill moved from the package's 30 s to the class's 45 s
(15 s × 3).

**The plant:** one line at the top of the test's body, a finite 50 s wait marked `PLANTED-HANG-E3`. With it the
test needs about 84.5 s. The marker counted 1 in the file before each run.

Both runs are the same command but for the profile, the test alone, in the mutation run's own target dir:

```
[NEXTEST_PROFILE=mutants] AGENT_RUN_KEEP_HOMES=0 AGENT_RUN_KEEP_FAILED=0 CARGO_TARGET_DIR=target/mutants \
  cargo nextest run --package viola-e2e --features fake-agent \
  -E 'test(/verify_window_boot_with_an_unknown_cli_version/)'
```

### Reading 1: under `NEXTEST_PROFILE=mutants`, the hang is still killed, at the moved bound

Started 15:58:27Z, exit 100. Status lines, with nextest's own padding:

```
        SLOW [> 15.000s] (───) viola-e2e::harness_lifecycle verify_window_boot_with_an_unknown_cli_version_is_verify_failed
        SLOW [> 30.000s] (───) viola-e2e::harness_lifecycle verify_window_boot_with_an_unknown_cli_version_is_verify_failed
 TERMINATING [> 45.000s] (───) viola-e2e::harness_lifecycle verify_window_boot_with_an_unknown_cli_version_is_verify_failed
     TIMEOUT [  45.005s] (1/1) viola-e2e::harness_lifecycle verify_window_boot_with_an_unknown_cli_version_is_verify_failed
     Summary [  45.007s] 1 test run: 0 passed, 1 timed out, 257 skipped
```

The kill is the class's 45 s: the test passed the 30 s line, where the package's override killed it before the
move, with a `SLOW` line and no kill.

### Reading 2: the same plant under the default profile, which holds no kill, passes

Started 15:59:20Z, exit 0. `NEXTEST_PROFILE` was removed from the command's environment; the default profile has
no `slow-timeout` kill.

```
        SLOW [> 60.000s] (───) viola-e2e::harness_lifecycle verify_window_boot_with_an_unknown_cli_version_is_verify_failed
        PASS [  84.513s] (1/1) viola-e2e::harness_lifecycle verify_window_boot_with_an_unknown_cli_version_is_verify_failed
     Summary [  84.514s] 1 test run: 1 passed (1 slow), 257 skipped
```

84.513 s is the 50 s plant and the 34.5 s floor. So the plant hangs the test and nothing else fails it: with a
kill it ends `TIMEOUT` at the bound, with none it passes.

### The plant removed

After reading 2 the planted line was removed. `grep -c 'PLANTED-HANG-E3'
crates/viola-e2e/tests/harness_lifecycle.rs` printed `0` (exit 1). The file's whole diff against the base commit
is the one renamed `fn` line.

## The order case, red on the old order and green on the moved one (testing.md 2026-09-25)

`tests/contract_lints.rs` gained `mutants_verify_window_override_stands_before_the_harness_package_override`. Its
guard is the order of the overrides in `.config/nextest.toml`, so the guard removed is the old order. The case was
written before the override moved and run both sides with
`CARGO_TARGET_DIR=target/harness cargo nextest run --workspace --features viola/fake-agent --profile ci -E
'binary(contract_lints)'`:

| side | started | exit | the case's line | summary |
|---|---|---|---|---|
| the old order (`package(viola-e2e)` first) | 15:57:46Z | 100 | `FAIL [   0.012s] (2/6) viola::contract_lints mutants_verify_window_override_stands_before_the_harness_package_override`, panicked at its order assertion | `6 tests run: 5 passed, 1 failed, 0 skipped` |
| the moved order (`verify_window_` first) | 15:58:13Z | 0 | `PASS [   0.016s] (5/6) viola::contract_lints mutants_verify_window_override_stands_before_the_harness_package_override` | `6 tests run: 6 passed, 0 skipped` |

On the old order `test_deadlines_sit_below_the_nextest_kill_line` passed too (`PASS [   0.046s]`): it now finds the
`package(viola-e2e)` override wherever it stands, so it read the same 30 s kill line on both sides.
