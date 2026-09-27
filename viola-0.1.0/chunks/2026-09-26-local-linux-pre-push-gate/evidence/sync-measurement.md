# Sync, as implemented (implement, 2026-09-26)

Research M8 chose clone + fetch + temp-index patch by measurement (first 4.47 s, repeat ≈0.9 s, vs rsync 9.85 s /
2.74 s). The implemented `pre-push` sync, read from its own documents:

| run | `sync.ms` | `sync.files` | `sync.tree` (clone `write-tree` = the Windows temp-index tree) |
|---|---|---|---|
| entry 11, run 1 (first: includes the clone) | 6254 | 60 | `26db20d5711f8b1593b8109133f0e6ac4c3740de` |
| entry 11, run 2 | 1303 | 61 | `7a46d0f3f8411b4a6eb2c746054f9f05254a3325` |
| shim-direct run | 1208 | 75 | `c1602cdcae27cba4b1bc66b1ebd8bd950b66cf72` |

- Every run's tree ids were equal on both sides, or the gate would have stopped at `sync-mismatch`: the uncommitted
  working tree — this chunk's own unstaged Rust edits and untracked new files included — reached Linux each time. The
  file counts move because each gate run leaves new untracked run-dir files in the Windows tree.
- `head` = `a69c5efb082f1d04f067073a5638618fd709be7e` every run, the Windows HEAD.
- Cold vs warm: the first `pre-push` took 71.43 s to its `linux-tests` stop (clone + first instrumented build + suite);
  the warm re-run 33.42 s.
- The Linux run's environment is `env -i` with `HOME` and `PATH` only (names; no values recorded).
- Native-host proof of the sync mechanism: `pre_push_sync_reproduces_a_dirty_tree` (modified / untracked / deleted /
  ignored files, the real index untouched, a stray clone-side file removed by `clean`).
- Cosmetic: `git apply` prints whitespace warnings for run-dir logs in the patch; the apply succeeds.
