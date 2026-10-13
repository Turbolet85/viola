
## 2026-10-13-epoch-4-cleanup — moved unit tests re-cited after the send and human split
**Section:** §4 Unit Test Strategy (root bin: the budget governor and the send-confirmation matcher bullets); §5 Integration Test Strategy (the `viola list` board)
**Change:** `send_budget_paused_*` sit in `src/run/send/budget.rs`; of the six send-module `trailing_newline` tests three sit in `src/run/send/classify.rs`, two in `src/run/send/relabel.rs` and one in `src/run/send.rs`; the two `inner_cr_is_one_lf` tests in `src/run/send/relabel.rs`; the eight `send_local_command_` functions in `src/run/send/classify.rs`; the board units `bay_line_*` and `board_row_*` in `src/human/board.rs`. Was `src/run/send.rs` and `src/human.rs`. Every test kept its name and cases (223 test lines before and after).
**Why:** the pure-move split of `src/run/send.rs` and `src/human.rs` by phase.
**Kept:** the wrapper's `send` (`typed_text`'s product caller) and the three `send_empty_text_*` cases stay in `src/run/send.rs`.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/
