# P5 controls — the preservation guard (entry 15)

Fired as the entry through `gate.py run --entry 15 --project-root {clone}` by `p5-controls.sh` (this run dir), over a
`git clone --shared` of the project at HEAD `8fc3d1df12f9` holding the chunk folder, in the script's own `$(mktemp -d)`,
on 2026-10-26 at 14:49Z (validation.md check 4 (9)).

| clone input | the entry's line | reading |
|---|---|---|
| unmodified | ` 15 probe       green · exit 0 · 0.15s · 316 B → 15.log` | `guard: clean`, as on the real tree |
| `src/run/links.rs` and `.config/nextest.toml` changed, `src/cmd/link.rs` and `tests/channel_links.rs` minted (must pass) | ` 15 probe       green · exit 0 · 0.15s · 400 B → 15.log` | passes: every changed or new file is on the plan lists |
| `src/cmd/send.rs` edited and an untracked `src/new_file.rs` (must fail) | ` 15 probe       red · last line 'guard: clean' ✗ (last: 'guard: outside 2 files') · exit 0 · 0.15s · 458 B → 15.log` | fails: two files outside the lists, the untracked one among them |
