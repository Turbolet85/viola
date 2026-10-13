# CARRY 2 — the four timeout-graded mutants, killed by assertion

Chunk 2026-10-13-paste-newline-ledger-row, plan step 6. The four mutants CARRY 2 names (measured at chunk
2026-10-09-epoch-3-cleanup-ii as `caught` by the `mutants` profile's 10 s kill alone, with no failing assertion),
re-located at HEAD `19483111dbe9` by function name (research.md §Measured facts 11). No `cargo mutants` run, witness
run or workflow dispatch was made (`inputs#I1`); the grading stays owed to the Epoch 5 boundary audit.

Each mutation was applied by hand by one exact replacement whose site the script found exactly once, with
`rustfmt` in the same process, and read through
`cargo nextest run --profile mutants --features fake-agent -p viola --bin <binary> -E 'test(/(^|::)tests::<case>(::|$)/)' --no-fail-fast`
(the `mutants` profile, so a hang would be cut by its kill line). The file was then restored from its saved bytes,
confirmed byte-equal, and the same command read green. Each mutant run selected only its killing case. The
`ProbeDir::typed` case builds `ProbeDir` by struct literal, not `ProbeDir::create`, because under the mutant
`create` would resolve `plugin/` against the test's cwd (the repository root). `git status --short -- plugin` read
empty before and after the four applications.

Written 2026-10-13T21:30:11Z (`date -u`); the readings were taken just before, in this chunk's implement run
(`.andromeda/runs/2026-10-13T21-22-15-implement/`).

| # | site (HEAD `19483111dbe9`) | mutation | killing case | mutant reading | restored |
|---|---|---|---|---|---|
| c1 | `src/bin/viola-fake-agent.rs:450:16` | delete `!` (the `PreToolUse` operand) in `Agent::replay_dialogs` | `src/bin/viola-fake-agent.rs::tests::replay_dialogs_replays_a_call_holding_only_a_pre_tool_use` (new) | FAIL, exit 100: panicked at `src/bin/viola-fake-agent.rs:1456:9`, ``assertion `left == right` failed``, left `[]`, right `["PreToolUse"]` | byte-equal; PASS, 1 passed |
| c2 | `src/bin/viola-fake-agent.rs:450:38` | delete `!` (the `PermissionRequest` operand) in `Agent::replay_dialogs` | `src/bin/viola-fake-agent.rs::tests::replay_dialogs_replays_a_call_holding_only_a_permission_request` (new) | FAIL, exit 100: panicked at `src/bin/viola-fake-agent.rs:1461:9`, ``assertion `left == right` failed``, left `[]`, right `["PermissionRequest"]` | byte-equal; PASS, 1 passed |
| c3 | `src/bin/viola-fake-agent.rs:107:17` | delete match arm `"--screens"` in `Opts::parse` | `src/bin/viola-fake-agent.rs::tests::opts_parse_reads_values_flags_and_ignores_unknown` (existing) | FAIL, exit 100: panicked at `src/bin/viola-fake-agent.rs:992:9`, `assertion failed: o.screens && o.turn_stop && o.dialogs && o.framing` | byte-equal; PASS, 1 passed |
| c4 | `src/cmd/verify.rs:318:9` | replace `ProbeDir::typed -> PathBuf` with `Default::default()` | `src/cmd/verify.rs::tests::probe_dir_names_each_run_its_own_dir_inside_it` (new) | FAIL, exit 100: panicked at `src/cmd/verify.rs:637:9`, ``assertion `left == right` failed``, left `""`, right the temp dir's `probe/typed` | byte-equal; PASS, 1 passed |

The panic lines are at the working tree's line numbers (the new cases sit below the sites). c3 needed no new case:
the existing case asserts `o.screens`. A third replay case, `replay_dialogs_replays_nothing_for_a_stem_with_no_call`,
pins the loop's end with no call recorded. Under c1 the PermissionRequest-only case, and under c2 the
PreToolUse-only case, would loop with no end. Each mutant run selected only the case that reads it red, so no run
waited on that loop.
