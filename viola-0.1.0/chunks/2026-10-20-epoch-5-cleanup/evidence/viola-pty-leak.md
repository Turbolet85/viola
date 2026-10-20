# Step 7: the viola-pty test guard under the caught `try_wait` mutant

## What was changed in the tree

- The guard `impl Drop for Child` in `crates/viola-pty/src/lib.rs`'s test module now calls `self.pty.kill()`
  unconditionally, its error ignored. Before, it killed only when `try_wait()` read `Ok(None)`.
- Both PTY children the tests spawn from the test binary (`spawn_child_entry`, `close_hangs_up_a_live_child`) now run
  a per-test copy of it in the case's own `tempfile` dir (`copied_test_binary`). The `std::process::Command` child of
  the piped-stdin case is unchanged.

## The remove-the-guard reading (Linux dev host, 2026-10-20)

- Two scratch clones of HEAD `d35644a58909aec12c4898e5a066bcfca5a8ebd2` in this session's scratchpad. In each, the
  product `PortablePty::try_wait` was hand-replaced by `Ok(Some(1))` (the caught mutant before the dispatch's unviables,
  its log line 441), by `$TMPDIR/{scratchpad}/plant_try_wait.py`.
  - `clone-pty-old`: the old guard (HEAD), target dir `target/e5c-pty-old`.
  - `clone-pty-new`: the chunk's diff applied first, target dir `target/e5c-pty-new`.
  - Both build the test binary `viola_pty-77dd4bdbf6cc1891`. The census keys on that name, so it holds a child run
    from the target dir and one run from a tempdir copy alike.
- The readings, in order (`gate.py census`, trail `.andromeda/runs/2026-10-20T08-57-19-implement/gate-2026-10-20-epoch-5-cleanup.json`):

| # | reading | result |
|---|---|---|
| 1 | census before either run (`--args` both target dirs) | `census: none running` |
| 2 | old guard: `cargo nextest run --package viola-pty --no-fail-fast` | `40 tests run: 35 passed, 5 failed` (the mutant caught) |
| 3 | census `--args viola_pty-77dd4bdbf6cc1891` | `census: none running` |
| 4 | new guard: the same run | `40 tests run: 35 passed, 5 failed` |
| 5 | census `--args viola_pty-77dd4bdbf6cc1891` | `census: none running` |

- No row was found, so no process was signalled.

## What the reading shows, and what it does not

- **The plan's expected reading (leaked children on the old guard) does not hold on Linux.** On Unix, dropping the
  `PortablePty` closes the master, and the kernel hangs up the child's terminal. The child dies whether or not the
  guard kills it: the module's own `close_hangs_up_a_live_child` pins exactly that. So on this host the old and new
  guards leave the same nothing, and the reading cannot tell them apart.
- The leak the step closes is the Windows mechanism research names, still a hypothesis: a running image there cannot
  be replaced, and the dispatch's `mutants (viola-pty)` turned every later mutant `unviable` right after the
  `try_wait` mutants were caught. That needs a Windows host. It is owed to the next boundary's Windows leg
  (`unviable` ≤ `caught` in `mutants (viola-pty)`), as the plan already places it.
- Acceptance «shows leaked children … on the old guard and none on the new one» is therefore not met as written, by a
  measured host fact. This is raised in the outcome as a premise disproof for the wrap, not worked around.
