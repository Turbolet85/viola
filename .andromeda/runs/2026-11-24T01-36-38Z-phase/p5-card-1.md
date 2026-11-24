# P5 review card 1 — 2026-11-24-viola-focus («`viola focus`», chk-fw7hhw4r) · 2026-11-24T02:21:22Z

**Plan to read:** `.andromeda/cache/docs/2026-11-24-viola-focus/plan.md`, a render of `doc:2026-11-24-viola-focus/plan`
rev 3. Open it in an editor. No plan file lives in a chunk's folder, and writing in that render changes no record
until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-viola-focus`
diffs the chunk's records.

## Goal
`viola focus <name> [--json]` brings a live session's window forward and shows its workspace on Hyprland. It uses
the two focus dispatches the seam already records for 0.56.2. It finds the window by the wrapper's own identity:
- the window `viola open` recorded, trusted only when that window's pid is in the wrapper's ancestry;
- else the window of the wrapper's nearest ancestor pid (a hand-started `viola run`).

Every other outcome is a named refusal. It is built on the conservative arm of a boundary widening, PROVISIONAL
(founder-forks F-A): a CLI-only human verb with no MCP tool, raising only a live wrapper's own terminal window, and
printing no window address or workspace.

## Top acceptance criteria
- Opened session: in order, the workspace dispatch, then the window dispatch; exit 0 and `<name>  focused`.
- Hand-started session: the same, the window found by ancestry.
- A recorded address outside the wrapper's ancestry is never focused (the address-reuse guard, with its control).
- Refusals:
  - `window-not-found` (no dispatch);
  - `unsupported` (no `hyprctl` on PATH, nothing spawned);
  - `unreachable` and each other seam word;
  - not live: exit 21 before any hyprctl read;
  - a widened home: exit 21 before any log.
- No output or log line holds an address, pid or workspace; focus writes no snapshot and no event line.
- `matrix#v2-11`, GATED on R3.

## Touchpoints
- New:
  - `src/cmd/focus.rs`
  - `tests/cli_focus.rs`
  - `scripts/r3-focus.sh`
- Modified:
  - `src/cmd/mod.rs`
  - `src/human/hints.rs`
  - the fake `hyprctl` (`terminal` mode answers the two focus dispatches on its own window; a `hyprctl-asked`
    receipt) and its cases
  - `schemas/diag-line.v1.json`
  - the five tables: `cli_output_plain`, `cross_exit_causes`, `contract_error_bodies`, `cli_error_bodies`,
    `contract_diag_schema`
- Untouched: `src/cmd/open.rs` and the start path.

## Test commands (19 entries)
- The checks:
  - fmt and clippy;
  - the root units, and the new-case filter `::focus_` — at the first baseline an unanchored `focus_` selected three
    wheel cases; fixed and re-read 0 of 1288;
  - the new integration cases `^focus_`;
  - the seven reader binaries (141 green);
  - `bash -n` of the R3 script;
  - the R3 rehearsal over the fakes (the must-pass reading of the live literal);
  - the host read: QUIET, 0 s stalled;
  - the harness's three layers.
- The gated entry 13 reads `evidence/r3.md`. Its control is typed beside it (`control-entry-13.md`).
- Operator pass:
  - pre-push, hygiene, the push and the CI read;
  - then, only on his word, the release-shaped build and R3 (ONE claude start, two focus changes on his screen).
- Three remove-the-guard controls, all at implement: `focus-reuse-guard`, `focus-order`, `focus-no-dispatch`.

## Concretization preview — matrix#v2-11 (the never-weaken checkpoint)
- **Now:** «Focusing a named session brings its window forward on its workspace on Hyprland, for a session viola
  opened and for one started by hand; with no window found it refuses with a named cause, and outside Hyprland it
  refuses as unsupported.»
- **Proposed:** «Focusing a named LIVE session brings its window forward on its workspace on Hyprland, for a session
  viola opened and for one started by hand, read live at R3 as `r3: green hand=focused opened=focused` — the active
  window after each focus is that session's terminal window. With no window found it refuses `window-not-found`
  (exit 1, no dispatch). Outside Hyprland, with no `hyprctl` reachable, it refuses `unsupported` (exit 1, nothing
  spawned); the last two are proven over the fakes by their named cases.»
- **Gated criterion:** the live clause waits on the block «R3, viola focus once on a hand-started session and once on
  a viola-opened session at the operator host, unlocked, its moment his to name (founder-forks F-D)». A stand-in,
  stub or recorded artifact never verifies it.
- **Read for weakening:** «outside Hyprland → unsupported» is read as «no `hyprctl` reachable → unsupported». A host
  with `hyprctl` installed but no compositor answering refuses `unreachable`, a named cause but not the word
  `unsupported`. The seam's own row (security-plan · decision/hyprctl-read-deadline: «a deadline kill or a non-zero
  exit reads as unreachable») decides this (T3). If you read it as a weakening, the other arm is T3's remap,
  which amends that row.

## Validation
- Mechanical: all pass on rev 3. 0 planlint hits; the one «untouched» claim (line 223, `src/cmd/run.rs`) is
  contradicted by no step. inputs 2, 0 unparsed.
- The rows files total 303 779 B, past 250 000 B, so P4 read them by search. Each cited row was checked present in
  its rows file.
- Validation-1: aligned with scope rev 3 (its four P3 corrections carried).
- Aggregate B: 0 masters read `picked 0`.

## Beyond the plan
- The operator's forks F-A (the widening), F-B (the help row), F-C (the refusal wording) and F-D (the live reading R3),
  in `.andromeda/runs/2026-11-24T01-36-38Z-phase/founder-forks.md` — each built on its PROVISIONAL arm, his word owed;
  carried to the wrap and the handoff for him. F-A takes the playbook's Boundary-widening escalation
  (`rul-r74hshif`) at the wrap.
- The technical forks T1–T7 (the same file) on their leans, the overseer's relay owed before implement.
- Left out, no owner proposed: a driver-facing focus (an MCP tool) — a further widening, only on his word (F-A arm 2).
- Left out, reading only: a footclient/foot-server window (its pid no ancestor) refuses `window-not-found`; a special
  workspace skips the workspace dispatch — both unmeasured live, carried in the report.

Apply? (yes / review / cancel)
