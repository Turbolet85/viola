# Outcome — 2026-11-17-supply-chain-green-and-booted-tests-end

**Outcome:** green (2 fix iterations) · smoke skipped — no product boot-path change · Next → the OPERATOR PASS
(entries 12 pre-push, 13 hygiene, 14 push, 15 CI read), then `/andromeda-wrap-session`.

## Files changed
- `deny.toml` — RUSTSEC-2026-0543 ignore entry with its reason; the comment above the list states both grounds.
- `crates/viola-e2e/src/harness/boot.rs` — `tie_to_caller(cmd, signal)`; `stamp` passes `SIGTERM`, `spawn_supervisor`
  `SIGKILL`; three doc comments say which child takes which signal and why.
- `crates/viola-e2e/tests/harness_lifecycle.rs` — `options()` `tied: true`.
- `crates/viola-e2e/tests/cli.rs` — `booted()` `tied: true`.
- `crates/viola-e2e/tests/harness_tied.rs` — `harness_stamped_victim_entry`, `killed_test_leaves_no_stamping_verify`,
  `viola_e2e_census_finds_no_untied_boot` (+ helpers); `spawn_victim` takes its entry name.
- `tests/support/source_scan.rs` — its rstest case table ported to a labelled case table (scope record, companion).

## Deviations, with justifications
- Scope record: `- tests/support/source_scan.rs · companion · serves step 6 · self`. The shared masker held an rstest
  table; viola-e2e has no rstest, so taking the file by `#[path]` did not compile. The table (14 labelled cases,
  unchanged inputs and outputs) became one `#[test]` over a labelled table — one masker, no copy (testing.md
  2026-11-13), no new dev-dependency. The four root census binaries that take it ran green (53 passed, `run --e2e`
  over contract_bounded_readers | contract_error_bodies | contract_os_gated_tests | cross_killed_test).
- Step 5's wait: «a verify … which has at least one child of its own» let the kill land on verify's `--version` probe
  child before any run dir existed (first green in 0.38 s, the dir read vacuous). The wait now needs a verify child
  whose cwd sits in a `viola-verify-` dir, and the case asserts at least one of the owned dirs stood at the kill.
- Step 5's reads + the `vfy-tie` control: the plan said vfy-tie holds «on its process read». Measured false
  (control-1: DOES NOT HOLD): an untied verify orphaned by the kill swallows its dead-pipe writes (`src/human.rs`
  `result`, `let _ = write_result(…)`), runs to its end, removes its own dirs and is gone within WITHIN. The witness
  also reads `<home>/ledger/stamps.json` absent (a stopped verify returns `Interrupted` before `update_stamps`);
  control-2: vfy-tie holds on that read.
- Red-before-green: the census ran red before steps 3–4 («untied in-test boots in [crates/viola-e2e/tests/cli.rs,
  crates/viola-e2e/tests/harness_lifecycle.rs]»); step 2's red is the vfy-term control (step 2 was written before
  step 5's witness).

## Gates (implement run 2026-11-17T13-38-25Z-implement, second full block after the witness change)
1 fmt green · 2 clippy green · 3 `cargo deny check` green (`advisories ok, bans ok, licenses ok, sources ok`) ·
4 fuzz audit green (`advisories ok, sources ok`) · 5 tomllib green (`RUSTSEC-2026-0457 RUSTSEC-2026-0461
RUSTSEC-2026-0462 RUSTSEC-2026-0543`) · 6 deny-probes green (`deny-probes: 14/14 banned, control clean`) · 7 viola-e2e
lib green · 8 harness_tied green (6 passed) · 9 harness_lifecycle | cli green · 10 mcp_tools | mcp_recorded_exchange
green · 11 contract_lints green · 12–15 not run — leg operator.
Summary line: `entries 15 · green 11 · red 0 · recorded 0 · timeout 0 · not-run 4`. No guard entry. No deferral.

## Controls (`--timeout 600`, entry 8)
control-2: `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` — vfy-term on the dir
read, vfy-tie on the stamp read, e2e-census on the census (evidence/control-2.txt; control-1's DOES NOT HOLD in
evidence/control-1.txt).

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate block entries 1–11 (×2), control clones (×2), direct `cargo nextest` runs of harness_tied (×8) and the root census run | this run | terminated (each exited; census below lists none of this tree) |
| `viola` ×7 listed by `gate.py census` (`~/.local/viola-product`, `viola-lab/prototype`, `~/.viola/bin/0.1.0-…` mcp, a `viola hook`) | NOT DERIVED — other trees and this session's own MCP bridge | left running: not this run's |
Census: `census: 7 running`, none a `projects/viola/target` binary. The two vfy-term firings' Run A dirs
(`$TMPDIR/viola-verify-dGaQwu`, `$TMPDIR/viola-verify-Nhl7wD`, empty) removed by rmdir.

## Decisions & corrections
- Bash guard refusals: two `cd`-led calls and one cat-heredoc to a file; each re-issued in the passing form.
- Sweep hazard: the shared masker `tests/support/source_scan.rs` cannot be taken by a crate without rstest while its
  cases are an rstest table — now a labelled table.
- A kill witness whose only proof is «processes gone and dirs gone» cannot tell a tied verify from an untied one: the
  untied one finishes by itself inside the bound (dead-pipe writes swallowed). The stamp's absence is the separating
  read.
