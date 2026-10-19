# P5 review — 2026-10-19-dialog-hold-for-the-driver: "Dialog hold for the driver"

Plan: `viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/plan.md` · research and scope beside it · run dir
`.andromeda/runs/2026-10-19T07-38-25-phase/`.

## Goal
Close gap-list group 2 before the overseer switch, in the founder's own shape (`inputs#I2`):
- **Q1, the hold length.** A new `config.json` key, `dialog_hold_s`: 60–3600 s, default 60, with a bad value falling
  back to 60. The hook's bound (3605 s) and the `hooks.json` dialog timeout (75 → 3615) are compiled at the ceiling,
  so the plugin files stay identical across starts. Q1 ratifies the key as a widening.
- **Q2, at expiry.** The reply stays `null` and the CLI's menu is the human's. A late `answer` is still
  `unknown-dialog`.
- **The record.** A new wrapper-written event `dialog-expired {dialog_id, dialog_kind}` wakes `viola wait`, CLI and
  MCP alike, and the wrapper logs `dialog-answered{deadline_hit:true}`.
- **Q3, the live round.** One `viola verify` and one wrapped claude 2.1.287 start on the Linux dev host. One dialog
  is answered after at least 660 s, no key is typed into any dialog, and a red is never re-fired.

## Top acceptance criteria
- `DIALOG_HOLD_DEFAULT` / `_MIN` / `_MAX` (60 / 60 / 3600 s) are the hold's only homes, and `DIALOG_DEADLINE` is gone
  from every Rust source (count entry 10). The hook timeout outlasts the ceiling by at least 10 s.
- `dialog_hold_s` accepts whole numbers from 60 to 3600; anything else gives 60 and exactly one `parse-rejected`
  line. No env var or flag sets it, and it is a row of the controls table.
- On the injected clock a 120 s hold is still pending one step before 120 s and ends `null` at it. An expiry writes
  exactly one `dialog-expired` line and one `dialog-answered{deadline_hit:true}`. A hand-back or an answer writes
  none.
- `EventKind::ALL` has 16 kinds and `WAIT_WAKE` 6. `viola wait` prints
  `dialog-expired  <name>  dialog <id>  cursor <n>`, in ASCII with no SGR.
- The live round is recorded in `evidence/live-round.md`: the dialog is answered ≥ 660 s after it was raised; the
  wrapper's and the hook's `duration_ms` are ≥ 660 000; `decision_emitted` is true; `last` equals the chosen word;
  and `dialog_pending` reads true at 300 s.
- Clippy runs and closes the PREREQ, and CI is green on the operator pass's final HEAD.

## Touchpoints
- **New:** `evidence/`, holding the copied `live-pty.py` and `live-ledger.py`, the start ledger and `live-round.md`.
- **Modify (14):**
  - `crates/viola-core/src/lib.rs`, `src/run/dialog.rs`, `src/cmd/run.rs`, `src/cmd/hook.rs`
  - `plugin/hooks/hooks.json`, `crates/viola-agent-claude/src/lib.rs`, `src/obs.rs`, `schemas/config.v1.json`
  - `src/cmd/wait.rs`, `src/human/wait.rs`, `src/run/wait.rs`
  - `crates/viola-state/tests/state_events.rs`, `crates/viola-state/src/events.rs`
  - `tests/cli_controls_not_disableable.rs`

## Test commands (23 entries)
- **Lint:** fmt, clippy (closes the PREREQ), G1, lint probes.
- **Test layers:** unit, the new unit cases by name, integration, the two binaries by name, E2E.
- **Guards:** the `DIALOG_DEADLINE` count, the preservation guard (base `55699c4f2d26`), `cargo deny`.
- **Build:** `cargo build --bin viola`.
- **Booted smoke:** cleanup, boot, status, G2, G4, cleanup.
- **Pre-push and the operator pass:** `pre-push`, then hygiene → push → CI.

Baselines are written on every new entry (14), and the controls are in `p5-controls.md`:
- **Red, the subject not yet built:** 6 (0 tests selected; the control reads ok) and 10 (13 hits, all in
  modify-list files).
- **Green:** all the rest.

The live round is attended rig work, not an entry, so a light-gate re-run never spends the founder's starts.

## Concretization preview
No capability is claimed. `v1-21` (timing constants named and located) stays pooled. A P5 note will record that the
dialog deadline's homes move to `DIALOG_HOLD_DEFAULT` / `_MAX` plus the `dialog_hold_s` key, and that the cap still
needs every MCP call proved below the client's tool-call timeout.

## Notes for the review
- **Decisive leans** (plan Provenance):
  - The hook bound and the timeout are compiled at the ceiling, not rendered per start.
  - `dialog-expired` takes the `wait` line's dialog form (a planning form; the founder's wording is owed).
  - `dialog-answered{deadline_hit:true}` is reused, with no new obs event.
  - A late answer keeps `unknown-dialog`.
  - The wrapped start runs from rig B's already-trusted `proj/`, `target/e2e-home/viola-live-plugin-20261019T061245Z/proj/`
    (on the founder's desk). It is the one trusted `proj/` outside the work tree on disk, read `trusted` at P4, with
    empty plugin settings. A fresh one would show the trust dialog, and Q3 allows no key. This round adds one
    transcript dir for that path under `~/.claude/projects/`.
- **Size.** The plan is about 545 lines, or about 390 without the gate fence: inside the ~400 guide.
- **WATCH.** `send_budget_paused…` case_4 is observed only. This chunk runs nextest, not plain `cargo test`, so it
  stays at 0/3.
- **One slip at P5.** The first clone firing of entries 10 and 11 (unmodified) piped the gate tool's output through
  `grep`, against the setup letter. The filter kept each entry's own verdict row, which matched the real tree's
  reading. Every later control was fired and read unfiltered. The slip is recorded at the evolve checkpoint.

## Beyond the plan
1. **Owner of the hold's Windows leg:** "Windows-only live measurements" (an existing entry): a CARRY naming the
   live reading of a dialog hook held past 75 s on claude 2.1.287 (measured on the Linux dev host at this chunk).
2. **Owner of the 3600 s ceiling beyond the measured ~660 s:** "Overseer switch to viola" (an existing entry): a
   CARRY saying that its live switch, with the overseer's home at `dialog_hold_s` 3600, is where a hold near the
   ceiling is first read live. The ceiling stays PROVISIONAL until then.
3. **The founder's wording owed** for the `dialog-expired` kind word and its `wait` line (design-system /
   layout-templates planning forms): carried on the handoff for his Epoch 5 boundary batch.
