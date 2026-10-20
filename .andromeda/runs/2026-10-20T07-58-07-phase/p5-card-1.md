# P5 review — 2026-10-20-epoch-5-cleanup ("Epoch 5 cleanup")

**Plan:** `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/plan.md` (13 steps, 15 acceptance criteria, 26 gate entries) ·
**Partition:** the founder's own word (inputs#I2, «Два чанка», 2026-10-20T08:32Z): items (1)(2)(3)(6)(8)(9)(10) here.

## Goal
Close seven of the ten Epoch 5 boundary debts, with no mutation run, witness run or dispatch (inputs#I1 §1):
- **(1)** The root package's tests pass under the nextest `mutants` profile's 10 s kill. The replay walks run their
  sets concurrently, and the paste-hint pair gets a class kill line.
- **(2)** `verify_window_`'s `mutants` kill moves 45 → 60 s, its `ci` kill, by the by-design-floor exception with its
  planted-hang pair.
- **(3)** The Windows leg's three failed baselines:
  - (3a) the scratch case makes its plain control;
  - (3c) `viola-pty`'s tests stop leaving their own test binary running;
  - (3b) is recorded.
- **(6)** `dialogless_home` derives its summary from `LedgerRow::relied()`.
- **(8)** The truncated `.profraw`'s cause, read two-sided.
- **(9)** The keepers refuse below a `/tmp` headroom floor.
- **(10)** The web leftovers in four rule files and the `viola-ui` stub.

## What P5 measured (untouched tree, HEAD d35644a58909)
- **The root baseline under `mutants`** (entry 14): 1601 tests, 1 TIMEOUT. The timeout is
  `fake_agent_dialog_replay_matches_every_recorded_dialog_set` at 10.003 s. Four more cases sit within 0.6 s of the
  kill: the framing replay 9.453 s, the statusline replay 9.431 s, the stamped-set probe 9.447 s, and
  `verify_window_without_screens…` at 43.034 s of 45. The paste-hint pair reads 8.7 s.
- **`pre-push` (entry 23) read RED on the WATCH's own class.** 2974 of 2974 tests passed, and the merge refused one
  90 112 B `viola` profile, whose 7 969 siblings are 146 344 B. The first case to end after it was
  `chaos_wait_parked_wrapper_killed_exits_21`, again (`.andromeda/runs/2026-10-20T07-58-07-phase/p5-prepush-red.md`).
  The WATCH's retire condition, «a recurrence», is met. That red is item (8)'s subject: the plan reads a `pre-push`
  red of exactly that shape through step 10's finding, never as a pass.
- **The Windows dispatch's `viola-pty` log:** 11 caught, then every mutant `unviable` right after the `try_wait`
  mutants. The tests' guard kills its child only when `try_wait` reads `Ok(None)`, and that child is the test binary
  itself (step 7's mechanism, a hypothesis until the next boundary's Windows reading).
- **CI JUnit (run 39383206773, `ci` profile):** the dialog replay reads 14–17 s on all three OSes, and the scratch case
  is green on windows-2025 (0.019 s).

## Top acceptance criteria
- Entry 14 reads `exit 0` with no TIMEOUT after steps 2–5; the before/after walls are in `evidence/baseline.md`.
- Each moved or added kill has its planted-hang pair: TIMEOUT under `mutants`, PASS under the default profile.
- The scratch case keeps both assertions, and CI's windows-2025 leg reads it green.
- `viola-pty`'s leak is gone under a hand-applied `try_wait -> Ok(Some(1))` (over a scratch clone).
- `dialogless_home`'s literal line is gone (entry 12), and the test oracle in `tests/cli_verify.rs` is unchanged.
- Item (8): a two-sided census reading, with either the hangup named and fixed (0 refused over the same runs) or the
  no-reproduction recorded.
- The keepers refuse below the floor with two counts and no path; the floor's basis is in `evidence/headroom.md`.
- No rule file's `paths:` names `e2e-web/` or `crates/viola-ui/` (entry 13).

## Touchpoints
`.config/nextest.toml` · `tests/contract_lints.rs` · `tests/contract_fake_agent_drift.rs` ·
`tests/contract_ledger_probes.rs` · `crates/viola-e2e/src/harness/run/mutants/scratch.rs` · `crates/viola-pty/src/lib.rs`
· `tests/support/home.rs` · `tests/cli_instance_state.rs` · `crates/viola-e2e/src/harness/mod.rs` ·
`crates/viola-e2e/Cargo.toml` · `Cargo.lock` · `src/bin/viola-fake-agent.rs` (only if step 10 names the hangup) ·
`scripts/profraw-census.sh` · four `.claude/rules/*.md` · `.claude/docs/services/viola-ui.md` (removed).

No master edit. The Expected amendments (wrap): test-plan §3 / §10, design-system §Brand Identity, a11y-plan §1, obs-plan
§4, and arch's tree (the `CLAUDE.md` services pointer).

## Test commands
The lints (Windows-target clippy included), `run --unit`, the changed binaries by name, the two by-name filters (red
until built), the two literal probes, the root baseline under `mutants`, the `gate.py guard` (controls both ways over a
scratch clone), `cargo deny`, the harness smoke, G2 / G4, `pre-push`, then the operator trio (hygiene, push, `ci.py` with
the OS-sensitive cases and the legs that witness each).

## Concretization previews
None: no matrix capability is fully provable by this chunk (v1-07 and v1-03 read short; no claim).

## Validation-1 — where the plan claims less than the entry's words (yours to accept)
- **(3b)** `path6_the_boundaries_and_the_window_word` is recorded, not changed. Its 10 s in the dispatch against
  5.5 s in CI is unmeasured, and measuring it on the runner that failed is a dispatch.
- **(8)** If step 10 reproduces nothing, or names a killer it cannot fix in-chunk, the item ends as a recorded finding
  with the red open. The entry's «has its cause found» is then not met, and the report says so.
- **(1)/(3)** "pass" on Windows and macOS is the next boundary's reading; here it is this host's run and the CI legs.

## Beyond the plan
| # | proposal | owner / place |
|---|---|---|
| 1 | Route insertion «Epoch 5 cleanup II» directly after this entry, carrying items (4), (5), (7) as scope.md folds them | the wrap writes it on the founder's word (inputs#I2) |
| 2 | Deferral: the Windows mutation-copy readings of (3a), (3b) and (3c) (the scratch case inside a cargo-mutants copy, `path6_…` inside its kill, `viola-pty` unviable ≤ caught) | owed to the next epoch boundary's code audit (its Windows leg); no route entry |
| 3 | The `.profraw` WATCH retired by its recurrence (P5, 2026-10-20T08:48Z) into item (8). If step 10 ends with no reproduction or an unfixed killer, its open red gets an owner | the wrap proposes the owner then |
