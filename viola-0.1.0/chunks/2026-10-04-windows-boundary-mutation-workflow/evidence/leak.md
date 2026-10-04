# Leak witness — one full Linux-host `run --mutants --package viola-e2e` (plan step 5)

Host: the Linux dev host, 2026-10-04, with step 1 in place (`.config/nextest.toml` `[profile.mutants]`
`fail-fast = { max-fail = 1, terminate = "wait" }`; the gate entries 12 and 13 read it green).

## Attempt 1 — not a witness (setup error, cause known)
- `TMPDIR=<repo parent>/viola-mutants-scratch/leak-witness-2026-10-04` (fresh, NOCOW inherited: `lsattr -d` reads `C`;
  `ls -A | wc -l` = 0). 02:08:36Z → 02:09:02Z, exit 1, document `{"ok":false,"reason":"mutants-exit-4","suites":[]}`.
- cargo-mutants: `FAILED Unmutated baseline in 10s build + 5s test`, no mutant tested. The one failing test,
  `harness::cleanup::tests::unconnectable_is_true_only_once_nothing_listens`, panicked at
  `crates/viola-e2e/src/harness/cleanup.rs:380:61`: `local socket name length exceeds capacity of sun_path of
  sockaddr_un`.
- Cause: the test binds `$TMPDIR/.tmpXXXXXX/viola-test-chan-<pid>-gone.sock`. Under that 24-character subdir name the
  path is 116 bytes, past `sun_path`'s 107. m3.md run 4 used the scratch root itself, about 92 bytes. The subdir name
  is the defect. A test that binds a Unix socket under `TMPDIR` puts a length ceiling on any mutation `TMPDIR`
  (≈ 60 bytes before the `.tmpXXXXXX/` segment). The dir is left empty (0 entries).

## Attempt 2 — the witness
- `TMPDIR=<repo parent>/viola-mutants-scratch/lw` (fresh, NOCOW inherited: `lsattr -d` reads `C`; before:
  `ls -A | wc -l` = **0**; the socket path is at most 97 bytes).
- Command: `TMPDIR=<it> bash scripts/agent-run.sh run --mutants --package viola-e2e`.
- 02:09:37Z → 03:27:43Z, **wall 78 m 06 s** (cargo-mutants: `711 mutants tested in 78m: 2 missed, 649 caught, 60
  unviable`; baseline `ok Unmutated baseline in 10s build + 20s test`). Exit 1, as designed for two misses.
- Document: `ok:false`, `mutants.tested` 711, `verdict:"package"`, suite `mutants` passed 649 · failed 2 · survived 2;
  `failures` = exactly the two owed coordinates:
  - `crates/viola-e2e/src/harness/run/mutants/scratch.rs:48:5: replace prepare -> Result<Option<(PathBuf, u64)>, String> with Ok(None)`
  - `crates/viola-e2e/src/harness/run/mutants/scratch.rs:54:8: delete ! in prepare`
  archived `target/run-archive/72`.

| run | wall | tested | caught | missed | timeout | unviable |
|---|---|---|---|---|---|---|
| m3.md run 4 (`terminate = "immediate"`) | 22 m 49 s | 711 | 649 | 2 | 0 | 60 |
| this run (`terminate = "wait"`) | 78 m 06 s | 711 | 649 | 2 | 0 | 60 |

Verdict over the measurable set: `missed == 0` (the two misses are the owed Windows-only `scratch.rs` mutants,
`HOST_SCRATCH = cfg!(windows)`), `timeout == 0`, `unviable` 60 ≤ `caught` 649. Wall time is longer than run 4, as
predicted, by 3.4×. Each caught mutant now waits for its running tests. Outcome-line test times: 611 × `5s`,
24 × `20s`, 7 × `30s` (the kill line), 1 × `8s`, 9 × `0s`.

## Leftovers after the run, in that dir
| class | predicted | measured |
|---|---|---|
| `.tmp*` | 0 | **38** |
| `cargo-mutants-*` (nested copies) | 0 | **0** |
| everything else except `mutants.out` | 0 | **0** |

(On this host `mutants.out/` lands in the repo root, not in `TMPDIR`: `HOST_SCRATCH` is Windows-only.) The last
boundary run left 25 275 `.tmp*` + 62 nested copies (research.md §Measured facts). The nested-copy half reads 0, the
mechanism the CARRY named. The `.tmp*` half is cut by 99.85 % but is **not 0**: **the acceptance "0 `.tmp*`" is
not met as written.** Every leftover is traced below and none was swept. The dir is left as measured.

### 17 — a test killed mid-flight (the one path `wait` cannot drain)
Each dir's mtime sits ≤ 30 s before a per-mutant log (`mutants.out/log/`) that carries a nextest kill: either
`TERMINATING [> 30.000s]` then `TIMEOUT [30.0s]` under the `package(viola-e2e)` 15 s × 2 kill, or a mutant-made
`SIGKILL` ("test aborted with signal 9").

| leftovers (local mtime, UTC+2) | content | the kill log | killed tests |
|---|---|---|---|
| `.tmpsIjW5N` `.tmpUapWmq` `.tmp1k32rp` `.tmpWfQ9wZ` (04:15:30–31) | 2 empty · 2 `r.ndjson` | `harness__mod.rs_line_125_col_5_001` (04:16:01) | TERMINATING: `boot::tests::wait_ready_times_out_with_the_missing_checks`, `cleanup::tests::cleanup_waits_its_deadline_for_a_target_that_outlives_its_kill`, `supervise::tests::reap_gives_up_at_its_bound_and_on_a_probe_error`, `supervise::tests::stop_presses_ctrl_c_again_when_the_first_is_lost` |
| `.tmpf7K3eC` `.tmp1TajD9` (04:16:13) | empty · `target/agent-run/s/{session.json,stop.request}` | `harness__mod.rs_line_161_col_9` (04:16:43) | TIMEOUT: `viola-e2e::cli cleanup_keeps_the_home_under_agent_run_keep_homes`, `cleanup::tests::cleanup_all_stops_recorded_sessions_and_refuses_foreign_homes`, `cleanup::tests::cleanup_waits_its_deadline_for_a_target_that_outlives_its_kill` |
| `.tmpY5jFvp` `.tmpeUBAY6` (04:16:46) | empty · `target/agent-run/s` | `harness__mod.rs_line_161_col_28` (04:17:16) | TIMEOUT: `cleanup_all_stops_recorded_sessions_and_refuses_foreign_homes`, `cleanup_waits_its_deadline_for_a_target_that_outlives_its_kill` |
| `.tmpzExAUQ` `.tmp6Mbqwf` (04:17:45) | empty · `target/agent-run/s` | `harness__mod.rs_line_171_col_61` (04:17:46) | SIGKILL: the same two `cleanup::tests` |
| `.tmp1AXctR` `.tmpQdUkvJ` (04:18:09) | empty · `target/agent-run/s` | `harness__mod.rs_line_171_col_79` (04:18:09) | SIGKILL: the same two |
| `.tmpqWirpC` `.tmpXIbncC` (04:18:14) | empty · `target/agent-run/s` | `harness__mod.rs_line_178_col_16` (04:18:44) | TIMEOUT: the same two |
| `.tmpajrla3` `.tmpTGkNHY` (04:25:15) | empty · `viola-harness`/`viola-fake-agent`/`viola` copies | `harness__boot.rs_line_398_col_44_001` (04:25:45) | TIMEOUT: `boot::tests::wait_ready_times_out_with_the_missing_checks`, `harness_lifecycle boot_that_never_gets_ready_times_out_and_stops_its_supervisor` |
| `.tmpjYvMwp` (04:57:49) | bin copies | `harness__supervise.rs_line_116_col_5` (04:58:20) | TIMEOUT: `harness_lifecycle boot_that_never_gets_ready_times_out_and_stops_its_supervisor` |

Disposition: by design. A mutant that hangs a test must end as a nextest kill (testing.md 2026-09-24), and a killed
process runs no `Drop`. With `terminate = "wait"` only the hung test itself is lost, not every test running beside
the first failure.

### 21 — a throwaway git repo whose removal stopped part-way (`terminate`-independent)
`.tmp4shJov` `.tmp62yyNY` `.tmp6FTaz7` `.tmp7nuTIw` `.tmp8Yb1MD` `.tmpbOPloJ` `.tmpCcgFsC` `.tmpcmw4MJ` `.tmpcz1FWw`
`.tmpDPeReA` `.tmpg9u34R` `.tmpgWQ7tF` `.tmpH231cf` `.tmpH40Csc` `.tmphEAoVH` `.tmphKPVtQ` `.tmpimjfhE` `.tmpJVWsaw`
`.tmprUVNKX` `.tmpszxO37` `.tmpXwnRN7` (04:27–05:26 local).
- Producer, by content: `a.rs` + `.andromeda/master-route.md` (`## p-0.1.0` / `a · complete · …` / `m · pending · …`)
  is the `Pass(tempfile::TempDir)` fixture of `crates/viola-e2e/src/harness/run/mutants/base.rs:234`. Its `commit`
  (`:248-265`) runs `git add -A` and `git commit`. `run.rs:530` is the other `a.rs` repo writer.
- State: no dir is a repository any more (`.git/HEAD` absent in every one sampled). What remains in `.git` is
  `objects/pack/tmp_idx_*`, `objects/pack/tmp_rev_*`, `logs/`, `index`, `COMMIT_EDITMSG`, `rr-cache/`, `MERGE_RR`. The
  test's `TempDir` drop ran and stopped part-way.
- No kill: most of these dirs have no nextest kill line in the log next to them in time, and the time-adjacent failing
  mutants are unrelated (secret_scan, supervise, perf…).
- Measured: each `git commit` under the host's git 2.55.0 starts a detached `git maintenance run --auto --quiet
  --detach` (`GIT_TRACE`: `run_command: git maintenance run --auto --quiet --detach`, once per commit over six
  commits in a scratch repo).
- **HYPOTHESIS (not reproduced):** that detached process writes into `.git/objects/pack/` while the test's `TempDir`
  drop walks the tree, so `remove_dir_all` fails on a non-empty directory and the drop ignores the error. The pack
  write itself was not reproduced in a six-commit scratch repo. The cause stays open.
- The class predates this chunk: the previous chunk's run 1 in the host `/tmp` root (`terminate = "immediate"`)
  left 461 `.andromeda` repos there, 26 of them with no `.git/HEAD`. So `wait` neither causes nor cures it.
- Disposition: open, not swept. It is a test-fixture removal racing a detached host process, outside step 1's
  mechanism and outside this chunk's file lists. A candidate fix, not taken: pass `-c maintenance.auto=false` on the
  fixture's git calls. That is new behaviour in `base.rs` / `run.rs` with its cause unproven, so it goes to the
  operator for a route owner.

## Disposition — the overseer, founder-delegated (2026-10-04, after /implement's surfaced report)
Verbatim: "ACCEPT the leak acceptance as measured. 25,275 dirs + 62 copies became 38 + 0. The 17 by-design kills are
recorded as such. The 21 half-removed fixture git repos are an [inferred] item with a named owner: route-resolve folds
it into the next chunk (:72), as a hypothesis (git maintenance --auto --detach racing the TempDir drop; fix candidate
-c maintenance.auto=false) with its own two-sided acceptance."

- The 17 kill-path leftovers: accepted as by design.
- The 21 fixture repos: an `[inferred]` HYPOTHESIS item. Owner: `working-route.md:72` (Readiness gate and timing
  constants), folded by the wrap's route-resolve. /implement edits no route.
