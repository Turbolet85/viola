# P5 review — 2026-10-13-epoch-4-survivors-ii · "Epoch 4 survivors II"

**Plan:** `viola-0.1.0/chunks/2026-10-13-epoch-4-survivors-ii/plan.md` (304 prose lines, 21 gate entries) ·
**Research:** `research.md` · **Scope:** `scope.md` (premise-corrected at P3).

## Goal
The Epoch 4 audit's 16 missed mutants in `src/cmd/verify/typed.rs` (`viola verify`'s Run B driver) are each disposed
by a behaviour-preserving restatement plus a root-package nextest case. Each case is shown red under its hand-applied
mutant and green restored. None is owed again, none is called equivalent, and no "not measured here" record is
expected, since every site compiles and runs on Linux.

## How (three restatements, one file)
1. **Framing waits (5)** become two named predicates over the captures dir, `added_turn_ended` and
   `local_command_answered`. One named deadline, `probe_until(clock) = clock.now() + PROBE_DEADLINE`, takes all six
   sites of that expression. Their cases plant capture files in a tempdir and read a fixed clock.
2. **`SharedPty` (5)** becomes generic over its `Pty`. A hand-written `Recording` double in the test module checks
   that `resize`, `close` and `child_pid` reach it. There is no `mockall`, because the root package declares none.
3. **`Run`'s Ctrl-C ending (6)** becomes `ctrl_c_ending(clock, ctrl_c, ended, kill)` and
   `ended_by(clock, until, finished)`, plain functions with an rstest table over scripted readings. `Run::wait_pump`
   is removed, since a kept bool wrapper would re-generate `→ true` where no test reaches it.

No test starts any PTY child. Run B is unchanged: its pastes, their order, the modal guard, 120 s, the Ctrl-C → 500 ms
→ Ctrl-C → kill ending, `MAX_FRAME` and the spawn pairs.

## Top acceptance criteria
- 16 ledger rows (`evidence/survivors.ndjson`), keyed exactly on the earlier chunk's 16 `owed` rows, each with its
  restated site and its disposer. 14 distinct hand-applications: `:286:29`, `:651:45` and `:671:53` become the one
  `+` in `probe_until`.
- Run B unchanged:
  - `cli_verify` (29 cases) is green, covering the seventeen steps and the stamp, the six spawn pairs, the typed
    order, the modal guards and no run dir left;
  - the removed-lines guard on `typed.rs` reads `0`;
  - the preservation guard holds every other file.
- No mutation run, witness run or dispatch, and no live `claude` (`inputs#I1`).
- Unit, pre-push and the CI read across the three OSes are green.

## Touchpoints
- `src/cmd/verify/typed.rs`, the only source file. All callers are inside it (research.md §Graph impact).

## Test commands (21)
- Lint and checks: fmt · clippy · Windows-target clippy (check only) · G1 · lint probes · `run --unit`.
- New entries:
  - `run --integration --filter 'binary(cli_verify)'` (new, baseline green 29/29);
  - the ledger probe (new, baseline red: the ledger is absent; controls 16 rows → 1, 15 rows → 0);
  - the removed-lines guard (new, baseline green; controls: a removed `const` → red, a `probe_until` restatement →
    green);
  - the preservation guard (new, baseline green; controls: `src/main.rs` → red, `typed.rs` → green);
  - `cargo deny` (new every chunk, green).
- Smoke: cleanup / boot / status / G2 / G4 / cleanup (new session `p-e4s2-smoke`, all green).
- `pre-push`.
- Operator pass: hygiene → push → `ci.py conclusion --wait 1800`.

## Sizing (`inputs#I1`)
One window, as a decisive lean: 16 survivors in one file, three restatements, 8 new cases. The earlier chunk disposed
29 across ten files in one window. No split proposed.

## Concretization preview
None. No verification-matrix capability is claimed. Of the 37-id unclaimed pool, none names mutation strength,
`viola verify`'s driver or a survivor site.

## Expected amendments (wrap)
- test-plan §10, the Mutation gate paragraph: the 16 disposed by restatement and cases, none left owed.

## Beyond the plan
none
