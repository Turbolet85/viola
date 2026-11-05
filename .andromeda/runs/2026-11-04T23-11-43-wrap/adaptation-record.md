# Adaptation record — the 2026-11-04T23-11-43-wrap 0-pending wrap: «Viola becomes Linux-only»

A 0-pending `/andromeda-wrap-session` (no chunk in flight) carrying the operator's route-adaptation request. No report,
no drift fan-out, no master amendment: the no-op path with route-resolve §Operator-requested adaptation.

## Inputs (snapped `--no-chunk` into `inputs/`)

- **I1** `inputs/I1-wrap-linux-only-directive.md.txt` — the operator's directive for this wrap (the overseer,
  founder-delegated): run the route-resolve per item, recommended-first; v1-53's fate the founder's, asked as a dialog;
  pin only «Linux-only code and CI» first; a tool fault stops the wrap; this wrap ends the run.
- **I2** `inputs/I2-restructure-linux-only.md.txt` — the overseer's relay of the founder's rulings, live, 2026-11-04
  (21:43Z, 21:44Z, 21:50Z, 21:53Z), recorded as his: Viola's target is Linux alone until it works well there; seven items.
- **I3** `inputs/I3-relay-1.md.txt` — the route-resolve dialog (Q1–Q4), closed with no option chosen and answered by the
  operator's message: Q1–Q3 the overseer's technical answers, founder-delegated; Q4 the founder's own word, live, by
  dialog in the overseer's session, 2026-11-05 about 05:20Z, «Переписать под Linux».
- This wrap's second dialog (answered, 2026-11-05 06:00Z) — Ledger: «Re-word 3, note 22»; Masters: «At the removal
  chunk»; both the overseer's technical answers, founder-delegated («Wrapping on Linux stays a capability of its own;
  nothing is deferred» · «The masters change once, with the code, at «Linux-only code and CI»'s wrap; the CARRY names the
  39 sites»).
- This wrap's re-wording card (answered, 2026-11-05 06:03Z) — «Land all four», the overseer's technical answer,
  founder-delegated, with the direction: «the ground is the founder's own Linux-only ruling (2026-11-04 21:43Z, the
  relay's quote); only the choice to re-word rather than defer is the overseer's delegate answer. Name both in their
  notes.» Both are named in every note and ruling text below.

## Items and their dispositions

1. **«Windows-only live measurements» leaves the route** (I2 item 1; I3 Q1, option 1). Its 17 CARRYs landed as 17 open
   residuals, origin `2026-11-04T23-11-43-wrap`, target `another-os`, each the carry whole in «…» plus the ruling and
   «open until another OS is ruled in» (`residuals-batch.txt`, tx 166, `res-wqnre2ex … res-4ozabvar`). The entry
   retired with its BLOCKED-ON and its 17 carries (`rec.py retire chk-7xqu5fpb`: 19 nodes, tx 167). Read back: the
   `open_residuals` view holds 17 rows of that origin, target `another-os`; `pins` holds none for the entry.
2. **«Version done-check» reads «every gate green on Linux»** (I2 item 2; I3 Q2, option 1): `@chunk id=chk-jfhs4dxi`,
   `old: every gate green on three OSes` / `new: every gate green on Linux` (`route-batch.txt`, tx 168). Its 11 CARRYs
   stand; its v1-53 carry (the live Linux run) stands as written.
3. **A new entry first in the tail, «Linux-only code and CI»** (I2 item 3; I1 item 1; I3 Q3, option 1 — the head of
   Epoch 7, which stays open and closes at this entry's wrap): `chk-4rtgjgi7`, after `chk-bqkmwcli` («Linux and macOS
   parity»), «Linux-only code and CI — no Windows or macOS code, tests, fixtures or CI jobs left, recoverable at a pushed
   tag; every gate green on Linux», with four CARRYs (tx 168): the scope whole (`cry-u5ggbegw`), its acceptance and the
   hypotheses this wrap re-measured at HEAD `abf00915` (`cry-snu5ujyi`: the pattern `cfg!?\(.*(windows|target_os =
   "macos")` over `git ls-files '*.rs'` matches 274 lines in 64 files of 96 185 `.rs` lines; the four Windows-only files
   1 146 lines; no tag `multi-os-final`), the msrv red (`cry-nqm6i63f`, item 6) and the masters (`cry-fcbxszgj`, item 5).
   Read back: `markerless` lists it first with freight 4; `pins` reads four `CARRY:` rows, no `UNPARSED`; `abstained`
   holds no row; `position` reads next `chk-4rtgjgi7` «Linux-only code and CI».
4. **The ledger** (I2 item 4).
   - No capability is wholly a Windows or macOS behaviour: none deferred (the overseer's technical answer, «Re-word 3,
     note 22»).
   - Re-worded to the Linux half by `matrix.py refine --unclaimed`, date 2026-11-05, after the card: `v1-01` «Linux CI
     pipeline from the first commit» (ruling `2026-11-04T23-11-43-wrap`) · `v1-07` «coverage on Linux meets»
     (`2026-11-04T23-11-43-wrap`) · `v1-27` «`viola run` wraps the unmodified CLI on Linux», the `.cmd`/`.bat` clause gone
     (`2026-11-04T23-11-43-wrap`) · `v1-53` «The fake-agent suite and one live run on Linux» (`inputs#I3`, the founder's
     own word). Texts: `rewrite-{id}/`. Each stays planned and unclaimed.
   - One dated note on each of the 22 verified caps whose acceptance names Windows or macOS (`note-{id}.md`): v1-02,
     v1-12, v1-14, v1-15, v1-16, v1-17, v1-18, v1-23, v1-25, v1-28, v1-29, v1-30, v1-31, v1-32, v1-37, v1-38, v1-40, v1-42,
     v1-43, v1-44, v1-45, v1-46 — status, ref and acceptance untouched.
   - Read back: `coverage_line` reads `verified 30/53 · deferred 6 · planned 17 · implemented 0 · unclaimed 17`.
5. **The masters** (I2 item 5): not amended at this wrap — they change once, with the code, at «Linux-only code and
   CI»'s wrap (the overseer's technical answer, «At the removal chunk»; the letter's rule for retiring built code,
   route-resolve «A version census», `retire` with `built: yes`). The CARRY `cry-fcbxszgj` holds item 5 whole and the 39
   master hits (29 facts) that name «Windows-only live measurements», read by `rec.py find --root . --doc all --text
   'Windows-only live measurements'` at this wrap. Until then the masters still read three-OS and cite the retired entry.
6. **The msrv red** (I2 item 6): folded into «Linux-only code and CI» as an `[inferred]` CARRY with its own acceptance
   (`cry-nqm6i63f`). This wrap read `gh run view 41930831242`: head `abf00915`, job `msrv` (126926985207) concluded
   `failure`; the per-case reading is the overseer's.
7. **Viola 0.2.0's intent** (I2 item 7): no write — the version's `intent` is never a wrap's, and 0.2.0's already reads
   Linux-only by the relay.

## Other duties of the path

- `gated` records: none (the `gated` view holds no row) — no premise to re-verify.
- Curation: not run — this conversation carried no correction to curate.
- Consolidation, judgment-base supersession: not requested.
- No `Requirement:` line: no requirement was added; four were rewritten (`Requirement: ~{id}` in the commit body).
