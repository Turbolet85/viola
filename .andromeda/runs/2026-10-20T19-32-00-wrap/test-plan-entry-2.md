
## 2026-10-20-epoch-5-cleanup-ii — the moved units re-cited and the orphans probe line
**Section:** §5 Integration Test Strategy (the `tests/cli_list.rs` board bullet; the `tests/cli_verify.rs` bullet) · §9 CI Integration (Lint row; the Lint errors bullet) · §10 Quality Gates & Coverage Targets (Mutation gate: the Epoch 4 `typed.rs` dispositions)
**Change:**
- The board units are the `src/human/board/` units (was `src/human/board.rs`, twice); the stop's unit cases sit in the `src/cmd/verify/typed/` module (was `typed.rs`); the Epoch 4 dispositions name `typed.rs`'s code under `src/cmd/verify/typed/`, "inside that module" and "a case in its unit tests" (was "inside `typed.rs`", "`typed.rs`'s `mod tests`").
- The orphans probe fires on each of its two planted orphans, last line `orphans-check probes: 2/2 fired, control clean`, and the Lint errors trigger reads `2/2 fired`. Was: `1/1 fired`.
**Why:** the size split moved code with every case's name kept. The probe line was a stale literal from before this chunk: the script already held two planted cases, and its log reads 2/2.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/
