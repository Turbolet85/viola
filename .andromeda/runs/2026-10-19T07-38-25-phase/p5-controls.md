# P5 controls — 2026-10-19-dialog-hold-for-the-driver

Every control fired as the entry (`gate.py run --plan … --entry {n} --project-root {clone}`, no `--run-dir`), over a
scratch clone `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/31f2d6e7-a2ba-4915-8008-415399b36d2a/scratchpad/clone`
(`git clone --shared` of the project, HEAD `55699c4f2d26`). Inputs live in the clone only.

## Entry 10 — the `DIALOG_DEADLINE` count
- Real tree (baseline): `red · last line '0' ✗ (last: '13')` — every hit in the four modify-list files
  (`src/cmd/hook.rs` 5 · `src/run/dialog.rs` 4 · `crates/viola-core/src/lib.rs` 2 · `crates/viola-agent-claude/src/lib.rs` 2).
- Clone, unmodified: `red · last line '0' ✗ (last: '13')` — the same reading.
- Clone, the name replaced by `DIALOG_HOLD_MAX` in those four files (must-pass): `green · exit 0`.

## Entry 6 — the new unit cases by name
- Real tree (baseline): `red · exit 0 ✗ (exit 1)` — `"ok":false`, nextest-unit `failures ["nextest-exit-4"]`, 0 tests
  run, 2354 skipped: the cases do not exist yet.
- Control (typed, the filter operand alone changed, `/usr/bin/bash -o pipefail -c`, on the real tree): the same form
  over `hook_dialog_expires_at_the_deadline_on_the_injected_clock` → exit 0, `"ok":true`, nextest-unit 1 passed 0 failed.

## Entry 11 — the preservation guard
- Real tree (baseline): `green · exit 0` on the untouched tree.
- Clone, unmodified: `green · exit 0`.
- Clone, the four modify-set files edited and `viola-0.1.0/chunks/…/evidence/live-round.md` minted (must-pass):
  `green · exit 0`.
- Clone, `Cargo.toml` edited (must-fail): `red · exit 0 ✗ (exit 1)`.
- Clone, an untracked `src/new_file.rs` (must-fail): `red · exit 0 ✗ (exit 1)`.
