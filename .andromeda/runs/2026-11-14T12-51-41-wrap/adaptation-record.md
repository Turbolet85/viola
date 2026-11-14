# Adaptation record — 0-pending wrap 2026-11-14T12-51-41 (viola-0.2.0, the Epoch 1 boundary)

Path: SKILL Setup 6, the no-op path with an operator route-adaptation request (route-resolve §Operator-requested
adaptation). 0 pending, 0 gated; the tree dirty only with expected-transient bookkeeping (the journal, the handoff,
the last wrap's `evolve-2026-11-14-hyprland-seam.json` trail, the untracked diagnosis and code-audit run dirs).

Inputs (snapped `--no-chunk` into `.andromeda/runs/2026-11-14T12-51-41-wrap/inputs/`):
- inputs#I1 — the Viola overseer's route-adaptation relay, `../additional/viola-overseer/e1-0.2.0-route-adaptation.md`
  (copy, no-repo): the context, the one item for disposition, the anchors.
- inputs#I2 — the wrap invocation message: the go-ahead, and both CI ids right — ci#43579735207 read the pre-CI commit
  `93b2ca1d`, ci#43589375075 read the wrap commit `25348549` (green 7/7, the overseer's read).

## Items

1. One corrective entry for the code audit's two surviving mutants in `viola-hyprland` (run
   `2026-11-14T11-39-19-code-audit` at `25348549`, 96.00 %, 48 caught · 2 missed): `crates/viola-hyprland/src/read.rs:77`
   (`read_within`) and `crates/viola-hyprland/src/reply.rs:151` (`parse_dispatch`), each `>` → `>=` on the `MAX_FRAME`
   bound — one missing case, a reply of exactly `MAX_FRAME` bytes accepted and one byte more `oversize` (inputs#I1 §2).
2. Placement: the corrective entry first in the markerless tail, as Epoch 2's first entry; «Budget gate removed» next
   after it; nothing else in the tail moves; no carry re-homed (inputs#I1 §3).
3. Out of the entry: the two mutation units the audit declined (`viola-e2e`, `viola`), waiting on a pipeline mend; the
   audit's «consider» row (the `test_capture` clone) and its two «deliberately not» rows (`viola_hyprland::PROGRAM`, the
   complexity list) — no route item (inputs#I1 §2).
4. No requirement added and no capability claimed by the entry: a test-only corrective (inputs#I1 §3).

## Dispositions

1. APPLIED — a new markerless entry `chk-ut3m6zi6` «Hyprland exact-frame bound pinned» (route-contract §Write row 17),
   its body worded by what the next boundary's audit reads (both survivors caught), never a mutation result the chunk
   must produce; its coordinates in one carry `cry-pqnlx5pz` (`CARRY:`, row 10), citing the relay snapshot by its path.
   Authority: the trajectory fork is the founder's — his selection «Корректирующий чанк первым (Recommended)», given in
   overseer1's question dialog at 2026-11-14T12:48:08Z, relayed verbatim by overseer1 via the Viola overseer, 2026-11-14,
   inputs#I1 — PROVISIONAL by gate-contract §Scope, *Whose word* (a relayed answer on a fork he owns): it satisfies the
   trajectory gate and the work proceeds on it; his own later word supersedes it by rule. The entry's wording and the
   overseer's recommended disposition: relayed by the Viola overseer, 2026-11-14, inputs#I1.
2. APPLIED — `rec.py put` TX 1110: `@chunk in=epc-h7tbdeih` placed the entry, and «Budget gate removed»,
   «Status-line interception removed», «Live round on the trimmed product» re-placed after it in their standing order
   (ords 4096 · 5120 · 6144 · 7168 — the three entries' relative order unchanged; no other epoch touched). Read back:
   `position` names `chk-ut3m6zi6` next (markerless 26, entries 30); `pins` lists its one carry, introducer `CARRY:`;
   `abstained` 0 rows. «Budget gate removed» carried no freight (0 blocks), so no PREREQ or WATCH moved; the placement
   is the overseer's technical answer by relay (the operator away, inputs#I1 §1).
3. NOTHING WRITTEN — named in the entry's carry as out of it; no route entry, no residual.
4. NOTHING WRITTEN — no `matrix.py add`, no claim; the entry carries `no requirement added, no capability claimed`.

## The no-op path's other duties
- Gated premises: the `gated` view holds 0 rows — nothing to re-verify.
- Curation: not run — this window carried no correction of the work (the CI-id question was answered as a
  clarification, inputs#I2).
- No master amendment, no consolidation, no seed-rule supersession requested.
