# The forced window, red without the guard and green with it (plan step 5)

Read 2026-10-27 on the Linux dev host (tempdir on the `/tmp` tmpfs, mounted without `noexec`), run dir
`.andromeda/runs/2026-10-27T02-08-46-implement/`, last reading at 02:16:48Z. Command each time:
`cargo nextest run -p viola-state [--no-fail-fast] -E '<filter>'` under a 600 s `timeout`, output to a file read
by its PASS / FAIL / panic lines.

The case: `fs::tests::place_private_new_placed_copy_execs_at_once` — the bytes of the host's `/usr/bin/true`
placed at 0700 through `place_private_new_with`, whose `placed` hook spawns the placed path directly inside the
window (right after the copy appears, before the call returns).

1. **Green, with the guard** (the temp file closed by `into_temp_path`, then `TempPath::persist_noclobber`):
   `-E 'test(/(^|::)tests::place_private_new_/) | test(/(^|::)tests::pin_/) | test(/(^|::)tests::place_absent_/)'`
   → exit 0, `Summary [ 0.016s] 13 tests run: 13 passed, 294 skipped`, the case `PASS [ 0.010s]`.
2. **Red, the guard removed inside `place_private_new_with`** (the placement made by
   `NamedTempFile::persist_noclobber` with the write handle still open, the handle dropped only after the hook;
   the edit read back at line 339 `match tmp.persist_noclobber(path) {` before the run):
   `-E 'test(/(^|::)tests::place_private_new_/)'` → exit 100, `Summary [ 0.005s] 4 tests run: 3 passed, 1 failed,
   303 skipped`, the case's own panic line:
   `panicked at crates/viola-state/src/fs.rs:452:49: the placed copy spawned: Os { code: 26, kind: ExecutableFileBusy, message: "Text file busy" }`
   — `ETXTBSY`, raw 26.
3. **Green, restored** (read back at line 339 `match tmp.into_temp_path().persist_noclobber(path) {`):
   the same filter → exit 0, `Summary [ 0.005s] 4 tests run: 4 passed, 303 skipped`, the case `PASS [ 0.004s]`.

macOS gives no `ETXTBSY`, so the case has no red leg there (its doc comment says so); its Windows form
`place_private_new_placed_copy_runs_at_once` (`whoami.exe`) is green-only on `windows-2025`, its red leg unmeasured
(no Windows host).
