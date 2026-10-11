
## 2026-10-11-the-board-viola-list — the board's caption pinned by literal; separator, help groups and colour not built
**Section:** §1 (Surface cli ARIA roles inventory and notes; Critical path 6); §4 (per-surface landmark roles, cli row; P1 cli mirror; P6 setup and the outer-PTY SGR bullet); §6 (CLI equivalent); §8 (Consistent identification, Plain language verification, Error recovery cli)
**Change:**
- The caption order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI is pinned by the `CAPTION` literal in `list_human_board_is_the_literal_rows` and the unit `board_row_caption_and_default_row_are_the_literals` (was: a trycmd case); `list`'s fixed strings and its exit-21 refusal pairs are pinned by literals (`tests/cli_list.rs`, the `src/human.rs` units, `instance_and_home_state_pairs_are_the_fixed_texts`).
- The board as built is BAY line → caption row → wrapped rows; the `-- UNWRAPPED - READ-ONLY --` separator is owed to "Unwrapped sessions on the board", the `--help` group labels to "CLI output discipline".
- P6: `list` reads no `claude agents --json`; its row escape is held by unit cases on a recorded CLI version (ESC, C1, DEL, `\n`, `\t` → `\xHH`) and two root cases asserting no ESC or non-ASCII byte on either stream.
- No SGR on the board: each state is carried by its word alone; the amber / dim / bold spans and their outer-PTY check wait on "CLI output discipline".
**Why:** the board as built; owners per the review card the founder approved.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/
