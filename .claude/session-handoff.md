# Session Handoff

**Last Updated:** 2026-09-27T06:40:51Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-27-instance-state-and-start-order — feat: append-only event log, atomic snapshot, heartbeat, pinned bin copy with re-hash refusal, plugin folder, live/stale name refusal, VIOLA_* at spawn

## Position
- Done: 2026-09-27-instance-state-and-start-order — `viola-state` (events, snapshot, heartbeat, liveness, `replace_private`),
  the `viola run` start order (collision → pin copy → plugin → snapshot → heartbeat → start events → spawn), the pinned
  `bin/` copy refused on a failed re-hash, and `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` + the PATH prefix at child spawn.
- Next: Wrapper channel (working-route:40): `/andromeda-phase` to promote and plan it.

## Work done
- Wrap resumed across a session boundary. It had paused at the context alarm after P3, so the P3 evolve checkpoint and
  P4–P7 ran in this session from `.andromeda/runs/2026-09-27T06-12-23-wrap/`.

## Drift resolved
- 48 fan-out proposals plus 1 raised by the orchestrator, applied to architecture, security-plan, obs-plan and test-plan
  bodies, with four sidecar entries. The biggest change: atomic-write-file → tempfile `persist` through the one
  `replace_private` helper, because BSD-3-Clause is outside the licence allow list. Also: `endpoint` is absent until
  "Wrapper channel"; a gone pid is `gone`. 19 leaf lines re-derived in 9 files.
- 0 escalations open. The Snapshot-writer reversal and the pre-push `TMPDIR` `env -i` carve-out were settled by
  recorded overseer directions.
- Route: 8 CARRYs. Four on "Wrapper channel": snapshot `endpoint` + exclusive-bind arbiter + Scenario 1 spans; the
  viola-pty CI flake; the pre-push Windows coverage `test` stage; the `replace_private_shared` reasoned-fix recurrence.
  Four more: plugin hooks + `events.ndjson` line 3 on "Hooks to normalised events", `settings.json` on "Statusline
  pass-through", and the `viola-state` round-trip suite and Path 1 `path_` binary on "Self-healing state" and "The
  board".

## Notes
- **Every pre-push / light gate:** stop rust-analyzer by exact ExecutablePath first (host-win32.md).
- **For the operator (playbook proposal):** "a locked-decision reversal settled by a recorded operator direction →
  apply, and record the direction in the sidecar" (two such reversals applied this wrap).
- **recurrence-despite-learning (4):** events.md "append + exclusive lock fails on Windows"; verification-harness.md
  "Never pipe agent-run.sh boot"; host-win32.md "Stop a process by its exact ExecutablePath"; host-win32.md §Paths
  MSYS conversion (`wsl.exe --exec /usr/bin/…`). Each rule was already curated and the work re-hit it anyway.
- **Deferred learnings (carried):** a `clean` guard's red half needs a stray clone-side file (0.8, cap); a
  `cfg!()`-valued fn is an equivalent mutant on one OS's leg, so make it a const (0.8); `check-runs` by sha mixes
  superseded runs after a force-push (0.8).
- **Carried:** the code-metrics `mutation.survivors` correction (`lib.rs:9:31` / `:9:38`) is owed at the next
  ledger-mode audit; two Windows-only unviable fake-agent `main` mutants observed, not chased. Prior-chunk overseer
  items: `2026-09-26-local-linux-pre-push-gate/evidence/plan-template-proposal.md` and the planlint check-9 slot question.
- Epoch 2 header wording stays as is (overseer decision: the friction-log grouping keys on it).
- Last failed command: none.
