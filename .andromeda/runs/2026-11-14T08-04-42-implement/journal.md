# Journal — implement 2026-11-14-hyprland-seam

Operator directive for this run (relayed in the invocation): no live `claude` start; L1 and L2 NOT granted —
build steps 1-9 and 12-13 green on synthetic replies, stop at step 10 and name what waits; the controls file
carries a `case` per control as plan step 14 names it; journal each long run here before a wait; after the
recordings, the operator pass with the ci.py conclusion read (leg=operator).

## 2026-11-14T08:15:24Z — full block, firing 1
- About to fire: `gate.py run --chunk 2026-11-14-hyprland-seam --run-dir .andromeda/runs/2026-11-14T08-04-42-implement`,
  backgrounded (pre-push carries a 7200 s timeout).
- Tree: steps 1-9, 12, 13 written; step 11 (`tests/contract_hyprland_seam.rs`) not written — it waits on L1/L2.
- Reds expected by design before L1: `contract_fixture_hygiene`'s `hyprland_fixtures_pass_hygiene` (empty walk),
  and so the entries that run it — the hygiene entry, `run --integration`, `pre-push`; the
  `contract_hyprland_seam` entry (no test target yet). The three operator entries (L1, locked read, L2) and the
  three seeded operator entries are not fired by `run`.
- Pre-checks already read in this session: `cargo nextest run -p viola-hyprland --no-fail-fast` 81/81;
  `-p viola-e2e --bin viola-hypr-tap` 7/7; contract_fixture_hygiene 118/119 (the empty walk); the censuses
  and schema 93/93; fuzz workspace `cargo +nightly-2026-09-20 check --offline --bins` exit 0;
  `cargo tree --manifest-path fuzz/Cargo.toml --locked … -i veil` exit 0.
- Read: `entries 38 · green 24 · red 7 (15,17,18,20,21,23,35) · recorded 1 · timeout 0 · not-run 6`.
  17 no test target (step 11 unwritten); 18 and 23 the empty walk alone; 15, 20, 21 one proptest red
  (`version_prop_parse_never_panics_and_reads_back_its_tag`: tag `v00.0.0` reads version 0.0.0 but prints
  `v0.0.0` — the property compared text, fixed to compare versions; case `leading_zeros` pinned; the
  regression seed proptest wrote is kept); 35 the empty walk plus that proptest.

## 2026-11-14T08:22:40Z — re-runs and the controls
- `--only 6,7,15,20,21`: all green. `--entry 35` (pre-push): red on `hyprland_fixtures_pass_hygiene` alone
  (3615 run, 3614 passed).
- About to fire `gate.py control --controls-file …/control-1.toml`, backgrounded: four controls on entries 15
  (oversize, deadline-kill), 16 (title-scrub), 18 (hygiene-title). hygiene-title expected UNREPRODUCED before L1
  (entry 18 in place red on the empty walk); it is re-fired once the recorded set stands.
- Read: `control: 3 holds · 0 timeout · 0 does not hold · 1 unreproduced · clone removed` (after one exit-2
  refusal of the `<binary-id> <name>` case form). Evidence `evidence/control-1.txt`.

## 2026-11-14T08:36:44Z — L1 and the block, firing 2
- Relay `relay-1.md` (inputs#I5): the operator's word for L1, «Да, L1 разрешаю», 08:35Z; L2 NOT granted (host
  locked, `hyprctl locked` true at 08:35Z, the operator away). Entries 4-5 are not fired.
- `--operator 3`: `3 probe green · exit 0 · 0.38s · 0 B → 3.log · history: unmoved`. Recorded
  `fixtures/hyprland/0.56.2/`: ActiveWindow, Clients, Version, Workspaces `.recorded-1.json`,
  `Version.absent-1.json` (exit 1, the placeholder). Every stdout string read: an address, `v0.56.2` or the
  placeholder; numbers only otherwise.
- Step 11 written over that set (`tests/contract_hyprland_seam.rs`, `[[test]]` added): 5/5 alone.
- About to fire the whole block again (operator entries excluded by the tool), backgrounded.
- Read: `entries 38 · green 31 · red 0 · recorded 1 · timeout 0 · not-run 6`; controls firing 2:
  `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.

## 2026-11-14T09:07:29Z — L2, the argv correction, the block, firing 3
- Relay `relay-2.md` (inputs#I6): L2 granted, the session unlocked (`hyprctl locked` false 09:04:53Z), not to be
  locked; then the block, the controls and the operator pass with the ci.py read.
- Entry 4 green (`false`); entry 5 green, but both dispatches recorded exit 7 and the placeholder.
  `hyprctl dispatch workspace 6` read by hand: 0.56.2 runs dispatch as Lua `hl.dispatch(<args>)`.
  Corrected argv measured `ok\n` exit 0 for both (`hl.dsp.focus({ workspace = 6 })`,
  `hl.dsp.focus({ window = "address:0x555a9c521790" })`); `Request::args()` corrected, its pins moved;
  333/333 for viola-hyprland and viola-e2e. Entries 4, 5 fired again: both dispatch files exit 0, `ok\n`.
  Evidence `evidence/operator-4-5-L2.txt`.
- About to fire the whole block (firing 3), backgrounded, then the controls file (firing 3).
