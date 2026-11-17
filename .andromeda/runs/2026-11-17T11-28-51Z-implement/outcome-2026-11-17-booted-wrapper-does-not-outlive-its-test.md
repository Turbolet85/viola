# Outcome — 2026-11-17-booted-wrapper-does-not-outlive-its-test

**Outcome:** green (P2 first firing green, 0 fix iterations) · smoke ✓ (by hand) · Next → the OPERATOR PASS (entries
10 pre-push · 11 hygiene · 12 push · 13 CI read · 14 cache-hit read), then `/andromeda-wrap-session`.

## Files changed
- New: `tests/support/tied.rs` · `tests/cross_killed_test.rs` · `crates/viola-e2e/tests/harness_tied.rs`
- Modified, listed: `tests/support/home.rs` · `tests/support/mod.rs` · `tests/support/piped.rs` · `tests/support/cli.rs` ·
  `tests/support/start.rs` · `tests/support/verify.rs` · `tests/cli_verify.rs` · `crates/viola-e2e/src/harness/boot.rs` ·
  `crates/viola-e2e/src/harness/session.rs` · `crates/viola-e2e/src/bin/viola-harness.rs` · `.config/nextest.toml`
- Derived `tests/*.rs` (21 files absorbed by the derived item, `gate.py scope`): each `Command::new(VIOLA)` →
  `support::tied::viola()`, and the imports the move left unused removed.
- Recorded companions (scope record rev 1): `crates/viola-e2e/src/harness/run/perf.rs` ·
  `crates/viola-e2e/tests/harness_lifecycle.rs` · `crates/viola-e2e/tests/cli.rs` (each a `BootOptions` literal
  gaining `tied: false`, serving step 5).

`gate.py scope`: `scope: clean — changed 38 · listed 35 · recorded 3 (companion 3 · mechanical 0 · in-intent 0 · widening 0) · absorbed 21 · excluded 225`

## Deviations (with justification)
- **The census pattern is wider than the plan's.** The plan's and research's sweep
  (`Command::new\((support::|home::)?VIOLA\)`, 44 hits in 24 files) missed a fourth spelling,
  `std::process::Command::new(support::home::VIOLA)` at `tests/cross_exit_causes.rs:461`. That site was tied too,
  since the file is inside the derived `tests/*.rs` item, so 45 sites moved. `untied_spawns` matches `Command::new(` followed by any path
  equal to or ending in `::VIOLA`, then `)`, in code only, so the census reads that spelling too. Its walk covers
  every `.rs` under `tests/` recursively, wider than the plan's `tests/*.rs` and `tests/support/*.rs`. The walk is
  marked `andromeda:walks-tree`.
- **The victim's report form.** Step 6(b) names «the shared `Watch` report form». `Watch` writes only under the OS temp
  dir and carries change notes, no pid lines. The victims instead write a `<subject> <pid> <start>` report at a path the
  witness names under its own test home, 0600, a sibling renamed into place. The harness victim's report adds a
  `session <id>` line.
- **`recorded_tree`'s type.** The plan states `Vec<(u32, String)>`; it returns `Vec<(u32, u64)>`, with the start time as
  sysinfo's number. `gone_within` takes a `subjects` map (`&[(&str, u32)]`) to name survivors by subject.
- **The witnesses' tree is the reported pids plus the census.** Each victim reports its wrapper's and agent's (and the
  supervisor's) pid with its start time. The kill witnesses assert the recorded tree holds each one, and the panic
  witness reads the reported set, since its victim's `Drop` may end the wrapper before a census.
- **The witnesses pre-stamp.** `Victim::start` calls `stamp_dir()` before spawning, so the victim's boot only copies the
  build's stamp and its report lands inside `WITHIN`.
- **The harness victim boots unstamped** (`SessionOptions::unstamped`). The tie does not depend on the stamp, and the
  boot stays inside `WITHIN`. The tied stamping `viola verify` (step 5) is therefore not witnessed by a kill, only built.
- **Not tied, found:** the in-test boots of `crates/viola-e2e/tests/harness_lifecycle.rs` (`options`) and
  `crates/viola-e2e/tests/cli.rs` (`booted`) build `BootOptions` literals, now with `tied: false`. The plan ties only
  `harness_session`, so those tests' supervisors would still outlive a SIGKILLed test. Reported for the wrap; no
  widening was made.
- **P3 smoke fired by hand.** The plan lists no smoke entry, and the CLI `boot` arm of the `viola-harness` binary was
  touched. `agent-run boot → status → cleanup` was driven by hand (evidence/smoke.txt).

## Gates (trail `.andromeda/runs/2026-11-17T11-28-51Z-implement/gate-2026-11-17-booted-wrapper-does-not-outlive-its-test.json`)
- `cargo fmt --all --check` — green · exit 0
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · exit 0
- `cargo nextest run -p viola-e2e --lib -E 'test(/boot_ties_its_supervisor_only_when_asked/)'` — green · exit 0
- `bash scripts/agent-run.sh run --integration --filter 'binary(cli_verify) & test(/stamped_home/)'` — green · exit 0, `"ok":true`
- `bash scripts/agent-run.sh run --e2e --filter 'binary(cross_killed_test)'` — green · `"ok":true`, 21 passed
- `bash scripts/agent-run.sh run --integration --filter 'binary(harness_tied)'` — green · `"ok":true`, 2 passed
- `bash scripts/agent-run.sh run --e2e --filter 'binary(contract_lints)'` — green · `"ok":true`
- `bash scripts/agent-run.sh run --integration` — green · `"ok":true`, 509 passed (baseline 505 + 4 new)
- `bash scripts/agent-run.sh run --e2e` — green · `"ok":true`, 369 passed (baseline 348 + 21 new)
- operator entries (pre-push · hygiene · push · ci.py conclusion · cache-hit read) — `not run — leg operator`: the operator pass's.
- No deferral: every gate ran.

## Controls (`gate.py control`, `--timeout 600`, evidence/control-1.txt)
- `control stamp-hollow · entry 4 · holds — in place green, removed red · exit 0 ✗ (exit 1) · 50.44s · 13.8s → 4.without-stamp-hollow.log`
- `control tied-pdeath · entry 5 · holds — in place green, removed red · exit 0 ✗ (exit 1) · 1.11s · 20.7s → 5.without-tied-pdeath.log`
- `control sup-tied · entry 6 · holds — in place green, removed red · exit 0 ✗ (exit 1) · 1.31s · 23.03s → 6.without-sup-tied.log`
- `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
- With the tie removed, the piped and direct witnesses read FAIL (survivors `["wrapper", "agent"]`). The outer-PTY and
  frozen ones read PASS: they end by hangup, as P3 measured. The supervisor's removal reads FAIL with survivors
  `["supervisor", "wrapper", "agent"]`. Every survivor was killed by `gone_within`'s failure path.

## Smoke
smoke ✓ — `agent-run boot` ok; `status` `ready` after `boot` exited (the untied CLI supervisor outlived `boot`);
`cleanup` `processes_gone:true, endpoint_gone:true, killed:[]` (evidence/smoke.txt).

## Matrix
`v2-37` planned → implemented, ref the six `tests/cross_killed_test.rs` cases and `crates/viola-e2e/tests/harness_tied.rs::killed_test_leaves_no_harness_session`.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| nextest test processes, victims (`killed_test_victim_entry`, `harness_victim_entry`), wrappers, fake agents, harness supervisors | this run (gates, controls in their clone, the local `cross_killed_test` read) | terminated |
| `smoke-tied` harness session (supervisor, 2 wrappers, 2 fake agents) | this run (P3 smoke) | terminated — `cleanup` `processes_gone:true` |

Census: `gate.py census --comm viola-fake-agent --comm viola-harness --args projects/viola/target/debug/viola --args projects/viola/target/harness/debug/viola --args killed_test_victim_entry --args harness_victim_entry` → `census: none running`.

## Decisions & corrections
- Directive at invocation (the operator, 2026-11-17): no live claude start; a red that moves a plan entry stops before
  the operator pass; the operator pass with the `ci.py` conclusion read (leg=operator); its pre-CI push must hit the
  saved rust-cache key as it stands, with no cache deleted, and the pass record says what that run read; every
  process started is ended; no scheduled wake or timer. Kept: no live start made, no wake set, every process ended (census).
- Sweep hazard: a grep for `Command::new\((support::|home::)?VIOLA\)` misses `Command::new(support::home::VIOLA)`
  (tests/cross_exit_causes.rs:461). The census matches any path ending `::VIOLA`.
- Guard refusal: the Bash guard refused `cd` into the skill's references dir (it moves the session cwd). It was
  re-issued as an absolute-path read.
