# /implement outcome — 2026-10-20-epoch-5-cleanup ("Epoch 5 cleanup")

**Outcome: green + smoke.** Every non-operator block entry read green on its first firing: `entries 26 · green 22 ·
red 0 · recorded 1 · timeout 0 · not-run 3`, the three not-run entries being `leg = 'operator'`. One acceptance
criterion is not met as written, by a measured host fact; it is surfaced below for the wrap, not worked around. Next →
the operator pass (entries 24, 25, 26), then `/andromeda-wrap-session`.

## Files changed

- Source and tests: `.config/nextest.toml` · `tests/contract_lints.rs` · `tests/contract_fake_agent_drift.rs` ·
  `tests/contract_ledger_probes.rs` · `crates/viola-e2e/src/harness/run/mutants/scratch.rs` · `crates/viola-pty/src/lib.rs`
  · `tests/support/home.rs` · `tests/cli_instance_state.rs` · `crates/viola-e2e/src/harness/mod.rs` ·
  `crates/viola-e2e/Cargo.toml` · `Cargo.lock` (one `libc` edge) · `src/bin/viola-fake-agent.rs` (step 10(c)) ·
  `scripts/profraw-census.sh`
- Rule and doc files: `.claude/rules/frontend.md` · `.claude/rules/a11y.md` · `.claude/rules/api.md` ·
  `.claude/rules/testing.md` · `.claude/docs/services/viola-ui.md` removed (`git rm`, not refused)
- Evidence: `evidence/{baseline,kill-lines,viola-pty-leak,windows-baselines,profraw-cause,headroom}.md`;
  `operator-pass.md` is the operator pass's.

## What each item reads now

- **(1)** The root package under `NEXTEST_PROFILE=mutants` (entry 14): `1605 passed`, no TIMEOUT (it was 1 TIMEOUT at
  P5). Each replay walk runs its sets on scoped threads, with its assertions unchanged: the dialog replay ran 10.003 s
  TIMEOUT before and 4.877 s after, and the framing, statusline and ledger-probe walks run about 4.8 s. The paste-hint
  pair has its own 30 s override and the pin `mutants_paste_hint_override_holds_the_class_above_the_default_kill`.
- **(2)** `verify_window_`'s `mutants` kill is 60 s, still first among the overrides.
- **(3)** (3a) the Windows scratch case's plain control is granted by `icacls` and compiles for the Windows target.
  (3b) `path6` is recorded at three readings, its reading owed by name. (3c) the `viola-pty` guard kills
  unconditionally and the children run per-test copies. All Windows readings are owed to the next boundary's Windows
  leg (`evidence/windows-baselines.md`).
- **(6)** `dialogless_home` derives `20 pass  4 fail` (Unix) / `17 pass  4 fail` (Windows) from `LedgerRow::relied()`.
  Entry 12 reads `last line 0`, and its caller is green in entry 8.
- **(8)** The cause is found, two-sided (`evidence/profraw-cause.md`). Before: 17 refused of 726 profiles in 200 runs
  of `chaos_wait_parked_wrapper_killed_exits_21`, and every attributable file (7) is a `viola hook` cut off after
  `Client::notify`. After the fake agent's commands left the foreground process group: 0 refused of 800, 4 per run.
  The killer is the terminal hangup after a test kills the wrapper. `pre-push` (entry 23) read green with no
  `header is corrupt` line.
- **(9)** Both keepers refuse a linked backing below 9 216 MiB free with the fixed two-count message
  (`evidence/headroom.md`, with the basis and the remove-the-guard reading).
- **(10)** No rule file's `paths:` names `e2e-web/` or `crates/viola-ui/` (entry 13, exit 1, no output). `a11y.md`
  loads on the terminal surfaces. The `viola-ui` services stub is removed.

## Deviations from the plan, with their justifications

1. **Step 7's acceptance is not met as written (surfaced, a premise disproof).** «Step 7's remove-the-guard reading
   shows leaked children under the hand-applied `try_wait -> Ok(Some(1))` on the old guard and none on the new one.»
   Measured: no leaked child on either guard (census `none running` before, after the old-guard run, and after the
   new-guard run). On Unix, dropping the `PortablePty` closes the master and the kernel hangs up the child, which
   `close_hangs_up_a_live_child` pins. So the Linux host cannot separate the two guards. The change stands; its proof
   is the Windows `mutants (viola-pty)` reading owed to the next boundary. The wrap resolves the criterion's wording
   (`evidence/viola-pty-leak.md`).
2. **The leak census keyed on the test binary's name** (`viola_pty-77dd4bdbf6cc1891`), not «the clone's target dir».
   The new side's children run from a tempdir copy, which a target-dir key would never read, making "none on the new
   one" vacuous.
3. **The scratch clones' target dirs sit under the repo's `target/`** (`target/e5c-*`), not in the scratchpad. The
   scratchpad is on the quota'd `/tmp` tmpfs, 14 G free, and each clone build is several GB. The clones' sources are
   in the scratchpad as planned.
4. **Step 3's remove-the-guard was read by `cargo nextest` inside the scratch clone,** not `gate.py run
   --project-root`: the e2e entry would build the harness target inside the clone, on `/tmp`.
5. **The floor is 9 GiB** (the EDQUOT reading of about 8 GiB plus 1 GiB), not that reading plus the whole run's
   footprint. A check reads the backing with the run's own homes already in it, so the footprint on top would refuse
   the run's own later homes at today's headroom (`evidence/headroom.md`).
6. **Step order:** steps 11 and 12 were written before step 10(b)'s census. The census needs only the instrumented
   build of HEAD. The plan's order rules (10(a) before (b), and (b) before (c)) were kept.
7. No scope-record line: every changed file is in research's lists (`gate.py scope`: `clean — changed 13 · listed 13
   · recorded 0`).

## The guard's pass line (entry 15, `gate.py show … --n 15`)

```
guard: allowed 13 — .config/nextest.toml, Cargo.lock, crates/viola-e2e/Cargo.toml, crates/viola-e2e/src/harness/mod.rs, crates/viola-e2e/src/harness/run/mutants/scratch.rs, crates/viola-pty/src/lib.rs, scripts/profraw-census.sh, src/bin/viola-fake-agent.rs, tests/cli_instance_state.rs, tests/contract_fake_agent_drift.rs, tests/contract_ledger_probes.rs, tests/contract_lints.rs, tests/support/home.rs · companions excluded 0 · changed 13 · excluded 0
guard: clean
```

No file was excluded as a companion.

## Smoke

The plan's smoke entries ran as P2 gates, green: 17 cleanup, 18 boot (6.92 s, through the changed `ensure_e2e_home`),
19 status `ready`, and 22 cleanup with `processes_gone` and `endpoint_gone` true. G2 (20) and G4 (21) were green.

## Watches

- The truncated `.profraw`: `pre-push` (entry 23) ran green, 0 `header is corrupt` lines. So did the earlier `run
  --coverage` of this tree (2 980 passed, the merge clean). Its cause is item (8)'s finding.
- The macOS readiness stall: not run here (no macOS leg on this host). The operator pass's CI read sees `test
  (macos-latest)`.

## Matrix

No capability is claimed by this chunk (`matrix.py show --chunk`: `claimed by 2026-10-20-epoch-5-cleanup: 0`). No ref
was written.

## Process hygiene

`gate.py census --args projects/viola/target --args …/scratchpad/clone --args viola_pty-77dd4bdbf6cc1891 --args
profraw-census` → `census: none running` (trail `gate-2026-10-20-epoch-5-cleanup.json`).

| process | started by | final state |
|---|---|---|
| the harness supervisor and the booted wrapper (`p-e5c-smoke`) | this run (entries 18–22) | terminated (entry 22: `processes_gone` true) |
| the two census runs' test processes, wrappers, hooks and fake agents | this run | terminated (none running) |
| the planted-hang, remove-the-guard and `viola-pty` clone test runs | this run | terminated (none running) |

## Decisions & corrections

- The operator's direction for this run (2026-10-20, this session): «No mutation run, witness run or dispatch; read
  df -h /tmp before the stamped-home gates. Then the operator pass as usual, the ci.py read included; stop before the
  wrap.» `df -h /tmp` was read before every stamped-home firing: the censuses, the coverage run and the block's entry 1.
- Guard refusal: the Bash guard refused a `cd` into a scratch clone (it moves the session cwd). It was re-issued once
  as a subshell `( cd … && … )`. This is the recurrence of host-linux.md §Paths & the working directory.
- Step 7's premise disproof (deviation 1) is carried to the wrap as a finding, not resolved here.
