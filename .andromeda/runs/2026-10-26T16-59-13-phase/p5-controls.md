# P5 controls — 2026-10-26-home-and-code-bearing-file-integrity

## Entry 15 (the preservation guard) — known-positive controls
Fired as the entry, `gate.py run --plan {clone}/viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/plan.md
--entry 15 --project-root {clone}`, over a scratch clone (`git clone --shared` of the project at HEAD `99df45362ac6`,
the plan copied into its chunk folder) at `$TMPDIR/{scratchpad}/guard-clone`.

| control | input | line |
|---|---|---|
| A unmodified | none (the plan file only) | `15 probe green · exit 0` — `guard: clean` (the real tree read the same) |
| B allowed | `src/cmd/send.rs` and `crates/viola-state/src/strict.rs` edited, `tests/security_negatives_strict_modes.rs` minted (untracked) | `15 probe green · exit 0` |
| C outside | B plus `src/cmd/list.rs` edited and `src/new_file.rs` minted (untracked) | `15 probe red · last line 'guard: clean' ✗ (last: 'guard: outside 2 files')` |

## Entries 8 and 9 — the literal `contains tests run:`
Red at baseline (nextest exit 94, «operator didn't match any binary names»: the files steps 13 and 16 write). Must-pass
of the literal: entry 10, the same `cargo nextest run --workspace --features fake-agent -E 'binary(…)'` form differing by
the binary operand alone, read green with «43 tests run: 43 passed, 0 skipped» on the real tree at HEAD `99df45362ac6`.
