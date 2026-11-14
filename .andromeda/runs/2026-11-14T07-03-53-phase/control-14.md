Check 4 (9) control for gate entry 14 (the seam's normal graph) — a command typed beside the entry that differs by its
input operand alone (`-p viola-mcp` for `-p viola-hyprland`), run as the tool types an entry:
`/usr/bin/bash -o pipefail -c "cargo tree -p viola-mcp -e normal --prefix none | grep -E '^(viola-mcp|tokio|viola-agent-claude|viola-state|viola-channel) v'"`
on the tree at HEAD 4255b3ed, 2026-11-14.

Reading: exit 0; lines `viola-mcp v0.1.0 (…)`, `tokio v1.53.3` (and `(*)` repeats), `viola-agent-claude v0.1.0 (…)`,
`viola-channel v0.1.0 (…)`, `viola-state v0.1.0 (…)`. Each literal the entry's atoms turn on (`<crate> v`, `tokio v`,
`viola-agent-claude v`, `viola-state v`, `viola-channel v`) is the form `cargo tree --prefix none` prints, so the
entry's `contains` and `lacks` atoms read the run's own output representation.

Baseline of entry 14 on the untouched tree: red — exit 1, `error: package ID specification viola-hyprland did not
match any packages` (the crate this chunk creates is absent).
Superseded form (rev 2): `grep -c … expect ['exit 1', 'last line 0']` read green vacuously on the absent package
(grep -c printed 0 over cargo tree's empty stdout) — re-spelt at P5.
