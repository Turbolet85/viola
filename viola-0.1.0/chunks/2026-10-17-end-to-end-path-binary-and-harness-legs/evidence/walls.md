# Wall times — the Path 1 flow (step 10) and the Path 6 leg (step 8)

Read by /andromeda-implement from P2 block run 2's own nextest status lines on the Linux dev host, 2026-10-17. The
kill lines they are held against are the nextest `mutants` profile's: 10 s for a root test with no `verify_window_` /
`send_window_` prefix (`path1_run_start_one_home_in_order` is one), 30 s for `package(viola-e2e)`
(`path6_two_instance_release_over_a_harness_session`). The plan predicted 6–9 s for the flow and called over 9 s a red.

| case | run | wall | kill | margin |
|---|---|---|---|---|
| `viola::path1_run_start path1_run_start_one_home_in_order` | entry 11, `run --e2e --filter …` (11.2.log) | 4.687 s | 10 s | 5.3 s |
| `viola::path1_run_start path1_run_start_one_home_in_order` | entry 19, pre-push's `run --coverage` (19.2.log, instrumented) | 5.140 s | 10 s | 4.9 s |
| `viola-e2e::path6_budget_harness path6_two_instance_release_over_a_harness_session` | entry 11 (11.2.log) | 5.028 s | 30 s | 25.0 s |
| `viola-e2e::path6_budget_harness path6_two_instance_release_over_a_harness_session` | entry 19 (19.2.log, instrumented) | 5.287 s | 30 s | 24.7 s |

The flow sits under the predicted 6–9 s band and under the 9 s red line in both readings. Block run 1 read the flow at
4.671 s through entry 12, which then still selected it into `nextest-integration` (before the regex widening,
`inputs#I2`).

Logs: `$TMPDIR/andromeda-gate/2026-10-17-end-to-end-path-binary-and-harness-legs/implement-2026-10-17T01-21-28/`.
