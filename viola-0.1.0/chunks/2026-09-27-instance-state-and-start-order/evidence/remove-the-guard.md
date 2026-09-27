# Remove-the-guard runs — 2026-09-27-instance-state-and-start-order

testing.md 2026-09-25: each new guard test, shown red with its guard neutralised and green with it restored.
Every neutralising edit was asserted to match exactly once and read back as landed before its run, and the
restore was read back the same way. Windows rows ran through `scripts/agent-run.sh run --unit|--integration
--filter …` on the host (verdict = the harness document's `ok` and its `suites[].failures`); the stale row is
Unix-only (`#[cfg(unix)]`, SIGSTOP) and ran in the WSL2 `Ubuntu` pre-push clone synced to the same tree
(`cargo nextest run --features fake-agent --test cli_instance_state -E 'test(=run_refuses_a_stale_name)'`).

| guard | site | neutralised to | neutralised reading | restored reading |
|---|---|---|---|---|
| pinned-copy re-hash compare | `crates/viola-state/src/pin.rs` `pin_exe` | `Ok(_) => {}` | unit `pin_exe_copies_then_refuses_a_tampered_copy` FAILED (exit 1, ok:false); integration `run_refuses_a_tampered_pinned_copy` FAILED (exit 1, ok:false) | both passed (exit 0, ok:true) |
| live refusal | `src/cmd/run.rs` `collision_check` | `Liveness::Live => return None,` | integration `run_refuses_a_live_name` FAILED (exit 1, ok:false) | passed (exit 0, ok:true) |
| stale refusal | `src/cmd/run.rs` `collision_check` | `Liveness::Stale => return None,` | Linux: `run_refuses_a_stale_name` FAIL, 0 passed / 1 failed (nextest exit 100) | Linux: PASS, 1 passed (exit 0) |
| plugin rewrite on every start | `src/cmd/run.rs` `pin_and_plugin` | write only `if !path.exists()` | integration `run_rewrites_the_plugin_folder_each_start` FAILED (exit 1, ok:false) | passed (exit 0, ok:true) |
| fixture sweep: owner liveness (overseer ruling, condition 1) | `tests/support/home.rs` `sweep_gone_owners` | the `process_start(pid) == Some(started_at)` skip removed | integration `fixture_sweep_removes_only_homes_whose_owner_is_gone` FAILED — the live sibling's home was removed (exit 1, ok:false) | passed (exit 0, ok:true) |
| mutation run keeps no homes (overseer ruling) | `crates/viola-e2e/src/harness/run/mutants.rs` the `cargo mutants` command | both `AGENT_RUN_KEEP_*=0` env lines removed | unit `run_mutants_never_keeps_test_homes` FAILED (exit 1, ok:false) | passed (exit 0, ok:true) |
| mutation scratch off the tmpfs (overseer second fold) | `crates/viola-e2e/src/harness/pre_push.rs` `linux_leg` | `&[tmpdir]` → `&[]` (no `TMPDIR` on the leg) | unit `pre_push_mutation_scratch_is_on_the_clone_disk_and_wiped_first` FAILED (exit 1, ok:false) | passed (exit 0, ok:true) |
| 5 s live/stale boundary | `crates/viola-state/src/liveness.rs` `classify` | `age < STALE_AFTER` | unit `liveness_classify_boundaries::case_2_at_the_bound` FAILED, the other 6 cases passed (exit 1, ok:false) | 7/7 passed (exit 0, ok:true) |

The neutralised `live refusal` build warns `refuse_live is never used`; the run still reached the test, and the
harness document names `run_refuses_a_live_name` among its failures (re-read from the document, not the log).
