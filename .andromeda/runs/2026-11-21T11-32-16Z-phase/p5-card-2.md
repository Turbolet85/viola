# P5 review card 2 — «Cross-session prefix ledger row» (`2026-11-21-cross-session-prefix-ledger-row`)

**The plan to open:** `.andromeda/cache/docs/2026-11-21-cross-session-prefix-ledger-row/plan.md` (rev 5). No plan stands
in the chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-21-cross-session-prefix-ledger-row` is a diff of the
chunk's records. Card 1 (`.andromeda/runs/2026-11-21T11-32-16Z-phase/p5-card-1.md`) stands for everything below does not
name; this card folds the overseer's review of it, `inputs#I3` (items 1–5, technical, the overseer's: none of the
operator's answers moves).

## What changed (plan rev 3 → rev 5)
1. **Control ids within 16 characters** (`gate.py` `CONTROL_ID`, `^[A-Za-z0-9_.-]{1,16}$`): `prefix-match` stays;
   `names-only-no-payload` → **`no-payload`**; `unknown-shape-unread` → **`unread-shape`** — carried through the
   controls table, the step and gate notes and the acceptance criteria.
2. **Each control fires on an entry filtered to its own guard's cases** (new entries **16–18**, phase B, each `new`):
   - 16 `prefix-match`: `cargo nextest run -p viola-agent-claude --no-fail-fast -E 'test(/cross_session_prefix_fails_when/)'`,
     control `--timeout 300`;
   - 17 `no-payload`: `cargo nextest run -p viola --bin viola --no-fail-fast -E 'test(/names_only_capture_files_closed_words_and_no_payload/)'`
     (builds the root-bin test binary the case lives in), control `--timeout 600`;
   - 18 `unread-shape`: `cargo nextest run -p viola-agent-claude --no-fail-fast -E 'test(/send_result_word_reads_only_the_boolean/)'`,
     control `--timeout 300`.
   Each selects 0 tests on the untouched tree (baseline red, exit 4: 0 of 752 · 0 of 1125 · 0 of 752 selected) and at
   least one once its case exists.
3. **The after-reads of state other actors move are keyed to the ROUND leg, right after `XS`** (entries **29–31**, before
   the operator census 32): `kept-floor: unchanged`, `live-stamp: unmoved`, and the stale-socket read. The whole-round
   command (`gate.py run --live-legs`) still fires them in its span, in block order; the wrap's light gate never re-fires
   them. Their `new` / `baseline` keys left with the re-key (a `leg` entry carries none).
4. **`stale-sockets` is a READING** (entry 31): it prints `stale-sockets: before {B} after {N}`, its expect only its own
   echo (`contains stale-sockets: before`); both counts go to `evidence/live-round.md`. It fails only when the before file
   is absent. **No per-pid atom stands:** the rig cannot name the pids its start spawned once they ended — its census
   reads only live processes, by its marker or cwd at the call (`crates/viola-e2e/src/rig/procs.rs:105-114`), and its
   ledger keeps no child pid. The criterion and step 21 say so.

## Checks re-run (plan rev 5)
- `gate.py run --dry-run`: 47 entries parse, exit 0.
- `planlint.py check`: `hits: 0`; four «unchanged»/«untouched» claims listed and read — none contradicted by a step.
- `inputs.py verify`: 3 entries (`inputs#I1`–`I3`), all unchanged, none uncited.
- `gate.py scope --lists-only`: `scope lists: parsed 31 items`.
- `gate.py baseline`: 16 seen green · 8 `new` · 23 `leg`. The 8 `new` entries baseline-run again: 12 green (100 rig
  cases); 14, 16, 17, 18 red, exit 4, 0 selected (vacuous until their cases exist; their controls are implement's
  red-before-green run and the three one-shots); 33 red and 37 red on absent inputs, 34 recorded (report-only).
- **Controls of the re-keyed entries** (`p5-controls.md`, run 3): `gate.py` drives a `round` entry only inside a live
  round, so each entry's own `run` was typed as the tool types one (`/usr/bin/bash -o pipefail -c`) in a scratch clone
  at `3d281c17`: 29 all-absent exit 1 · must-pass `kept-floor: unchanged` · one stamp moved exit 1; 30 sha absent exit 1
  · must-pass `live-stamp: unmoved` · another sha exit 1; 31 before absent exit 1 · must-pass
  `stale-sockets: before 7 after 141`. Runs 1–2 (card 1's) stand for 33 and 37.
- Validation-1: aligned with the scope (rev 3) and the working entry; the fold moves no step of the design.
- **Size:** the plan is 794 lines, 444 of them prose outside the gate fence — about 11 % past the ~400 soft bound (warn).

## The concretization preview (v2-25)
Unchanged from card 1: the matrix acceptance with «`S1` (rig label `HF`)» → «`XS` (rig label `XS`, kept stamp
`cross-stamp.json`)» and «A red `XS` verifies nothing.»; nothing else moves.

## Beyond the plan
none

Apply? (yes / review / cancel)
