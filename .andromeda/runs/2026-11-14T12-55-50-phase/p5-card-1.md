# P5 review — 2026-11-14-hyprland-exact-frame-bound-pinned · «Hyprland exact-frame bound pinned»

The plan's render to open in an editor: `.andromeda/cache/docs/2026-11-14-hyprland-exact-frame-bound-pinned/plan.md`.
No plan stands in a chunk's folder. Writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-14-hyprland-exact-frame-bound-pinned`.

## Goal
Pin by tests the one case the Epoch 1 audit found missing on `viola-hyprland`'s two survivors, `>` → `>=` at
`read.rs:77` (`read_within`) and `reply.rs:151` (`parse_dispatch`). The case is exactly `MAX_FRAME` bytes accepted
and one more `oversize`. The work is test-only: no mutation run, no product line, no capability (`inputs#I1`).

## Top acceptance criteria
- `read_takes_a_reply_of_max_frame_bytes_and_refuses_one_more_as_oversize` passes. Through a planted `hyprctl` stub,
  exactly `MAX_FRAME` reads `Ok(Clients([]))` with no `parse-rejected` line; `MAX_FRAME + 1` reads `Err(Oversize)`
  with one `parse-rejected{detail:"oversize"}` line.
- `parse_dispatch_takes_max_frame_bytes_as_refused_and_refuses_one_more_as_oversize` passes. Exactly `MAX_FRAME`
  reads `Err(Refused)` and one more `Err(Oversize)`. `Refused` ≠ `Oversize` keeps it from passing vacuously.
- `gate.py control` (kill deadline `--timeout 600`): `read-77` and `reply-151` each read `holds`, each its case by NAME
  and its entry `cargo nextest run -p viola-hyprland --no-fail-fast`.
- Test-only: the three bound lines stand (`grep -c` reads 1 and 2), and the tracked delta is the two test modules
  (`git diff --name-only 2414270c1ce5 … | wc -l` reads 2).
- Every gate passes: fmt, clippy, check, nextest, census + seam contract, `run --unit` + gate, pre-push. The operator
  pass then reads CI `verdict: green`.

## Touchpoints
- `crates/viola-hyprland/src/read.rs` — one test in its `tests` module
- `crates/viola-hyprland/src/reply.rs` — one test in its `tests` module

## Test commands (15 entries)
- lint: fmt · clippy `-D warnings` · `cargo check`.
- unit: `nextest -p viola-hyprland --no-fail-fast` · the two new tests by name (`-E`, expects `2 tests run: 2 passed`).
- integration: `contract_bounded_readers` + `contract_hyprland_seam`.
- probes: the two bound `grep -c` entries · the `git diff` count.
- harness: `run --unit` · `gate --require nextest-unit` · `pre-push`.
- operator: hygiene · push (clean-tree form) · `ci.py conclusion --sha HEAD --wait 1200`.

Baselines of the 5 new entries:
- entry 5: red as predicted (0 tests to run); the filter form was controlled green over two standing names.
- entry 6: green, 30/30.
- entries 7 and 8: green; each one's known-positive (the mutant, in a clone) reads red.
- entry 9: red on the empty input; in a clone, 2 files read green and 3 read red.

All of this is in `.andromeda/runs/2026-11-14T12-55-50-phase/control-baseline.md`.

## Capabilities
None claimed. There is no concretization preview: the carry rules «no requirement added, no capability claimed»,
and no pooled cap names this bound.

## The directive's item-6 block (`inputs#I1`), verbatim where a tool printed it

**The union:**
- `architecture: rows 994 · packs 10 · picked 13 (8 by every draw) · standing 16 · 9254 B`
- `security-plan: rows 618 · packs 7 · picked 38 (25 by every draw) · standing 9 · 12486 B`
- `design-system: rows 325 · packs 4 · picked 0 (0 by every draw) · standing 0 · 2486 B`
- `layout-templates: rows 225 · packs 3 · picked 0 (0 by every draw) · standing 0 · 416 B`
- `test-plan: rows 1375 · packs 14 · picked 62 (36 by every draw) · standing 25 · 17996 B`
- `obs-plan: rows 1060 · packs 11 · picked 15 (6 by every draw) · standing 83 · 7216 B`
- `a11y-plan: rows 241 · packs 3 · picked 17 (8 by every draw) · standing 1 · 6643 B`
- `rows-union: 7 master(s) · picked 145 of 4838 · rule any`

**Rows the plan cites, against the union:** every row the plan and research cite stands in the union's rows file or
its `*-standing.md`. That is arch's 6, security-plan's 4, test-plan's 12 and obs-plan's 2.
`decision/unit-tests-inline` and `decision/test-function-naming` stand in `tests-standing.md` (S2, S1); the rest are
picked rows. **None is in neither.**

**[RETYPED] lines:** none. **[EVERY_ROW] re-spawns:** none. Every agent passed on its first try:
- `rowtask 7/7`;
- `pick 156/156` in 3 parts;
- `rowhistory 5/5`;
- `draws-dead:0`, `batches-on-one:0`.

**Non-zero exits, refusals, UNPARSED / INDETERMINATE / NOT DERIVED:**
- Bash guard, once, during P2's wait: `Blocked: cd (…/.andromeda/runs/2026-11-14T12-55-50-phase) moves the session cwd
  for every later call`. It was re-issued as Read-tool reads of the same files.
- Baseline entry 5: `red · exit 0 ✗ (exit 4)` (`error: no tests to run`), the predicted red.
- Baseline entry 9: `red · last line '2' ✗ (last: '0')`, the predicted red on an empty input.
- Controls in the clone, read red by design:
  - entry 7: `red · exit 0 ✗ (exit 1)`;
  - entry 8: `red · last line '2' ✗ (last: '1')`;
  - entry 9: `red · last line '2' ✗ (last: '3')`.
- CI at Setup 5a: `2414270c1ce5 verdict: in progress · checks 7/7 · runs ci#43597047331 in_progress/-`. It is read as
  no verdict, never green.
- No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed by any tool this run.

**Where a letter was wrong or silent:**
1. *Which «fake `hyprctl`».* The carry and directive item 3 say the read's case runs «through the fake `hyprctl`»,
   which test-plan's rows define as the root fake agent. A root-package test cannot kill a `viola-hyprland` mutant:
   - the harness passes no `--test-workspace` (`crates/viola-e2e/src/harness/run/mutants.rs:151-152, 234-249`);
   - cargo-mutants 27.1.0 runs only the mutated package's tests;
   - test-plan · decision/mutants-tested-by-owning-package says the same.

   Read that way, the letter defeats its own entry («the next audit catches both survivors»). I leaned to the crate's
   own planted `hyprctl` stub, the form its five read tests already use. It is a fake, never the host's. The lean is
   recorded in the plan's Constraints.
2. *`references/fanout-contract.md`* is 477 lines; the Read tool returned 1–400. P2's letter names §The batch, which
   was read whole. The unread tail is §Re-spawn's instruction table end and §Status. No re-spawn or resume arose.
3. *Research's first draft* put `read.rs`'s test module at `:172`. It is `:163`, corrected in research rev 2 before the
   plan cited it.
4. *The plan template's smoke entry.* Check 4 (2) names it for a boot-path touchpoint. No smoke runs: the seam has no
   product caller (graph: `read_within` ← `read` only; `read` ← the tap and tests). The absence is stated in the
   Test Commands prose.

## Beyond the plan
- Carry-forward (directive item 4, found at P3 outside the pinned scope): the `hyprctl reply` row of
  `tests/contract_bounded_readers.rs` (`:240`) names a parse test as its witness
  (`parse_clients_takes_max_frame_bytes_and_refuses_one_more_as_oversize`), not one that drives the read's own drain
  past its cap. Owner: the existing entry «`viola open`», for the wrap's route adaptation to pin.

Apply? (yes / review / cancel)
