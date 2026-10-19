# P5 review card 1 — 2026-10-19-overseer-switch-to-viola

## Goal
The viola overseer drives `viola-builder` through this repository's `viola` instead of the prototype. The
switch is v1-33's live proof on the Linux dev host.

The founder's own answers to the four P4 forks (`inputs#I14`):
- Start cap: claude 2.1.296, with a cap of 8.
- Switch: mid-implement.
- Takeover: he types the key and runs `viola release` himself, later.
- New gaps: both built, the background tasks as a count only. He ratifies the option-text widening.

## What the chunk does
- **Part A, under the prototype.** Two additive event keys, for the gaps research found beyond the 2026-10-08 list:
  - `question` data gains `option_details`, one `{description, preview}` per option, parallel to `options`.
  - `turn-ended` data gains `background_tasks`, the Stop array's length or `null`.
  - Unit, integration and E2E cases cover both.
- **Part B, under the prototype.**
  - Build `target/release/viola`.
  - One `viola verify` of claude 2.1.296 in `~/.viola` (live starts 1–6), which must print
    `stamped 2.1.296  24 pass  0 fail`. A failed row halts for the founder.
  - Then the switch point: the builder ends its turn for the overseer.
- **The overseer's own steps O1–O7, outside this repository, never files of it:**
  - O1: copy the binary to `~/.local/viola-product/bin/viola`.
  - O2: `~/.viola/config.json` with `dialog_hold_s` 3600.
  - O3 and O4: product start lines for viola-builder only. The other builders keep `vstart.sh`'s prototype form
    byte-for-byte.
  - O5: `waitm.sh` and `waitlong.sh` on the product's kinds and the two new keys.
  - O6: end the prototype builder, then start it with `run viola-builder -- claude … --resume <id>` (live start 7).
  - O7: the driving guide.
- **Part C, under the product.**
  - The re-entry send of `/andromeda-implement` is the skill sent.
  - A switch-record review dialog is answered with `viola answer`.
  - `wait` and `last` read the turn, then `/clear` is read back, and `/andromeda-implement` is sent again.
  - Each act is one line in `evidence/switch-readings.ndjson`: kinds, cursors, exit codes and durations, never text.
  - The operator pass and the wrap then run under the product.
- **Part D, gated: the founder at his keyboard.** He presses a key in the viola-builder window, the next driver
  send exits 10, and his own `viola release` returns the wheel. If he is not back before the wrap, the chunk wraps
  `gated`, and the gated mode of `/andromeda-implement` runs the step later.

## Top acceptance criteria
- The two event keys are present and tolerant (a missing or non-string value reads `null` and refuses nothing). No
  human line form changes; the new data reaches `--json` and MCP only.
- `run --unit`, `--integration` and `--e2e` read `"ok":true`. The four new unit cases and the two new integration
  cases are green. `pre-push` reaches `linux-tests`, and CI is green on the pushed HEAD.
- The verify stamps 2.1.296 24/24, and the switched snapshot reads `cli_verified` true.
- Seven acts are recorded: `verify-stamped`, `switched`, `skill-sent`, `review-answered`, `turn-waited`,
  `final-read`, `clear-confirmed`. No text is copied into the chunk folder, and no panic line appears in the live
  home.
- Gated: the founder's `takeover` act.

## Touchpoints
- Modify:
  - `crates/viola-agent-claude/src/dialog.rs`
  - `crates/viola-agent-claude/src/hook.rs`
  - `crates/viola-core/src/lib.rs`
  - `crates/viola-state/tests/state_events.rs`
  - `tests/hook_events.rs`
  - `src/run/dialog.rs`
- New:
  - `evidence/switch-round.md`
  - `evidence/switch-readings.ndjson`

## Test commands (25 entries)
- Lint ×4 · unit ×2 · integration ×2 · e2e · preservation guard (base `f23586fbda77`) · `cargo deny` ·
  `cargo build --release --bin viola` · booted smoke chain (`p-osv-smoke`) · G2 · G4 · `pre-push` · the operator
  trio (hygiene, push, `ci.py`).
- Three evidence readers; the takeover reader is `defer = "gated: …"`.
- Baselines:
  - Red as planned: the new-case filter (0 selected) and the readers (file absent). Each has a must-pass
    control (`p5-controls.md`).
  - Every other new entry reads green. The guard is controlled both ways.

## Concretization preview — v1-33 (this chunk claims it, GATED)
- **Current acceptance:** With the product binary on the Linux dev host, an overseer session sends a pipeline skill
  to a builder session, waits for the turn, reads the final text, answers a review and clears between skills, and a
  key typed in the session terminal takes the wheel; afterwards the overseer drives through viola instead of the
  prototype.
- **Proposed concrete form:**
  - Setting: on the Linux dev host, with the product binary (`target/release/viola` of this chunk's tree, its sha256
    recorded) and claude 2.1.296 stamped 24/24 in `~/.viola`, the viola overseer drives viola-builder through the
    product's CLI.
  - Sends a pipeline skill: `/andromeda-implement`, `send-confirmed{confirmed:true}`.
  - Waits for the turn: a `wait` woken by `turn-ended`.
  - Reads the final text: `last`, exit 0.
  - Answers a review dialog: a `dialog-answered` with `from`.
  - Clears between skills: `/clear` read back by `session-start{cause:"clear"}` with a new id.
  - Takes the wheel: a key the founder types in the session terminal writes
    `wheel{holder:"human",cause:"human-input"}`, the next send exits 10, and his `viola release` returns it.
  - Afterwards: the overseer drives viola-builder through the product, O3–O5 reported in `evidence/switch-round.md`.
- **Gated part:** the founder's key. Block: "the founder at his keyboard in the viola-builder terminal for his own key
  and his own viola release (he is away from the builders' terminals at the take-up, inputs#I14)". The cap is claimed
  with `--gated-file`, and this chunk's route line gains that `BLOCKED-ON`. Every other clause is proved now. The
  outcome is not weakened: the founder's own key replaces the earlier "agent-typed" reading.

## Notes for the review
- Size: about 330 lines of plan prose outside the gate fence, inside the 150–400 band.
- Validation-1: aligned. scope.md gained the founder's answers and the `gate:` bullet (intent-incomplete, justified).
- Live-start ledger: 6 verify + 1 switch + 1 spare revive = 8, the founder's cap. A ninth waits for his word.
- The option-text widening (playbook "Boundary widening") is the founder's own answer to P4 fork Q4. The wrap halts
  once on it, as the answer card.

## Beyond the plan
1. **CARRY on "Windows-only live measurements":** the Windows readings of claude 2.1.296's stamp, and of the two new
   event keys on a live CLI. This chunk reads Linux only; CI covers Windows under the fake agent.
2. **CARRY re-pin, only if no hold near the ceiling arises naturally in this chunk:** the 3600 s ceiling's first live
   reading stays PROVISIONAL and moves from this entry to "Version done-check".
