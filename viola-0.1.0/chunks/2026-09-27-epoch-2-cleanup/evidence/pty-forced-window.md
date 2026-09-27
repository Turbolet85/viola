# viola-pty forced-window reproduction (plan step 2)

Subject: the recurrence watch `tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code`, whose one CI red
(run 36296402785, windows-2025 under llvm-cov) lost the key written right after `resize(120x40)`.

## The forced window

- `pty_child_entry` gained `CHILD_MODE=hold`. After it reports the first byte, the child sleeps `CHILD_HOLD` = 750 ms before its second
  `read_exact`. The hold only widens the window and synchronises nothing (testing.md 2026-09-27).
- New test `tests::spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading`: spawn at 100x30, write `x`, wait for
  the `byte 78` line (the child is now holding), `resize(120x40)`, write `y`, then expect `byte 79 size=120x40`, `restored=true`
  and exit code 3. The resize and the second key both reach ConPTY while the child is out of its read.
- Hypothesis H1 under test: the red lost the key because the resize and the key reached ConPTY before the child re-entered its read.

## Reading — this host, before any split (tree `a0e6506` + step 2 only)

The gate entry `for i in $(seq 20); do bash scripts/agent-run.sh run --unit --filter
'test(=tests::spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading)' …; done`, run by hand at
2026-09-27 ~14:55Z: **20/20 green.** Each run's document read `ok:true`, `nextest-unit` passed 1 / failed 0, and nextest printed
one `PASS` line for the test. Test durations (s): 0.795 0.788 0.806 0.838 0.791 0.792 0.790 0.790 0.843 0.787 0.789 0.789 0.793
0.792 0.790 0.791 0.792 0.789 0.792 0.795. That is the 750 ms hold plus about 40–90 ms.

## Disposition (per plan step 2)

- **H1 is falsified on this host.** A key that reaches ConPTY right after a resize, while the child is not reading, still arrives
  once the child reads. The test stays as a standing witness, and the watch keeps its expiry (3 consecutive green CI runs).
- **H2 is owed to the operator, whatever this reading says.** The platform record (rstudio/rstudio#18884) describes "a keystroke
  typed within ~50ms of a process_set_size RPC was lost on Windows CI (ConPTY + MSYS bash)", below the application. The watch
  test already writes its key inside that window and ran 60/60 green on this host, so a host run does not force H2. If H2 is real,
  it is a product question, because viola forwards both resizes and keys ("the human always wins"). This chunk builds no fix for it.
- The gate entry re-runs the 20 iterations at the chunk's gate pass, after the pump split (step 3).
