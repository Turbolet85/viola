# Session Handoff

**Last Updated:** 2026-09-25T13:25:14Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (HEAD 8e25ca7 = the pushed pre-CI commit)
**Status:** clean
**Last Commit:** 2026-09-25-security-prerequisites — the chunk commit on top of the pre-CI commit 8e25ca7

## Position
- Done: `2026-09-25-security-prerequisites`.
  - The SQOS spike passed: the Windows client open reads `SecurityIdentification`, and `FILE_FLAG_OVERLAPPED` is required.
  - `sha2 =0.11.0` is picked for the `<hash>`.
  - Both are pinned by tests and recorded in the security-plan Decisions Log.
  - The repo is licensed MIT OR Apache-2.0.
  - `deny.toml` admits interprocess's two 0BSD crates by per-crate exceptions only.
  - Folded on the operator's word: the harness `run --mutants` verdict `test-only-rust-delta`.
  - CI run 36138441784 on 8e25ca7: 15/15 success.
- Next: `/andromeda-phase` to promote and plan "PTY wrapper on Windows", the next markerless Epoch 2 entry.

## Work done
- 7 files modified (manifests, `deny.toml`, the harness `mutants.rs`) and 5 new files (2 root tests, `LICENSE-MIT`, `LICENSE-APACHE`, `README.md`).
- Evidence: `chunks/2026-09-25-security-prerequisites/evidence/`: 5 remove-the-guard readings and the operator pass.

## Drift resolved
- **security-plan:** 7 proposals applied, plus the Decisions Log `2026-09-25` entry. It covers the SQOS spike, the SHA-256 crate and the 0BSD exceptions.
  - The 0BSD escalation (boundary widening) was resolved by the operator's recorded ratification.
- **architecture:** 10 applied (1 orchestrator-raised) and 1 rejected. Changes: the IPC client open, a Content hash row, the licence, the 0BSD policy, the tree, and the test-only pipe.
- **test-plan:** 7 applied: `test-only-rust-delta` in §3 / §10, the §6 SQOS and content-hash pins, and a §12 entry.
- **Leaves re-derived:** 9 (`security-summary`, `stack`, `services/viola-channel`, `services/viola-state`, `conventions`, `tests-summary`, rules `security` / `testing` / `verification-harness`).
- **Route:** 2 CARRYs, on "Instance state and start order" (sha2 as a `viola-state` dependency, plus the re-hash refusal test) and "Wrapper channel" (reuse the pinned SQOS open and land the viola-client SQOS negative).

## Notes
- **Operator decisions:**
  - The licence is MIT OR Apache-2.0, holder Turbolet85 (founder ruling).
  - 0BSD is admitted only per crate; each new exception needs a Decisions Log entry.
  - Option A: the harness `test-only-rust-delta` verdict. It covers `tests/`, `benches/` and `examples/` (measured); a mixed diff with no outcomes stays red.
- **Measured for the channel chunk:** a safe-Rust equivalent open (std `OpenOptions::security_qos_flags` + `custom_flags(FILE_FLAG_OVERLAPPED)`) also reads Identification. Choosing between the two opens is that entry's call.
- **Carried from the prior handoff:**
  - Deferred learnings (0.8 each): a `cfg!()`-valued fn is an equivalent mutant on one OS's leg, so make it a const; `check-runs` by sha mixes superseded runs after a force-push.
  - The code-metrics ledger's `mutation.survivors` correction (`lib.rs:9:31` / `:9:38`) is still owed at the next ledger-mode audit.
  - Two Windows-only unviable fake-agent `main` mutants were observed and not chased.
- Last failed command: none open.

## Session End Status
Completed normally at 2026-09-25 17:04:58
