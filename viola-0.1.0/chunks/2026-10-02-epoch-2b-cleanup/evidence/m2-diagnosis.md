# M2 — the in-repo suite red: diagnosis (in progress; stopped at the disk guard)

/implement run `2026-10-02T19-38-47-implement`, 2026-10-02. Every reading is from this session. Paths are repo-relative,
or `%TEMP%` / "C: scratch" (the session scratchpad on C:). No absolute path is recorded.

## Reproduction (in-repo, D:)
- `run --integration --filter 'binary(hook_fail_open)'`: 10 passed, 5 failed (`spine_bound` cases 01/02/08/09 and
  `forced_panic`). The failure text is `took 3.67 s` / `6.64 s` against `SPINE_BOUND` 1 s, or a `watch.rs` deadline.
- P5's run-archive 420 (5-binary witness set): 39 failed, 23 passed. Every first panic sits at a `watch.rs` deadline or the
  spine bound. The failing tests ran 17–66 s against `WITHIN` 7 s.

## Suspects varied one at a time (plan step 2)
| arm | change | hook_fail_open |
|---|---|---|
| (a) | `target/e2e-home` renamed to `target/e2e-home.m2-held`, then restored | 10 pass / 5 fail: same set, same timings |
| (b) | `TMP`/`TEMP` set to a fresh empty dir for that one process tree | 10 pass / 5 fail: same set, same timings |

Neither (a) nor (b) explains the red. Run alone (`--exact`), a home-making case still costs about 1.1 s, against 40 ms for a
case with no home. That holds with the 4 804 homes present (1.11 s median ×5) and with them held aside (1.08 s median ×5),
so the owner sweep is not the cost. Arm (a) leaked one home (`viola-test-*`, `owner.json` present). The owner-record sweep
reclaimed it once its owner was gone. Its name was merged back into `e2e-home` before the restore.

## (c) The tree location — the cause
A temporary timing probe in `TestHome::new` (reverted; `tests/support/home.rs` is byte-equal to HEAD) split one home-make,
run alone: sweep about 100 ms, `tempdir_in` 80–97 ms, `process_start` 8 ms, and `write_owner` about 205 ms (one 30-byte
file).

Small-file latency, median of 40 (or 20) operations, same python script, same minute:
| location | mkdir | small write | rename-over | remove 40 dirs |
|---|---|---|---|---|
| D: `target/agent-run/…` | 79–113 ms | 158–227 ms | 360–500 ms | 10.2–13.0 s |
| D: repo-sibling scratch · `target/` · `.git` | 74–104 ms | 149–164 ms | 348–375 ms | 4.8–5.1 s (20 dirs) |
| C: scratch | 0.20 ms | 0.28–0.29 ms | 0.30–0.31 ms | 10.6–11.0 ms |

D: is a ReFS Dev Drive (`DevDrive` label) on a file-backed virtual disk. C: is NTFS on NVMe. During the burst D:'s disk
counters read 99 % idle, 0.2 ms per write and queue 0.2, while the calling process burned the CPU. So the cost is
synchronous per-operation work in D:'s filesystem stack, not I/O. Defender reads real-time on, Dev Drive performance mode
status 1. Its filter list and exclusions are unreadable without admin.

**The equality.** The same `target/harness/debug/viola.exe verify -- <fake agent> --cli-version 2.1.283 --fixtures
fixtures/claude` (the stamped-home fixture step, `tests/support/verify.rs`) takes:
- home on D: — 17.43 s median (12.22–21.54), 3/3 stamped;
- home on C: — 0.86 s median (0.79–0.93), 3/3 stamped.

`wait_bounded` kills it at `WITHIN` = 7 s. So the suite's top panic, "viola never exited", is the stamped-home `viola
verify` doing its home's metadata operations on D:, at 75–500 ms each, which pushes it past `WITHIN` with no test load
at all. The other deadlines (`wrapper builder not ready`, the spine bound, the start waits) are the same operations under
the wrapper and the hook.

A third reading, the operator's (overseer1, relayed 2026-10-03): `cargo clean` on this repository removed 80.1 GiB in
241 256 files in 5 h 48 min, about 86 ms per file removed. That is the same order as the per-operation figures above, on a
workload no test drives. The clean also removed `target/e2e-home`, so every arm below runs from a cold build.

## Witness (two-sided, same day)
| pair | order | arm | result |
|---|---|---|---|
| 1 | 1st | C: copy of the tree (`git ls-files -co --exclude-standard`, 2 819 files), 5-binary witness set | **62 passed, 0 failed, `"ok":true`**; slowest test 4.8 s |
| 1 | 2nd | in-repo D:, same command | first attempt stopped at the operator's disk guard (D: 39.83 GB < 40 GB) while it was still relinking (856 s, no test ran). Retaken 2026-10-03T02:55:53Z after the operator freed D: (150.7 GB) and cleaned `target/`: **22 passed, 40 failed**, 2 739 s including the cold build |
| 2 | 1st | in-repo D:, same command, warm | **22 passed, 40 failed**, 92 s (2026-10-03T03:41:34Z) |
| 2 | 2nd | C: copy, same command, warm | **62 passed, 0 failed, `"ok":true`**, 16 s (2026-10-03T03:43:07Z) |

The red-with / green-without split holds in both orders. Pair 2's D: and C: arms ran the same 62 tests one after the
other: 92 s with 40 red against 16 s all green. Every first panic in both D: arms is a `watch.rs` deadline or the spine
bound. Both D: JUnits are full documents with different failure orders, so neither is a stale copy. The run-to-run grading
the host-reds CARRY describes does not appear here: both D: arms read 40 failed.

**Leaks, as measured by the same runs.** After D1 and D2, `target/e2e-home` held 0 entries, and `%TEMP%` held 0 new
`.tmp*` dirs against the step 1 snapshot. Two full root runs with 80 failing tests between them left no home and no temp
dir. The historic remnants (4 768 ownerless homes, all removed by the operator's clean) did not reproduce in those two runs.
Later D: runs did leave them, at 0–3 per run. That class appears only with D:'s deadline failures (`leaks.md`, "Leak B on
D:'s red runs").

**M2's standing.** The cause is named with a two-sided witness. It is outside the repository, so M2 stays an **open red**
at this chunk's wrap. Fixing the volume or moving the test homes is the founder's decision (overseer, 2026-10-03, carried
in the overseer's FOR DISCUSSION).

## What a fix by cause means here
The cause is outside this repository: the host volume's metadata latency. Nothing in the test or product code makes D:
slow, and the suite reads green off it. Under the plan's rules:
- raising `WITHIN` or another bound is rejected (testing.md 2026-09-28);
- moving test homes off `target/e2e-home` changes the test-data contract (test-plan §3, testing.md §Test data), so it is
  not this chunk's to write;
- the leaks (steps 4–5) are real but are not this red's cause: arm (a) green-lights nothing.

Owed to the operator / founder: why D: costs 75–500 ms per metadata op (an admin read of `fltmc instances -v D:`, the
Defender Dev Drive trust state, and the volume's fill: 87 %), and whether the suite moves to a fast volume.

## Concurrent host load (not this repo's)
Throughout the session: four viola-lab prototype `viola.exe` sessions and two `viola wait` calls, a `pulse-app` nextest
build, three rust-analyzer instances. None was stopped. A `du` walk this session started overlapped D:'s first ~2 minutes
of the stopped arm and was stopped by its exact pid.
