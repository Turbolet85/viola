# Operator pass 4 — 2026-09-24-epoch-1-cleanup (GREEN)

Founder-approved form (as pass 3). The repo is now public: Actions minutes are free and the billing block no longer applies.

| step | what | reading |
|---|---|---|
| amend | the leak guard (`tests/cli.rs`), the swamp rule + tests (`run/mutants.rs`), the gate trail JSON and the evidence files folded into the pre-CI commit, subject kept | `d14f234` (`d14f2347e83f2d3f2e84ab481be6f4faf06bbdb1`), parent `9df9e45`; tree clean |
| rewind | `git push --force-with-lease=build/viola-0.1.0:74dadd4 origin 9df9e45:build/viola-0.1.0` | exit 0 · `+ 74dadd4...9df9e45 9df9e45 -> build/viola-0.1.0 (forced update)` |
| re-push | `git push origin build/viola-0.1.0` | exit 0 · `9df9e45..d14f234  build/viola-0.1.0 -> build/viola-0.1.0` |
| CI run | `gh run list --commit d14f234…` | run **36126924953** (push, `ci`), completed **success** |
| run identity | the check-runs on sha d14f234, grouped by run id | only `36126924953`. Gates 17/18 read THIS run alone (a new sha, no superseded runs on it) |
| base check | job logs | `mutants (ubuntu-latest)` and `mutants (windows-2025)` both: `AGENT_RUN_CHUNK_BASE: 9df9e45f98592f4cd0cd9b34ff82323f131c09a7` ✓ |
| billing | `mutants-verdict` job 108052795442 | started and ran: 9 steps, success (11:26:07Z → 11:26:32Z) ✓ |
| gate 17 | `gh api …/commits/$(git rev-parse HEAD)/check-runs --jq '[.check_runs[] \| .conclusion] \| unique \| join(",")'` | exit 0 · last line `success` · **green** |
| gate 18 | `gh api …/check-runs --jq '[… select(.name \| startswith("mutants")) \| .conclusion] \| join(",")'` | exit 0 · last line `success,success,success` · **green** |

## Mutation legs (streamed per-mutant lines, both legs)

| leg | span (UTC) | result |
|---|---|---|
| ubuntu-latest | 10:59:40 → 11:10:07 | `148 mutants tested in 10m: 144 caught, 4 unviable` |
| windows-2025 | 10:59:23 → 11:26:04 | `148 mutants tested in 26m: 142 caught, 6 unviable`; longest unviable run 3; the old turn mutant `viola-harness.rs:157:5: replace cleanup_cmd -> ExitCode with Default::default()` caught in 2s build + 9s test (no leak afterwards) |
| mutants-verdict | 11:26:07 → 11:26:32 | success (the union of both legs) |

All other jobs are success: test ×3, lint ×3, release ×3, msrv, supply-chain, fuzz-replay.

## Observation (not red, not fixed; for the operator)

On windows, 2 of the 6 unviable are `src/bin/viola-fake-agent.rs:463:5: replace main -> ExitCode with Default::default()` and `:482:8: delete
! in main`, each `in 1s build`. The ubuntu leg caught both, and neither body is OS-specific, so the union covers them.

**Inferred, not measured:** a short-lived holder of `viola-fake-agent.exe` blocked the relink. The candidate is the `--exit-no-eof` test's
grandchild, which by design holds the inherited stdout for `HOLD_FOR` = 10 s. It is the same class as the leak (a process holding an exe
during relink), but bounded. The swamp rule does not fire (6 ≤ 142).
