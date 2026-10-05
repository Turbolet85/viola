# The 45 s `verify_window_` override — planted-hang control

The `test(/verify_window_/)` override in `.config/nextest.toml` `[profile.ci]` (15 s × 3 = 45 s, the first
matching override) had no planted-hang control. The 20 s binary override's pair is in
`../../2026-10-05-dialog-rows-and-re-probe/evidence/verify-window-class.md` §Controls. This record is the 45 s
override's pair, taken the same way: a one-off, never a committed test.

- **Host:** the Linux dev host (`Linux 7.2.5-3-omarchy`), cargo-nextest 0.9.146
- **Date:** 2026-10-05, about 15:24–15:27Z
- **Plant:** in `tests/cli_verify.rs` `verify_window_without_screens_fails_every_interactive_row`, the scratch
  line `std::thread::sleep(std::time::Duration::from_secs(60));` right after its `verify_without_screens` call.
- **Revert:** the plant was removed after both runs. `git diff --quiet 2bd08e949138 -- tests/cli_verify.rs`
  exits 0.

## Controls
In the table, the bracket's column padding is collapsed (`[  45.004s]` reads `[45.004s]`), the same form as the
20 s pair's record. The verbatim lines follow the table.

| control | form | result |
|---|---|---|
| a hang is still caught | the planted 60 s sleep, `cargo nextest run --profile ci --features fake-agent -E 'test(/verify_window_without_screens/)'` | `SLOW [> 15.000s]`, `SLOW [> 30.000s]`, then `TIMEOUT [45.004s]`: killed by the 45 s override (exit 100) |
| the same, without the override | the same plant, `cargo nextest run --features fake-agent -E 'test(/verify_window_without_screens/)'` (the default profile) | `SLOW [> 60.000s]`, then `PASS [80.480s]`: nothing bounds it, so the override is what kills a hang (exit 0) |

`--features fake-agent` is required: `cli_verify` is a `required-features = ["fake-agent"]` test target.

## Verbatim status lines
`--profile ci`:
```text
        SLOW [> 15.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
        SLOW [> 30.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     TIMEOUT [  45.004s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     Summary [  45.006s] 1 test run: 0 passed, 1 timed out, 764 skipped
```

The default profile:
```text
        SLOW [> 60.000s] (───) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
        PASS [  80.480s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row
     Summary [  80.481s] 1 test run: 1 passed (1 slow), 764 skipped
```

So the class meets both conditions of the designed-floor exception. The floor was measured across CI rounds
(`../../2026-10-05-dialog-rows-and-re-probe/evidence/ci-rounds.md`), and a planted hang is still killed under each
moved bound: 20 s for the twelve verify-driving binaries, 45 s for `test(/verify_window_/)`.
