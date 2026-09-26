# Session Handoff

**Last Updated:** 2026-09-26T21:14:00Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (HEAD acd08c7, the pre-CI commit)
**Status:** clean
**Last Commit:** 2026-09-26-ci-chunk-base-and-union-verdict — feat: harness-derived whole-chunk mutation base, compiling-leg union, chunk.diff out of the secret scan

## Position
- Done: `2026-09-26-ci-chunk-base-and-union-verdict`.
  - `run --mutants` derives its base (last master flip before the chunk's oldest pre-CI commit, `HEAD^` on the wrap push) and prints it as `base`; CI passes no `github.event` value.
  - `gate --mutants-legs` judges each mutant only by the legs whose `#[cfg]`s compile its line (`harness::cfg_legs`, `syn`).
  - `secret-scan` skips exactly `target/agent-run/chunk.diff`; the `harness-<os>` upload excludes it.
  - CI run 36270173848 on acd08c7: 15/15 green, both legs `base` fcca1ce, 79 tested / 74 caught / 0 survived.
- Next: `/andromeda-phase` promotes "Local Linux pre-push gate" (WSL2 Ubuntu). Its new CARRY: the Linux clone needs git history for the derived base; judge its leg by verdict, not counts.

## Work done
- New `crates/viola-e2e/src/harness/cfg_legs.rs`; `resolve_base`, `gate`/`union`, `secret_scan::scan` changed; `commit_exists` removed; syn 2.0.119 + proc-macro2 1.0.107 (test-only); `ci.yml` base env line removed, upload exclusion added.
- Evidence: `chunks/2026-09-26-ci-chunk-base-and-union-verdict/evidence/` (7 guard pairs, the local leg record, the operator pass).

## Drift resolved
- 23 fanned proposals + 2 orchestrator-raised + 1 sweep-found = 26 amendments, 0 escalations: test-plan 10 (§3 Base / Output / secret-scan / union / CI paragraph, §6 Canary, §9 ×2, §10, §12 entry), architecture 8, obs-plan 4, security-plan 4.
- 6 leaves re-derived, 9 lines (docs/stack, docs/commands ×3, docs/workflow, docs/tests-summary, rules/testing.md, rules/verification-harness.md ×2).

## Notes
- **Observation (overseer-requested):** cargo-mutants graded the identical final tree 73/6 then 71/8 (caught/unviable) locally; CI read 74/5 per leg. All green; cause unmeasured. Curated as a verification-harness rule.
- **Curated:** host-win32 extension (rust-analyzer holds `mutants.out` and restarts after edits: stop it by exact ExecutablePath before each `run --mutants`) — retires last session's deferred learning.
- **Open for the operator:** the Epoch 2 header still names "events, ledger", which moved to 2b.
- **Deferred learnings (carried):** a `cfg!()`-valued fn is an equivalent mutant on one OS's leg, so make it a const (0.8); `check-runs` by sha mixes superseded runs after a force-push (0.8).
- **Carried:**
  - The code-metrics `mutation.survivors` correction (`lib.rs:9:31` / `:9:38`) is owed at the next ledger-mode audit.
  - Two Windows-only unviable fake-agent `main` mutants were observed and not chased.
- Last failed command: none open.
