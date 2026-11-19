# P5 review card 1 — 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin

Written 2026-11-18T23:00:20Z. Plan: `doc:2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/plan` rev 6. You can open it in an editor at
`.andromeda/cache/docs/2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin/plan.md`. No plan stands in a chunk's folder, and writing in that
file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin` shows a diff of the chunk's records.

## Goal
The plan stamps the founder's real `~/.viola` live at all 24 relied rows on claude 2.1.325, and makes `viola verify`
say so when it meets the browser-tools prompt. It also reads `/context`'s post-condition and what `/mcp disable`
writes, then places the re-pin binary beside the product, stopping before the switch. The founder's cap
(`inputs#I2` item 1):
- **`L1`**: verify under his real home, from the repository root, through a new test-only rig arm `VH`.
- **`L2`**: one `/context` send under the rig's private config, through arm `CX`.
- **`L1-2`**: the spare, only after a plan revision on a cause `L1` showed.

Verify's line, on stdout just before the summary, in his words:
`the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again`.

## Top acceptance criteria
- The line prints once before the summary when a run met the prompt. The stamp reads `browser-tools-prompt: pass`, and
  the summary stays the last line. A passing verify's stdout stays pinned whole. The `bt-line` control must read
  «holds».
- P0 prints `present` before `L1` is made.
- `L1` passes only if every one of these reads:
  - 24/24 with `failing: none`;
  - `browser-tools-prompt: absent`;
  - `stamp-others: kept`;
  - `disabled-mcp` reads `written` or `none` (Run B's own entry was found);
  - `user-config: names-only`;
  - `token-window: not-read`;
  - every teardown word.
- `L2` reads one `context-post` word from a readable transcript and tears down whole.
- Re-pin, after CI reads green:
  - `viola.2026-11-15` byte-equal to the product (8049168 B, md5 `50c9be2a…`);
  - `viola.<sha8>` at 0700, byte-equal to the release build of the green commit;
  - `viola` and `viola.2026-10-19` unchanged;
  - no switch.

## Touchpoints (9 files to modify, none new)
- **Product:**
  - `src/cmd/verify.rs` (one conditional stdout line)
  - `crates/viola-agent-claude/src/ledger.rs` (`browser_tools_stood`)
- **Tests:**
  - `tests/cli_verify.rs`
  - `tests/support/fake.rs`
  - `tests/cli_send.rs` (the synthetic screen moved)
- **Test-only rig:**
  - `crates/viola-e2e/src/rig/mod.rs`
  - `crates/viola-e2e/src/rig/isolate.rs`
  - `crates/viola-e2e/src/bin/viola-live-rig.rs`
  - `crates/viola-e2e/tests/rig_rehearsal.rs`

## Test commands (27 entries)
- **Lint:** 1–2.
- **Unit:** 3–4.
- **Integration:** 5–7:
  - 5: `cli_verify` + `cli_send`;
  - 6: `rig_rehearsal`;
  - 7: the whole layer.
- **E2E:** 8.
- **The round:** 9–18:
  - the install read, census, `df`;
  - the release build with release-check and its hash;
  - P0;
  - `L1`;
  - census;
  - the debug `viola` build and its hash;
  - `L2`;
  - census.
- **Operator:** 19–22 (pre-push, hygiene, push, CI read).
- **Re-pin (operator):** 23–27 (`df`, clean-tree release build, rollback copy, placement, listing).
- No smoke entry: no boot-path file changes. No mutation run.

## Baselines read at P5 (on the untouched tree `c4c8ae75`)
- Entry 3: green, 650 passed.
- Entry 5: green, nextest-integration 87 passed.
- **Entry 6: RED** — 23 passed, 1 failed: `rig_rehearsal_reads_the_box_empty_after_x_and_del` read `box-text: absent`
  where it asserts `empty`.
  - This is an existing property; this chunk does not build it.
  - Alone, the case read green 5 of 5 (22:59Z).
  - Hypothesis, unmeasured: R1's settle after DEL accepts any frame after 400 ms of quiet, so a frame read
    mid-redraw has no prompt row.
  - Its cause is open. Its owner is the first row of `## Beyond the plan` below.

## Concretization preview
None: no capability is claimed. All 18 `planned, unclaimed` caps of viola-0.2.0 belong to other entries' surfaces.
No live `CARRY: proves` stands on this entry.

## Leaned (named here, per the plan's notes)
- **`VH` reads no credentials and checks no token window.** Your answer 3 lets nothing read
  `~/.claude/.credentials.json` except `L2`'s token check. The overseer's item 13 listed «token window» among what the
  arm keeps; your word wins over a delegate's. The window guards a start that holds a COPY (`isolate.rs:19`). Under
  the real config, the CLI refreshes its own file.
- **`L1` runs before `L2`.** A red `L1` stops the round before `L2` is spent.
- **Around `L1`, `~/.claude.json` is read by key NAMES only.** There is no whole-file hash, and `projects` keys (they
  are host paths) are counted, never printed.

## Expected amendments at the wrap (one answer card)
- **security-plan:** `decision/live-rig-isolated-child` and `decision/live-rig-credentials-copy` are widened.
  - The `VH` arm runs under your real home and config, with no copy and no credentials read.
  - This is a BOUNDARY WIDENING (`rul-r74hshif`, `rul-bznresq5`). You answered it yourself (answers 1 and 3,
    relayed by the overseer, `inputs#I2`), so the wrap halts once for your confirm.
- **design-system, layout-templates, test-plan, obs-plan:** the verify output rows gain the line (your answer 4).
- **architecture and test-plan:** the rig labels `L1` / `L1-2` / `L2` and arms `vh` / `cx` are added.

## The phase's own record (directive item 10)
- **Union, per master, verbatim:**
  - `architecture: rows 967 · packs 10 · picked 250 (155 by every draw) · standing 28 · 72701 B`
  - `security-plan: rows 616 · packs 7 · picked 177 (117 by every draw) · standing 9 · 52089 B`
  - `design-system: rows 316 · packs 4 · picked 75 (41 by every draw) · standing 24 · 18715 B`
  - `layout-templates: rows 222 · packs 3 · picked 54 (41 by every draw) · standing 3 · 12094 B`
  - `test-plan: rows 1347 · packs 14 · picked 218 (111 by every draw) · standing 29 · 56698 B`
  - `obs-plan: rows 1028 · packs 11 · picked 244 (121 by every draw) · standing 79 · 61862 B`
  - `a11y-plan: rows 235 · packs 3 · picked 74 (61 by every draw) · standing 11 · 20316 B`
  - `rows-union: 7 master(s) · picked 1092 of 4731 · rule any`
- **How the rows were read:** by search, not whole. Their sum is 294 475 B, past the 250 000 B bound.
  - Every cited row was checked present in its rows file or its standing list: 19 handles, all found.
  - The standing lists and history files were read whole.
- **`[RETYPED]` lines, verbatim:**
  - `arch-pick-p4 · [RETYPED] line 94: contract/mcp-list-results/cacheScope is no row of the pack — read as contract/mcp-list-results/cachescope, the pack's row 94`
  - `arch-pick-p4 · [RETYPED] line 95: contract/mcp-list-results/ttlMs is no row of the pack — read as contract/mcp-list-results/ttlms, the pack's row 95`
  - `tests-pick-p6.d3 · [RETYPED] line 81: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 81`
  - `obs-pick-p2.d3 · [RETYPED] line 23: decision/no-telemetry-from-screen-content is no row of the pack — read as decision/no-telemetry-from-screen, the pack's row 23`
- **Re-spawns:** `re-spawned 1 agent(s) · EVERY_ROW 1 · wall 159 s` (the `a11y-pick-p3.d3` draw, its raw twin
  `.raw-a11y-pick-p3.d3.md` kept). Dead draws: 0.
- **Non-zero exits and refusals:**
  - Entry 6's baseline red (above).
  - The first plan put read `UNPARSED: Expected newline or end of document after a statement (at line 87, column 15)`
    (a doubled quote in a TOML literal), fixed at rev 2.
  - The Bash guard refused one `cd` into the run dir; it was re-issued with absolute paths.
  - No `INDETERMINATE` or `NOT DERIVED` line.
- **Where a letter was wrong or silent:**
  - (a) Relay items 3 and 13 disagree on `L1`'s token window. Leaned above.
  - (b) The relay's item 2 calls `L2` «the `/context` reading». The P3 block's shape (one wrapped start) is planned as
    written.
  - (c) The skill letter's resume (c) says the rows files are «read there once, whole», while P4's letter says that
    past 250 000 B they are read by search. The plan followed P4's letter.

## Beyond the plan
1. **The R1 rehearsal's open red** (entry 6 baseline; an existing property). Proposed owner: **this chunk**, as a
   scope expansion.
   - Implement reads a red firing's kept `after-del` frame first, so the cause is measured, not inferred.
   - It then mends the rig's after-key settle in `crates/viola-e2e/src/rig/mod.rs` (already in this chunk's files),
     with its red-before-green.
   - The plan gains that step at the fold; your word re-runs P5.
   - The alternative is a CARRY on «A live StopFailure payload». But entry 6 gates this chunk, so an intermittent red
     there would stop implement here first.
2. **The switch** (an operator step, no entry), owned by the overseer and the founder after the pass:
   - replace `~/.local/viola-product/bin/viola` by the placed `viola.<sha8>`;
   - restart this builder by `/exit` through the product, then the operator's launcher.
3. **C5's two unmeasured items** (when the extension cache is written; the flag's live value or its cache) are left
   unmeasured (`inputs#I2` item 17). The wrap's route-resolve places them or drops them with the reason.

Apply? (yes / review / cancel)
