# Item (8): who cuts the `viola` profile short (plan step 10)

Each reading below was written before the next step was taken.

## (a) The census's second form

`scripts/profraw-census.sh --case <binary stem> <test name> <runs> <workers> [<name>]` runs one integration case of the
instrumented build `run --coverage` leaves (`target/llvm-cov-target/debug/deps/<stem>-<hash>`, the newest that lists
the test), `<runs>` times over `<workers>` loops. Each run's profiles are named as the coverage run names them
(`r<i>-%p-%<nproc>m.profraw`, so the binary signature is in the name) and each is offered alone to the toolchain's
`llvm-profdata merge --sparse -o /dev/null`. A refused file is kept under `refused/` and printed with its signature and
its size beside the largest size its signature reached in the census. Verdict: exit 0 only when every run passed and no
profile was refused. The first form (the fixed unit test) is unchanged. A dev-host script: no CI, `pre-push` or harness
hook.

## (b) The red side

### P5's own file, read first

- `viola-3929550-11199965415080257827_14.profraw`, 90 112 B, kept in this session's scratchpad `profraw-p5/` (sha256
  `9de3dda876dfb2327536c43fcbf96378c47a3d87f7ae0aae6fe7423f4f2fee61`, the same as P5's copy).
- Its version-10 header: 1 455 data records from byte 160 to 93 280, then the counters from 93 280 to 120 656. The file
  ends at 90 112 B = 22 whole 4 KiB pages, inside the data records (1 405 of 1 455 whole). No counter survives, so the
  counter method (session-learnings, 2026-10-04) cannot name its writer.
- Both truncated files on record end on a page boundary: this one, and the switch chunk's 118 784 B (29 pages).

### The census over the kill-shaped case

- Case: `chaos_wait_vanish chaos_wait_parked_wrapper_killed_exits_21`, the switch chunk's closest candidate. It boots
  an unstamped wrapper, parks a `viola wait` on it, then sends the wrapper `SIGKILL`.
- Build: the instrumented tree P5's `pre-push` left at HEAD `d35644a58909`, the untouched tree
  (`target/llvm-cov-target/debug/viola`, 104 194 856 B, written 2026-10-20T08:48:33Z).
- Call: `bash scripts/profraw-census.sh --case chaos_wait_vanish chaos_wait_parked_wrapper_killed_exits_21 200 8
  e5c-chaos-wait-1`. `df -h /tmp` before: `tmpfs 32G 18G 14G 57%`.
- Result: `runs 200 · passed 200 · profiles 726 · refused 17`. Basis: 200 runs over 8 concurrent loops, every run's
  profiles offered to `llvm-profdata merge` one by one. All 17 refused files carry the `viola` binary's signature
  `11199965415080257827`, against a whole size of 146 344 B:

| size (B) | files | where the cut falls | writer (counter method) |
|---|---|---|---|
| 145 232 | 6 | names section (every counter whole) | `viola hook`: `main` → `cmd::dispatch` → `cmd::hook::{hook, handle, decided, deliver, spine_deadline}` → `viola_channel::client::Client::notify`, and the `viola_agent_claude::hook` normaliser (`HookEvent::{from_arg, kind, as_str, is_dialog}`, `data_of`, `normalise`, `known_fields`); 70 nonzero functions, all named |
| 118 784 | 1 | counters | `viola hook`: the same chain to `Client::notify`; 49 nonzero functions, all named |
| 94 208 | 2 | the first counter page | only `obs::viola_obs_init` and the role-file open: any `viola` role |
| 90 112 | 6 | data records | not named: no counter survives |
| 4 096 | 2 | data records | not named |

- The census dir `target/profraw-census/e5c-chaos-wait-1/` keeps the 17 files under `refused/`. The attribution was
  read by a scratchpad copy of the switch chunk's script with a guard for a cut inside the data records
  (`$TMPDIR/{scratchpad}/profraw_name2.py`).
  The complete sibling was a 146 344 B profile of the same signature from P5's coverage run.
- Reading: every refused file that keeps its counters (7 of 17) is a `viola hook` that had already delivered its event
  and was cut off in its exit-time profile write. No refused file names the wrapper (`cmd::run`). The case kills the
  wrapper, whose PTY then closes, and the fake agent is that terminal's session leader. Its hooks run in its own
  process group, the terminal's foreground group (`src/bin/viola-fake-agent.rs` `run_receipted`). The hangup reaches
  them there, while the fake agent's stdout holder already leaves that group for exactly this signal
  (`hold_inherited_stdout`). The writer matches the research hypothesis. The hangup as the killer is inferred, not yet
  read: (c) tests it by taking the hooks out of that group.
- The other `Wrapper::kill` callers were not run: the first case reproduced the class.

## (c) The hooks out of the foreground group: the same census

- Change: `src/bin/viola-fake-agent.rs` `run_receipted` spawns each command (every hook, and the status line command
  that shares the function) with `std::os::unix::process::CommandExt::process_group(&mut cmd, 0)` on Unix, as
  `hold_inherited_stdout` already does. Nothing else changed in the fake agent.
- Build: `bash scripts/agent-run.sh run --coverage` on the chunk's tree, `"ok":true`, suite `coverage` 2 980 passed 0
  failed, its merge refusing nothing. `target/llvm-cov-target/debug/viola-fake-agent` written 2026-10-20T09:10:01Z,
  after the edit (09:09:36Z).
- Call: `bash scripts/profraw-census.sh --case chaos_wait_vanish chaos_wait_parked_wrapper_killed_exits_21 200 8
  e5c-chaos-wait-2`, the host otherwise idle as for (b). `df -h /tmp` before: `tmpfs 32G 19G 14G 59%`.
- Result: **`runs 200 · passed 200 · profiles 800 · refused 0`**, exit 0.

| census | runs | passed | profiles | per run | refused |
|---|---|---|---|---|---|
| (b) HEAD's build | 200 | 200 | 726 | 3.63 | 17 |
| (c) the process-group build | 200 | 200 | 800 | 4 exactly | 0 |

- The profile count moved too. Before, 74 processes of 200 runs left no profile at all: killed before their exit-time
  write began. After, every run leaves the same four. Over the same runs, the hangup was the killer of every cut-short
  and missing profile this case makes.

## The finding

- **The killer is named:** the terminal hangup that follows a test killing the wrapper. The wrapper's PTY closes, and
  the fake agent, that terminal's session leader, runs its hooks in its own process group, the terminal's foreground
  group. A `viola hook` the hangup reaches after `Client::notify`, in its exit-time profile write, leaves a page-cut
  profile that `llvm-profdata merge` refuses ("file header is corrupt"), or none.
- **The fix is the fake agent's** (test-only, behind `fake-agent`): its commands leave the foreground group. No product
  code changed.
- Basis and limits: one kill-shaped case, 200 runs per side, 8 workers, this host. P5's own file (90 112 B) is not
  attributable by counters. Its size and page-cut shape match six of the reproduced files exactly, which is a match,
  not a reading of its writer. The WATCH's class is now explained. The `pre-push` runs of this chunk read it from here
  on (plan §Test Commands: after a named and fixed cause, a red of that shape is this chunk's red).
