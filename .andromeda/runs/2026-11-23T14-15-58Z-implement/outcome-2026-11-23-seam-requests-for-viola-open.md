# Outcome — 2026-11-23-seam-requests-for-viola-open (implement run 2026-11-23T14-15-58Z-implement)

**Outcome: green** — steps 1–19 built and green, R3 recorded, the three controls hold. Implement stops before R1 as the
plan and `inputs#I3` order. Next → the OPERATOR PASS: entry 15 (`pre-push`), entry 16 (hygiene), then entry 17 (the lock
probe), which is where this window's pass stops (`inputs#I3`); entries 18–22 (R1, its two readers, the push, the CI
read) wait for the operator's word, moment and workspace number. The wrap waits for R1.

## Files changed
- Modified (10): `crates/viola-hyprland/src/lib.rs` · `crates/viola-hyprland/src/reply.rs` ·
  `crates/viola-hyprland/src/read.rs` · `crates/viola-e2e/src/bin/viola-hypr-tap.rs` · `crates/viola-e2e/tests/hypr_tap.rs` ·
  `tests/contract_bounded_readers.rs` · `tests/contract_hyprland_seam.rs` · `tests/contract_fixture_hygiene.rs` ·
  `schemas/hyprland-fixture.v1.json` · `fuzz/fuzz_targets/hyprctl_json.rs`.
- New (2): `fixtures/hyprland/0.56.2/Monitors.recorded-1.json` (R3, written by the tap) · `fuzz/corpus/hyprctl_json/monitors-row`.
- Not touched: `crates/viola-hyprland/proptest-regressions/reply.txt` (the new property found no seed).
- Still unbuilt behind R1: `fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json`, and so the replay of the move
  through the fake, the replay contract's parse of its recorded argv, and entries 19–20 that read it.

## Deviations (each with its why)
1. Step 11 names no code for a terminal that fails to spawn: folded into its `no-terminal-window` (no new word, per
   `inputs#I3`'s stop on a new closed word).
2. Step 18 asks the fuzz target that «every kept monitor has its id»; the fuzz crate has no JSON reader (adding
   serde_json would move `fuzz/Cargo.lock`), and `id` is a required `i64` by type. Written instead: each kept monitor
   stood as an object holding an integer id, so `monitors.len() * 8 <= bytes.len()`.
3. `tests/contract_fixture_hygiene.rs`'s planted `unknown_request` case named the word `monitors`, which the schema enum
   now admits: re-pointed to `exec`, still outside the set (a listed file; research's sweeps keyed on `focus-window`).
4. `tests/contract_bounded_readers.rs`'s `hyprctl reply` row: its `types` text also names `Monitor` beside the witness
   move (its `needle` and `cap` unchanged).
5. Beyond the named cases, in listed files: `parse_args_takes_each_verb_form_and_refuses_every_other` (the tap's new
   argument form, a labelled table); `parse_routes_each_request_to_its_own_parser` and
   `every_parser_refuses_one_byte_past_the_cap_as_oversize` each gained the two words; the property strategy's `KEYS`
   gained `activeWorkspace` and `focused` (16 → 18); `Monitor` re-exported from the crate root; `hypr_tap.rs`'s stub
   writer split into a shared `planted` helper.
6. The tap's `move-refused`, `move-unseen` and `window-left` codes have no rehearsal case (the plan names two:
   the success and the no-window refusal).
Scope record: none — `gate.py scope` clean (changed 12 · listed 12 · recorded 0).

## Gates (this run's trail `.andromeda/runs/2026-11-23T14-15-58Z-implement/gate-2026-11-23-seam-requests-for-viola-open.json`)
- `cargo fmt --all --check` green · clippy green · `test -s …Monitors.recorded-1.json` green (after R3).
- `cargo nextest run -p viola-hyprland --no-fail-fast` green, 108/108 (one 5.01 s wait).
- ctl-hypr filter green, 23 selected / 23 passed · the tap binaries green · ctl-tap filter green, 5 selected / 5 passed.
- the three contract binaries green · `agent-run.sh run --fuzz-replay` green.
- hostwatch recorded (QUIET) · `run --unit` green · `run --integration` green · `run --e2e` green.
- R3 (entry 3, `--operator 3`, once): green, history unmoved — `.andromeda/cache/chunks/2026-11-23-seam-requests-for-viola-open/evidence/operator-3-r3.txt`.
- Controls: `drain-cap` holds · `deadline-5s` holds · `monitors-keep-none` holds — `evidence/remove-the-guard.md`.
- Not run by implement: entries 15–22 (the operator pass).
- Smoke: skipped — no boot-path touchpoint, no smoke or self-verify entry.
- No guard entry in the block. No deferral.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / nextest / harness trees of the gate entries | this run (gate.py) | terminated (each entry ended; census: none running) |
| `viola-hypr-tap record-monitors` and its two hyprctl reads (R3) | this run (`--operator 3`) | terminated (exit 0) |
| the control clone `~/.cache/andromeda-control/4dfb0612` and its builds | this run (gate.py control) | terminated · clone removed |
| planted stub terminals and stub hyprctl in `hypr_tap.rs` / `read.rs` cases | the tests | terminated — reaped, each case asserts its `/proc/<pid>` gone |
| a first scripted lib.rs edit (a `cat` waiting on stdin) | this run | terminated (TaskStop) |
Census (`gate.py census --comm hyprctl --comm foot --comm sleep --comm viola-hypr-tap --args viola-hypr-tap --args andromeda-control`):
10 running, none this run's — the host's standing foot sessions (overseers, builders) and a `sleep 20` / `sleep 60` of
other parents; no `viola.hypr-tap` window, no `sleep 30`.

## Decisions & corrections
- The stamp-ahead hook refused an estimated `ts` in a scratch evolve script (written about 11 minutes ahead of the
  clock; evolve.py refused it too, exit 3); re-stamped from `date -u` — a recurrence of the standing learning «a
  timestamp … is read from `date -u`».
- The Bash guard refused a heredoc with a file target (the hygiene edit script); re-issued once through the Write tool.
- Sweep hazard: widening a closed enum in a schema turns a planted «unknown word» case naming the new word valid
  (`tests/contract_fixture_hygiene.rs` `unknown_request`); a sweep keyed on a neighbouring word does not find it — sweep
  the new words themselves.
