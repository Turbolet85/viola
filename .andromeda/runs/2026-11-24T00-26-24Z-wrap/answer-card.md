# Answer card Q1 — 2026-11-23-viola-open-2

## The answers it rests on (the operator's own, relayed verbatim by overseer1, `inputs#I4`, answered 2026-11-23T22:20:06Z)
- **W1** → «Происхождение и воркспейс (Recommended)» — arm (a): the origin and the target workspace cross onto the started `run`'s argv as two hidden checked flags.
- **W2** → «Запись, иначе обход предков (Recommended)» — arm (a): the parent's recorded window first, else the ancestry walk of at most 8 steps, held in memory, written nowhere.
- **W3** → «Да, окно и происхождение (Recommended)» — arm (a): `list --json` and MCP `list` items gain `window?` and `origin?`; the six human columns unchanged.

## The escalated operations lying wholly inside those answers (7)
- ops-architecture.md · contract/list-payload/items — standard-contracts — fit: fits type 3 — W3: the item shape gains `window?` and `origin?`
- ops-architecture.md · contract/viola-cli-arguments/run-origin — occupied-resources — fit: fits type 3 — W1: the hidden `--origin` flag
- ops-architecture.md · contract/viola-cli-arguments/run-place — occupied-resources — fit: fits type 3 — W1: the hidden `--place` flag
- ops-security-plan.md · decision/run-window-flags-checked — input-validation — fit: fits type 1 — W1: the two flags' closed-word and integer checks
- ops-security-plan.md · decision/board-window-and-origin-re-parsed — input-validation — fit: fits type 1 — W3: window and origin shown only when they re-parse
- ops-security-plan.md · decision/open-ancestor-walk-in-memory — data-protection — fit: fits type 1 — W2: at most 8 ancestor pids, held in memory
- ops-test-plan.md · contract/viola-list-json/items — integration-test-strategy — fit: fits type 3 — W3: the list --json item gains `window?` and `origin?`

Named beside the card, not asked (fit read `not_master`): ops-architecture.md · decision/open-window-target — not_master type 4.

## The confirm
The operator is AWAY (`inputs#I9`). The overseer's directive confirms in advance an answer card whose every row lies inside his own recorded answers: «An answer card whose every row lies inside his own recorded answers (item 2) is confirmed by the overseer.» (`inputs#I9`, the overseer's wrap directive, 2026-11-24). Every row above lies inside W1–W3 arm (a). The confirm is a delegate's — PROVISIONAL, to be superseded by the operator's own word in the batch at the epoch boundary; the ratification of the three widenings stays his own answers above.

Answer: confirm — delegate overseer, 2026-11-24, inputs#I9.
