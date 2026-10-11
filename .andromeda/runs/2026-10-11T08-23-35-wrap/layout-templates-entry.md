
## 2026-10-11-the-board-viola-list — viola list as built, its widths and its refusals
**Section:** §Surface: cli (Primary screens; Output structure — `viola list` piped and `--json`; Primary content block 1 strip rows; Primary content block 2 refusal lines)
**Change:**
- Output structure gains an As built paragraph: the uncoloured form on every stream as one stdout write; the styled TTY form not built ("CLI output discipline"); the unwrapped rack not built and STATUS always `unknown` ("Unwrapped sessions on the board"). Widths: NAME the longer of 4 and the longest name, LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI unpadded; no trailing space, nothing truncated; `unknown` WHEEL / CLI and `DIALOG unknown` cells; the planning forms named.
- `list --json` gains its refusal document, exit 21, stderr empty, no log, mode as found.
- Block 1 gains an As built note (wrapped rows only, no styling or separator; `escape_row` covers DEL and C1; one upstream cell, the CLI version).
- Block 2: the fixed-message exceptions gain `list`'s two exit-21 state pairs, stdout empty; the catch-site verbs gain `list` (an unlistable `instances/`, a failed log start; a per-instance read error gives no row); the exit-code tail notes `list`'s 21 prints the pair.
- `--help` is marked not built (clap's ordered list), owed to "CLI output discipline".
**Why:** the board as built. The refusal and the planning forms stand on the operator's word, not ruled by the founder; WHEEL 23 and the exit-1 paths are implement's decisions, accepted by the operator at this wrap.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/
