
## 2026-10-17-cli-output-discipline — the terminal colour facts NO_COLOR / TERM / COLORTERM join the admitted env set
**Section:** §Input Validation (a new row); §Secret Management → Storage; §Security Anti-Patterns → Universal
**Change:**
- Input Validation: a row for `NO_COLOR`, `TERM`, `COLORTERM`, read in every build by `viola list`'s human mode only: through `std::env::var_os` as text with stdout's `IsTerminal`, compared only, never echoed, decided into the closed `Depth` (`Truecolor` · `Ansi256` · `Ansi16`) or none — none under `--json`, a passthrough role, `NO_COLOR` present (empty included), `TERM` exactly `dumb`, a non-terminal stdout, or a failed Windows VT enable (`SetConsoleMode(mode | 0x0004)`, restored on drop, never `0x000c`); they decide only SGR bytes around rows already escaped, reach no log line, error or refusal, change no escaping or check, disable no control, widen no redaction; `COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE`, `CI` are not read; the width is `viola_pty::host_size()`.
- Storage: was "the two variables outside `VIOLA_*` plumbing, the test-only seams"; now the seams plus the three colour facts, read in every build for colour only.
- Universal: was "the two carve-outs are the test seams"; now the three colour facts stand beside them, switching off no control; NEVER another seam or environment read, or a colour fact changing a control or reaching a log, error or refusal, without its own Decisions Log entry.
**Why:** a boundary widening ratified by the founder's own answer «Читать их» to P4 fork Q1 (relayed verbatim by the operator); the answer card confirmed by a delegate, the Viola overseer — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-17T08-28-14-wrap/
