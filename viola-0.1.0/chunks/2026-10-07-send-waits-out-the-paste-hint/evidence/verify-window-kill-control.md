# The moved `verify_window_` kill lines — planted-hang control (step 9)

`.config/nextest.toml` now holds, for `test(/verify_window_/)`: `[profile.ci]` 20 s × 3 = 60 s (was 15 s × 3 =
45 s) and `[profile.mutants]` 15 s × 3 = 45 s (was 15 s × 2 = 30 s). The class waits the gate's maximum four times
by design, and this chunk moved that maximum from 5 s to 8.5 s (the overseer's answer, inputs#I2;
`.claude/rules/testing.md` 2026-09-28, extended 2026-10-05). This record is the moved lines' pair, taken in the
form of `../../2026-10-05-permission-end-to-end/evidence/verify-window-hang-control.md`: a one-off, never a
committed test.

- **Host:** the Linux dev host, cargo-nextest 0.9.146
- **Date:** 2026-10-07, 12:32:03Z to 12:37:22Z (`date -u`, written by the run script at each start and end)
- **Subject:** `tests/cli_verify.rs` `verify_window_without_screens_fails_every_interactive_row`
- **Command, per profile:** `cargo nextest run --profile <ci|mutants|default> --features fake-agent --test cli_verify
  --no-fail-fast -E 'test(/verify_window_without_screens/)'`

## The unplanted test's own duration (the floor, measured)

Predicted about 34.5 s (20.5 s measured at 5 s, four waits moved by 3.5 s each). Measured 34.3 s under every
profile:

`--profile ci` (12:32:03Z to 12:32:38Z, exit 0):
```text
        SLOW [> 20.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
        PASS [  34.340s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     Summary [  34.340s] 1 test run: 1 passed (1 slow), 28 skipped
```

`--profile mutants` (12:32:38Z to 12:33:13Z, exit 0):
```text
        SLOW [> 15.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
        SLOW [> 30.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
        PASS [  34.332s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     Summary [  34.332s] 1 test run: 1 passed (1 slow), 28 skipped
```

The default profile (12:33:13Z to 12:33:47Z, exit 0):
```text
        PASS [  34.333s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     Summary [  34.334s] 1 test run: 1 passed, 28 skipped
```

So the floor sits 25.7 s under the 60 s CI kill and 10.7 s under the 45 s `mutants` kill on this host. It would
have crossed the old 30 s `mutants` kill.

## The planted hang

- **Plant:** the scratch line `std::thread::sleep(std::time::Duration::from_secs(60));` right after the test's
  `verify_without_screens` call, so the test runs about 94 s with nothing to bound it.
- **Revert:** the plant was removed after the three runs. The site was re-read (the call is followed directly by
  `assert_eq!(ran.code, Some(1));`), `from_secs(60)` no longer occurs in the file, and the file compares byte for
  byte (`cmp`, exit 0) with the copy saved before the plant.

| control | profile | result |
|---|---|---|
| a hang is still caught on CI's line | `ci` | `TIMEOUT [  60.003s]`: killed by the 60 s override (exit 100) |
| a hang is still caught on the mutation line | `mutants` | `TIMEOUT [  45.005s]`: killed by the 45 s override (exit 100) |
| the same plant with no kill | `default` | `PASS [  94.359s]`: nothing bounds it, so the overrides are what kill a hang (exit 0) |

Verbatim status lines.

`--profile ci` (12:34:01Z to 12:35:02Z):
```text
        SLOW [> 20.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
        SLOW [> 40.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     TIMEOUT [  60.003s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     Summary [  60.004s] 1 test run: 0 passed, 1 timed out, 28 skipped
```

`--profile mutants` (12:35:02Z to 12:35:47Z):
```text
        SLOW [> 15.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
        SLOW [> 30.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     TIMEOUT [  45.005s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     Summary [  45.005s] 1 test run: 0 passed, 1 timed out, 28 skipped
```

The default profile (12:35:47Z to 12:37:22Z):
```text
        SLOW [> 60.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
        PASS [  94.359s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     Summary [  94.359s] 1 test run: 1 passed (1 slow), 28 skipped
```

After the two killed runs (read 12:37:36Z): no `viola`, fake-agent or `claude` process whose executable or cwd is
under this repository; `target/e2e-home/` empty; the root's `.viola-verify-*` dirs are the eleven that stood
before this chunk.

STOP 4 did not fire: the planted hang is killed under each moved bound.
