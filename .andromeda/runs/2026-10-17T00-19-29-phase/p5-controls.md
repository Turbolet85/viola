# P5 controls — 2026-10-17-end-to-end-path-binary-and-harness-legs

Every control fired as the entry, through `gate.py run --plan <clone plan> --entry 8 --project-root <clone>` (no
`--run-dir`), over a `git clone --shared` of the project at HEAD `6042af635c3e` in the session scratchpad
(`$TMPDIR/<session scratchpad>/clone8`).
The plan was copied into the clone at its chunk path.

## Entry 8 — the no-product-change guard
- clone, unmodified (HEAD `6042af635c3e`): **green**, exit 0, the same reading as the real tree's baseline.
- clone, `src/main.rs` with one appended comment line, left uncommitted (a modified product file): **red**, exit 1.
- clone, an untracked empty `fixtures/control-new.json` (a new file in a guarded tree; `src/main.rs` restored first):
  **red**, exit 1.
- The control inputs lived only in the clone and were reverted there.

## Entry 12 — the empty-intersection atom
- The atom `contains nextest-exit-4` is spelt from the harness's own empty-selection output, read at entry 7's
  baseline on the real tree: `{"suite":"nextest-unit","passed":0,"failed":1,…,"failures":["nextest-exit-4"]}`.
- At baseline entry 12 reads `artifact-missing` instead: `binary(path1_run_start)` matches no binary, so nextest fails
  to parse the filterset and writes no JUnit.
