
## 2026-10-13-owed-ledger-rows — Run B's probe --settings, the statusline capture arm and the moved rows
**Section:** §Established Decisions → [CLI Version Compatibility], [Hook Contract], [Plugin Scope]; §Standard Contracts → list, hook statusline; §Occupied Resources → Hidden flags, Claude Code integration names, viola home, Repository; §Cross-cutting Patterns → Diagnostic output channels, Capability ledger; registry `project-directory-structure`
**Change:**
- Run B, on Unix and only for shell-plain paths, gets one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair (`statusline::probe_document`, 0600). Its `statusLine` command is `<pinned viola> hook statusline --capture <dir> --shell "$0"`.
- Windows passes no `--settings`. Runs A, C and D keep their argv, and no run is added.
- On Unix, Run B's status line is the probe capture, not the user's.
- New hidden arm: `hook statusline --capture <DIR> [--shell <WORD>]`. It files `StatusLine.<k>.json` and `StatusLine.<k>.shell` (`bin-sh` | `other`) under the same claim and 0600 write. The word is written nowhere.
- The arm reads no `VIOLA_*`, does no obs init, opens no channel, writes no `budget.json`, prints nothing and exits 0.
- The probe dir gains `typed/statusline/` and `typed/statusline.json`.
- Repository:
  - `2.1.287` gains the two `StatusLine` variants (31 files) and is stamped at the relied set.
  - New `schemas/claude-statusline-fixture.v1.json`.
  - A `workspace.repo` naming this repository's own remote is kept in a fixture.
- The fake agent's `--framing` replays the variants through a `--settings` statusLine command. The `"$0"` word is handed over as `/bin/sh`, with no shell. The first-Stop replay comes after the turn screen.
- Moves:
  - `--resume` and `/compact` were owed on "Owed ledger rows"; now owed on "Resume and compact ledger rows".
  - The `claude agents --json` join row and probe were owed on "Owed ledger rows"; now owed on "Unwrapped sessions on the board".
**Why:**
- The probe `--settings` and the capture arm are a boundary widening of the ratified verify child set. Ratified by the founder's own answer to P4 fork 3, relayed verbatim by the operator (`inputs#I1`). The wrap's card confirm of the proposals inside it is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary.
- The kept `workspace.repo` is the founder's own word (`inputs#I3`, relayed verbatim by the operator, given after it was shown to him). Its landing in each master is the overseer's application of it.
- The moves are his answers to forks 1 and 2 (`inputs#I1`).
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/
