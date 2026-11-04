# P5 controls — 2026-11-04-linux-and-macos-parity

Every control fired as its entry, `gate.py run --chunk 2026-11-04-linux-and-macos-parity --docs-root . --project-root
$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/ffec5821-da16-4051-8a6b-8f70581744bb/scratchpad/p5-clone --entry {n}`,
no `--run-dir`, over a `git clone --shared` of the project at HEAD `fdb28e90d1cafab9025c136f75a15a418557b5a2` (role: scratch
clone, P5 controls), 2026-11-04 ~20:40Z.

## Entry 12 — the root waits' count (`Instant::now() + WITHIN` under `tests/`, expected 32)
- the clone unmodified: `green · exit 0` — reads what the real tree read.
- the clone with one line `// let deadline = Instant::now() + WITHIN;` appended to `tests/run_cli.rs` (33 hits):
  `red · exit 0 ✗ (exit 1) · exit 1` — the input it must fail on.

## Entry 13 — the preservation guard (base `fdb28e90…`, the plan's allow set)
- the clone unmodified: `green · exit 0`, `guard: clean` — reads what the real tree read.
- the clone with `tests/run_cli.rs` changed (a path the allow set names): `green · exit 0` — the input it must pass.
- the clone with `src/main.rs` changed besides (a path outside the allow set): `red · last line 'guard: clean' ✗ (last:
  'guard: outside 1 files')` — the input it must fail on.

## Entries 10 and 11 — must-pass readings, typed one input operand apart (the real tree, HEAD `fdb28e90…`)
Typed as the gate tool types an entry, `bash -o pipefail -c '…'` from the project root, by the script
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/ffec5821-da16-4051-8a6b-8f70581744bb/scratchpad/p5_ctl.sh`; the
clone pair cannot hold: each entry's subject is a binary or a case this chunk creates.
- entry 10 with the filter `binary(cross_list_rows) | binary(cross_exit_causes) | binary(contract_bounded_readers)` (the
  entry's own less the binary step 7 creates): exit 0, `58 tests run: 58 passed, 0 skipped`, the harness document
  `"ok":true` — the entry's literal `"ok":true` read from the run's own output.
- entry 11 with the two existing stale-wrapper cases `run_refuses_a_stale_name` and
  `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` as its operands: exit 0, `2 tests run: 2 passed (2 slow),
  3444 skipped` — the entry's literal `2 tests run: 2 passed` read from the run's own output; both cases ran past the
  `mutants` profile's 5 s period and inside its 10 s kill.
