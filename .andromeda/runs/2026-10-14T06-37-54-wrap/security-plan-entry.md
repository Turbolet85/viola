
## 2026-10-13-owed-ledger-rows — Run B's probe --settings, the statusline capture arm, the relied set and the kept workspace.repo
**Section:** §Threat Model Summary (verify child set); §Input Validation (Hook stdin row, CLI arguments row, statusline shell-out row); §Data Protection (probe captures, Repository fixtures, accepted risk of verify's trusted runs); §Error Handling; §Security Anti-Patterns → Universal, → Data Protection
**Change:**
- Run B, on Unix and only for shell-plain paths, carries exactly one `--settings <probe dir>/typed/statusline.json` (0600, `replace_private`). Its command is `<pinned viola> hook statusline --capture <dir> --shell "$0"`.
- Windows passes none. Runs A, C and D keep their argv, and no run or start is added. On Unix, Run B's status line is the probe capture, not the user's.
- New hidden arm `hook statusline --capture <DIR> [--shell <WORD>]`, taken before the pass-through:
  - `<DIR>` must be absolute and exist;
  - stdin goes through `take(MAX_FRAME + 1)` and is refused past `MAX_FRAME`;
  - the payload is written raw at the first free `StatusLine.<k>.json` (`create_private_new` + `replace_private` 0600);
  - `StatusLine.<k>.shell` holds exactly `bin-sh` or `other` (the closed `ShellVerdict`), and the word is written nowhere;
  - it prints nothing, exits 0 on every path, and reads no `VIOLA_*`, obs init, channel, `budget.json` or snapshot.
- verify reads the files back through `take(MAX_FRAME + 1)`, the shell file capped at 16 bytes.
- The probe dir holds `typed/statusline/` (0700) and `typed/statusline.json`.
- The shell-out row: the shell, the override and `rate_limits` were "relied on with no ledger row, owed to Owed ledger rows". They are now the rows `statusline-shell`, `statusline-override` and `statusline-rate-limits`, stamped on 2.1.287 with two fixtures. The Windows shell-out stays owed to "Windows-only live measurements".
- Universal: the ledger has twenty-one rows, and a stamp is read over the relied set (`/21` on Unix, `/18` on Windows). An all-`pass` 18-id stamp reads `cli_verified:false` on Unix.
- Repository fixtures: a `workspace.repo` naming this repository's own remote is kept, neither rewritten nor refused; nothing else in the rule widens.
**Why:**
- The probe `--settings` and the capture arm are a boundary widening of the ratified verify child set. Ratified by the founder's own answer to P4 fork 3, relayed verbatim by the operator (`inputs#I1`). The wrap's card confirm is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary.
- The kept `workspace.repo` is the founder's own word (`inputs#I3`, relayed verbatim by the operator, given after the gap was shown to him). Its wording is the operator's (`inputs#I4`), and its landing here is the overseer's application of it.
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/
