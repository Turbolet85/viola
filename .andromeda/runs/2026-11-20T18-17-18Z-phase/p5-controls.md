# P5 controls — 2026-11-20-harness-prefix-and-identity-floor-ledger-rows (card 1)

Fired as the entries themselves, `gate.py run --chunk 2026-11-20-harness-prefix-and-identity-floor-ledger-rows
--docs-root . --entry {n} --project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/42312720-4cbd-4984-bdf2-8623b59034d5/scratchpad/clone-p5`,
over a `git clone --shared` of the project at HEAD `b3eb184c` (role: the control clone). The inputs were written into
the clone alone by `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/42312720-4cbd-4984-bdf2-8623b59034d5/scratchpad/p5_inputs.py`
(`pass` / `fail`). It writes:
- a synthetic 29-row `target/live-rig/floor-stamp.json`;
- a record dir `target/live-rig/HF/record/2.1.328/` of empty files: the committed names minus
  `Mcp.tools-list-1.json`, plus the two new names, plus on `fail` one stray name;
- the evidence sha file: the live home stamp's real sha on `pass`, sixty-four zeros on `fail`.

| entry | unmodified clone | pass input (must-pass) | fail input |
|---|---|---|---|
| 23 kept stamp 29 ok | red · exit 1 (stamp absent), as on the tree | green · last line `29 ok` | red · last line `29 short` (identity-floor `fail`) |
| 24 env names (report-only) | — | recorded · exit 0 | — |
| 25 live stamp unmoved | red · exit 1 (sha file absent), as on the tree | green · `live-stamp: unmoved` | red · exit 1 (a moved sha) |
| 26 name diff atoms | red · `contains '< Env.recorded-1.json'` ✗, as on the tree | green · exit 1 with the three named lines | — |
| 27 name diff count 3 | red · last line `31`, as on the tree | green · last line `3` | red · last line `4` (one stray name) |

Logs: `$TMPDIR/andromeda-gate/2026-11-20-harness-prefix-and-identity-floor-ledger-rows/run-20261121T0600{17..34}Z/`.
The clone stays in the scratchpad: no recursive delete was run (`inputs#I1` item 10).
