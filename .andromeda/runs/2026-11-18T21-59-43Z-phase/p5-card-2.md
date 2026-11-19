# P5 review card 2 — 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin

Written 2026-11-18T23:15:06Z, after the review of card 1 (`inputs#I3`, the overseer's, and `inputs#I4`, the founder's
own word on `VH`). Plan: `doc:2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/plan` rev 9. Open it in an editor at
`.andromeda/cache/docs/2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/plan.md`. No plan stands in a chunk's folder, and writing in that
file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin` shows a diff of the chunk's records.

## What the review changed (rev 6 → rev 9)
1. **The `VH` widening's Expected amendment** now cites the founder's own word «Разрешить ветку VH (Recommended)», read
   2026-11-18T22:44:06Z, as a NAMED EXCEPTION to the row «the rig runs under a private config only»
   (`inputs#I4`; `inputs#I3` item 1). It no longer cites answers 1 and 3.
2. **`VH` reads no credentials and makes no copy.** Confirmed (`inputs#I3` item 2): item 13's «token window» is
   withdrawn for `VH`. An `L1` red on an expired login is named by its own summary. A spare may rest on it only after
   a plan revision.
3. **The intermittent R1 rehearsal is folded in** as steps 18a–18d, bounded as `inputs#I3` item 3 says. The changes
   are in the test-only rig and its rehearsal only.
   - **18a, the instrument:** in that one case, a red prints the rig summary and the kept `after-del` frame. Today
     the case's `TempDir` keeps nothing.
   - **18b, the cause read first:** fire the counted entry until its first red (up to 30), then name the cause from
     what it printed (`evidence/r1-red.md`).
     - **Cause A**, the rig read the screen before the redraw after DEL settled: go to 18c.
     - **Any other cause, or no red in 30:** STOP before the live round and say so.
   - **18c, the mend (Cause A only):** the settle after X and DEL in `r1_keys` waits for a frame with the box up AND
     its prompt row readable, bounded as now.
   - **18d, the count:** new entry 7 runs the whole `rig_rehearsal` binary 30 times in a row.
     - 30 of 30 green after the mend is the mended reading.
     - A red before it is the case failing without the fix.
4. Nothing else moved. The entries after 6 are renumbered by one: the round is 10–19, the operator pass 20–23, and
   the re-pin 24–28.

## What the count rests on (read at P5 on the untouched tree)
- **Entry 6 (the whole `rig_rehearsal` binary):** RED, 23 passed and 1 failed. `box-text` read `absent` where the
  case asserts `empty`.
- **A first form of entry 7, 10 firings:** 10 of 10 green. A 10-count could not show the failure, so I raised it.
- **Entry 7, 30 firings:** RED at firing 25, after 24 greens. Same case, same `absent`-for-`empty` failure (379 s).
- **So far:** 2 reds in 36 whole-binary firings.
  - 30 firings hold at least one red with probability ≈ 0.82, so 18b reads the red about 4 times in 5.
  - The other 1 time in 5, 18b reads no red and implement stops before the round (the bound).
  - An unmended tree passes 30 of 30 about 18% of the time, so after the mend 30 of 30 is evidence, not proof. The
    report records the count it read, and never calls the case «fixed» on that count alone.

## Goal (unchanged)
The plan stamps `~/.viola` live at 24 rows on claude 2.1.325, and verify says when it meets the browser-tools prompt.
It reads `/context` and what `/mcp disable` writes. Then it places the re-pin beside the product and stops before the
switch.
- **`L1`:** `VH`, under the real home, from the repository root.
- **`L2`:** `CX`, under the rig's private config.
- **`L1-2`:** the spare, only after a plan revision.

Verify's line, on stdout just before the summary:
`the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again`.

## Top acceptance criteria
- The verify line prints once, just before the summary, on a met prompt. The stamp, the exit and the summary's place
  are unchanged. `bt-line` reads «holds».
- R1:
  - the cause is named from the red's own output;
  - on Cause A, the count reads red before the mend and 30 of 30 after;
  - otherwise implement stopped before the round.
- P0 prints `present` before `L1`.
- `L1` passes only if every one of these reads:
  - 24/24 with `failing: none`;
  - `browser-tools-prompt: absent`;
  - `stamp-others: kept`;
  - `disabled-mcp` is not `unread`;
  - `user-config: names-only`;
  - `token-window: not-read`;
  - every teardown word.
- `L2` reads one `context-post` word from a readable transcript.
- Re-pin after CI reads green:
  - `viola.2026-11-15` byte-equal to the product (8049168 B, md5 `50c9be2a…`);
  - `viola.<sha8>` at 0700, byte-equal to the green build;
  - `viola` and `viola.2026-10-19` unchanged;
  - no switch.

## Touchpoints (9 files to modify, none new; unchanged by the fold)
- **Product:** `src/cmd/verify.rs`, `crates/viola-agent-claude/src/ledger.rs`.
- **Tests:** `tests/cli_verify.rs`, `tests/support/fake.rs`, `tests/cli_send.rs`.
- **Test-only rig:** `crates/viola-e2e/src/rig/mod.rs`, `crates/viola-e2e/src/rig/isolate.rs`,
  `crates/viola-e2e/src/bin/viola-live-rig.rs`, `crates/viola-e2e/tests/rig_rehearsal.rs`.

## Checks re-run on rev 9 (the whole set)
- The fence parses: 28 entries.
- The baseline listing: seen green 5, not seen 0, new 4 (3, 5, 6, 7, each baselined), leg 19.
- planlint: 0 hits.
- Research's lists: 9 items parsed.
- Inputs: I1–I4 unchanged and cited.
- The evidence paths are gitignored.
- Validation-1 is still aligned. The fold is a scope expansion the review ordered; it is not a divergence.

## Concretization preview
None: no capability is claimed.

## Expected amendments at the wrap
- **security-plan:** `decision/live-rig-isolated-child` and `decision/live-rig-credentials-copy` take a named
  exception for `VH`, citing the founder's own word of 22:44:06Z (`inputs#I4`). It goes on the answer card, and he
  is asked nothing new.
- **design-system, layout-templates, test-plan, obs-plan:** the verify line (his answer 4).
- **architecture and test-plan:** the rig labels and arms.

## The phase's own record (directive item 10; additions since card 1)
- The union lines, the 4 `[RETYPED]` lines and the one re-spawn stand as card 1 quoted them, verbatim.
- **Non-zero exits added:**
  - entry 6's baseline red;
  - the 30-count's baseline red (firing 25);
  - none other.
- **Where a letter was wrong or silent:**
  - `inputs#I3` item 3 asks implement to read «what the red run kept», but the rehearsal's `TempDir` keeps nothing
    today. Step 18a builds the instrument first.
  - It also asks for «a count» without one. 30 is chosen on the measured rate above.
  - Card 1's (a)–(c) stand.

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-1.md`) **The switch** (an operator step, no
  entry), owned by the overseer and the founder after the pass:
  - replace `~/.local/viola-product/bin/viola` by the placed `viola.<sha8>`;
  - restart this builder by `/exit` through the product, then the operator's launcher.
- (standing — `.andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-1.md`) **C5's two unmeasured items** are left
  unmeasured. The wrap's route-resolve places or drops them with the reason.
- (ended — `.andromeda/runs/2026-11-18T21-59-43Z-phase/p5-card-1.md` row 1) **The R1 rehearsal's owner.** It is now
  this chunk's own steps 18a–18d (`inputs#I3` item 3), so it is no longer a row beyond the plan.

Apply? (yes / review / cancel)
