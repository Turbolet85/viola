# P5 review card 2 — 2026-11-24-viola-focus («`viola focus`», chk-fw7hhw4r) · 2026-11-24T02:26:00Z

**Plan to read:** `.andromeda/cache/docs/2026-11-24-viola-focus/plan.md`, a render of
`doc:2026-11-24-viola-focus/plan` rev 4. No plan file lives in a chunk's folder, and writing in that render changes
no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-24-viola-focus` diffs the chunk's records.

## What the review folded (`inputs#I3`, the overseer's review of card 1, snapped)
- **Technical answers**, the overseer's as a delegate: T1, T2, T4, T5, T6 and T7 each take their lean. T2's guard
  stands as built: a recorded address is trusted only inside the live wrapper's ancestry.
- **T3 → the operator's fork F-E** (`founder-forks.md`, the P5 review fold). It is about what «outside Hyprland →
  unsupported» means for v2-11.
  - Arm 1, taken PROVISIONAL: `unsupported` only with no `hyprctl` reachable; with `hyprctl` and no compositor,
    `unreachable`, so the seam's row stands.
  - Arm 2: remap a quick non-zero exit to `unsupported`, which amends security-plan ·
    decision/hyprctl-read-deadline.
  - The plan's Goal, the criteria, the constraints, the Expected amendments and the concretization below each say
    «PROVISIONAL, F-E» where they state this reading.
- **Change 1:** the control ids are now `focus-reuse` and `focus-no-client`; `focus-order` stands. All three are
  16 characters or fewer.
- **Change 2:** R3b fires only after a green R3a.
  - A red R3a ends the script at `r3: red hand=<word> opened=unfired`, with no `open` and no claude.
  - The script header and entry 19's note say that R3b's `open` is the ONE capped claude start, reached only
    through a green R3a.
- **Change 3:** R3a's stand-in is a `/bin/sh` file the script writes for itself, as r2-open.sh does
  (scripts/r2-open.sh:216-228). It answers `--version` alone and otherwise sleeps.
  - Not the fake agent: R3's build has no `fake-agent` feature, and a `fake-agent` build re-points the
    `target/debug/viola` hard link R3 runs (verification-harness 2026-10-20).
  - The preconditions read the stand-in written and its `--version` answered before anything starts.
- **Change 4:** if R3 is granted while R2 is still unfired, R3b is the first live `open`.
  - The open's own exit and `placement` are read first. A failed open reads `opened=open-<code>`, and focus is not
    fired; only after that does `opened=<focus word>` stand.
  - Entry 19's note and a new «live, unproven until R3» criterion line carry it into the report.

## Goal (unchanged)
`viola focus <name> [--json]` brings a live session's window forward and shows its workspace on Hyprland, through
the two dispatches the seam records for 0.56.2. The window is found by the wrapper's own identity:
- the recorded window, trusted only inside the wrapper's ancestry;
- else the window of the wrapper's nearest ancestor pid.

The verb is built on F-A's conservative arm, PROVISIONAL: CLI-only, no MCP tool, a live wrapper's own terminal
window only, nothing of the window printed.

## Top acceptance criteria
- Opened and hand-started sessions: the workspace dispatch, then the window dispatch; exit 0, `<name>  focused`.
- The address-reuse guard, with its `focus-reuse` control.
- Refusals:
  - `window-not-found` (no dispatch);
  - `unsupported` (no hyprctl, nothing spawned);
  - `unreachable` with hyprctl and no compositor (PROVISIONAL, F-E);
  - each seam word;
  - exit 21 for not live (before any hyprctl read) and for a widened home (before any log).
- No address, pid or workspace in any output or line; no snapshot or event write.
- `matrix#v2-11`, GATED on R3.

## Touchpoints (unchanged)
- New: `src/cmd/focus.rs`, `tests/cli_focus.rs`, `scripts/r3-focus.sh`.
- Modified: `src/cmd/mod.rs`, `src/human/hints.rs`, the fake `hyprctl` and its cases, `schemas/diag-line.v1.json`,
  and five test tables.

## Test commands (19 entries, no `run` changed)
- Every baseline of card 1 stands. The baseline listing reads: 5 seen green, 8 new (each baselined), 6 operator-leg.
- Entry 13's typed control stands (`control-entry-13.md`).
- Entry 8, the rehearsal, is the must-pass reading of entry 19's literal.

## Concretization preview — matrix#v2-11 (the never-weaken checkpoint)
- **Now:** «Focusing a named session brings its window forward on its workspace on Hyprland, for a session viola
  opened and for one started by hand; with no window found it refuses with a named cause, and outside Hyprland it
  refuses as unsupported.»
- **Proposed:** «Focusing a named LIVE session brings its window forward on its workspace on Hyprland, for a session
  viola opened and for one started by hand, read live at R3 as `r3: green hand=focused opened=focused` — the active
  window after each focus is that session's terminal window. With no window found it refuses `window-not-found`
  (exit 1, no dispatch). Outside Hyprland, read PROVISIONAL (F-E arm 1) as no `hyprctl` reachable, it refuses
  `unsupported` (exit 1, nothing spawned); a host with `hyprctl` and no compositor answering refuses `unreachable`
  until the operator answers F-E, which lands before any `verified`. The two refusals are proven over the fakes by
  their named cases.»
- **Gated criterion:** the block «R3, viola focus once on a hand-started session and once on a viola-opened session
  at the operator host, unlocked, its moment his to name (founder-forks F-D)». A stand-in, stub or recorded artifact
  never verifies it.

## Validation (re-run whole on rev 4)
- Mechanical: `gate.py run --dry-run` parses 19 entries; `planlint` 0 hits; the one «untouched» claim (line 239,
  `src/cmd/run.rs`) is contradicted by no step; `scope lists: parsed 13 items`; inputs 3, 0 unparsed (I3 cited 8
  times).
- Validation-1: aligned with scope rev 3.

## Beyond the plan
- The operator's forks F-A (the widening), F-B (the help row), F-C (the refusal wording), F-D (the live reading R3)
  and F-E (what «outside Hyprland» verifies) in `.andromeda/runs/2026-11-24T01-36-38Z-phase/founder-forks.md`. Each is
  built on its PROVISIONAL arm with his word owed, and is carried to the wrap and the handoff for him. F-A takes the
  playbook's Boundary-widening escalation (`rul-r74hshif`) at the wrap.
- The technical forks T1, T2, T4–T7 answered by the overseer as a delegate (`inputs#I3`); T3 is F-E.
- Left out, no owner proposed: a driver-facing focus (an MCP tool) — a further widening, only on his word (F-A arm 2).
- Left out, reading only: a footclient/foot-server window refuses `window-not-found`; a special workspace skips the
  workspace dispatch — both unmeasured live, carried in the report.
- R3b may be the first live `open` if R2 stays unfired; its red is then the open's, named `opened=open-<code>` —
  carried in the report's unproven lines.

Apply? (yes / review / cancel)
