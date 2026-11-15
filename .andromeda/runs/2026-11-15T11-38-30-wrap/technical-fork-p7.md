# Technical fork — P7 step 3d (the seal), wrap 2026-11-15T11-38-30 of 2026-11-15-live-round-on-the-trimmed-product

The wrap stopped at P7 step 3d, before the master flip and the commit, on the seal's refusal (SKILL.md P7 3d: «a
refusal halts with its line»). The chunk stays `pending`; nothing is committed.

## The refusal, as printed
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py seal --root . --run-dir .andromeda/runs/2026-11-15T11-38-30-wrap`
→ `seal: 2026-11-15T12-10-18-phase is newer than the sealer's own run 2026-11-15T11-38-30-wrap — another run is open; nothing sealed` (exit 3)

Cause: this wrap is a resume, so its run dir is named for 11:38:30, while the eighth and ninth revisions ran after
it, by the founder's two one-time overrides: `2026-11-15T12-10-18-phase`, `12-19-22-implement`, `12-29-32-phase`,
`12-32-40-implement` and `12-35-39-implement`, all finished. The seal reads any run dir newer than its own as open
(records-contract §The sealed tier). Read-only probes: `--run 2026-11-15T05-56-26-wrap --dry-run` → `217 new row(s) …
own … left as files`, exit 0; `--run 2026-11-15T12-35-39-implement --dry-run` → the same refusal, exit 3.

## Where it stands
- Done and landed: P1–P6 (report rev 4; masters TX 1507–1511; leaves; curation; route — «The 2.1.325 recorded
  fixture set» at Epoch 4's head, PROVISIONAL, and the DEL carry; the run node, session 118; the handoff).
- P7: new-text listing `unchanged`; light gate `green 19 · red 0 · recorded 0 · timeout 0 · not-run 11` (the 11 are
  `leg operator`, re-verified from their evidence); drift = 0; coverage flipped (v2-02, v2-03 verified · version
  `verified 7/31 · deferred 0 · planned 24 · implemented 0 · unclaimed 24`) and the v2-05 note landed; 3b no-op
  (every sidecar's set and prunable 0); 3c hygiene `clean`, scope `clean`; 3d `rec.py land` tx 1531, `outputs land`
  tx 1532; `rec.py check` clean.
- Left: the seal, the flip (`route.py flip`), the commit, the push; the gates evolve checkpoint.

## Q — how the seal is treated
1. **Commit without a seal this time (Recommended).** Every run dir rides this commit as files, as an unsealed run
   always may («a run with no wrap … is sealed at the next sealer»); the next wrap's run dir is newer than all of them
   and seals them whole. The commit body names the refusal. Nothing is routed around: the bare call stands refused.
2. **Seal the 19 older runs by `--run`, one call each, the five newer left as files** to the next sealer. A documented
   form, but a per-run loop the letter does not name for this step.
3. **Another treatment, named by the overseer.**

**Answer:** _(the overseer's word, appended here verbatim, then snapped; P7 resumes at 3d)_

The driving overseer's message, verbatim (2026-11-15):

Answer to technical-fork-p7.md: option 2, the contract's own form — read ~/dev/projects/additional/viola-overseer/relay-wrap-live-round-p7.md whole, append it under the fork and snap it: seal each run dir OLDER than this wrap's own by `rec.py seal --run`, one call a run, dry-run first; the five newer ones ride the commit as files and the commit body says so in one line; then check, the flip, the commit, the push. Stop if a call refuses or a dry run surprises. After the push start NOTHING.

The relay it names, `~/dev/projects/additional/viola-overseer/relay-wrap-live-round-p7.md`, whole and verbatim:

# Wrap of 2026-11-15-live-round-on-the-trimmed-product — the answer to technical-fork-p7.md (the seal)

**Option 2 — the contract's own form** (the pipeline owner's technical ruling, relayed): records-contract.md's seal
paragraph says the seal «refuses … a run dir newer than its own (another run is open)» and «`--run {run}` seals that
run alone»; nothing is overridden.

At P7 step 3d seal by `rec.py seal --run-dir {run_dir} --run {run}`, ONE call a run, every run dir OLDER than this
wrap's own run (`2026-11-15T11-38-30-wrap`), `--dry-run` first for each. The five run dirs NEWER than it (the phase
and implement runs of revisions 8 and 9, `12-10-18` … `12-35-39`) are NOT sealed: they ride this commit as files, as
a sealer's own run does, and the next sealer seals them. The commit body says so in ONE line: which five, and why
(the wrap is a resume; its run dir carries its first start's name).

Then `rec.py check`, the flip, the commit and the push, as written.

If any `--run` call refuses, or a dry run lists a run you did not expect, STOP there and say which.
After the commit and push start NOTHING.
