# Control — gate entry 13 (the gated criterion of matrix#v2-11)

The tool fires no deferred entry, so its control was typed beside it in the tool's shell form
(`/usr/bin/bash -o pipefail -c '…'`), differing from the entry's `run` by the input path alone, over scratch files
made in `$TMPDIR/tmp.vCZhKe7c1W/` (outside the tree and the run dir; `$TMPDIR` = /tmp), 2026-11-24 P5:
- `g/r3.md` holding `r3: green hand=focused opened=focused` → printed `1`, exit 0 (the entry's atoms hold: green)
- `r/r3.md` holding `r3: red hand=focused opened=window-not-found` → printed `0`, exit 1 (red)
- an absent `r3.md` → exit 1, nothing printed (red)
