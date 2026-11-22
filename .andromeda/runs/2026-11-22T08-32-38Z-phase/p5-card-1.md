# P5 review card 1 — 2026-11-21-key-after-a-resize («Key after a resize»), revision 1

Plan `doc:2026-11-21-key-after-a-resize/plan` rev 5 · scope rev 4 · research rev 2. The word this revision stands on:
the operator's «Починить риг, затем запасной (Recommended)» and the overseer's items 2–7 (`inputs#I6`), the revision
directive (`inputs#I7`).

**To read the plan in an editor:** `.andromeda/cache/docs/2026-11-21-key-after-a-resize/plan.md`. No plan stands in a
chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-21-key-after-a-resize` is a
diff of the chunk's records.

## Where it stands

- **Done and kept:** steps 1–6 (the `KR` arm). Phase A read green 5/5 in the stopped run.
- **`KR`** (2.1.330): `kept` 12/12. **Stands as taken.**
- **`KR8`** (2.1.328): 7 of 7 rounds kept, then `dirty-box: 8`, `unverified`, exit 1.
- **The cause is a reading, not a measurement.** `Session::settle` counts quiet from the last byte drained, which can
  come before the key. In round 7 the DEL's settle returned at its first 50 ms pump; rounds 1–6 waited about 410 ms.
  A CLI that really did not take the DEL is not ruled out.

## What this revision does

Implement starts again at step 7. Only the rig's test crate changes; no product file and no fake-agent file.

1. **Step 7, the after-key settle.** A new `Session::settle_since` waits for a byte drained after the key (the rig's
   own clock at the write) and keeps the same bound as today.
   - `Session::settle` is not changed at all, so `HB` and every earlier arm keep their readings by construction.
   - Entry 5 runs the whole rehearsal binary to show it: 45/45 green today.
   - One leftover gap: a byte that lands between the drain and the write is counted as after the key.
2. **Step 8, a dirty box vs a late redraw.** A box read dirty is read once more after a further settle.
   - Clean on the re-read: the new summary count `late-redraw`.
   - Still dirty: `dirty-box: <round>`, `unverified`, and the start stops. That is a finding about the CLI, never
     retried, and it goes to the operator.
   - The whole-start rule does not change.
3. **Steps 9–11, the rehearsals: «its like»** (you asked which). The stand-ins are rig-side and accepted by
   `rehearse kr` only:
   - `--kr-del-seam withheld:<n>` (the DEL is never typed) and `late:<n>` (the DEL is typed after the next round's
     first read).
   - `--quiet-ms`.
   - `live` takes none of them: clap refuses with `usage`, exit 2, and a case checks it with `CI=1` set.
   - Why not the fake's `hold`: the fake agent is a root-package file, outside «the rig's test crate only».
   - Three new rehearsals: the late redraw (`late-redraw: 1`, kept, exit 0), the still-dirty box (`dirty-box: 3`,
     `unverified`, exit 1) and the seam refusals.
4. **Step 12, the margin.** The late-redraw case adds a re-read to a 12-round start, so the condition you set holds.
   - The fix is `--quiet-ms 100` on the three whole `kr` rehearsals. No bound moves.
   - Today: 12.66 s alone, 13.12 s inside the whole binary.
   - Prediction: under 10 s. Implement records the measured time, and a reading of 13 s or more stops it for you.
5. **Step 13, the round: `KR8-2` alone.** It runs with `--cap 2 --spare 1` and is the spare line on the same ledger.
   - Taken again into files keyed to `KR8-2`: the kept products' sha (now including `KR`'s and `KR8`'s out dirs), the
     out-dir list (`KR8-2`'s not there yet) and the live stamp.
   - Then the live leg and the after-reads: products unchanged, exactly one new out dir and none gone, stamp unmoved.
   - Then the readings: `KR`'s line as it stands, and `KR8-2`'s line alone.
   - If `KR8-2` exits 1, the round stops there and no start follows without you.

## Top acceptance criteria

1. **(e2e, matrix#v2-27)** `KR kr-reading=kept` (2.1.330, as taken) and `KR8-2 kr-reading=kept` (2.1.328), 12/12 each.
   `KR8` is recorded as it read.
2. **(tests)** The rig cases pass, and both controls read `holds`:
   - `kr-after-key` removes the after-the-mark clause;
   - `kr-box-reread` removes the late-redraw arm.
   - Both fire on the filtered entry 3, `--timeout 1200`.
3. **(tests)** The five `KR` rehearsals pass (entry 4), and the whole binary passes (entry 5: `HB` and the earlier
   arms unchanged).
4. **(security)** `KR8-2` types exactly 13 `x` and 13 DEL and resizes only its own pty under `--grant box,resize`.
   No seam, no prompt, closed words only, `hygiene: clean`.
5. **(security)** The kept products (with the two KR out dirs) and the live stamp read the same before and after, and
   exactly one new out dir appears.
6. **(tests)** The whole integration layer, `pre-push` and CI read green.

## Touchpoints

The same five `crates/viola-e2e` files. Revision 1 writes `src/rig/mod.rs`, `src/bin/viola-live-rig.rs` and
`tests/rig_rehearsal.rs`.

Research: lists synced to the revision's delta (+0 −0), rev 2; not re-derived since the take-up's rev 1.

## Test commands — 30 entries

- **Phase A, 1–6:** fmt; clippy; the rig lib cases; the `KR` rehearsal filter; the whole `rig_rehearsal` binary;
  the unit layer.
- **The round, 7–20:** host read; the 2.1.328 install; `mise ls`; census; rig build and hash; the three files
  re-taken; **`KR8-2`**; three after-reads; the `KR` reading; the `KR8-2` reading.
- **After the round:** 21 census (`operator`); 22 the ledger read (report-only, `late-redraw` and `settle-ms` added);
  23 the whole integration layer; 24–30 the operator pass.
- **Not re-fired:** `KR`, `KR8`, and the 2.1.330 install probe.

**Control ids:** `kr-after-key` (12 characters), `kr-box-reread` (13), both on entry 3, which is filtered with
`--no-fail-fast`.

**The whole-round dry-run** (`gate.py run --chunk 2026-11-21-key-after-a-resize --live-legs --dry-run`, as printed):
```
  7 probe       not run — dry-run · python3 ~/dev/projects/additional/andromedaV3/andromeda-ov… (103 chars)
  8 probe       not run — dry-run · test -x "$HOME/.local/share/mise/installs/claude/2.1.328/c… (134 chars)
  9 probe       not run — dry-run · mise ls claude
 10 probe       not run — dry-run · cargo run -q -p viola-e2e --bin viola-live-rig -- census
 11 build       not run — dry-run · cargo build -q -p viola-e2e --bin viola-live-rig && sha256… (102 chars)
 12 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (660 chars)
 13 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (251 chars)
 14 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (216 chars)
 15 e2e         not run — dry-run · cargo run -q -p viola-e2e --bin viola-live-rig -- live KR8… (227 chars)
 16 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (668 chars)
 17 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (350 chars)
 18 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (264 chars)
 19 probe       not run — dry-run · python3 -c 'import json;r=[d for d in map(json.loads,open(… (286 chars)
 20 probe       not run — dry-run · python3 -c 'import json;r=[d for d in map(json.loads,open(… (292 chars)
not run — excluded · 16: 1-6,21-30
round: DRY-RUN · legs 1 · entries 30 · green 0 · red 0 · recorded 0 · timeout 0 · indeterminate 0 · not-run 30
```

## Concretization preview — `v2-27` («A key typed right after a resize», e2e), re-claimed after the word

- **Today:**
  > …in 12 of 12 rounds per version (`KR kr-reading=kept`, `KR8 kr-reading=kept`)…
- **Proposed:** the same sentence, with the per-version readings changed to:
  > `KR kr-reading=kept` (2.1.330, the round of 2026-11-22T08:26:30Z, as it stands) and `KR8-2 kr-reading=kept`
  > (2.1.328, the spare, after the rig's after-key settle waits for a byte drained after its key and a dirty box is
  > re-read once); `KR8` (2.1.328) read 7 of 7 rounds kept and was cut at round 8 by the rig's settle, `unverified`,
  > recorded as read.

  Everything else stands as written: the rehearsals hold the mechanics in CI, now including the late-redraw and
  still-dirty words, and any `lost` goes to the driver-paste fix.
- **Not weakened:** each version still needs 12 of 12 kept. Only the 2.1.328 start that proves it changes, from the
  base line to the spare. It is re-claimed with a dated `--note-file`, `chunk = 2026-11-21-key-after-a-resize`.

## Checks (P5 step 1, re-run whole after the baseline edit)

- **Block and tools:** the block parses (30 entries); `planlint` reads `hits: 0`; the lists parse (5 items);
  `scope sync: PASS · delta +0 −0`.
- **Snapshots:** 7 entries. I1, I2, I6 and I7 are cited. I3 and I5 are uncited (the earlier word and the round's
  word); I4 is uncited (implement's directive). None is unparsed.
- **Baselines:** entry 4 green (2 passed, 12.66 s); entry 5 green (45 passed); entry 22 green (recorded, 2 summary
  lines).
- **Typed controls** (`p5-controls.md` of this run):
  - entries 19 and 20 pass on `kept` and fail on `lost`, `unverified` and an absent summary, each reading its own
    label alone (a `KR8` line never stands in for `KR8-2`);
  - entry 17 passes on one new out dir with none gone, and fails on none new, two new or one gone.
- **Check 12 (d)**, the «unchanged» claims: all 8 read. Each one holds after the chunk: `settle`, the whole-start
  rule, the bounds and `KR`'s reading are not edited.
- **Validation-1:** aligned. The fix comes only after a reading; the scope's Revision 1 section is closed against the
  tree.
- **Host reach:** no recorded harm. The stopped round read `census: 0`, every rig process ended, and the seam case
  spawns nothing.

## Beyond the plan

| proposal | owner / place |
|---|---|
| The driver-paste fix F1, should any reading be `lost` (standing — `.andromeda/runs/2026-11-21T23-32-36Z-phase/p5-card-1.md`) | a plan revision of THIS chunk, on the operator's word after the reading (`inputs#I2` item 4) |
| `cry-7mgt7ybs` (the paste's missing time bound) re-pinned whole (standing — `.andromeda/runs/2026-11-21T23-32-36Z-phase/p5-card-1.md`) | the wrap's route-resolve, its own entry «Paste write bounded» still the operator's |
| `cry-m267ghbx` (the `CHILD_WITHIN` comment) re-pinned whole (standing — `.andromeda/runs/2026-11-21T23-32-36Z-phase/p5-card-1.md`) | the wrap's route-resolve (no `viola-pty` edit here) |
| `cry-lqe3bfwx` retired as met in `1b22388f` (standing — `.andromeda/runs/2026-11-21T23-32-36Z-phase/p5-card-1.md`) | the wrap: `src/bin/viola-fake-agent/input.rs:74-85`, held by `rig_rehearsal_reads_the_box_empty_after_x_and_del` (`crates/viola-e2e/tests/rig_rehearsal.rs:417`) |
| `cry-234cbfoa` (the rehearsal red) re-pinned whole unless met (standing — `.andromeda/runs/2026-11-21T23-32-36Z-phase/p5-card-1.md`) | the wrap, acted on only at its next red |
| `cry-teayoana` (the advisory ignores' re-read) (standing — `.andromeda/runs/2026-11-21T23-32-36Z-phase/p5-card-1.md`) | the Epoch 4 boundary, the founder's |
| The H2 leaves re-worded with the reading: `.claude/docs/gotchas.md:109`, `.claude/docs/services/viola-pty.md:31` (standing — `.andromeda/runs/2026-11-21T23-32-36Z-phase/p5-card-1.md`) | the wrap's leaf cascade |
| The HYPOTHESIS that `cry-234cbfoa`'s red shares the settle mechanism this revision mends for `KR` alone, named beside that carry's re-pin (its rule unchanged) | the wrap's route-resolve, on that carry |
| Any start after `KR8-2` (a second spare or a third base start), should `KR8-2` give no reading | the operator's word alone (`inputs#I6` item 1) |

**Apply? (yes / review / cancel)**
