### surface#15.4 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 143–143
| 10 | `unable  human-typing` (+ `manual-pause`) | `{"v":1,"refusal":"human-typing","detail":…}` |
### surface#15.5 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 144–144
| 11 | `unable  budget-paused  five-hour` / `seven-day` | refusal object |
### surface#15.6 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 145–145
| 12 | `unable  unverified-cli` | refusal object |
### surface#15.7 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 146–146
| 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` / `empty-text` | refusal object |
### surface#15.8 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 147–147
| 14 | `unable  unknown  <detail as opaque text>` | refusal object |
### surface#15.9 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 148–148
| 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
### surface#15.10 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 149–149
| 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / server verification); `viola list`'s refused strict-modes check, and `viola link` / `viola unlink`'s of the driven instance, print instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer), and so does the instance check `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance before any frame, printing the instance-state pair naming it, with no frame and no line written | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), the `list`, `link` and `unlink` refusals and the six frame verbs' strict-modes refusals included, with stderr empty |
### surface#16.1 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 155–155
- Exit 0 under `--json`: `viola link`'s document is `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link, and `viola unlink`'s is `{"v":1,"ok":{}}`.
### surface#16.2 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 156–156
- Exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
### surface#17.1 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 162–162
- Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.
### surface#18.1 · values · cli-help · ## Surface: cli › ### Navigation Pattern · lines 170–170
- `viola --help` groups verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install` (`revive` beside `run`: the operator's answer at the chunk 2026-10-10-viola-revive wrap). As built (chunk 2026-10-17-cli-output-discipline) it lists the served verbs only: `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.
