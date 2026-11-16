# Outcome — 2026-11-16-survey-key-keeps-the-wheel · run 2026-11-16T13-33-50-implement

**Outcome: stuck — S4 (entry 17) red; the operator pass is NOT run.** Plan step 10 says "Any other reading is red
and stops the chunk before the operator pass". The operator's direction for this run was "A red that moves a plan
entry stops you BEFORE the operator pass". The live cap of 4 is spent, so no fifth start is made without the
founder (`inputs#I2`). The working tree is preserved.

## What the code does (steps 5–8, on the founder's word `inputs#I6`: keys `0` and `3` only)
- `crates/viola-agent-claude/src/screen.rs`:
  - `Survey` and `SURVEY`: the prompt literal, the options-row literal and the keys `b"03"`, documented as
    measured on 2.1.325 S1–S3, with the ledger row owed to «The 2.1.325 recorded fixture set».
  - `Screen::survey_standing`: both literals stand by `contains` on a screen that is not poisoned.
  - Tests: `survey_standing_literals_are_the_measured_ones` and the 7-row table
    `survey_standing_reads_both_literals_and_no_poison`.
- `src/run/gate.rs`:
  - `Shared.survey`, an `AtomicBool`, published by the feed thread after every step through `publish_survey`.
    It reads false on a poisoned or overflowed model.
  - `SurveyFlag`: one atomic load, no lock. `Gate::survey()` hands it out.
  - Tests: `feed_sets_…`, `feed_clears_…`, `feed_panic_clears_…` and `feed_overflow_clears_the_survey_flag`.
- `src/run/wheel.rs`:
  - `Observed` takes an `Option<SurveyFlag>`. A read the classifier calls editing still takes the wheel, unless
    `survey_answer` holds (the read is exactly one byte of `SURVEY.keys`) and the flag stands, read once after
    the classifier. The bytes always come back whole.
  - Tests: the keep table (`0`, `3`), the no-flag table, the neighbour table (20 rows, `1` `2` `4` `y` `n` `d`
    and Enter included), `…_after_the_flag_falls_…` and the `None` table.
- `src/cmd/run.rs`: `pump_child` passes `Some(gate.survey())` to `Observed`.

## Gates (final tree)
- Block: `entries 21 · green 14 · red 0 · recorded 0 · timeout 0 · not-run 7` (the 7 are the operator legs).
- Counts:
  - entry 4: 336 passed;
  - entry 5: 580;
  - entry 6: 32 (every survey case);
  - entry 7: 8;
  - entry 8: 76;
  - entry 11 `run --unit`: green;
  - entry 18 `pre-push`: green, `"ok":true` with `"stage":"linux-tests"`.
- The first block pass read entry 2 red: clippy `unused_io_amount`, my own new test's unchecked `read`s at
  `src/run/wheel.rs:861`. It was fixed (each read now asserts 1) and re-run green; then the whole block was re-run.
- Controls (`evidence/control-1.txt`): `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone
  removed`. The four are `survey-flag`, `survey-exact`, `survey-poison` and `survey-options`, each at
  `--timeout 300`.
- Entry 12 (rehearse): green after the `tempfile.gettempdir()` respell (firing 2), and green in each block.

## The red: S4 (entry 17)
`start: S4 · version: 2.1.325 · build: cff4a91d8b69 · token-window: ok · turn-ended: yes · shown: no ·
prompt-in-stream: present · first-send: confirmed · keys-written: 0 · ctrl-c: 3 · killed: no · folder: absent ·
ledger: starts 4`

No key was written, so `wheel-after-key` and `second-send` were never read. **The build is not shown wrong:**
- the start booted `viola run` on the plain build (`pump_child`, the changed path);
- the first `viola send` was confirmed;
- the turn ended (its `turn-ended` event);
- the survey's prompt literal reached the byte stream.

**The likely cause is the rig's S4 order, from plan step 1's letter.** The rig looks for a standing survey only
after the `turn-ended` event. S1–S3 read that the survey stands beside the turn while it runs (S3:
`turn-running-at-key: yes`). So in S4 the survey most likely stood and closed within the turn, before the rig
looked. Typing over the box may also close the survey: the bundle has a `typed_over` close, and the first send
types into the box. Either way, this is not measured: telling which needs a start.

**What this means for the acceptance:** `matrix#v2-21`'s S4 half (no `wheel{human,human-input}` after the key;
the next send confirmed) is unproven. Its CI half (entry 6's tables) holds. So **no `ref` is written and v2-21
stays unclaimed.**

**For the founder** (a fifth start is his):
- (a) one more start, with the rig keying `0` whenever the survey has stood 1 s at any point after the first
  send (not only after `turn-ended`), the same readings asserted;
- or (b) re-plan S4 (`/andromeda-phase` offers the plan's revision).

## Deviations from the plan (with justification)
1. **`survey_answer(bytes)` takes no `after_survey_key` argument.** The word (`inputs#I6`) ratified no Enter form,
   so there is no lone-CR follow-on to track.
2. **The flag's names:** `publish_survey` on `Shared`; the test-only `SurveyFlag::fixed` / `set` constructors.
3. **Test names:** `observed_no_survey_flag_is_byte_for_byte_today` (with "survey", so entry 6's filter runs it)
   in place of the plan's `observed_no_flag_is_byte_for_byte_today`. The original
   `observed_returns_every_byte_and_moves_the_wheel_on_a_key` stays, with `None`.
4. **Extra rows and cases:**
   - the neighbour table adds `1`, `2` (now neighbours by the word), `y` `n` `d`, `30` and a kitty `3`;
   - the step 5 table adds a `4: Unsure` options row (reads not standing);
   - a separate literal oracle, `survey_standing_literals_are_the_measured_ones`.
5. **`feed_panic_clears_the_survey_flag` drives `Model::step` with a forced panic and `publish_survey` directly,
   not through the feed thread.** No byte stream panics vt100 at a width where the survey's literals can stand.
   The other three flag tests go through the thread (`gate_at` / `start`), with a 5 s bounded wait.
6. **The rig** (outside the tree). Before the word:
   - the version check before the ledger line;
   - the token window read from the source file;
   - its own screen model (`pyte` is absent);
   - two readings added between S1–S3;
   - S4 bounds set per start.

   On the word (`inputs#I6`): the temp folders spelled through `tempfile.gettempdir()` and the docstring
   `$TMPDIR/viola-survey-<pid>/`, by the Edit tool; `import tempfile` was added for it. S1–S3 ran on the rig
   before the respell, S4 after it.

Scope: `scope: clean — changed 4 · listed 4 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0)
· absorbed 0 · excluded 230`. No guard entry exists in the plan.

## P3 smoke
S4 booted `viola run` on the plain build through `pump_child`, and `first-send: confirmed`. Entry 8's booted cases
(`tui_wheel`, `chaos_feed_panic`, `cli_send`) also start through `pump_child`: 76 passed.

## Process hygiene
| process | started by | final state |
|---|---|---|
| rehearse arms (fake agent, `viola run`) | this run (entry 12, 4 firings) | terminated — folders absent |
| claude 2.1.325, S1–S3 | this run (entries 13–15) | terminated — Ctrl-C, `killed: no` |
| `viola run survey-s4` + its claude 2.1.325 | this run (entry 17) | terminated — Ctrl-C ×3, `killed: no`, folder absent |
| control clones' builds | this run (`gate.py control`) | terminated — `clone removed` |
| the 15 census rows (overseers, prototype builders, `~/.viola` mcp, product `viola wait`) | NOT DERIVED — not this run's | left running: the host's own sessions — their owners |

Census (after S4): `census: 15 running`. None carries `viola survey probe` or `survey-s4`.

## Decisions & corrections
- The founder's word on the grammar card (`inputs#I6`): «Только 0 и 3». Keys `0` and `3` only; `1` `2`, Enter and
  y/n/d take the wheel.
- The overseer's technical answer (`inputs#I6`): the rig's temp names spelled through `tempfile.gettempdir()` by
  the Edit tool; rig checks run with `python -B`; the `.pyc` removed at the desk.
- The earlier permission-classifier refusal ("Undermining Oversight") of my own respell-plus-`rm` call stands
  recorded. The cure came by the word, not by a re-issue.
- A Bash-guard refusal: a census piped into `tail` ("a pipeline tool's output piped into a filter"). Re-run bare.
- A clippy red on my own new test (`unused_io_amount`), fixed in the test.
- The operator's direction: stop before the operator pass on a red that moves a plan entry; no scheduled wake or
  timer. Both followed.
