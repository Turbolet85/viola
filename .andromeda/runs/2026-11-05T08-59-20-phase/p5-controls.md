# P5 controls — 2026-11-05-linux-only-code-and-ci

Each control fired AS THE ENTRY through `gate.py run --chunk 2026-11-05-linux-only-code-and-ci --docs-root . --entry {n}
--project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/199f0400-48b6-4808-8671-38cc66c1b5d0/scratchpad/ctl-clone`,
no `--run-dir`, over a `git clone --shared` of the project at HEAD `ecbe7ab36cb222294b804c933eb7732dfe1decd1`
(validation.md check 4 (9)).

## Entries 8, 9, 10, 11 — the removal's guards (predicted red on the untouched tree)

Unmodified clone (HEAD ecbe7ab36cb2), read as on the real tree:
- `8 probe red · exit 1 ✗ (exit 0) · exit 0 · 17478 B` (the carry's pattern: 274 lines)
- `9 probe red · exit 1 ✗ (exit 0) · exit 0 · 1632 B` (21 workflow lines)
- `10 probe red · exit 1 ✗ (exit 0) · exit 0 · 999 B` (11 manifest lines)
- `11 probe red · exit 0 ✗ (exit 1) · exit 1 · 22 B` (`stands: vendor/conpty`)

Must-pass input (the subject as the plan builds it, left uncommitted in the clone): every `.rs` file the carry's pattern
names `git rm`-ed; `.github/workflows`, `Cargo.toml` and the three crates' manifests holding a `cfg(windows)` table
`git rm`-ed; the thirteen named paths `git rm`-ed (79 paths changed):
- `8 probe green · exit 1 · 0 B`
- `9 probe green · exit 1 · 0 B`
- `10 probe green · exit 1 · 0 B`
- `11 probe green · exit 0 · 0 B`
