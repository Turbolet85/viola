# Operator pass 3 — 2026-09-24-epoch-1-cleanup (rewind + re-push)

Founder ruling: rewind the remote to 9df9e45, then fast-forward it to 74dadd4, so that `github.event.before` = 9df9e45.

| step | what | reading |
|---|---|---|
| rewind | `git push --force-with-lease=build/viola-0.1.0:74dadd4 origin 9df9e45:build/viola-0.1.0` | exit 0 · `+ 74dadd4...9df9e45 9df9e45 -> build/viola-0.1.0 (forced update)`; local branch stays on 74dadd4 |
| side run | the rewind push's own CI | run 36118098013 on 9df9e45 (created 09:22:49Z). Its diff is empty (merge-base 9df9e45), so it is not this chunk's witness; left running |
| re-push | `git push origin build/viola-0.1.0` | exit 0 · `9df9e45..74dadd4  build/viola-0.1.0 -> build/viola-0.1.0` |
| CI run | `gh run list --commit 74dadd4…`, not 36117447745 | run **36118112104**, completed **failure** |
| base check | job logs | ubuntu leg: `AGENT_RUN_CHUNK_BASE: 9df9e45f98592f4cd0cd9b34ff82323f131c09a7` ✓ · windows leg: the same value ✓ |
| gate 17 | `gh api …/commits/$(git rev-parse HEAD)/check-runs --jq '[.check_runs[] \| .conclusion] \| unique \| join(",")'` | exit 0 · last line `failure,success` · **red** |
| gate 18 | `gh api …/check-runs --jq '[… select(.name \| startswith("mutants")) \| .conclusion] \| join(",")'` | exit 0 · last line `failure,success,success,failure,failure,failure` · **red** |

Gates 17 and 18 read EVERY check-run on sha 74dadd4. That includes the superseded run 36117447745's three base-missing failures, so their
lines mix two runs. For this run alone, the jobs are:

| job (run 36118112104) | conclusion | span (UTC) |
|---|---|---|
| mutants (ubuntu-latest) | success | 09:23:02 → 09:40:01 — `143 mutants tested in 16m: 139 caught, 4 unviable` |
| mutants (windows-2025) | success **(false green)** | 09:23:16 → 09:53:20 — `143 mutants tested in 29m: 8 caught, 135 unviable` |
| mutants-verdict | **failure — never started**: annotation `The job was not started because recent account payments have failed or your spending limit needs to be increased. Please check the 'Billing & plans' section in your settings` | 09:53:20 → 09:53:23, 0 steps |
| test ×3, lint ×3, release ×3, msrv, supply-chain, fuzz-replay | success | 09:23 → 09:41 |

## The windows leg is a false green (measured from its streamed log)

- Baseline: `ok Unmutated baseline in 78s build + 38s test` · `Auto-set build timeout to 390s` · `Auto-set test timeout to 191s`.
- The first 9 outcomes are genuine: 8 caught, plus 1 legitimately unviable (`Some(vec![Default::default()])`, 0 s build). The 9th is
  `crates/viola-e2e/src/bin/viola-harness.rs:157:5: replace cleanup_cmd -> ExitCode with Default::default() … caught`.
- From outcome 10 on, EVERY mutant is `unviable` after 8–12 s of build, 134 in a row. That is far below the 390 s build timeout, so these
  builds FAILED to build rather than timing out.
- The ubuntu leg (same tree, same diff) caught 139. The local windows host caught 137. Both saw 0 mass unviable.

**Mechanism (the turn is measured; the cause is inferred, and to be proved by a local control):**
- Under the `cleanup_cmd → Default` mutant, `crates/viola-e2e/tests/cli.rs::status_logs_and_cleanup_drive_a_booted_session` boots a real
  session from the scratch target: a `viola-harness supervise`, wrappers and fake agents.
- The mutated binary `cleanup` returns exit 0 without cleaning, so the test goes red (the kill) and the booted processes keep running.
- Windows cannot relink a running `.exe` (host-win32 Session Additions, `os error 5`), so every later build that relinks those binaries
  fails, and cargo-mutants grades each one `unviable`. Linux allows replacing a running binary, so ubuntu was unaffected.
- This is the same class that plausibly explains run 36046091888's 2 h 45 m silence: leaked processes holding the captured test-output
  pipe open. That run recorded nothing to confirm it.
- The harness counts `unviable` as "reported, never red" (`mutants_suite`), so the leg printed `ok:true`: the gate went vacuous on
  windows CI.

## Owed (not done; awaiting the founder's ruling)

1. A cleanup guard in `crates/viola-e2e/tests/cli.rs` (in the modify-set). The booted-session tests clean their session through the
   library `cleanup` (harness/cleanup.rs, outside the diff and so never mutated) on Drop. A broken binary `cleanup` then cannot leak
   processes. Proof: a local control with `cleanup_cmd` stubbed shows the processes left alive before the guard and gone after it.
2. A guard against the vacuous verdict: a leg whose unviable count dwarfs its caught count is not a pass. That is a change to test-plan §10
   Mutation-gate semantics, so a spec amendment for the wrap, or an operator decision.
3. The billing block on GitHub Actions (operator-owned): `mutants-verdict` could not start. Every later CI run is blocked until it is cleared.
