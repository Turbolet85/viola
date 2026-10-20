# Steps 3 and 4: the two `mutants` kill lines, each with its planted-hang pair

Both kills sit on a by-design floor measured across runs, so each move carries the one-off pair testing.md
2026-09-28 (extended 2026-10-05) asks for: a planted hang is still killed under the new bound, and the same plant
passes with no bound.

## The changes (`.config/nextest.toml`, `[profile.mutants]`)

- **Step 3:** a new override `filter = 'test(/send_under_the_paste_hint_/)'` with `slow-timeout = { period = "15s",
  terminate-after = 2 }` (30 s, the `send_window_` class's kill). Its floor: 8.7–9.9 s on windows-2025 / ubuntu /
  macOS under `ci` (run 39383206773's JUnit), against the 10 s default kill. The case names are unchanged.
- **Step 4:** the `verify_window_` override moves from `period = "15s", terminate-after = 3` (45 s) to `period =
  "20s", terminate-after = 3` (60 s, the class's `ci` kill). Its floor: 42.874 / 43.078 / 43.714 s under `ci`
  (run 39383206773) and 42.952 s alone on this host. It stays first among the overrides. No other kill line moved.

## The planted-hang pairs

The scratch clone is `clone-k`, a `git clone` of HEAD `d35644a58909aec12c4898e5a066bcfca5a8ebd2` into this session's
scratchpad with the chunk's diff applied (`git diff HEAD`, before step 10(c)'s edit). Its target dir is
`target/e5c-clone-target`. Each case had one planted line, the test body's first statement:

- `tests/cli_send.rs` `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`:
  `std::thread::sleep(std::time::Duration::from_secs(40));`
- `tests/cli_verify.rs` `verify_window_without_screens_fails_every_interactive_row`:
  `std::thread::sleep(std::time::Duration::from_secs(70));`

The form was `cargo nextest run --package viola --features fake-agent -E 'test(=<name>)'` with `NEXTEST_PROFILE` set
as shown and `AGENT_RUN_KEEP_HOMES=0 AGENT_RUN_KEEP_FAILED=0`
(`$TMPDIR/{scratchpad}/killpairs.sh`).

| case | profile | started (UTC) | reading |
|---|---|---|---|
| paste-hint (40 s plant) | `mutants` | 09:11:04 | `TIMEOUT [  30.003s] (1/1) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` |
| paste-hint (40 s plant) | default | 09:11:34 | `PASS [  48.413s] (1/1) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` |
| verify_window_ (70 s plant) | `mutants` | 09:12:23 | `TIMEOUT [  60.003s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row` |
| verify_window_ (70 s plant) | default | 09:13:23 | `PASS [ 112.961s] (1/1) viola::cli_verify verify_window_without_screens_fails_every_interactive_row` |

So each hang is still killed at its new bound, and each plant passes with no bound: the bound, not the plant, ends
the `mutants` run.

## The new pin's remove-the-guard reading (step 3)

`mutants_paste_hint_override_holds_the_class_above_the_default_kill` (`tests/contract_lints.rs`) was run over a
second scratch clone, `clone-h` (HEAD `d35644a58909…` + the chunk's diff, target dir `target/e5c-h`), by `cargo
nextest run --package viola --features fake-agent -E 'test(/mutants_paste_hint_override_holds_the_class_above_the_default_kill/)'`
(`…/scratchpad/pin_guard.sh`). This was a direct nextest run inside the clone, not `gate.py run --project-root`: the
e2e entry would build the harness target inside the clone, on the quota'd `/tmp`.

- With the override's block deleted from the clone's `.config/nextest.toml` (0 lines naming the class): `FAIL`,
  `panicked at tests/contract_lints.rs:313:28: no filter = 'test(/send_under_the_paste_hint_/)' among [[…verify_window_…],
  […package(viola-e2e)…], […send_window_…]]`.
- With it restored (2 lines naming the class): `PASS [   0.004s] … mutants_paste_hint_override_holds_the_class_above_the_default_kill`.
