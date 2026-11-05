# P5 in progress — 2026-11-05-linux-only-code-and-ci (stopped before the word: the context alarm fired at 85 %)

Written 2026-11-05T11:51:54Z. No review card has been shown and no word taken; `matrix-2026-11-05-linux-only-code-and-ci.json`
does not exist, so Setup 3 (a) offers **resume at P5** in this run dir. P1–P4 stand: scope rev 6 (tx 216), research rev 3
(tx 211), plan rev 2 in the store (tx 214); the plan's working copy `.andromeda/cache/docs/2026-11-05-linux-only-code-and-ci/plan.md`
holds two edits not yet put (below).

## The two answers on record (each snapped; neither is an operator review of the card)
- `inputs#I2` (`relay-1.md`) — P4: the closed words no Linux build emits leave with their code. Folded into scope rev 5,
  research rev 2–3 and the plan.
- `inputs#I3` (`relay-2.md`) — P5 step 1: RUSTSEC-2026-0441 (`pin-utils` via `portable-pty =0.8.1` → `nix 0.25.1`) is owned
  by a CARRY on «Version done-check» naming the remedy (move `portable-pty` off 0.8.1 to a release whose graph drops
  `pin-utils`; the 0.9.0 Windows bug behind the pin leaves with this chunk); no ignore widening. Folded into scope rev 6.
  NOT yet folded into the plan.

## Owed, in order, before the card
1. Land the plan's working copy (`put --expect-rev 2`): it already holds (a) step 6's `CASE_INSENSITIVE` (`lib.rs:32`)
   bullet and (b) the harness-renders line moved out of `Expected amendments (wrap)` (check 2).
2. Fold `inputs#I3` into the plan:
   - split entry `cargo deny --format json check` into `cargo deny check bans licenses sources` (asserted, `new = true`)
     and `cargo deny check advisories` (`expect = []`, report-only, its note naming RUSTSEC-2026-0441 and the CARRY);
   - split the `ci.py conclusion` operator entry into one per check-name prefix (`--name test`, `lint`, `msrv`, `perf`,
     `release`, `fuzz-replay`, each `contains verdict: green`) plus `--name supply-chain` report-only (`expect = []`), its
     disposition the CARRY; note that supply-chain's later steps (fuzz audit, tokio ban, ban probes, zizmor) do not run
     in CI after its deny step fails, so the local entries are their reading;
   - reword the (tests) CI criterion and the (security) deny criterion to that exception;
   - add the `## Beyond the plan` row for the card: the CARRY on «Version done-check» (`inputs#I3` wording), and the
     `v1-01` owner row (provable at «Version done-check»'s CI read).
3. Remove entry `bash scripts/agent-run.sh secret-scan`: it timed out at 1800 s on the untouched tree reading the 12 GB
   `target/agent-run` (measured `du -sh`); the scan runs in CI only — say so in the Test Commands prose and in the (obs)
   criterion.
4. Write the `baseline` keys (P5's own write) and set `new = true` on every NOT SEEN entry, from the readings already taken
   (`gate-2026-11-05-linux-only-code-and-ci.json` trail; logs `$TMPDIR/andromeda-gate/2026-11-05-linux-only-code-and-ci/phase-2026-11-05T08-59-20/`):
   1 recorded (tmpfs 32G, 6.0G used, 26G avail, 20 %) · 5 green · 8/9/10/11 red, the predicted reds (274 lines; 21 workflow
   lines; 11 manifest lines; `stands: vendor/conpty`), must-pass readings green in `p5-controls.md` · 12 green (3.46 s) ·
   13 green (42.87 s) · 14 green · 15 green · 17–25 green (16 red: the advisory, now split) · 22 green · 27 green (6.85 s) ·
   28 green (5 hyperfine rows, `ok:true`) · 29 green · 30/31/32/36 green. Baseline-run the two new deny entries.
5. Re-run P5 step 1's whole set (the plan changed): `gate.py run --dry-run`, `planlint.py check`, `gate.py scope
   --lists-only`, `inputs.py verify`, `gate.py baseline`, check 2's path read.
6. Validation-1 (aligned at the last read: the plan matches scope rev 6 once item 2 folds), then the card
   `p5-card-1.md` (sampler: off; no cap claimed — `v1-01` and `v1-53` notes; the directive's item 10: every refusal with
   the letter line — the bash guard's `cd` ×2, bounded-grep ×1, pipe-to-filter ×1 refusals; the detail v1.28 tool fault and
   the stopped first P2; the five first-attempt `EVERY_ENTITY` refusals; the ten strong refusals, eight cleared by `move`,
   two by one re-spawn), then the word, step 4, step 5 (`rec.py outputs land`, its `landed tx` line on the report), the
   validate checkpoint.
