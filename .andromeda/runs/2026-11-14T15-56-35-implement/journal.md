# Journal — implement 2026-11-14-budget-gate-removed

Long runs, each written here before its wait.

- 2026-11-14T16:16Z — P2 firing 1: `gate.py run --chunk 2026-11-14-budget-gate-removed --run-dir .andromeda/runs/2026-11-14T15-56-35-implement`, backgrounded, the whole block (31 entries; the three `operator` entries not fired). No deferral: Rust, the lockfiles, schemas and fixtures all changed. Verdict = the printed SUMMARY line.
- 2026-11-14T16:22Z — firing 1 read: green 23 · red 7 (3,16,18,19,20,21,31). Causes: clippy single-element loop (tests/cli_instance_state.rs); the new release case raced an unrecorded human_input (now a recorded pause); start-record counts one lower (hook_events, cli_links); the statusline marker file appends; cross_list_rows BAY literal; verify's statusline replay once. P2 firing 2: `--only 3,16,18,19,20,21,31`, backgrounded.
- 2026-11-14T16:25Z — firing 2 read: green 7 · red 0 (3,16,18,19,20,21,31). P2 firing 3: the whole block again, backgrounded.
- 2026-11-14T16:30Z — firing 3 read: entries 34 · green 30 · red 0 · recorded 1 · not-run 3 (the operator entries). Next: `gate.py control --controls-file control-1.toml` (release-budget, entry 16), backgrounded.
