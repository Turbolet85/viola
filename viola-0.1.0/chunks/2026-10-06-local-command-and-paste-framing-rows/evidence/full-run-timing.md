# Step 9 — the verify-driven tests' durations, read after the second record round

Read from the Linux dev host's own runs of 2026-10-06, the tree as this chunk leaves it (seventeen rows, Run B at
four pastes with the wait after each added turn):

- **the default selector** (`bash scripts/agent-run.sh run`, gate entry 19, the second full block, 21:26Z, a
  quiet host): `1312 tests run: 1312 passed` in 5.5 s and `326 tests run: 326 passed (4 slow)` in 28.9 s;
- **the instrumented run** inside the native pre-push gate (entry 25, the same block): `1638 tests run: 1638
  passed (4 slow)` in 30.8 s.

The first full run after the round (entry 19 of the first block, 21:18Z) is not the basis: it ran under another
project's link on the same volume and read 20 slow tests (`block-reds-host-contention.md`).

## The verify-driving binaries, against their 20 s kill (`.config/nextest.toml`)
Seconds per test; `n` is the binary's test count.

| binary | n | median | max, default run | max, instrumented run |
|---|---|---|---|---|
| `channel_endpoint` | 5 | 0.19 | 4.90 | 5.79 |
| `cli_answer` | 9 | 4.24 | 5.12 | 5.70 |
| `cli_fake_agent` | 35 | 0.08 | 5.18 | 5.51 |
| `cli_instance_state` | 9 | 5.08 | 5.56 | 6.08 |
| `cli_send` | 10 | 1.22 | 10.88 | 10.69 |
| `cli_verify` (without its two `verify_window_` cases) | 26 | 3.92 | 4.86 | 4.85 |
| `cli_version_gate` | 9 | 4.18 | 4.35 | 4.28 |
| `cli_wheel` | 3 | 4.59 | 4.62 | 4.66 |
| `contract_fake_agent_drift` | 9 | 0.04 | 7.92 | 7.87 |
| `contract_ledger_probes` | 2 | 0.01 | 4.05 | 3.96 |
| `tui_wheel` | 4 | 4.43 | 4.54 | 4.52 |

- The longest under the 20 s kill are the three `send_window_` cases (10.8 s to 10.9 s: the product's 10 s
  confirmation window, unchanged) and `fake_agent_dialog_replay_matches_every_recorded_dialog_set` (7.9 s: one
  fake-agent verify per recorded set).
- A test that boots through the stamped home holds one fake-agent verify and reads 4.2 s to 5.6 s here.

## The `verify_window_` class, against its 45 s kill
| test | default run | instrumented run |
|---|---|---|
| `verify_window_without_screens_fails_every_interactive_row` | 20.62 | 20.60 |
| `verify_window_paste_hint_past_the_gate_maximum_still_stamps` | 9.87 | 9.89 |

The new case reads 9.9 s (predicted 10 s to 11 s): one fake-agent verify and the 6 s hold.

## Reading
- No bound in `.config/nextest.toml` was moved; the file is untouched.
- Not measured here: the same tests at HEAD on this host (no archived run of HEAD is left, `target/run-archive`
  keeps the newest ten), and any CI runner. What three more turns cost on `windows-2025`, `macos-latest` and
  `ubuntu-latest` is read from the operator pass's CI run; the test to watch there is
  `fake_agent_dialog_replay_matches_every_recorded_dialog_set`, the one case that runs verify once per recorded
  set under the 20 s kill.
