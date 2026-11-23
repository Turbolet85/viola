# Light gate, first firing — red recorded before any re-run

- Run: `gate.py run --chunk 2026-11-23-model-and-effort-at-launch --run-dir .andromeda/runs/2026-11-23T10-41-57Z-wrap` (trail `gate-2026-11-23-model-and-effort-at-launch.json`), logs under the gate's temp folder `wrap-2026-11-23T10-41-57Z/`.
- Summary: `entries 22 · green 12 · red 2 (14,15) · recorded 1 · timeout 0 · not-run 7`.
- Entry 14 `bash scripts/agent-run.sh run --integration`: `red · exit 0 ✗ (exit 1)` — nextest-integration 472 passed, 163 failed.
- Entry 15 `bash scripts/agent-run.sh run --e2e`: `red · exit 0 ✗ (exit 1)` — nextest-e2e 283 passed, 60 failed.
- First diagnostic line (every failing case read, the same): `panicked at tests/support/home.rs:139:17: e2e-home backing: 8831 MiB free, below the 9216 MiB floor` — the test-home headroom guard (`prepare_home_base`, `HEADROOM_FLOOR_MIB`), no assertion failed.
- Cause measured: `target/e2e-home` links into the `/tmp` tmpfs (32 G, ~11 G free at rest); the per-user Claude temp folder holds 20 GB of it (neighbours' sessions), and the block takes ~2.4 GiB while it runs (the overseer's measure, `inputs#I11`). hostwatch over 11:13:05Z..11:28:05Z read QUIET.
- Disposition: the overseer's word (`inputs#I11`): free headroom (a neighbour's dead build output, on the operator's word), then re-run the whole block once; nothing deleted and no entry started by this wrap meanwhile.
