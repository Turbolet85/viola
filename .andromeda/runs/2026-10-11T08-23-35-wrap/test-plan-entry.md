
## 2026-10-11-the-board-viola-list — the board's readings landed: E5, Path 6, Chaos; the literal pin and the kill line
**Section:** §4 Unit Test Strategy (root bin escaping; snapshot read-or-replay); §5 Integration Test Strategy (Drivers, CLI trycmd bullet, driver table); §6 (Scenario E5 As landed, Path 6, Chaos suite torn append and forward-compat, Cross-surface parity, E3 As landed); §3 → Bootstrap phases (the `[profile.ci]` kill line); §3 → Status endpoint shape; §3 → 5-command implementation (`boot --statusline-echo`)
**Change:**
- §4: `wait` / `last` keep `\n` and `\t`; `list` rows escape every control character, `\n`, `\t`, DEL and C1 included (was: all three keep them). The read-or-replay function has a second product caller, `board::read`.
- §5: the `viola list` board is pinned by literals in `tests/cli_list.rs` (`CAPTION`, `list_human_board_is_the_literal_rows`, the empty-board case) and the `bay_line_*` / `board_row_*` units, not a trycmd case (was: trycmd); trycmd keeps the send mirror and help.
- §6 E5 is landed in part (was: owed, not built): the `list --json` leg (`e5_list_reads_a_live_wrapper_through_the_replay` ×2); steps 1–3 wait on the five other readers taking the replay, owed to "Server verification before any frame".
- §6 Path 6: `list --json`'s `budget`, `budget_paused` and the board's cell landed (`path6_list_shows_the_reading_and_the_pause`, three CI runners); the harness-session form, step 7's two-instance leg and `--statusline-echo` are owed to "End-to-end path binary and harness legs" (was: the board entry).
- §6 Chaos: `torn_lines` on `list --json` landed; the forward-compat `list --json` leg landed (`chaos_forward_compat.rs`); `/api/sessions` stays Epoch 8's. Parity names the `CAPTION` literal. E3's hold moved to "Explicit session links".
- The kill line: eighteen verify-driven binaries (was fifteen; the rule: the `binary(…)` names on `.config/nextest.toml`'s filter line, every binary that drives verify): + `cli_list`, `chaos_forward_compat`, `chaos_torn_append`.
- Status endpoint: the interim's premise is now "while `status` runs no `viola list`" (was: `viola list` unbuilt); its owner is "End-to-end path binary and harness legs".
**Why:** the board as built; owners per the review card the founder approved.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/
