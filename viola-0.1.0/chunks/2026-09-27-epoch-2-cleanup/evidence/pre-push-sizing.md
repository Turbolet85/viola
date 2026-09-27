# Pre-push sizing reading and the host-scratch temp confirmation (plan step 15, implementation notes)

## The implement gate's pre-push (gate entry 34), this host, 2026-09-27

Tree: `a0e6506` plus this chunk's uncommitted work. rust-analyzer was left running (plan Test Commands: the stop is needed only
until step 7 lands).

- Verdict: exit 0, `{"cmd":"pre-push","ok":true}`, `stage:"union"`, gate `ok:true`, breaches `[]`.
- Wall-clock: **1470.39 s ≈ 24.5 min** (gate tool record), against the plan's prediction of ≈ 25 min (≈ 23 min legs + ≈ 2 min other).
- `legs`: both at base `69abc0d038732cff8d45602f82f8bebaf28bbb72` (the last master flip; `a0e6506` changed route documents only),
  `verdict:"counted"`:
  - `ubuntu-latest` tested **200**, log line `200 mutants tested in 9m: 26 missed, 160 caught, 14 unviable`;
  - `windows-2025` tested **200**, log line `200 mutants tested in 14m: 184 caught, 16 unviable`.
  - The union judged each mutant by the legs that compile its line, and the ubuntu leg's 26 misses were caught on the windows
    leg, so there are 0 breaches.
- Prediction vs measure: the plan predicted ≈ 165 mutants per leg; each leg measured 200. The extra ≈ 35 are this implementation's
  own additions beyond the plan's list: the diff classifiers moved to `mutants/base.rs`, the per-leg verdict to `mutants/leg.rs`,
  and the archive, scoped-mode and scratch code as written. The wall-clock still landed on the prediction.
- `cache` (all path-free byte counts): `bytes` 12 415 099 263, `cap` 42 949 672 960, `cleaned:false`, `scratch_bytes` 82 522 477,
  `windows_scratch_bytes` **8 725 784** (what the scoped `run --mutants` entry 27 left), `bytes_after` 12 694 062 001,
  `scratch_bytes_after` 61 772 564, `windows_scratch_bytes_after` **74 202 213** (the windows leg's `mutants.out`, its temp
  copy already removed by cargo-mutants).
- `vm`: `terminated:true`, `free_kib_before` 14 036 364, `free_kib_after` 14 386 408.

## `TMP`/`TEMP` reach cargo-mutants' temp copy (the plan's one-time confirmation)

The step-9 scoped entry (`bash scripts/agent-run.sh run --mutants --file crates/viola-e2e/src/harness/run/mutants/scratch.rs`) was
re-run while a 10 s handle-free poll listed `cargo-mutants-*` in the sibling scratch, and any new `cargo-mutants-viola-*` in
`%TEMP%` (those present before the run were excluded).

- First sighting 2026-09-27T15:44:31Z: **`viola-mutants-scratch/cargo-mutants-viola-IE1lTy.tmp`** (the scratch).
- `%TEMP%`: no new `cargo-mutants-viola-*` dir during the run.
- Run: exit 0, `verdict:"scoped"`, 21 tested, 21 caught, `scratch_bytes` 74 202 213 (the pre-push windows leg's leftover `mutants.out`,
  wiped at this run's start), `archived:"target/run-archive/33"`.

So on Windows, `std::env::temp_dir()` in cargo-mutants reads the `TMP`/`TEMP` the harness sets, and the copy lands on D:, beside the
repo, not on C:.
