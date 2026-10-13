
## 2026-10-13-epoch-4-cleanup — the board units re-cited after the human split
**Section:** §4 ARIA Patterns & Roles (P1 cli mirror); §8 Cognitive Accessibility (the pinned strings)
**Change:** `board_row_caption_and_default_row_are_the_literals`, `bay_line_*`, `board_row_*` and `instance_and_home_state_pairs_are_the_fixed_texts` sit in `src/human/board.rs`; was `src/human.rs`.
**Why:** the pure-move split of `src/human.rs` by phase; every test kept its name.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/
