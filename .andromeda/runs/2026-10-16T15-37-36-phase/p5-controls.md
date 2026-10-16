# P5 controls — 2026-10-16-remove-the-browser-leg

Every control fired as its entry through `gate.py run --plan … --entry {n} --project-root {clone}` (no run dir),
over the scratch clone `$TMPDIR/<the session scratchpad>/clone-a`
(`git clone --shared` of the project at HEAD `b6bbbd68428a`), unless marked typed.

## Gate 14 — `bash scripts/deny-probes.sh`, `contains deny-probes: 12/12 banned, control clean`
- clone, unmodified: red — `deny-probes: 13/13 banned, control clean` (the same reading as the real tree's baseline).
- clone, the `axum-http2` row removed from `scripts/deny-probes.sh`: green — 4.76 s (12/12, the must-pass reading).

## Gate 11 — the product preservation guard (base `b6bbbd68428a`)
- clone, unmodified: green (empty change).
- clone, `src/cmd/client.rs` edited: red (exit 1).
- clone, `src/obs.rs` edited (a file this chunk touches): green.
- clone, an untracked `src/new_untracked.rs`: red (exit 1).
- clone, `crates/viola-state/src/lib.rs` edited: red (exit 1).

## Gate 8 — `gate --require playwright`, `contains "detail":"unknown-suite"`
- typed control, operand only (`playwright` → `bogus`), run as `/usr/bin/bash -o pipefail -c 'bash scripts/agent-run.sh gate --require bogus'`
  on the real tree: exit 2, `{"v":1,"cmd":"gate","ok":false,"reason":"usage","detail":"unknown-suite"}` — the literal
  is spelt from this output.

## Gates 18 and 21 — `lacks api_sessions_equal_list` / `lacks port_free`
- The tokens are spelt from the entries' own baseline output (`"api_sessions_equal_list":null`, `"port_free":null`).

## Gates 9 and 10 — the absence guards
- Read red at the base on the real tree; every hit (gate 9: 145 lines in 14 files; gate 10: 74 lines in 17 files, the
  gate logs `9.log`, `10.log` under `$TMPDIR/andromeda-gate/2026-10-16-remove-the-browser-leg/phase-2026-10-16T15-37-36/`)
  read and found to be a line steps 1-15 remove or reword; the `git grep` reads the raw files, the representation its
  pattern was written from.
