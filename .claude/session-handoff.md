# Session Handoff

**Last Updated:** 2026-09-27T12:58:32Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-27-wrapper-channel — feat: wrapper channel — JSON-RPC ndjson endpoint, hardened listener, exclusive-bind start arbiter, first spans

## Position
- Done: 2026-09-27-wrapper-channel — the `viola-channel` crate (frames ≤ `MAX_FRAME`, `v`/`sender`/`conn`, `-32602`
  newer peer, `-32601` for unbuilt methods), the protected pipe DACL and SQOS client, the per-user 0700 Unix socket dir
  with chmod 0600 and the `.lock` arbiter, `squatted-name`, `endpoint` in the first snapshot, the Scenario 1 spans,
  the `channel_frame` fuzz target, `pre-push` `vm-release` + `windows-tests`. Epoch 2 is complete.
- Next: the Epoch 2 cleanup chunk the boundary ritual mints at the head (a 0-pending `/andromeda-wrap-session`
  route adaptation), then Hooks to normalised events (working-route:43) via `/andromeda-phase`.

## Work done
- Wrap resumed at P2 in `.andromeda/runs/2026-09-27T12-33-51-wrap/` (P1 had paused at the context alarm). Epoch-close
  sidecar consolidation ran (P7 3b) as the sidecars' backfill.

## Drift resolved
- 43 proposals applied outright (arch 10, security 5, test-plan 17, orchestrator raises 11); 7 arch proposals
  rejected for citing source/manifest lines, their report-carried facts re-raised; 1 escalation (E2 fd witness)
  resolved live: a premise fix, not a widening. 5 sidecar entries, incl. the founder's ratification of three
  standing widenings (on-disk basis: `ratification-evidence.md`). New playbook rule: what ratifies a widening.
- Route: 3 CARRYs — the two recurrence watches on the head entry (move once; retire after 3 consecutive green CI
  runs with no recurrence; the cleanup chunk tries a forced-window repro of the viola-pty hang first), landed-ahead
  controls on Epoch 6 "Windows endpoint admission" and Epoch 7 "Unix endpoint and home hardening".

## Notes
- **Every pre-push / light gate:** stop rust-analyzer by exact ExecutablePath first (host-win32.md).
- **For the operator:** `CARGO_BUILD_JOBS=16` is kept but its own effect on the host memory peak is not separable
  from the VM stop — the `.wslconfig` memory cap is the operator/founder's call (report Insufficient fixes).
- **Left for the operator (report):** 4 `%TEMP%/cargo-mutants-viola-*.tmp` dirs and `target/harness-check/`.
- **Deferred learnings (max-3 cap):** the `"777"` digit-substring sweep hazard over a doc carrying a git sha (0.8);
  the overseer's "a red found now folds into this chunk even outside its diff; a green re-run never closes a red"
  (0.7). The founder's boundary-widening ruling went to the playbook instead.
- **Carried from earlier wraps:** a `clean` guard's red half needs a stray clone-side file (0.8, cap); a
  `cfg!()`-valued fn is an equivalent mutant on one OS's leg — make it a const (0.8); `check-runs` by sha mixes
  superseded runs after a force-push (0.8). The code-metrics `mutation.survivors` correction is owed at the next
  ledger-mode audit. Prior-chunk overseer items: `2026-09-26-local-linux-pre-push-gate/evidence/plan-template-proposal.md`
  and the planlint check-9 slot question.
- The P1 evolve checkpoint could not be answered (its trace was in the prior window); recorded as `ok-degraded`.
- Last failed command: none.
