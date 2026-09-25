# Session Handoff

**Last Updated:** 2026-09-25T18:07:16Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (HEAD 7681c73)
**Status:** clean
**Last Commit:** 2026-09-25-pty-wrapper-on-windows — feat: viola run over ConPTY via viola-pty, R8 prefix strip with identity floor, npm-shim resolution and .cmd/.bat refusal

## Position
- Done: `2026-09-25-pty-wrapper-on-windows`.
  - `viola run` hosts its child in ConPTY / openpty through the new `viola-pty` seam, with zero bytes of its own.
  - The R8 strip is a `CLAUDE*` prefix rule. Persistent-environment names are exempt, except the 11-name identity floor.
  - `.cmd`/`.bat` children are refused, and the npm shim resolves to `claude.exe`.
  - The harness runs on outer PTYs. Also folded in: the orphans gate reads all 3 CI triples, and the ripgrep install is revocation-resilient.
  - CI run 36167590761 on 7681c73: all green.
- Next: the operator inserts "Local Linux pre-push gate" (WSL2 Ubuntu) at the route head via a 0-pending wrap. Then `/andromeda-phase` promotes it. Without that insert, the head is "Instance state and start order".

## Work done
- 2 new crates (`viola-pty`, `viola-agent-claude`), `src/run/env.rs`, 4 new root test binaries plus `tests/support/outer_pty.rs`, harness `supervise` on PTYs, and the orphans and ripgrep scripts.
- Evidence: `chunks/2026-09-25-pty-wrapper-on-windows/evidence/`. It holds 11 remove-the-guard pairs and the operator pass with 3 CI runs.

## Drift resolved
- 58 proposals: 51 applied, 7 rejected, plus 3 cascade fixes. Four escalations were resolved with the operator:
  - **E1:** R8 widening ratified via ruling 1, with a security Decisions Log entry.
  - **E2:** the `PtyError` hand-written exception was ratified.
  - **E3:** "ConPTY swallows focus reports" stays a labelled hypothesis.
  - **E4:** verbatim-section edits were rejected, and the playbook rule was widened.
- Masters amended: architecture, security-plan, test-plan, obs-plan (D-34 supersedes D-14) and a11y-plan. Design-system and layout-templates were clean.
- 8 leaves re-derived. 1 playbook rule appended.
- Route: 5 CARRYs, on "Instance state and start order" (`VIOLA_*`), "Wrapper channel" (E2 Unix fds, `pty.spawn` plus seam spans) and "The wheel" (the `^Z` and focus-report hypotheses).

## Notes
- **Operator decisions:**
  - **Direction 1:** the 2 union-breach mutants (`HostTerminal::enter -> Some(Default::default())`, run 36165685381) are a reasoned ratification, with no product change. Each is unviable on the leg that compiles it.
  - **Direction 3:** the mutation verdict is run 36165685381's legs plus fix runs 36166907442 / 36167590761.
- **For "Local Linux pre-push gate", from overseer direction 2:** pin a CARRY for the union-rule fix. The rule: a mutant unviable on the leg that compiles its code is not a survivor, whatever the other leg reads (`gate --mutants-legs`).
- **Unowned observation:** `secret-scan` read `target/agent-run/chunk.diff` residue left by a prior `run --mutants`, where it hit its own `?t=` pattern line. The scan's scope is a harness-owner question and has no route entry.
- **Deferred learnings:**
  - rust-analyzer (the session LSP) holds `mutants.out`, so cargo-mutants' rename fails (`os error 5`) until it is stopped (0.7). This extends the host-win32 2026-09-24 Monitor entry.
  - **Carried:** a `cfg!()`-valued fn is an equivalent mutant on one OS's leg, so make it a const (0.8); `check-runs` by sha mixes superseded runs after a force-push (0.8).
- **Carried:**
  - The code-metrics `mutation.survivors` correction (`lib.rs:9:31` / `:9:38`) is owed at the next ledger-mode audit.
  - Two Windows-only unviable fake-agent `main` mutants were observed and not chased.
- **Epoch 2** holds 9 chunks. The planned insertion makes 10, which is where a boundary would restore the diagnose/audit cadence. That call is yours.
- Last failed command: none open.
