# P5 controls — 2026-10-27-bounded-inputs-at-every-boundary

Every control fired as its entry, `gate.py run --plan viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/plan.md
--entry {n} --project-root {clone}`, over a shared clone of HEAD `4b0bf776d33f6e862d4a3e2b8d5ef0413ef7a930` at
`$TMPDIR/{scratchpad}/clone-p5` (no `--run-dir`). The entry numbers are the plan's after the E2E split (27 entries).

## Entry 14 — `bash scripts/deny-probes.sh` (the literal `last line deny-probes: 14/14 banned, control clean`)
- clone unmodified: red · last line `deny-probes: 13/13 banned, control clean` — the same reading as the tree (P5 baseline)
- clone with step 6 planted (the `deny.toml` `[[bans.features]]` table for serde_json `unbounded_depth`, and the probe
  row `serde_json-unbounded-depth` after `tracing-subscriber-env-filter`, by `$TMPDIR/{scratchpad}/plant_depth_ban.py`):
  green · exit 0 · last line `deny-probes: 14/14 banned, control clean` — the must-pass reading of the literal
- the plant restored (`git checkout -- deny.toml scripts/deny-probes.sh`) before the guard controls

## Entry 13 — `cargo deny check`, with the same ban planted
- clone with the ban: green · exit 0 · `advisories ok, bans ok, licenses ok, sources ok` — no package of the real graph
  enables `unbounded_depth`

## Entry 17 — the preservation guard (base `4b0bf776`)
- clone unmodified (the chunk's plan, research and scope copied in, untracked under `viola-0.1.0/`): green ·
  `guard: clean` (allowed 0, changed 0)
- two allowed files changed (`crates/viola-mcp/src/lib.rs` appended, `tests/contract_bounded_readers.rs` minted):
  green · `guard: allowed 2 — crates/viola-mcp/src/lib.rs, tests/contract_bounded_readers.rs` · `guard: clean`
- three paths outside the lists added on top (`src/cmd/mcp.rs` appended, `.config/nextest.toml` appended, an
  untracked `crates/viola-mcp/src/new_file.rs`): red · `guard: outside 3 files`
