# Operator pass — 2026-09-25-pty-wrapper-on-windows

**Chunk base: `0253507`** (`02535078faca200f099920313eb52dbeec424a88`). HEAD's history holds exactly ONE operator
pre-CI commit, `17ea8c7`, so the wrap's "read from that commit's PARENT" resolves to `0253507`; the changed-file set is
the whole chunk (`git diff 0253507..HEAD`).

## Commits (plain fast-forward pushes, no force)
| sha | subject | pushed | CI run |
|---|---|---|---|
| `17ea8c7c6d6593ec47780f706fc5ead8089b7d67` | chore(…): operator pre-CI commit, for the run this chunk's verdict reads | `0253507..17ea8c7` | **36165685381** (full-diff: mutation base `github.event.before` = 0253507) |
| `c05e6e25796167c61ec3d519014bbbd1439a1695` | fix(…): unix exit-no-eof holder leaves the foreground group | `17ea8c7..c05e6e2` | **36166907442** (fix-only: mutation base = 17ea8c7) |

## Run 36165685381 (17ea8c7) — the chunk's full-diff mutation verdict
- Entry 27 (push): exit 0, `0253507..17ea8c7`.
- Test jobs, measured by the overseer from the job logs: `test (ubuntu-latest)` and `test (macos-latest)` RED on one
  test each (355/356): `viola::tui_pty_seam pty_exit_is_read_on_the_handle_while_the_output_is_held` —
  "the output ended before exit: nothing was holding it" (0.009 s ubuntu, 0.017 s macos). Windows passed.
  Entry 28 on this run is therefore NOT all-green.
- Mutation legs + union: PENDING at the time of writing — to record: the ubuntu leg's and the union's verdict, and
  whether the 5 `#[cfg(unix)]` survivors of the local Windows leg (`crates/viola-pty/src/lib.rs` HostTerminal::enter
  unix termios ×4, host_size ioctl ×1) read caught on ubuntu and in the union.

## Cause of the Unix red (read from code; Unix not measurable on this host)
- portable-pty 0.8.1 `src/unix.rs:57-58` sets FD_CLOEXEC on its own master/slave fds; the child gets the slave on fds
  0/1/2 via `as_stdio()` dups (not close-on-exec); `close_random_fds` closes fds > 2 only. The holder inherited fd 1.
- `src/unix.rs:206-236`: the child resets SIGHUP to SIG_DFL, `setsid()`, `TIOCSCTTY` → the fake agent is its tty's
  session leader; the holder shared its (foreground) process group. Session-leader exit → SIGHUP to the foreground
  group → holder died → master EOF.
- Linux (kernel semantics, not measured here): a PTY driver is not vhung-up on session-leader exit, so a holder outside
  the foreground group keeps the slave open. macOS: HYPOTHESIS — XNU revokes the controlling tty on session-leader
  exit, which would end the output whatever the holder's group.
- This host: no Unix runtime (WSL has only `docker-desktop`, stopped; Docker engine not running; no images). No
  Docker start, per the operator.

## Fix (c05e6e2), test-side only
- `src/bin/viola-fake-agent.rs`: the `--exit-no-eof` holder spawns with `process_group(0)` on Unix and writes a
  `hold {pid}` receipt at start.
- `tests/tui_pty_seam.rs`: asserts the `hold` receipt before the held-output check (red message now names which case).
- Local gates on the fix: fmt 0, clippy 0, `tui_pty_seam` 2/2, `cli_fake_agent` 32/32, `tui_passthrough` 3/3,
  `run --integration` ok:true 112/112 (Windows).
- macOS decision (operator, option a): keep the held-output assertion on every OS. If ONLY macOS stays red with `hold`
  present, scope it to Windows + Linux in a follow-up that cites run 36166907442.

## Run 36166907442 (c05e6e2) — recorded 2026-09-25, run completed `failure`
- Entry 28 (`…/commits/c05e6e25796167c61ec3d519014bbbd1439a1695/check-runs`, unique conclusions): `failure,success` —
  RED. The only non-success job: `test (macos-latest)` (job 108176770703), 355/356:
  `viola::tui_pty_seam pty_exit_is_read_on_the_handle_while_the_output_is_held` panicked at `tests/tui_pty_seam.rs:47:5`
  "the output ended although the holder started (hold receipt present)". `test (ubuntu-latest)` and
  `test (windows-2025)` green.
- Entry 29 (mutants jobs on this sha): `success,success,success` — both legs and the union green (fix-only diff).
- MEASURED by this run: on Linux the holder outside the foreground group keeps the slave open (ubuntu green); on macOS
  the output ends at the session leader's exit although the holder is alive in its own process group (`hold` present)
  — the revoke hypothesis now has a measurement. Per option (a): the held-output assertion is scoped to Windows +
  Linux in a follow-up citing this run.

## For the wrap (operator-directed CARRYs / notes)
- HYPOTHESIS (unmeasured, not a finding): Rust std's Windows console stdin may treat a leading `^Z` as end of input,
  which would stop that key reaching the child. Carry it, labelled as a hypothesis, on the next route entry that feeds
  `viola run` stdin.
- The 16:44:58 session end was a hand-closed window (founder's account). The `contract_diag_schema` Ctrl-C race is a
  hang found and fixed, not the cause; any link to the killed exit-137 call is a hypothesis (friction-log retraction
  `2026-09-25T17:12:00Z-a`).
- Entry 26 (plan form, no `--leg`) cannot pass with `#[cfg(unix)]` bodies in the diff; read in the CI form
  (`--leg windows-2025`: ok:true, 186 mutants, 151 caught, 30 unviable, 0 timeout, 5 cfg(unix) survivors deferred).
