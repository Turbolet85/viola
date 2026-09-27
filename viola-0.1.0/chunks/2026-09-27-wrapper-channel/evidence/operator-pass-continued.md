# Operator pass, continued — after the pre-CI commit (see `operator-pass.md` for entry 21)

## Pre-CI commit and entry 22
- `3fca0ac3b114b66f2d2c2e3b75440f26d242a65b`
  `chore(2026-09-27-wrapper-channel): operator pre-CI commit, for the run this chunk's verdict reads`
  (`git add -A`; tree clean after).
- `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` → exit 0,
  `17c99c9..3fca0ac` (fast-forward, no force), 10:42:02Z.

## Entry 23 — CI run 36313377307 on 3fca0ac: RED (see `ci-red-36313377307.md`)
- Fail-fast read (the plan's loop pinned to HEAD) printed `failure` at 10:45:43Z (+3 min 41 s);
  trail `.andromeda/runs/2026-09-27T07-34-11-implement/op-23.{out,err,rc,start,end}`.
- Failed: `test (windows-2025)` + `mutants (windows-2025)` baseline (the DACL read-back compared the
  SID as `S-1-…`; the runner's built-in Administrator renders as `LA`) and `test (macos-latest)`
  (EPIPE on an oversize frame's tail write). Every other check completed success, `mutants
  (ubuntu-latest)` included (read after the red).

## Fix commit (V17 form)
- `08d24da215e37d952d9828dd57d6bdca6f9513d6`
  `fix(2026-09-27-wrapper-channel): the DACL read-back compares Windows' own canonical SDDL and the
  oversize sends take EPIPE on the write, measured in run 36313377307`.
- Host checks before the commit: viola-channel 103/103 (a new `canonical_sddl` alias case:
  `S-1-5-32-544` → `BA`, `S-1-5-18` → `SY`), `binary(channel_endpoint)` 5/5, clippy and fmt clean.

## `pre-push` on the fix commit — STOPPED by the host, no verdict
- Launched 10:48:38Z (rust-analyzer 0 at launch; trail `fix-pp.{out,err,start,ra}`, no `.rc`).
  Claude Code stopped it: the system was critically low on memory. Not restarted (awaiting the word).
- Its document before the stop: sync 4 files onto `08d24da` (tree
  `fc6e90085317c2f3efbb359c6f6207f41f786444`); Linux `coverage` 594/594 + gate ok; `ubuntu-latest`
  leg counted, 185 tested, base 17c99c9; then `windows-tests` red: `coverage` artifact-missing and
  doctest exit `-1073741502`. Cause, from its stderr: `linking with link.exe failed: exit code:
  0xc0000142` (STATUS_DLL_INIT_FAILED) on the instrumented `viola` and `viola-e2e` test binaries —
  processes failing to initialise under the memory exhaustion, not the code.
- Survivors: none of this run's (host and distro checked). One stray `viola-fake-agent.exe` from
  entry 21's windows mutation leg (a cargo-mutants temp copy, started 10:32Z, parent gone) stopped by
  verified pid. Free memory after: 18 375 MB.
- Not yet done at that point: `pre-push` on `08d24da` to a verdict → guarded push → entry 23.

## `pre-push` on 08d24da (after `drop_caches`, overseer's word) — RED at `linux-tests`, folded
- 11:04:03Z → 11:04:37Z, exit 1 (trail `fix-pp2.{out,err,rc,start,end,ra,head}`; rust-analyzer 0;
  the WSL page cache dropped first as root). Sync 10 files onto `08d24da`, tree `bea96dd6…`.
- Linux `coverage` 590 passed, 4 failed, all `viola-e2e harness::run::mutants::tests`
  (`…no_rust_delta…`, `…rust_delta_without_fresh_outcomes…`, `…test_only_delta…`,
  `…mixed_src_and_test_delta…`), each at `assertion failed: !out.doc.to_string().contains("777")`.
- Cause (inferred, not measured: the failing doc is not printed): each test commits the same `mini`
  repo content, so tests landing in one clock second share one commit sha, carried in the doc as
  `mutants.base`; a sha containing `777` fails the substring check in all four at once.
  Pre-existing (`git diff 17c99c9 -- crates/viola-e2e/src/harness/run/` was empty); folded under the
  standing direction.
- Fix: the four sites assert no NUMBER in the doc equals 777 (`carries_number`); a planted stale
  `outcomes.json` would surface as a count. Unit test
  `carries_number_reads_counts_never_digits_in_strings`: a sha-like `"0a1777bc"` and a path
  `"x/777/y"` are flagged by the old substring check and not by the new one; `caught: 777` is still
  caught. The mutants tests 35/35 on this host. Committed as `90eaf46`.

## `pre-push` on 90eaf46 — RED at `union`, one mutant, folded
- 11:07:06Z → 11:28:27Z, exit 1 (trail `fix-pp3.*`). Sync 17 files, tree `bb82587a…`. Linux
  `coverage` 595/595 + gate ok; windows-tests `coverage` 602/602 + gate ok; both legs base 17c99c9,
  185 tested; union breach: `crates/viola-channel/src/client.rs:152:37: replace + with - in open`
  (`let busy_until = Instant::now() + BUSY_WITHIN;`).
- Cause: the mutant (a deadline in the past) is visible only to a test that meets
  `ERROR_PIPE_BUSY`, and whether a back-to-back connect does is timing — caught in the earlier green
  run, missed in this one. Fix: the deadline is a pure `busy_deadline(start)`, pinned by
  `busy_deadline_is_two_seconds_after_the_start` (exact `Duration::from_secs(2)`); viola-channel
  104/104. Committed as `14cea02`.

## `pre-push` on 14cea02 — STOPPED by the host again, no verdict; the pass stops here
- Launched 11:29:38Z after rust-analyzer 0 and the WSL `drop_caches` (trail `fix-pp4.*`, no `.rc`).
  Claude Code stopped it: the system critically low on memory (the second such stop). Per the
  overseer's word ("if the host kills it again, record it and stop; do not loop"): not restarted.
- Its document before the stop: sync 24 files onto `14cea02`, tree `cd2ef4bc…`; Linux `coverage`
  595/595 + gate ok; then `windows-tests` red with `0xc0000142` (STATUS_DLL_INIT_FAILED, 7 hits in its
  stderr) — process start failing under the memory exhaustion, as in the first stop.
- Survivors: none (host and distro checked); free memory after the stop 19 276 MB.
- State left: HEAD `14cea02` (fix commits `08d24da`, `90eaf46`, `14cea02` on top of the pushed
  `3fca0ac`), tracked tree clean, NOT pushed; the last CI read is run 36313377307 on `3fca0ac`, red.
  Still owed then: `pre-push` to a verdict → guarded push → entry 23 on the pushed head.

## The memory fix (overseer direction), and the green pass
- `3efed41` `fix(2026-09-27-wrapper-channel): pre-push stops the WSL VM once the ubuntu verdict is
  home and caps the host stages at 16 build jobs, measured in two host memory stops of the
  fix-commit pre-push` — `release_vm` (`wsl.exe --terminate Ubuntu` after the verdict copy-back,
  host free memory before/after in the doc's `vm`), `CARGO_BUILD_JOBS=16` on every host-stage
  cargo call; tests `pre_push_stops_the_vm_after_the_copy_back_and_before_the_host_stages` (its
  remove-the-guard pair in `remove-the-guard.md`) and `pre_push_caps_the_host_stages_build_jobs`.
- `pre-push` on `3efed41`: 11:50:24Z → 12:14:53Z, **1469 s**, exit 0 (trail `fix-pp5.*`,
  rust-analyzer 0, WSL `drop_caches` first). `ok:true`, `stage:"union"`; sync 30 files, tree
  `4076a66a…`; Linux `coverage` 597/597 + gate ok; windows-tests `coverage` 605/605 + gate ok; both
  legs base 17c99c9, 190 tested; union 0 breaches.
- Memory, sampled from the OS every 5 s (`fix-pp5.mem`, 288 samples): start 37 558 MB free; the
  Linux stages drove it to **7 882 MB** (11:57:18); after the VM stop it was back at 37 610 MB
  within ~45 s; the host stages' minimum was **34 464 MB** (12:01:19). The doc's `vm` reads
  `free_kib_before` 10 143 700 / `free_kib_after` 10 242 020: taken the instant `--terminate`
  returned, before Windows reclaimed the VM's pages — the sampler, not that pair, shows the release.
  The 16-job cap's own share is not separable from the VM stop in one run.
- Entry 22: guarded push `3fca0ac..3efed41` (fast-forward, no force), 12:15:42Z, carrying
  `08d24da`, `90eaf46`, `14cea02`, `3efed41`.
- Entry 23: CI run 36318398739 on `3efed41` — fail-fast read `success`, 12:15:49Z → 12:32:39Z
  (trail `op-23b.*`); all 15 checks completed success (`test`, `lint`, `release` × 3 OSes,
  `mutants` × 2 + `mutants-verdict`, `msrv`, `fuzz-replay`, `supply-chain`).
