# P5 controls — 2026-10-16-unwrapped-sessions-on-the-board

Every control fired as the entry itself, through `gate.py run --plan … --entry {n} --project-root {clone}` with no
`--run-dir`, over a scratch clone `git clone --shared` of the project at HEAD `92264a7f254c`:
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/cceb2642-a818-400b-834a-0066b4df9c9b/scratchpad/clone-usb`.
Each input lived in the clone only.

## Entry 10 — the fixture guard
| clone input | expected | read |
|---|---|---|
| unmodified (as on the real tree) | red | `red · exit 0 ✗ (exit 1)` |
| `fixtures/claude/2.1.287/Agents.recorded-1.json` added, untracked (must pass) | green | `green · exit 0` |
| the variant added, plus `fixtures/claude/2.1.287/Stop.default-1.json` modified (must fail) | red | `red · exit 0 ✗ (exit 1)` |
| the variant and a second addition `Agents.recorded-2.json` (must fail) | red | `red · exit 0 ✗ (exit 1)` |

## Entry 6 — the named unit filter
Baseline red on 0 selected (`error: no tests to run`, nextest exit 4): the cases are this chunk's to write, so its
control is implement's red-before-green run. Its `lacks` atom was re-spelt from this entry's own output at P5: the
baseline log prints `{"suite":"nextest-unit","passed":0,"failed":1,…}` beside the doctest suite's
`{"suite":"doctest","passed":0,…}`, so the earlier bare `"passed":0,` would have matched the doctest object on every run
and could never pass.

## Entry 11 — the integration filter
Baseline red: nextest refuses the filterset because `binary(cross_list_rows)` matches no binary, as the suite is step
15's to write. The red is the subject this chunk builds. Its control is implement's red-before-green run.
