# The first full block: two reds, and their cause

**Entries 16 and 18 read red in the first full run of the gate block (2026-10-06T21:17:15Z to 21:20:10Z). The
cause is measured and is not this tree: another project's build on the same host wrote more than 10 GB of linked
binaries to the same volume inside exactly those two entries' windows. The same entries read green three times
afterwards with no such write under way, the last time inside a second full run of the block, 23 green of 23.**

## The reds
| entry | window (UTC) | reading |
|---|---|---|
| 16, the five binaries | 21:17:39 to 21:18:20 | exit 1; `117 tests run: 93 passed (5 slow), 2 failed, 22 timed out`, 39.8 s. The 87 tests that spawn no process passed in milliseconds. Every test that spawns `viola` or the fake agent stalled: 20 `cli_verify` cases, both `contract_fake_agent_drift` replays and `contract_ledger_probes_pass_over_every_stamped_set` hit the 20 s kill (`TIMEOUT [  20.0…s]`); `fake_agent_without_exit_no_eof_releases_stdout_at_exit` failed at 7.1 s (`timed out waiting for start`, no receipt line); `wrapper_boot_exiting_before_ready_fails_as_exited` failed at 18.0 s (`wrapper builder not ready`, `starts w1 c0`) |
| 18, the wrapped send | 21:18:29 to 21:18:38 | exit 1; `FAIL [   9.081s]`: `wrapper builder not ready`, `starts w1 c0 r0 snapshot false beat false` — the boot fixture, before any send |

- The stall has one size per test, about 18 s to 19 s, at its start: `verify_an_unreadable_version_refuses_and_
  writes_no_stamp` passed at 19.0 s (it spawns two processes and waits on nothing);
  `verify_window_without_screens_fails_every_interactive_row` passed at 39.4 s against its usual 20.6 s;
  `verify_window_paste_hint_past_the_gate_maximum_still_stamps` passed at 29.0 s against 9.7 s.
- No assertion of this chunk failed. No test failed on a value: each red is a start that did not happen in time.
- In the same block run, entry 17 (21:18:21 to 21:18:27) read green, entry 19 (the whole default selector, the
  same tests among its 326) read green with 20 slow tests, and entry 25, the native pre-push gate, read green.

## What was read, two-sided
The host is shared: four other builder sessions run on it. Files larger than 5 MB written under a sibling
project's cargo build dir on the same btrfs volume (`~/dev/projects/escher/target`), by modification time, inside
each window of this chunk's runs:

| window (UTC) | this chunk's run | reading here | the other build's writes |
|---|---|---|---|
| 21:12:30 to 21:13:10 | step 13's thirteen entries | green | 0 files |
| 21:17:15 to 21:17:39 | block 1, entries 1-15 | green | 8 files, 484 MB |
| 21:17:39 to 21:18:21 | block 1, **entry 16** | **red** | **88 files, 10 606 MB** |
| 21:18:21 to 21:18:28 | block 1, entry 17 | green | 12 files, 231 MB |
| 21:18:28 to 21:18:39 | block 1, **entry 18** | **red** | **40 files, 5 230 MB** |
| 21:18:39 to 21:19:22 | block 1, entry 19 | green, 20 slow | 45 files, 6 234 MB |
| 21:19:28 to 21:20:11 | block 1, entry 25 | green | 0 files |
| 21:22:04 to 21:22:28 | entries 16 and 18 alone | green (117 passed, 1 slow; 1 passed in 0.46 s) | 0 files |
| 21:24:08 to 21:24:58 | entries 1-7, 10-18 again | green | 0 files |
| 21:25:59 to 21:28:13 | block 2, the whole block | green, 23 of 23 | 0 files |

- The files are that project's linked test binaries, 120 MB to 220 MB each (its `seven_guis_native` at 21:17:28Z,
  then dozens under `deps/` from 21:17:55Z on). The rate inside entry 16 is about 250 MB/s and inside entry 18
  about 520 MB/s; inside entry 19, which passed slowly, about 145 MB/s.
- A later file overwrites an earlier one's time, so the table can under-count a window but cannot invent a write.
- What this tree's tests do at a start, read from a quiet run by a one-second sampler of process states: about
  25 `viola` processes sit in uninterruptible wait at once for one or two seconds, beside the volume's flush
  worker, each time a batch of tests starts (read again in the second full block: 25 to 28 at 21:26:21Z,
  21:26:55Z and 21:27:39Z). Under the other build's writes that wait became 18 s. Which write of the start it is
  (the test home's pinned copy of the binary is the large one) was not separated here.
- The kernel log holds no line for the window but the firewall's. Swap (zram) was in use on the host throughout.

## A hypothesis read and dropped
First reading: this chunk's own relink after step 6's control (two rebuilds of everything that depends on
`viola-agent-claude`, 25 s before the block) had filled the volume's writeback. Forced open, not sampled: the same
file was touched twice, each time followed by the harness build, then entries 1-7 and 10-18 in block order
(21:23:57Z to 21:24:58Z). Each rebuild took 4.3 s and every entry read green. So this tree's own builds do not
make the stall.

## What follows
- The outcome on the chunk's own gates is green: the second full run of the block reads
  `entries 28 · green 23 · red 0 · not-run 5` (the round's two legs, fired once at step 14, and the three
  operator entries).
- The red's cause predates the chunk and is the host's: any gate that boots many test homes at once is exposed
  to it while another session links on the same volume. Nothing in this chunk changes that exposure. It is
  listed for the wrap to place; the builder windows on this host are the operator's to schedule.
- **What the kills left behind, and its removal.** The census after the second block (21:30Z) found 21 probe dirs
  at the repository root beside the operator desk's `.viola-verify-2095228/`: 17 `.viola-verify-<pid>/`, 2
  `-dialogs` and 1 `-plan` among them, all mode 0700, all dated 21:17:59Z to 21:18:02Z, each owner pid dead, 20
  empty and one holding an empty `plans/`. They are the probe dirs of the `viola verify` processes whose tests
  the 20 s kill ended inside entry 16: a runner kill is not one of verify's exit paths, so its guard never ran.
  No process had a cwd in any of them and no process of this repository's binaries was running. They were removed
  with `rmdir` (which takes only an empty dir); `.viola-verify-2095228/` was not touched. No transcript dir for
  them exists under `~/.claude/projects/` (the fake agent writes none).
- The first run's logs of entries 16 and 18 were overwritten by their re-runs (one log per entry number per run
  dir); the readings above were taken from them before that.
