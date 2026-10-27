# P5 controls — 2026-10-27-pinned-copy-race-and-reproducible-builds

Every control fired as the entry, `gate.py run --plan … --entry {n} --project-root {clone}` with no `--run-dir`, over a
scratch clone (`git clone --shared` of the project at HEAD `99977116182b`, `$TMPDIR/{scratchpad}/guard-clone`), the
plan copied into the clone's chunk folder.

## Entry 14 — the preservation guard
| clone role | input | reading |
|---|---|---|
| unmodified | none | `14 probe green · exit 0`, `guard: allowed 0 · companions excluded 0 · changed 0 · excluded 0` / `guard: clean` |
| must pass | a comment appended to `crates/viola-state/src/pin.rs` and to `tests/path1_run_start.rs` (both on the allow list) | `14 probe green · exit 0`, `guard: allowed 2 — crates/viola-state/src/pin.rs, tests/path1_run_start.rs · companions excluded 0 · changed 2 · excluded 0` / `guard: clean` |
| must fail | beside the two above, a comment appended to `src/cmd/run.rs` and to `.config/nextest.toml`, and an untracked `src/new_file.rs` | `14 probe red · last line 'guard: clean' ✗ (last: 'guard: outside 3 files') · exit 0` |

On the real tree (P5 baseline, HEAD `99977116182b`): green, `guard: clean` — vacuous by itself; the pair above is its control.
