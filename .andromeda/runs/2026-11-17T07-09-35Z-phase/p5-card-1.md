# P5 review card 1 — 2026-11-17-standing-live-rig · revision 1 (changed lines only)

**Plan:** `doc:2026-11-17-standing-live-rig/plan` rev 8 (was rev 6, approved on p5-card-2.md of
`.andromeda/runs/2026-11-17T05-54-05Z-phase/`). Its render is at `.andromeda/cache/docs/2026-11-17-standing-live-rig/plan.md`
for an editor. No plan stands in the chunk's folder, and writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-standing-live-rig` diffs the
chunk's records.
**Why:** implement run `2026-11-17T06-49-10Z-implement` stopped before the operator pass. Entry 10 read red on its path,
the round stopped at 10, and 0 of 2 live starts were spent. Entries 1–9 are green and the five controls held
(`inputs#I5` the operator's word, `inputs#I6` the overseer's relay).

## What changed
1. **Entry 10 (the round probe).** `test -x "$HOME/.local/share/mise/installs/claude/2.1.325/claude" && … --version` is
   the install root's binary, with no `bin/`, which is the path the rig starts. Its readings
   (`controls-entry10.md`): 2.1.325 → `2.1.325 (Claude Code)`, exit 0, green. An absent version → exit 1, red. 2.1.326
   → no `2.1.325`, red.
2. **CLI stays 2.1.325 for R1 and R2.** mise's `latest` reads 2.1.326 and is not this chunk's. Both summaries print
   `cli:` with the version token **read from the binary** (step 6; R1 and R2 expect `contains cli: 2.1.325`).
3. **R2 by the full claude path.** *My call from the relay, confirmed by the overseer (`inputs#I7`).* R2's child
   program is the install root's `claude` by full path, as R1's already is. The built rig hands `viola run` a bare
   `claude` resolved on the child's `PATH` (step 7, `viola-live-rig.rs` `Verb::Live`).
4. **New entry 12, the plain build before R2.** `cargo build -q --bin viola && sha256sum target/debug/viola | cut
   -c1-16` (`role = 'build'`, `leg = 'round'`, fired with R2 by `--entry 13`). Step 12's fake re-alignment rebuilds with
   `fake-agent`, so this makes `target/debug/viola` plain again. R2's `viola-build:` must equal the printed key (R2
   expects `contains viola-build:`, and the equality is recorded in `live-starts.md`).
5. **R2's argument form, as built.** `viola --home <vhome> run rig-r2 -- <the 2.1.325 claude> --model haiku`, with
   `CLAUDE_CONFIG_DIR` kept by `vhome/config.json` `claude_env_keep`.
6. **Trust dialog: the moves are read before any key.** *My call from the relay, confirmed by the overseer
   (`inputs#I7`).* Both forms are handled:
   - the fake's: the selector on the trust row, numbered options, `trust-moves: 0`, Enter;
   - 2.1.325's, as read on 2026-11-15: unnumbered, `❯` first on «No, …» above the trust row, `trust-moves: 1`, one
     down-arrow, then Enter on a read `trust-yes`.

   A frame needing an up-arrow or a third move gets **no key at all** and reads `needs-up` / `needs-third-move`, then
   `trust-unpassed`. The grant is unchanged (≤2 down, Enter on a read row). The built rig types its down-arrows
   blind (step 5, unit cases in step 11, new criterion **A13**). Step 12 aligns the fake to R1's reading.
7. **The stopped run's deviations, folded as built:**
   - step 2: a fourth `box-text` word, `absent`;
   - step 9: the fake draws a box rule and a prompt row;
   - step 4: the census and `leftover` reach only a dead rig's leftovers, the marker is `<rig pid>-<start time>`, and
     the watchdog raises the signals' stop flag;
   - step 3e: the `vhome` config keeps `CLAUDE_CONFIG_DIR`.
   - **The controls as fired:** no `--timeout`, so under the tool's default 1 800 s (entry 4 has no `timeout` key)
     because the clone builds cold. Every firing read under 60 s and all 5 held. They stand, and one is re-fired only
     if a revision-1 edit changes its guarded line.
8. **Re-entry** (Implementation notes):
   1. the revision-1 clauses of steps 5, 6, 7 and 11;
   2. entries 1–9 again;
   3. `--live-legs --entry 11` (10, then R1);
   4. step 12;
   5. `--entry 13` (12, then R2);
   6. step 14;
   7. operator entries 14–17.

## Checks
- **Check 13 (revision sync): UNMEASURED.** `gate.py scope --sync` exits 2 (`not a {ISO}-phase run dir`). Its
  `RUN_NAME` takes no `Z`, and this chunk's take-up, stopped and revision run dirs are all Z-named. The overseer
  waived the check for this revision (`inputs#I7`) and reported the tool defect to the pipeline. Nothing was renamed
  and no tool was touched.
- **Hand-read delta: +0 −0.** How it was read:
  - plan rev 6's `### New files` and `### Files to modify`, read at tx 2214 (`rec.py read … --at-tx 2214
    --section …`, every item fact `r1`), against rev 8's: line-equal;
  - the rev-7 and rev-8 puts reported every touchpoint fact kept;
  - the revision's code names only files already listed (`crates/viola-e2e/src/rig/{keys,screen,mod}.rs` under the
    listed `crates/viola-e2e/src/rig/`, and `crates/viola-e2e/src/bin/viola-live-rig.rs`);
  - no file is outside the lists.
- **Research:** the lists stand unchanged (no delta), and `## Revision 1` was added by hand at rev 2. It has not been
  re-derived since the take-up's rev 1, on the operator's word (no row read, no codebase research). Lists `parsed 7
  items`.
- **Fence:** 17 entries, all parse. Baseline: 1–2 seen green, 3–9 unchanged and keep their take-up baselines, and the
  8 leg entries are exempt.
- **planlint:** 0 hits. Check 12 (d) lists line 174 («the product binary is unchanged» beside `target/debug/viola` in
  step 7). Read: no conflict. Entry 12 rebuilds the product from unchanged sources and edits none.
- **inputs verify:** I5–I7 are cited. I3 and I4 are the take-up's P5-word snaps, which the wrap report cites.
- **Validation-1:** aligned. Every change is rig-side or a gate entry inside scope rev 5's Revision 1 bullets, and
  nothing changes product behaviour.

## Capabilities
- **v2-32** stands as claimed. Its acceptance (R1's `trust`, `selected`, `options`, `box-text` live, the same four
  words in rehearsal A1/A2, and `modal: imports` in unit cases) is still proven by A2 and A10, with no change of
  concretization. The new `trust-moves` and `cli` words are evidence beside it, not part of its acceptance.

## Beyond the plan
| # | proposal | authority |
|---|---|---|
| 1 | (standing — .andromeda/runs/2026-11-17T05-54-05Z-phase/p5-card-2.md) New route entry «first readings on the standing rig», placed directly after row 2's entry: R3 (the fired wake's `source`, «a real wake reads 1», the hook-latency window) and R4 (the live `viola verify --record`). The CARRYs cry-skycqoql, cry-iugxdgc4 and cry-mqvgnhlo move to it from this entry. | the founder's own answer, inputs#I2 item 1 |
| 2 | (standing — .andromeda/runs/2026-11-17T05-54-05Z-phase/p5-card-2.md) New route entry, second in Epoch 4 (right after this one), WHAT-only: a wrapper a test boots does not outlive that test, killed or not. Its ground is the five mutant-build wrappers at about 99 % of a core for 5 h 50 min on 2026-11-17. Its requirement row comes by the wrap's dialogue. | the founder's own answer, inputs#I2 item 3 |
| 3 | (standing — .andromeda/runs/2026-11-17T05-54-05Z-phase/p5-card-2.md) One more CARRY for the wrap to pin on the new entry of row 1: read the external-imports modal LIVE on the standing rig — its shape (what raises it: a cwd whose `CLAUDE.md` imports a file outside the project, with no approval in the rig's private config; what closes it; what the start reads if it does not show), any key asked there (none granted today: a key into that modal is the founder's to grant), and the start cap that entry gets (the founder's, per chunk). It closes v2-32's provisional narrowing. | the Viola overseer as delegate, PROVISIONAL, for the founder to confirm (the review of card 1) |

This revision adds no row.

Apply? (yes / review / cancel)
