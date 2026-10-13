proposals:
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy (root bin, the budget governor thresholds bullet)'
    change: 'The `send_budget_paused_*` cases are cited at `src/run/send/budget.rs` (was `src/run/send.rs`); the other citations of the bullet stand.'
    sidecar: '2026-10-13-epoch-4-cleanup: §4 budget governor bullet re-cites `send_budget_paused_*` at `src/run/send/budget.rs` after the `run::send` split by phase.'
    rationale: 'Report Changes, Symbols / APIs: `src/run/send.rs` split by phase, and "`send_budget_paused_*` (8) → `src/run/send/budget.rs`"; Expected amendments name this site. The unit map must point at the file where the mandated unit cases now run.'
    basis: '.andromeda/test-plan.md:582; viola-0.1.0/chunks/2026-10-13-epoch-4-cleanup/report.md:364-372 (src/run/send/budget.rs 18-233 @19)'
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy (root bin, the send-confirmation matcher bullet)'
    change: 'Of the six `trailing_newline` send-module tests taking LF, CR and CRLF tails, three sit in `src/run/send/classify.rs` (the local-command ones), two in `src/run/send/relabel.rs` (`send_trailing_newlines_are_not_typed_and_the_send_is_confirmed`, `send_a_prompt_that_keeps_the_trailing_newline_claims_nothing`) and one in `src/run/send.rs` (`send_a_refused_character_before_a_trailing_newline_is_control_character`); the two `inner_cr_is_one_lf` tests sit in `src/run/send/relabel.rs`; the eight `send_local_command_` functions sit in `src/run/send/classify.rs`.'
    sidecar: '2026-10-13-epoch-4-cleanup: §4 send-confirmation matcher re-cites the trailing-newline, inner-CR and local-command unit tests at the `run::send` phase files.'
    rationale: 'Report Changes, Symbols / APIs ("Where the cited tests now sit"): the six `trailing_newline` tests → classify.rs 3, relabel.rs 2, send.rs 1; the two `inner_cr_is_one_lf` tests → relabel.rs; `send_local_command_*` (8 functions) → classify.rs. The bullet still says "six of the seven, in `src/run/send.rs`" and "two tests in `src/run/send.rs`".'
    basis: '.andromeda/test-plan.md:583; viola-0.1.0/chunks/2026-10-13-epoch-4-cleanup/report.md:373-405'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy (CLI bullet, the `viola list` board)'
    change: 'The board literals are backed by the `src/human/board.rs` units `bay_line_*` and `board_row_*` (was `src/human.rs`).'
    sidecar: '2026-10-13-epoch-4-cleanup: §5 CLI bullet re-cites the board units at `src/human/board.rs` after the `human` split by phase.'
    rationale: 'Report Changes, Symbols / APIs: `src/human.rs` is now the module root of the shared writers only; "`bay_line_*`, `board_row_*` (incl. `board_row_caption_and_default_row_are_the_literals`) and `instance_and_home_state_pairs_are_the_fixed_texts` → `src/human/board.rs`"; Expected amendments name `:655` as the same fact one section on.'
    basis: '.andromeda/test-plan.md:655; viola-0.1.0/chunks/2026-10-13-epoch-4-cleanup/report.md:271-293 (src/human/board.rs 189-505 @190)'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → 5-command implementation (`run` step 4 Mutation, Command, the Windows host scratch arm)'
    change: 'After measuring its bytes the run removes the scratch and re-creates it through `viola_state::fs::create_private_dir` (was a plain re-create): on Windows the re-created folder takes the protected owner-only DACL `D:P(A;OICI;FA;;;<user>)(A;OICI;FA;;;SY)`, so every tempdir laid inside it inherits owner-only and an instance tree there passes `check_instance`; pinned by the `#[cfg(windows)]` unit `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` (the same tree in a plain-created sibling reads `Refused::Writable`), run on CI test (windows-2025) with no dispatch; whether the root package Windows mutation baseline then reads green is owed to the next `windows-mutants.yml` dispatch.'
    sidecar: '2026-10-13-epoch-4-cleanup: §3 `run` step 4 records the owner-only re-create of the Windows host mutation scratch and its Windows witness unit.'
    rationale: 'Report Changes, Symbols / APIs (`scratch.rs` `wipe` now re-creates through `create_private_dir`, was `fs::create_dir_all`; the new `#[cfg(windows)]` witness) and Harness / gate surface ("on a Windows host `run --mutants` now re-creates `<repo parent>/viola-mutants-scratch` owner-only"); Cross-project: the witness PASSED in ci#38375200217 job 115221715737; Insufficient fixes: the baseline green reading is not measured. Expected amendments name this site ("removes and recreates it").'
    basis: '.andromeda/registries/contracts/test-plan/5-command-implementation.md:45; viola-0.1.0/chunks/2026-10-13-epoch-4-cleanup/report.md:232-234 (scratch.rs 252-281 @258)'
note: 'Not proposed: §9 Lint row — the Windows-target clippy is a chunk gate entry only (report Harness / gate surface: not a pre-push stage, not a CI step), and no test-plan detector governs a lint gate; D-tests-framework holds (nextest / rstest unchanged); D-tests-obs-harness holds (no harness verb, status shape or log format changed; the obs-plan scratch site :973 sits in its §8, not §3). Sites standing true: test-plan.md:441, :566, :590, :1123, :1226 and 5-command-implementation.md:46, :49, :57, :142, :163.'
