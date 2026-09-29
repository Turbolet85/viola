# Session Handoff

**Last Updated:** 2026-09-28T21:24Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit is pushed at P7)
**Status:** clean
**Last Commit:** 2026-09-28-mutation-testing-to-the-epoch-boundary — the chunk's wrap commit (P7)

## Position
- Done: 2026-09-28-mutation-testing-to-the-epoch-boundary — mutation testing left chunks, the pre-push and CI (7 jobs /
  15 check-runs); `run --mutants` kept, named-only, for `/andromeda-code-audit`; legs, union, `cfg_legs`, syn/proc-macro2
  gone; the two real-cargo-mutants harness tests excluded on macOS (runner-side, measured). CI ci#36483042659 green 15/15
  on `17b93c7`.
- Next: **H2 ConPTY resize probe** (working-route :57, founder ruling 16:54) → `/andromeda-phase`. Then **Verify-stamped
  test homes and harness** (:59), which now also owns the coverage-profile channel CARRY.
- **The founder asked for a pause after this wrap** — do not start the next chunk without their word.

## Work done
- Implement, the operator pass (4 pushes: pre-CI, ONE macOS measurement push, two fix pushes), and this wrap in one
  session. Two CI reds folded: the measurement's corrupt coverage profile (ubuntu), macOS ENOTCONN in
  `channel_endpoint_answers_protocol_faults`.

## Drift resolved
- 59 proposals (arch 12 · security 10 · test 37 · the other four 0): 57 applied, 2 rejected (security Threat Model —
  verbatim copy); 4 raised by the orchestrator (obs ×3, a11y ×1) and 2 cascade-found dependents (test-plan :645, :1273);
  5 sidecar entries; 9 leaves re-derived; 0 escalations.

## Notes
- **Plan defect settled:** the entry-13 grep probe is NARROWED (filters out the refusal test `tests/cli.rs`), per the
  operator + overseer ruling; not a standing red.
- **For the operator to approve (proposed, not appended):** a playbook discriminator rule — a verbatim-upstream-copy
  section beats "Accurate this-chunk addition" (applied by precedent at this wrap's P2, security S7/S8).
- **Curation:** 3 corrections (verification-harness.md ×2, testing.md ×1), 1 extension (testing.md timing rule), 1 Tier 3.
- **For the operator:** `~/.viola-record` stays on the host by design; 4 `viola.exe` of `additional/viola-lab/prototype`
  are running (not this chunk's).
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-09-29 07:22:44
