# Control — entry 16 (the gated reader of `evidence/r2.md`), typed beside the entry

Typed at P5 on 2026-11-23, because `gate.py run --entry 16` reads `not run — defer (key): gated: …` and the tool fires
no deferred entry. The form is the tool's own: `/usr/bin/bash -o pipefail -c '<the entry's run, verbatim>'`. Only the
working folder differs (an input operand).

| input | where | exit | last line |
|---|---|---|---|
| the untouched tree (no `evidence/r2.md`) | the project root at `d7b65a6d` | 1 | — |
| `r2: green placement=placed workspace=1 parent_workspace=1` | `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/50824a96-7604-4bd0-b27c-1ceef032a45f/scratchpad/ctl16/pass` | 0 | 1 |
| `r2: red placement=unplaced workspace=none parent_workspace=1` | `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/50824a96-7604-4bd0-b27c-1ceef032a45f/scratchpad/ctl16/fail` | 1 | 0 |

The literal is the verdict line step 27 has `scripts/r2-open.sh` print: it must pass on that line and fail on a red
one.
