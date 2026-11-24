# Control — gate entry 14 (the gated criterion of matrix#v2-12)

The tool fires no deferred entry, so its control was typed in the tool's shell form (`/usr/bin/bash -o pipefail -c`),
differing from the entry's `run` by its path operand alone, over scratch inputs in
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/ef143f68-5f7d-432a-9819-33e4485ca62c/scratchpad/ctl14/` (script
`control.sh` there), at HEAD `863ea534`, 2026-11-24:

- both phases green (`r4: green phase=windows …` then `r4: green phase=after …`): exit 0 · last line 1 — must pass, passes
- windows green, after red: exit 1 · last line 0 — must fail, fails
- two `phase=after` green lines and no `phase=windows` line: exit 1 — must fail, fails
- no file: exit 1 — must fail, fails

On the untouched tree the entry's own input `.andromeda/cache/chunks/2026-11-24-viola-up/evidence/r4.md` does not
exist: it reads red (exit 1), a red on an absent input.
