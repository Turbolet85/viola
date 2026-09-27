# The Linux red `pre-push` found — `tui_host_resize_reaches_the_child` (implement, 2026-09-26)

**Status: RESOLVED (2026-09-27) — cause MEASURED by a forced window, fixed in `viola-pty`, three consecutive green
`pre-push` runs. See §Experiment A and §Fixed at the end; the sections between are the history as it happened.**

## Reading
- `bash scripts/agent-run.sh pre-push` stopped at `stage:"linux-tests"` three times out of three, each with one failure
  of 399: `viola::tui_passthrough tui_host_resize_reaches_the_child`, panicking at `tests/tui_passthrough.rs:111:9`
  "the resize never reached the child", at 10.07–10.09 s (the test's `EXIT_WITHIN` = 10 s,
  `tests/support/outer_pty.rs:15`).
  - gate entry 11, run 1 (cold clone, first build): red.
  - gate entry 11, run 2 (warm): red.
  - `bash scripts/agent-run.sh pre-push` from the shim directly, no gate.py / GNU `timeout`: red.
- CI's `test (ubuntu-latest)` passed the same test on a69c5ef (run 36272899441, all 15 green).
- This chunk touches neither the test nor the code under it (`src/cmd/run.rs`, `crates/viola-pty`).

## What reproduces green (every run on this host, in the same clone, the same tree)
| probe | runs | red |
|---|---|---|
| the test alone, `env -i` HOME+PATH (the gate's env) | 3 | 0 |
| the test alone, `TERM=xterm-256color` | 3 | 0 |
| the full `run --coverage` suite, `env -i`, launched from Git Bash | 2 | 0 |
| the test alone through the gate's exact WSL argv, stdin null and inherited | 4 | 0 (the `failed:1` there is the coverage floor on a one-test filter, `llvm-cov-exit-1`) |
| the full suite through the gate's exact WSL argv, stdin null, stdout piped | 2 | 0 |
| the full suite after every `.rs` touched (forced rebuild) — this chunk's tree | 2 | 0 |
| the same on a clean worktree at HEAD a69c5ef (the two-sided control) | 2 | 0 |
| the full suite from a native Windows `CreateProcess` of wsl.exe (python), console inherited / `CREATE_NO_WINDOW` | 4 | 0 |
| `pre-push`'s clone-side sync replayed (reset --hard, clean, apply, add -A, write-tree), then the test ×2, two rounds | 4 | 0 |
| the test alone on HEAD's worktree, 20 unloaded + 20 with every core saturated | 40 | 0 |
| `du -sb` over `target/` (the cache stage), then the full suite | 2 | 0 |

So: red 3/3 inside `pre-push`, green 68/68 in every isolated reproduction of its parts. Nothing isolated here
reproduces it outside the gate.

**Instrumented vs uninstrumented (overseer question, answered from the runs above, no new probe):** 18 of the 68
used the instrumented `--coverage` build `pre-push`'s linux-tests stage runs — 14 full-suite (`run --coverage`:
coverage-repro 2, launch-full 2, race-probe 4, CreateProcess 4, du-replay 2) and 4 single-test
(`run --coverage --filter`, launch-shape) — all green. The other 50 were uninstrumented `cargo nextest` builds:
the 6 single-test env probes, the 4 sync-replay runs and all 40 stress runs. So the one probe aimed at the race
window (the saturated stress run) never ran the instrumented build, which slows both sides of the race.

## Candidate mechanism read from code (HYPOTHESIS — not measured; the stress probe did not trigger it)
- `viola run` sizes the inner PTY from `viola_pty::host_size()` at spawn (`src/cmd/run.rs:63`), then `pump()`
  takes its change baseline with a SECOND `host_size()` read (`crates/viola-pty/src/lib.rs:293`), after the child is
  spawned, `log_child_start` runs and the pump threads start.
- The test resizes the outer PTY as soon as the child's `start` receipt appears (`tests/tui_passthrough.rs:102-106`).
  If that resize lands before the pump's baseline read, the baseline already equals the new size, no change is ever
  seen (`lib.rs:316`), the inner PTY stays 80×24, and the loop runs to the 10 s deadline — exactly the reading.
- If this is it, the fix is product-side (take the baseline from the size the PTY was spawned with), outside this
  chunk's scope (plan §Constraints: no product-crate change). What makes the gate's timing hit the window, and no
  isolated probe's, is unexplained.

## What would decide it (for the owner)
- Instrument the pump (log the baseline size and each poll) or the test (dump the child's `size` receipts on failure),
  then run `pre-push` again — both edits are outside this chunk's modify-set.
- Or fix the baseline read and see whether `pre-push` goes green three times running.

## Instrumented samples (plan step 10, after the operator's scope widening)
The temporary `pump-probe.ndjson` stamps (`src/cmd/run.rs`) and the test's failure dump were in the tree for three
`bash scripts/agent-run.sh pre-push` runs (run dir `pp-instr-{1,2,3}.{out,err}`):

| sample | linux-tests (instrumented `--coverage`) | later stage |
|---|---|---|
| 1 | 399/399 green | both legs ran, base a69c5ef each (90 mutants each); union RED: 10 survivors — 6 on `CACHE_CAP_BYTES`, `pre_push.rs` `files > 0`, 3 in the temporary instrument |
| 2 | 399/399 green | union RED on 1 survivor, the temporary instrument's `probe_stamp` (the cap and `apply` kills added between samples held) |
| 3 | 399/399 green | **linux-leg RED**: `tui_host_resize_reaches_the_child` **TIMEOUT [10.006s]** in cargo-mutants' unmutated baseline run (`mutants-exit-4`) |

- The red recurred once in three instrumented runs — in the mutation leg's baseline, not in `linux-tests`.
- **The dump did not fire.** nextest's `mutants` profile killed the test at 10.006 s, before the test's own 10 s
  deadline assertion (which prints the dump) could run; cargo-mutants then deleted its scratch tree, the test home
  and `pump-probe.ndjson` with it (`find /tmp ~/viola-pre-push/target -name pump-probe.ndjson` → none).
- So the prediction `spawn < resize < baseline` is **neither confirmed nor refuted**. Per the operator's rule, no fix
  was applied. The instrument as built cannot capture a red that ends in a runner timeout.

## Experiment A — the window forced open (operator ruling, 2026-09-27; Windows host, no WSL)
- Seam: `src/cmd/run.rs` `hold_pump_start`, compiled only with the test-only `fake-agent` feature;
  `FAKE_AGENT_PUMP_DELAY_MS=1000` holds the pump after the child starts. Test:
  `tui_host_resize_in_the_pump_start_window_reaches_the_child` sends its resize on the child's `start` receipt,
  inside the hold. The control is the unchanged `tui_host_resize_reaches_the_child` (no seam).
- **Before the fix: forced 6/6 RED with the exact signature, control 6/6 GREEN** (run dir
  `forced-before-fix.log`, `forced-before-{1..5}.log`). The first run's temporary probe stamps (the instrument was
  still in the tree) read, in unix µs:
  - `spawn` 1790464805675191 → 80×24 (the `run.rs:63` read)
  - `resize-sent` 1790464805698884 (the test's trail; 23.7 ms after the spawn read)
  - `size-observed` 1790464805699660 → the child's receipt 80×24
  - `baseline` 1790464806688566 → **100×30** (the pump's second read, 990 ms after the resize)
  - The prediction holds exactly: `spawn < resize < baseline`, `baseline` = 100×30, and the child never saw 100×30.
- The race theory is established by the variation, not by a green.

## Fixed
- `crates/viola-pty/src/lib.rs` `pump(…, spawned: Size, host_size)`: the baseline is the size the PTY was spawned
  with, never a second read; `src/cmd/run.rs` passes `spec.size`; the temporary instrument (`probe_stamp`) removed
  whole. Unit test `pump_forwards_a_resize_that_lands_before_its_first_look` (spawned 80×24, host already 100×30 at
  the first look, exactly one `resize(100×30)` — no timing).
- **After the fix: forced 6/6 GREEN, control 6/6 GREEN** (`forced-after-fix.log`, `forced-after-{1..5}.log`);
  `viola-pty` 21/21. The forced test also asserts the new size arrived 0.9–3 s after the resize (it landed inside the
  1 s hold), so the seam itself is observable to the mutation legs.
- Guard pair g9 (`evidence/guards/readings.json`): the second read restored → the unit test and the forced test both
  red with the signature; restored → both green.
- Experiment B (kill-proof evidence): the resize test's deadline is now 8 s (`RESIZE_WITHIN`), below nextest's 10 s
  `mutants` kill, and it appends each observation to `viola-resize-<pid>.ndjson` in the temp dir as it goes (removed on
  success).
- **Three consecutive green `pre-push` runs over the fixed tree** (gate entry 13; run dir `fixed-full.out`,
  `fixed-pp-{2,3}.out`, documents in `fixed-pp-{1,2,3}.log`):

  | run | seconds | linux-tests | ubuntu-latest leg | windows-2025 leg | union | cache after |
  |---|---|---|---|---|---|---|
  | 1 | 846.2 | coverage 401/401 | base a69c5ef, 87 tested, ok | base a69c5ef, 87 tested, ok | ok, 0 breaches | 7.0 GiB |
  | 2 | 850.2 | coverage 401/401 | base a69c5ef, 87 tested, ok | base a69c5ef, 87 tested, ok | ok, 0 breaches | 7.1 GiB |
  | 3 | 852.2 | coverage 401/401 | base a69c5ef, 87 tested, ok | base a69c5ef, 87 tested, ok | ok, 0 breaches | 7.2 GiB |

## Consequence for this chunk (as written at the soft-exit, before the fix — superseded by §Fixed)
The gate cannot read green on this tree: it is fail-fast at `linux-tests`, so the ubuntu leg, the windows leg and the
union (entries 11–12, and the operator entry 15) have not run. Plan step 7 (the planted-red witness) needs a green
remove-the-plant reading and waits on this.
