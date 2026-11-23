# Outcome — implement 2026-11-23-viola-open-2

**Outcome: green** — every non-leg entry of the block green but the report-only host read (recorded) and the gated
entry 16 (its `defer` standing, R2 unfired); the three remove-the-guard controls read `holds`. Next → the OPERATOR
PASS (entries 17-24, then the scratch dirs and the STOP for the overseer's first go; entries 25-28 on the goes), then
the wrap.

## Files changed
- New: `src/cmd/open.rs` · `src/run/window.rs` · `src/bin/viola-fake-agent/foot.rs` · `tests/cli_open.rs` ·
  `scripts/r2-open.sh`.
- Modified (listed): `Cargo.toml` · `crates/viola-core/src/lib.rs` · `crates/viola-state/Cargo.toml` ·
  `crates/viola-state/src/{snapshot,replay,board,liveness}.rs` · `src/cmd/{mod,run}.rs` · `src/run/mod.rs` ·
  `src/human/hints.rs` · `src/bin/viola-fake-agent.rs` · `src/bin/viola-fake-agent/hyprctl.rs` ·
  `crates/viola-e2e/tests/hypr_tap.rs` · `schemas/diag-line.v1.json` · `tests/{cli_output_plain,cross_exit_causes,
  contract_error_bodies,contract_diag_schema}.rs`. Listed and NOT edited: `crates/viola-core/src/obs.rs` (below).
- Recorded (scope record, 15 lines): mechanical `Cargo.lock`; companion `crates/viola-state/src/strict.rs`,
  `crates/viola-state/tests/state_replay.rs`, `crates/viola-mcp/tests/handlers.rs`, `src/cmd/revive.rs`,
  `src/cmd/hook.rs`, `src/cmd/list.rs`, `src/human.rs`, `src/human/board/test_support.rs`, `src/run/dialog.rs`,
  `src/run/wheel.rs`, `src/run/send/slot.rs`, `tests/hook_fail_open.rs`,
  `tests/security_negatives_server_identity.rs`; in-intent `src/bin/viola-fake-agent/test_cases_modes.rs` (step 19).
  `gate.py scope`: `scope: clean — changed 39 · listed 24 · recorded 15 (companion 13 · mechanical 1 · in-intent 1 ·
  widening 0) · absorbed 0 · excluded 409`.

## Deviations, with their justification
1. Plan step 3 (`crates/viola-core/src/obs.rs`: the subject `terminal`, the details `terminal-not-found` / `not-live`)
   has no target: obs.rs holds no subject or detail list — the literals stand at their call sites and in
   `schemas/diag-line.v1.json`. The subject and details went into the schema alone (step 24); obs.rs is unedited.
2. `open` re-canonicalises its home after its `cli` log created it (`super::canonical_home(&client::start(…)?)`), as
   `run` does: first-run measurement (`open_spawns_the_terminal_with_the_closed_argv` red) showed the argv's `--home`
   and pinned path in the pre-creation spelling through the `target/e2e-home` link while the wrapper recorded the
   canonical one.
3. `viola_state::replay::Recovered::Replayed`'s `state` is now `Box<Replayed>` (clippy `large_enum_variant` once
   `Replayed` gained `window`; clippy's own suggestion). Ten constructor sites moved (`box_state.py`'s printed
   counts: `replay.rs` 2 + its one shorthand by hand, `tests/state_replay.rs` 6, `src/cmd/revive.rs` 1); field
   readers unchanged.
4. `scripts/r2-open.sh`: the open is handed `--cwd $XDG_RUNTIME_DIR/viola-r2/work` (a scratch dir it makes) so the
   ONE claude start never begins in the repository; it needs `jq` (checked first, a red `first=no-jq` without it);
   a red verdict line appends ` first=<reading>` after the plan's four fields (the green line is the plan's exact
   form, which entry 16 greps). Leaves one more scratch item than the plan names: `$XDG_RUNTIME_DIR/viola-r2/work/`.
5. `tests/cli_open.rs` runs every case under a seeded test-owned user home (`seed_user_home`, `--trust-config` in
   the fake's `fake-args`) per testing.md's rule for every test-started `viola run`, beyond the plan's «own home».
6. `tests/contract_error_bodies.rs`: open's `pinned-hash-mismatch` row is witnessed by run's
   `a_tampered_pinned_copy_is_pinned_hash_mismatch` (the shared `pin_bytes` refusal); no open-specific tampered-pin
   case was written. The `NOT_ERROR_BODIES` row of `HyprlandError` had «no verb calls the seam yet»; its why now
   reads that callers keep only the word, as a placement word. Two rows name open's `OPEN_PROGRAM` line and its ok
   document as no error body.
7. `open_args_refuse_a_word_after_dashes_other_than_claude` carries a sixth case beyond the plan's five,
   `claude claude`.

## Gates (the block, `gate.py run`, trail `.andromeda/runs/2026-11-23T23-08-26Z-implement/gate-2026-11-23-viola-open-2.json`)
- Firing 1 (`--only 1-12`): green 10 · red 1 (`cargo clippy …` — `large_enum_variant`) · recorded 1.
- Firing 2 (`--only 1,2,4,5,6,7,8,9`) after the box: green 8 · red 0.
- Host read before the layers (directive): `verdict QUIET - 5 s stalled on IO`, `df -m /tmp` 31975 MiB free
  (2026-11-23T23:27:00Z).
- Firing 3 (`--only 13,14,15`): green 3 · red 0 (`run --unit` 28.92 s · `run --integration` 70.91 s · `run --e2e`
  12.15 s).
- Entry 16 (`test -s …/evidence/r2.md && grep -c '^r2: green placement=placed ' …`): `not run — defer` — gated on R2.
- Entries 17-28: `leg operator`, the operator pass's.
- Controls (`gate.py control --timeout 900`, `evidence/control-1.txt`): `control: 3 holds · 0 timeout · 0 does not
  hold · 0 unreproduced · clone removed`.
- Smoke: the plan's smoke entries are the operator pass's (17-19); P3 drove none.
- Matrix: v2-08 `implemented` (four test refs + `by-construction: evidence/v2-08-by-construction.md`); v2-07 stays
  claimed gated, no ref.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| the stand-in `foot`, its `viola run` wrapper and fake `claude`, per `tests/cli_open.rs` case | this run (test processes) | terminated — each case's `Terminal` guard SIGKILLs the stand-in's recorded tree and `gone_within` waits it gone |
| the stub terminals of `crates/viola-e2e/tests/hypr_tap.rs` | this run (test processes) | terminated by the tap itself, `terminal_gone` asserted |
| the control clone's builds and tests | this run (`gate.py control`) | terminated — clone removed |

`gate.py census --args viola-e2e-home --args target/e2e-home --args andromeda-control --args
projects/viola/target/debug/viola`: `census: none running`.

## Decisions & corrections
- Guard refusals, each re-issued once in the passing form: a `cat` heredoc with a file target (→ Write-tool script
  run by path); a `cd` out of the project root, twice (→ the Read tool; an absolute-path grep).
- Recurrence of a standing learning (testing.md 2026-11-18 / test data): a test-started `viola run` under a seeded
  test-owned user home — applied to the new `tests/cli_open.rs` from its first run.
- Finding: a verb that opens its log on a home it resolved before the home existed holds the pre-creation spelling;
  `run` re-canonicalises after its log, and `open` now does too (deviation 2).
