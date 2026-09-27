# Session Handoff

**Last Updated:** 2026-09-27T17:41:38Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-27-epoch-2-cleanup — refactor(2026-09-27-epoch-2-cleanup): Epoch 2 cleanup — four splits under 700, clone pairs gone, host mutation scratch, scoped mutants loop, run archive, wsl-exec.sh

## Position
- Done: 2026-09-27-epoch-2-cleanup (the first Epoch 2b entry; operator pass green: pre-CI commit `f0e6dbc`, CI ci#36333711860 15/15).
- Next: /andromeda-phase to promote + plan working-route:45 "Browser verdict reachability", then "Hooks to normalised events" (:47).
  **At that phase's P1 (overseer direction, NOT a CARRY):** fold in the 8 root-package tests whose child wait bound is 10 s, equal to
  the nextest `mutants` profile's 10 s kill (the race that cost the viola-pty watch its report): `tests/support/fake.rs` `WAIT_WITHIN`,
  `tests/support/home.rs` `READY_WITHIN`, `tests/support/outer_pty.rs` `EXIT_WITHIN`, `tests/run_cli.rs` `READY_WITHIN` and `:262`,
  `tests/cli_instance_state.rs:220`, `tests/contract_diag_schema.rs:244` and `:264` (list: the chunk report, Decisions & corrections).

## Work done
- Four files under 700 tokei lines by concern splits; the three clone classes deduped into viola-channel's test-only `test-support`
  feature (release-check proves it absent, probe 5/5). On a Windows host `run --mutants` runs in the guarded host scratch
  `<repo parent>/viola-mutants-scratch`. Also: the scoped `--file` inner loop, the `target/run-archive/` per-run archive, the lifecycle
  keep-failed guard, `scripts/wsl-exec.sh` and the machete annotation.
- The viola-pty watch RECURRED once (first local red, 16:02:58Z, capture absent). It stays OPEN with the expiry at 1 of 3, and the
  recorder is folded in (`CHILD_WITHIN` 7 s, the report streamed to a known file). Its CARRY moved to :45.

## Drift resolved
- 32 amendments (architecture 9, test-plan 18, security-plan 3, obs-plan 2); 2 proposals rejected (cargo-machete in §Stack;
  submodules in the tree). One boundary widening: `wsl-exec.sh`, a second WSL launcher, ratified LIVE by the overseer under the
  founder's 2026-09-27 ruling. Its invariant (clarified live): no gate, harness or plan entry runs a command through it, `--probe`
  excepted. Sidecars appended; 15 leaves re-derived.

## Notes
- rust-analyzer no longer needs stopping before a pre-push or `run --mutants` (the host scratch; host-win32.md corrected).
- **Owed to the operator:** H2, the product question (a key lost within ~50 ms of a ConPTY resize, rstudio/rstudio#18884).
- **For the operator:** the kept control home `target/e2e-home/viola-session-NETvJg`; `CARGO_BUILD_JOBS=16` kept; the `.wslconfig`
  memory cap is the operator/founder's call; the 4 `%TEMP%/cargo-mutants-viola-*.tmp` dirs and `target/harness-check/` are still
  left. The two leaked `viola-fake-agent.exe` from `Bk11Ik` were stopped by the overseer.
- **Deferred learnings:** `recurrence-despite-learning: testing.md 2026-09-24 (extended 2026-09-27) — a test deadline below the kill
  line` (the watched test sat at 10 s = the kill); CARRY 2's third cargo-mutants fact ("unviable without the fake-agent feature" —
  evidence not located beyond the evolve P3 parenthetical); the `"777"` digit-substring sweep hazard (0.8). Rejected below threshold: "nest a throwaway test repo one
  level down so its sibling scratch is its own" (0.2).
- **Carried from earlier wraps:** a `clean` guard's red half needs a stray clone-side file (0.8, cap); a `cfg!()`-valued fn is an
  equivalent mutant on one OS's leg — make it a const (0.8; applied again this chunk as `HOST_SCRATCH`); `check-runs` by sha mixes
  superseded runs after a force-push (0.8). The code-metrics `mutation.survivors` correction is owed at the next ledger-mode audit.
  Prior-chunk overseer items: `2026-09-26-local-linux-pre-push-gate/evidence/plan-template-proposal.md` and the planlint check-9
  slot question.
- Last failed command: none.

## Session End Status
Completed normally at 2026-09-27 20:50:49
