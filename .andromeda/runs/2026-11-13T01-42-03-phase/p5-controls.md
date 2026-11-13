# P5 controls — chunk 2026-11-13-verify-readiness-on-the-input-box

Every control was fired as the entry, `gate.py run --chunk 2026-11-13-verify-readiness-on-the-input-box --docs-root
<the project> --entry N` (`--operator N` for a leg entry) `--project-root <scratch clone>`, no run dir, over
`git clone --shared` of the project at HEAD `2f9eaddb` in the session scratchpad (`$SCRATCH/ctl/clone`).

## Entries 12, 13, 14 (step 6) — must-pass over the step as the plan builds it
- Unmodified clone: 12 red (`exit 0 ✗ (exit 1)`), 13 red (`exit 101 ✗ (exit 0)`, rustix 1.1.5 under tempfile 3.27.0),
  14 red (`exit 0 ✗ (exit 1)`, advisories FAILED on RUSTSEC-2026-0457 · -0461 · -0462 · -0486) — the readings the real
  tree gave at P5.
- Modified clone: `deny.toml`'s `ignore` given the three reasoned entries of step 6 (each reason naming its disabled
  feature), and `cargo update -p rustix --precise 1.1.6` (crates.io index updated; `rustix v1.1.5 -> v1.1.6`):
  12 green (exit 0, 531 B, the three ids), 13 green (exit 101, `did not match any packages`), 14 green (exit 0,
  `advisories ok, bans ok, licenses ok, sources ok`).

## Entries 33, 34, 35 (step 7, operator readers) — the literals against a recorded verify output
Input: `<clone>/.andromeda/cache/chunks/2026-11-13-verify-readiness-on-the-input-box/evidence/verify-2.1.322.out`.
- A — the recorded live output `viola-overseer/verify-2.1.302-20261019.out` (24 step lines, rows 16 · 17 · 18 · 24
  `fail`, `stamped 2.1.302  20 pass  4 fail`), copied unchanged: 33 green (`last line 24`), 34 red (`exit 0`, the
  four fails counted — the reader can read red), 35 red (`contains 'stamped 2.1.322  '` ✗ on a 2.1.302 summary).
- B — the same file with every `  fail` row `  pass` but `statusline-rate-limits` `  fail`, and the summary
  `stamped 2.1.322  23 pass  1 fail` (a control input carrying the subject as the plan builds it): 33 green, 34 green
  (`exit 1`, `last line 0`), 35 green.
- The step-line and summary forms are the recorded output's own (A), matching architecture ·
  contract/viola-verify-output/step-line and test-plan · contract/verify-output/summary-line.

Entry 36 is report-only (`expect = []`): no control.
