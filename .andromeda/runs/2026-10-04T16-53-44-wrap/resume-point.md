# Wrap resume point — 2026-10-04-dialog-answers-by-dialog-id

- **Stopped after P1** on the operator's directive (context 86.7 % measured; 90 % alarm at the P1 evolve append).
- **Next: P2** (Reconcile docs: fan-out → validate → apply → cascade). Resume with Setup 2a's offer → **resume** in this
  run dir; `chunk_dir/report.md` exists and is reused as is; no `fanout-results.md` yet, so P2 fans out fresh.
- Done here: Setup (pending 1 = this chunk; base = `c540254`, the parent of pre-CI commit `7f1364f`; operator-pass commits
  `7f1364f 2080e3f 2484b77 ca69e84`; branch `build/viola-0.1.0`, 0 ahead; code-graph refresh fired in the background) ·
  P1 scope read `clean — changed 60 · listed 42 · recorded 18` · `report.md` written · P1 evolve record appended.
- Tree at stop: uncommitted `evidence/operator-pass.md` (CI read 4 green record), `report.md`, this run dir, one
  friction-log append. No spec master or sidecar touched.
- **Carry into P5 (route-resolve), operator-directed:**
  - `:111` pin — the creation half moved here: a home outside `%USERPROFILE%` gets the protected user + SYSTEM DACL at
    creation (`viola_state::fs::create_private_dir`), the founder's live ruling 2026-10-04 ~16:40Z; `:111` keeps the
    rest of its Windows set (the per-entry-point checks of home, instances, bin, plugin, ui and the trusted files).
  - `:82` pins from the plan: S3 / S7 / S8 / dialog-concurrency ledger rows + their own `viola verify` re-probe
    superseding the relayed fixtures + the decision effect half (closes R2's dated gap); `verification-matrix.json#v1-15`
    claimed there; the `permission` kind's e2e Path 4 case and its `v1-30` wake from a recorded ordinary-tool
    PermissionRequest; the PermissionRequest body for a `question` first raised there.
  - `:80` pin: `answer`'s `human-typing` slot and the `null` answer on a wheel move.
  - D-3 (a) walk-class marks if registry U40 fires (operator directive; check `upgrade.py detect` at P2/P5).
- Owed amendment not in the plan's list: obs-plan §6 detail catalog — `parse-rejected{ledger-stamps}` gains
  `strict-modes-failed` (report: Spec claims disproved).
- Operator-owned, outside this chunk: stamp the installed `claude` 2.1.288; the stray recording home
  `~/.viola-record-20261004T142325Z` (founder desk queue); `~/.viola-record-20261004T142437Z` (run 2's home) left as is.
