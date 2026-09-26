# Session Handoff

**Last Updated:** 2026-09-25T18:19:16Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (HEAD fcca1ce)
**Status:** clean
**Last Commit:** chore(route) — operator-requested adaptation, 0-pending wrap (session 13)

## Position
- Done: `2026-09-25-pty-wrapper-on-windows` (last chunk). This session made no chunk; it adapted the route only.
- Next: `/andromeda-phase` promotes "CI chunk base and union verdict" (`working-route.md:34`). "Local Linux pre-push gate" follows it.

## Work done
- The operator's route adaptation (founder rulings 2026-09-25) added 2 entries ahead of "Instance state and start order":
  - **A, "CI chunk base and union verdict".** It has 3 CARRYs:
    - the CI chunk base comes from the last master-flip pickaxe, not `github.event.before` (`ci.yml:166`);
    - the union rule for run 36165685381's `HostTerminal::enter` mutants;
    - secret-scan's `chunk.diff` residue scope.
  - **B, "Local Linux pre-push gate".** It has 1 CARRY: the WSL host facts measured this session, the CI tool pins, and a gate that runs before the push.
- Epoch 2 now ends after "Wrapper channel". A new `### Epoch 2b — Windows slice I b: events and ledger` holds Hooks, CLI output tokens, Capability ledger and Fake-agent drift contract.
- Record: `.andromeda/runs/2026-09-25T18-17-26-wrap/adaptation-record.md`.

## Drift resolved
- None. The 0-pending path runs no report and no fan-out.

## Notes
- **Operator decisions:**
  - **Direction 1 (last chunk):** the 2 union-breach mutants are a reasoned ratification, with no product change.
  - **Direction 3:** the mutation verdict is run 36165685381's legs plus fix runs 36166907442 / 36167590761.
  - The union-rule CARRY and the secret-scan observation now sit on entry A.
- **Open for the operator:** the Epoch 2 header still names "events, ledger", which moved to 2b. It was left unrenamed because friction records key on the header text byte-exact.
- **Deferred learnings:**
  - rust-analyzer (the session LSP) holds `mutants.out`, so cargo-mutants' rename fails (`os error 5`) until it is stopped (0.7).
  - **Carried:** a `cfg!()`-valued fn is an equivalent mutant on one OS's leg, so make it a const (0.8); `check-runs` by sha mixes superseded runs after a force-push (0.8).
- **Carried:**
  - The code-metrics `mutation.survivors` correction (`lib.rs:9:31` / `:9:38`) is owed at the next ledger-mode audit.
  - Two Windows-only unviable fake-agent `main` mutants were observed and not chased.
- Last failed command: none open.

## Session End Status
Completed normally at 2026-09-26 21:30:27
