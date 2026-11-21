# P5 controls — 2026-11-21-mcp-calls-bounded

- Entry 11 (`bash scripts/agent-run.sh run --integration --filter 'test(/rig_rehearsal_mb/)'`), its known-positive
  control: the same command typed with the filter's pattern alone changed to an existing rehearsal,
  `/usr/bin/bash -o pipefail -c "bash scripts/agent-run.sh run --integration --filter 'test(/rig_rehearsal_mt_writes/)'"`,
  on the working tree at HEAD `53669b68` (no source edit yet): exit 0, `Starting 1 test across 38 binaries (599 tests
  and 43 binaries skipped)`, `Summary … 1 test run: 1 passed, 599 skipped`, `"ok":true`. The integration layer holds
  `rig_rehearsal`'s binary, so the entry's filter selects the `MB` rehearsals once they exist; its own baseline (0 of
  600 selected, `nextest-exit-4`) is vacuous, its control implement's red-before-green run.
- Entry 6 (the `call-deadline` control entry): baseline exit 4, 0 selected — vacuous until step 7's rstest exists; its
  control is the `call-deadline` one-shot `gate.py control` (Test Commands prose).
- Entries 8, 9 (`scripts/channel-tokio-free.sh`, `scripts/rmcp-features.sh`): green on the untouched tree; their
  subjects (viola-channel's tokio-free graph, rmcp's feature set) are not what this chunk builds — it adds tokio's
  `time` feature to viola-mcp alone — and each ban's firing is proven by `scripts/deny-probes.sh` (security-plan ·
  §Dependencies), so no new control is owed here.
