# Control — the r4b-refuses grep entry (`grep -c -F -e .credentials.json -e claudeAiOauth scripts/r4-up.sh`)

Typed in the tool's shell form (`/usr/bin/bash -o pipefail -c`), differing from the entry's `run` by its path operand
alone, over scratch scripts in
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/ef143f68-5f7d-432a-9819-33e4485ca62c/scratchpad/ctl-grep/`
(script `control.sh` there), at HEAD `863ea534`, 2026-11-24:

- a script holding only the refusal: exit 1 · last line 0 — must pass, passes
- a script copying `.credentials.json`: exit 0 · last line 1 — must fail, fails
- a script reading `claudeAiOauth`: exit 0 · last line 1 — must fail, fails
- no script: exit 2 — must fail, fails (the untouched tree's reading)
