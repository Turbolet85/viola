# P5 review card 1 — «Switching a session's model and effort» (`2026-11-24-switching-a-session-s-model-and-effort`)

The plan is `doc:2026-11-24-switching-a-session-s-model-and-effort/plan` rev 4. Its render is at
`.andromeda/cache/docs/2026-11-24-switching-a-session-s-model-and-effort/plan.md`, for an editor:
- no plan stands in a chunk's folder;
- writing in that file changes no record until a `put`;
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-switching-a-session-s-model-and-effort`
  is a diff of the chunk's records.

## Goal
`viola switch <name> [--model M] [--effort E] [--json]` switches a RUNNING wrapped session by an in-place restart.
- The session's own wrapper serves a new channel method `switch`. It refuses unless the session is idle and the human
  is off the keys.
- It ends its OWN claude child (SIGHUP through the PTY seam, reaped within 10 s), appends one `launch` line (and the
  `window` line again where one stands), and respawns `claude --resume <id> --model M --effort E` in the same PTY,
  directory and terminal.
- A later `revive` or `up` replays the switched pair.
- Nothing is written to the user's Claude config or settings, nothing is typed into the session, and there is no MCP
  tool.

**Built on the founder's forks, each PROVISIONAL with his own word owed** (`.andromeda/runs/2026-11-24T12-23-47Z-phase/founder-forks.md`, Block 1):
- F1 arm 1, the in-place restart: a boundary widening — a new channel method, and a wrapper ending its own live child.
- F3 arm 1: refuse mid-turn, with a dialog pending or while the human types.
- F5 arm 1: CLI-only.
- Technical proposals for the overseer's relay: F2 (the verb's surface), F4 (records and words) and F6 (the 10 s
  reap bound, as its own a11y row).
- F7: one implement window (no split recommended).
- F8: the live readings R6a/R6b, carried to «The fleet on the founder's host».

## Top acceptance criteria (9 in the plan)
1. An idle switch exits 0 and the wrapper pid stays the same. The new child's argv (read from `/proc`) carries
   `--resume <id> --model M --effort E`. The log gains exactly one `launch {M,E}` and one `session-start` of cause
   `resume` with the same id.
2. The user's Claude config and settings stay byte- and mtime-equal, and no `--settings` is passed.
3. After a switch, `viola revive` with no pair resumes on the switched pair.
4. The refusal order: human typing (10), turn running (13), dialog pending (13), unverified CLI (12), no session (13).
   None appends a line or ends the child.
5. `launch` is appended after the old child's reap and before the respawn. A child not ended within the bound leaves
   the session as it was.

## Touchpoints
- **New:** `src/cmd/switch.rs`, `src/run/switch.rs`, `tests/cli_switch.rs`.
- **Modified:** `crates/viola-pty/src/pump.rs`, `crates/viola-pty/src/lib.rs`, `src/cmd/run.rs`, `src/run/mod.rs`,
  `src/cmd/mod.rs`, `src/human/hints.rs`, `tests/cli_output_plain.rs`, `tests/contract_error_bodies.rs`,
  `tests/cross_exit_causes.rs`, `schemas/diag-line.v1.json`.

## Test commands (18 entries)
- **Lint:** fmt and clippy.
- **Unit:** viola-pty (baseline green, 40/40) · the root binaries (green, 1370/1370) · the new `::switch_` units
  (red-vacuous, 0 of 1370).
- **Integration:** the new `^switch_` cases (red-vacuous, 0 of 4359) · the readers of the moved literals, tables and
  schema (green, 168/168).
- **Host read:** hostwatch (recorded CONTENDED: 25 s IO stall from escher's build).
- **Layers:** the harness's three.
- **Operator pass:** boot/status/cleanup smoke on `switch-smoke` · pre-push · hygiene · push · CI read.
- **Remove-the-guard controls** (implement's one-shots): `sw-turn`, `sw-dialog`, `sw-launch-first`,
  `pty-reap-bound`.
- No live entry and no round.

## Capabilities — no claim, so no concretization preview
- **v2-31 — not claimed.** Its requirement names the console surface, which this chunk does not build. Its «takes
  effect for its next turn» is the real CLI's, never read back (`architecture · decision/launch-pair-handed-not-verified`).
- **v2-30 — not claimed.** A switch adds no clause it names, and its up and adopt legs wait on R4 and R5.
- After the word, each gets one dated notes line (`matrix.py note`); nothing is claimed.

## Mechanical checks (validation.md), all run on rev 4
- **Passed:**
  - 1 sections present;
  - 2 the modify paths exist, none is a master, no ledger note on an unverified cap;
  - 3 the new paths are free, and «switch» occupies nothing in architecture (`rec.py find … --regex (?i)\bswitch\b`:
    0 hits);
  - 4 (0) the fence parses: 18 entries, exit 0;
  - 4 (2) the boot path changes, so a smoke trio was added;
  - 4 (9) all six new entries were baseline-run and their keys written;
  - 5 no placeholder;
  - 10 inputs: 2 entries, unchanged 1, n/a 1, uncited 0, unparsed 0;
  - 12 step references: 0 missing.
- planlint: hits 0. Research's lists: `scope lists: parsed 13 items`.
- **WARN, check 7:** no code-graph query was recorded. The caller sets were held back behind the mechanism fork
  (research §Graph impact); implement reads them once F1 stands.
- **Check 8:** criteria 1, 2, 4 and 5 turn on sites this chunk creates (out of scope). Criterion 3's reach was read
  at `src/cmd/revive.rs:278-298` (`given_pair`) and `crates/viola-state/src/replay.rs:125` (`apply_launch_line`):
  the last `launch` line is what revive hands.
- **Check 11:** each test ends only what it started, and the switch ends only the child its own PTY handle holds.
- **Validation-1:** aligned with `doc:…/scope` rev 3 and the working entry: the console and the real-CLI reading are
  out, nothing live, nothing in `~/.claude*`.

## The directive's item-11 block (verbatim where a tool printed it)
```
architecture: rows 1040 · packs 11 · picked 402 (275 by every draw) · standing 42 · 103286 B
security-plan: rows 660 · packs 7 · picked 309 (247 by every draw) · standing 17 · 81464 B
design-system: rows 360 · packs 4 · picked 131 (106 by every draw) · standing 4 · 32091 B
layout-templates: rows 234 · packs 3 · picked 95 (60 by every draw) · standing 6 · 21162 B
test-plan: rows 1385 · packs 14 · picked 385 (246 by every draw) · standing 74 · 89725 B
obs-plan: rows 1054 · packs 11 · picked 503 (365 by every draw) · standing 23 · 119810 B
a11y-plan: rows 242 · packs 3 · picked 148 (119 by every draw) · standing 19 · 38249 B
rows-union: 7 master(s) · picked 1973 of 4975 · rule any
```
- **Cited rows:** 15 checked. 13 stand in the union. 2 do not — `architecture · decision/pty-knows-no-agent` and
  `architecture · decision/focus-has-no-driver-form` — and were looked up small by `rec.py find` and cited by handle.
  The rows-file sum is 485,787 B, past 250,000 B, so the rows files were read by search, never whole.
- **[RETYPED] lines, 5, each read by check as the pack's row:**
  - `arch-pick-p4.d3 · [RETYPED] line 96: decision/fake-agent-test-seam-variables is no row of the pack — read as decision/fake-agent-test-seams, the pack's row 96`
  - `arch-pick-p8.d3 · [RETYPED] line 69: decision/verify-probe-dirs-drop-guard is no row of the pack — read as decision/verify-probe-dirs-drop-guarded, the pack's row 69`
  - `security-pick-p4 · [RETYPED] line 78: decision/live-rig-kept-stamp-owner-only-seeded-to-its-own-home is no row of the pack — read as decision/live-rig-kept-stamp-owner-only, the pack's row 78`
  - `security-pick-p4 · [RETYPED] line 98: decision/plugin-files-rewritten-every-start is no row of the pack — read as decision/plugin-files-rewritten-each-start, the pack's row 98`
  - `tests-pick-p10.d2 · [RETYPED] line 60: decision/external-processes-real-stub-executables is no row of the pack — read as decision/external-processes-real-stubs, the pack's row 60`
- **Re-spawns:** none. 159 pickers, 7 task writers and 7 history agents each passed first try; no draw dead.
- **Non-zero exits and refusals:**
  - `inputs.py snap --dry-run` exit 2: `--origin: 12:2xZ is not a time anyone read — a placeholder stands where a digit
    would`. Re-fired with an origin naming no time.
  - `rec.py find` exit 2: `find: --kind, or --doc`. Re-run per master through a scratchpad loop.
  - The Bash guard refused 3 calls: two `cd` into the run dir, and one pipe of a tool's output into grep. Each was
    re-issued once in the guard's form.
  - Gate baselines: entries 5 and 6 exit 4 (`no tests to run`), the red-vacuous readings recorded.
  - No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed.
- **Where a letter was wrong or silent:**
  - The plan template's skeleton offers no form for a boot smoke on an agent-driven project. The first plan put no
    smoke entry, and P5 check 4 (2) caught it, so the trio was copied from the launch chunk's plan.
  - SKILL P4 asks for every history file to be read whole. Four were (arch, security, tests, obs); design, layouts
    and a11y (42 KB) were read by search only, recorded as a deviation at the plan checkpoint.

## Beyond the plan
| # | Proposal the word approves | Owner |
|---|---|---|
| 1 | The founder's answers to F1, F3, F5 and F7 (his forks), meanwhile built PROVISIONAL on arm 1 / arm 1 / arm 1 / no split | the founder, through the overseer; `founder-forks.md` Block 1 |
| 2 | The live readings R6a (an idle switch of a real session of his, read on the next turn, his settings byte-equal) and R6b (a switch refused mid-turn), not fired by this chunk | carried to «The fleet on the founder's host» |
| 3 | v2-31's console leg (the switch reached by keyboard alone) | «Console actions» |
| 4 | v2-30 and v2-31 left unclaimed, each with a dated notes line now | the ledger, P5 step 4 |

Apply? (yes / review / cancel)
