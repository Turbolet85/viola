
## 2026-10-04-wait-and-last — wait / last lines as landed
**Section:** Surface: cli → signature placement · Output structure — `viola wait` / `viola last` · Component — Primary content block 2
**Change:**
- The output structure gains the `session-end` line (every non-dialog kind takes the `turn-ended` form), `dialog unknown`, the exit-21 pair `unable  <name>  instance-unreachable` + its `hint:`, `waiting:` only on a terminal stderr without `--json`, one `--json` document, and the once-printed `error: internal error`.
- Block 2: was "a failed or panicked `verify` prints `error: internal error`"; now every `cli` verb (`send`, `wait`, `last`, `verify`) from the one catch site; the wait/last exit-21 hint is listed; `send` keeps its `[/ ] unable` mirror.
- wait/last join the verbs sharing the `unable` word column.
**Why:** the lines the chunk shipped (the plan's lean: the layout form with the name, over the design line that omits it).
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/
