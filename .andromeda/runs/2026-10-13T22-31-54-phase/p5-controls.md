# P5 controls — 2026-10-13-owed-ledger-rows (HEAD 14e62329f808)

Each control fired AS THE ENTRY through `gate.py run --plan … --entry {n} --project-root {clone}` (no run dir) over a
scratch clone (`git clone --shared` of the project, checked out at 14e62329f808) under
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/0c5a34ac-0da0-4e5c-adf2-5f72ecabac7d/scratchpad/clone-p5`,
a fresh clone per control; its input lived in the clone only. Lines as the tool printed them.

## Entry 13 — the fixture guard
- unmodified (HEAD 14e62329f808): red · exit 1 — matches the real tree's baseline
- the two StatusLine variants minted untracked (HEAD 14e62329f808): green · exit 0 — the must-pass side
- minted plus a modified `UserPromptSubmit.paste-1.json` (HEAD 14e62329f808): red · exit 1
- minted plus an extra `StatusLine.extra-1.json` (HEAD 14e62329f808): red · exit 1
- the two variants committed (HEAD 29caf2170f3d): green · exit 0 — the post-pre-CI-commit form

## Entry 15 — the preservation guard
- unmodified (HEAD 14e62329f808): green · exit 0 — matches the real tree's baseline
- `Cargo.toml` edited: red · exit 1
- an untracked `src/new_file.rs`: red · exit 1
- `src/run/send.rs` edited: red · exit 1
- the schema minted untracked and `ledger.rs` edited: green · exit 0 — the must-pass side
- the schema committed (HEAD 40e8443a3fbc): green · exit 0 — the post-pre-CI-commit form
- an untracked `schemas/other.json`: red · exit 1

## Entry 6 — the nine row cases by name
A typed control differing from the entry's `run` by its filter pattern only, run as
`/usr/bin/bash -o pipefail -c "bash scripts/agent-run.sh run --unit --filter 'test(/check_paste_newline_/)'"` on the
real tree: exit 0, `"suite":"nextest-unit","passed":3,"failed":0` — the atom's shape reads from a real selection (the
real-tree baseline is 0 selected of 2039, `nextest-exit-4`).
