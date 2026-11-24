# Adaptation record — 0-pending wrap, an operator-requested route adaptation (V256)

Run dir `.andromeda/runs/2026-11-24T21-15-48Z-wrap/`, opened at 2026-11-24T21:15:48Z (date -u) over HEAD fd7f761e:
0 pending, 1 gated (`2026-11-24-viola-adopt`, v2-13 `planned, gated`, no ref — not a gated wrap).

The request: the Viola overseer's relay, snapped `--no-chunk` as
`.andromeda/runs/2026-11-24T21-15-48Z-wrap/inputs/I1-adopt-v256-route-adaptation.md.txt` (inputs#I1). It carries
the operator's own answers from his dialog in the overseer's session, each put whole with its options and costs.

## Items
1. word: «Новая запись в голове (Recommended)», his answer to «Как чиним?» on «`viola adopt`» after R5a's two reds
   (relayed verbatim, inputs#I1 §1) — (A), a fix entry of its own, FIRST on the route now, in Epoch 5 directly after
   «Switching a session's model and effort»; the gated record stands gated; after (A) lands ONE R5a fires through the
   gated door on his word and v2-13 closes there — (B), a carry on (A).
2. word: «До границы, после adopt (Recommended)», his answer to «где её ставим относительно границы Эпохи 5?»
   (inputs#I1 §1), with his earlier words «надо делать как положено чтоб закрыть требование» and «Отдельной записью
   перед Эпохой 6» and his grant of ONE live re-record of the move's fixture (one terminal window, no claude, the
   host unlocked) when the fix is planned — (C), a second entry directly after (A), still in Epoch 5, claiming v2-07
   again.
3. word: «Да, тем же wrap (Recommended)», his answer to «Снять сейчас, тем же 0-pending wrap, что вставляет записи?»
   (inputs#I1 §1) — (D), v2-07 lowered from `verified` by this wrap, through the letters' own door, its ledger note
   the relay's verbatim; no tool form → nothing written by hand.
4. (E), by the same relay (inputs#I1 §2): the carry `cry-r4rsj5ey` on «Console scaffold» (the F4 clause owed)
   moves to (C), by its id.

## Dispositions
The placements are the operator's own words (items 1–2), which name each entry and its slot: the trajectory gate is
satisfied by them. The entries' wording was put to the overseer as a dialog (no question dialog for the operator, per
inputs#I1): delegate Viola-overseer, 2026-11-24T21:18:11Z, «Edit» on (A)'s title («must not pre-decide its fork —
whether adopt looks the folder up or is typed in it») and «Reword, land» on (C)'s v2-07 clause — a delegate's word on
wording, PROVISIONAL as a wording.

- adaptation:1 → (A) `chk-m3dcvxca` «Adopt's resume folder — an adopted session is resumed where the CLI finds it; a
  child ending at once is said and recorded (serves v2-13)», after `chk-ke32dhty` in Epoch 5; (B) its carry
  `cry-vz3fiqnh`. `rec.py put … route-batch.txt`, tx 4880.
- adaptation:2 → (C) `chk-ktu6mxtx` «Placing without moving the view — a placed window reaches its workspace while the
  active workspace stays unchanged, as the operator's F4 asks (v2-07)», after (A); its carry `cry-g43vil5i` (the
  grant, the R2/R4a readings, and v2-07's missing door, reworded on the overseer's word). Same TX 4880.
- adaptation:3 → NOT landed as a lowering: no matrix v1.13 verb lowers a verified capability. `matrix.py defer --id
  v2-07 --dry-run`: «v2-07: chunk is '2026-11-23-viola-open-2' — un-claim first; a deferral is the operator's,
  outside any chunk», exit 3; `matrix.py unclaim --id v2-07 --dry-run`: «unclaim v2-07: chunk
  2026-11-23-viola-open-2 → null · status verified kept», exit 0 (not fired: it lowers nothing); `refine` and `claim`
  refuse a verified cap in code. Nothing written by hand; the relay's verbatim note was not landed (it states a
  lowering that did not happen). On the overseer's word, `matrix.py note --id v2-07 --text-file
  .andromeda/runs/2026-11-24T21-15-48Z-wrap/note-v2-07.md` landed its text instead: «note v2-07: +354 chars (total
  2276) · status verified · ref/acceptance untouched». v2-07 reads `verified`; coverage `verified 28/37 · deferred 2 ·
  planned 7 · implemented 0 · unclaimed 6`. The missing door is a finding for the pipeline's owner.
- adaptation:4 → `cry-r4rsj5ey` moved from «Console scaffold» (`chk-eg54zv5h`) to (C), its text unchanged
  (`@carry id=cry-r4rsj5ey expect_rev=1 in=@c`), same TX 4880.

Read back after tx 4880: `markerless` lists (A), (C), «Console scaffold» … «Version done-check», 8 rows; `pins` shows
(A) holding `cry-vz3fiqnh`, (C) holding `cry-g43vil5i` and `cry-r4rsj5ey`, «Console scaffold» holding two; `abstained` 0
rows; the `epoch` row for Epoch 5 reads 11 entries, 2 markerless, 8 frozen, 1 gated. No PREREQ or WATCH stood on the
previous first markerless entry, so nothing re-pins onto (A).

Gated re-check (the no-op path's P5 duty): `gated 2026-11-24-viola-adopt: standing` — R5b green, R5a red twice and
final for its arm (`.andromeda/cache/chunks/2026-11-24-viola-adopt/evidence/r5.md`); its gated part waits on (A).

P3 curation: not run — this conversation carried no correction that qualifies as a project learning (the wording edit
is entry-specific, and the missing matrix door is the pipeline's).
