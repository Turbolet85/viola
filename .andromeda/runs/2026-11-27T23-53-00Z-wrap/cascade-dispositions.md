# Cascade dispositions — 2026-11-27-epoch-5-cleanup-2

## Step 2 — the sweep
The one landed operation (tx 5229, test-plan `decision/unit-tests-inline`, its `except` set) changed prose alone: its
dry run wrote no patterns file (`patterns: none: 1 set(s) and retire(s) changed prose alone — no file`). Main's own
pattern, `.andromeda/runs/2026-11-27T23-53-00Z-wrap/cascade-patterns.toml` id `inline` (`inline (unit )?tests?` ·
`tests? (sit|live|stand)s? inline` · `same source file` · `inline `mod tests` · `cfg(test) tests module`, ignore
case), over every master, key file, leaf, curation home and base: `cascade.py sweep` (part
`sweep-pass-1.1.txt`), `total (1 pattern) · 6 rows over 6 files`, `per class · new 0/0 · standing 1/1 · leaf 3/3 ·
curation 2/2 · base 0/0`. Not looked for: wordings that state the inline placement without these phrasings.

- `.andromeda/test-plan.md:265` `decision/unit-tests-inline` — amended (the row itself, `edited ×4`: the rule keeps
  «inline … in the same source file», the new exception names the split form).
- `.claude/rules/verification-harness.md:59` (curation) — no change: a test's full path `<module>::tests::name` stays
  true, the moved cases keep their module paths.
- `.claude/docs/session-learnings.md:64` (curation) — no change: a test-only fix touching an inline test, still true.
- `.claude/rules/testing-src.md:24` (leaf) — no change: an index line naming a dated learning, no claim.
- `.claude/docs/services/viola-pty.md:39` (leaf) — re-derived at step 3: «the crate's inline `mod tests`» — the pump's
  tests now stand in `pump/test_cases*.rs` path files.
- `.claude/docs/services/viola.md:49` (leaf) — step 3's comparer reads it: «inline unit tests» stays true of the root
  binary's other modules; `cmd::run::tests` now stands in `src/cmd/run/test_cases*.rs`.

Lateral binds: no harness command, log field or violation-schema field changed — none touched.

## Leaves
`cascade.py leaves` pass mode (`epoch · «Epoch 6 — The fleet console» · markerless 3 · gated 0 · open`), `total · pass ·
4 group(s) · 10 leaves · 232842 B · unmapped 2`; `--add` `services/viola-pty.md`, `services/viola.md` (the sweep's
`leaf` rows). One comparer a group, batch `leaf` (`4 of 4 pass`); `leaves --apply` `total · blocks 1 · applied 1`.
- `test-plan-1` (commands.md, gotchas.md) · pass · findings 0.
- `test-plan-2` (tests-summary.md, rules/observability.md) · pass · findings 0.
- `test-plan-3` (rules/testing.md, rules/verification-harness.md, CLAUDE.md) · pass · findings 1 · applied 1
  (`testing.md:19`, omission: the inline-test line names the split exception) · by hand 0 · declined 0.
- `added` (services/viola-pty.md, services/viola.md, rules/testing-src.md) · pass · findings 0 · stands 3 (each read and
  accepted: the row's rule still reads inline, the exception sits beside it).
- By hand, outside the comparers (the wrap directive `inputs#I6` item 4, a tree fact no master states):
  `services/viola-pty.md` Entry points gains the respawn module `crates/viola-pty/src/pump/restart.rs` and the pump's
  test path files — `rec.py harness land` tx 5231.
- Unmapped: `.claude/rules/host-linux.md` (host, the host's own statements — no source) and
  `.claude/rules/testing-src.md` (placed in `added` by its sweep row) — no change.
