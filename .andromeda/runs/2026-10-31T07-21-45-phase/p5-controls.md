# P5 controls — 2026-10-31-sanitised-error-surfaces

The preservation guard, gate entry 15, fired as the entry over a scratch clone of HEAD `7bf5c32add0e`, by
`p5-guard-controls.sh` in this run dir (the clone in the script's own `$(mktemp -d)`, removed by its `trap`).

| Clone's input | Reading |
|---|---|
| unmodified | green, `guard: clean` |
| two allowed files changed (`src/cmd/client.rs`, `crates/viola-mcp/src/result.rs`) | green |
| three paths outside the lists (`src/cmd/mcp.rs`, `.config/nextest.toml.x`, an untracked `crates/viola-mcp/src/new_file.rs`) | red, `guard: outside 3 files` |

Host preparation before the baseline runs: `target/` and `target/e2e-home`'s link were gone since the clean of 2026-10-27,
and `/tmp/viola-e2e-home-<uid>` with them. The link was restored to the operator-set form, the target `mkdir -m 700` first,
`df -h /tmp` reading tmpfs 32G, 8.5G used, 23G avail.
