### surface#14.2 · values · cli-verify-run · ## Surface: cli › ### Component Patterns · lines 138–138
- **`revive --json`'s documents:** one exit-1 document for every refusal of the start it shares with `run`, its closed codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; `run` itself prints none of these documents.
### surface#14.3 · values · cli-verify-run · ## Surface: cli › ### Component Patterns · lines 139–139
- **`revive --json`'s `internal-error`:** a program not found prints `internal-error`, a cause with no human pair; an anyhow error of its start prints the same document, and the catch site logs its chain and its line.
### surface#14.4 · values · cli-verify-run · ## Surface: cli › ### Component Patterns · lines 140–140
- **A `pump_child` error under `revive --json`:** prints no document: the child has held the terminal and viola writes nothing past it.
### surface#15.1 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 146–148
| Exit | Human stderr first word | `--json` |
|---|---|---|
| 0 | none (TTY context lines only); the result line goes to stdout (`[RB] read back`, `answered`, `wheel human` …) | `{"v":1,"ok":{…}}` |
### surface#15.2 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 149–149
| 1 | `error: internal error` (fixed message; no chain with serde sources, no paths, no hint), or a `viola run` start refusal `unable: <name> is already live` / `… is still running but not answering` / `the endpoint for <name> is held by another process` / `the pinned viola copy failed its integrity check` / `<name>'s command is a .cmd or .bat script` / `the viola home is not private to you` / `<name>'s state files can be written by another user`, or a `viola revive` refusal `unable: <name>'s state files can be written by another user` / `<name> has no logged session to resume` / `<name> has no logged session with that id` / `<name>'s session is open in another claude` / `<name>'s recorded directory is missing` / `the viola home is not private to you` (or `run`'s `already live` pair), or a `viola verify` refusal `unable: the claude CLI was not found` / `the claude CLI is a .cmd or .bat script` / `the CLI version could not be read` / `the viola home is not private to you` (its home tree refused before its log, no step line and no summary, or a `stamps.json` update refused by its strict-modes check) / `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`; codes `home-path` · `absolute-path` · `username` · `email`; never the content), each followed by its own hint, or a `viola verify` stopped by Ctrl-C or TERM before its rows are checked, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` (stdout empty); a `verify` run with a failing ledger row prints no stderr word (its last stdout line reads `stamped <ver>  <n> pass  <m> fail`) | `viola revive --json`'s refusals of the start it shares with `run`: `{"v":1,"error":"<code>","detail":null}` (its closed codes in the `revive --json` item above; arch §Standard Contracts); the `cli` catch site's `{"v":1,"error":"internal-error","detail":null}` (the items below); a `pump_child` error and every other exit 1 have no `--json` document |
### surface#15.3 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 150–150
| 2 | clap usage text (never produced by `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (an oversize `viola answer`) or `UNUSABLE_TEXT` line (an unusable `viola send`) on stderr, whatever `--json` says; `tests/contract_error_bodies.rs` names them in `NOT_ERROR_BODIES` | no document |
### surface#15.4 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 151–151
| 10 | `unable  human-typing` (+ `manual-pause`) | `{"v":1,"refusal":"human-typing","detail":…}` |
### surface#15.5 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 152–152
| 11 | `unable  budget-paused  five-hour` / `seven-day` | refusal object |
### surface#15.6 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 153–153
| 12 | `unable  unverified-cli` | refusal object |
### surface#15.7 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 154–154
| 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` / `empty-text` | refusal object |
### surface#15.8 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 155–155
| 14 | `unable  unknown  <detail>`, the detail a code word or nothing | refusal object |
### surface#15.9 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 156–156
| 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
### surface#15.10 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 157–157
| 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / endpoint lost / server verification); `viola list`'s refused strict-modes check, and `viola link` / `viola unlink`'s of the driven instance, print instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer), and so does the instance check `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance before any frame, printing the instance-state pair naming it, with no frame and no line written | `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; the `list`, `link`, `unlink` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`; stderr empty |
### surface#16.1 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 163–163
- Exit 0 under `--json`: `viola link`'s document is `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link, and `viola unlink`'s is `{"v":1,"ok":{}}`.
### surface#16.2 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 164–164
- Exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
### surface#16.3 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 165–165
- Exit 20's `--json` `detail` is closed for the five known codes (`viola_channel::sanitised_fault`): `message` is each code's fixed text; `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params` with null `data`; every other known code has null `data`.
### surface#16.4 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 166–166
- A code outside the five prints the same human line, `error: wrapper fault  <code>`, and under `--json` the document `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
### surface#16.5 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 167–167
- Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.
### surface#16.6 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 168–168
- Under `--json` the `cli` catch site prints row 1's `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
### surface#16.7 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 169–169
- The catch site counts `--json` only among the arguments before a `--`, read from argv.
### surface#16.8 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 170–170
- `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
### surface#17.1 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 176–176
- Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` with stderr empty.
### surface#17.2 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 177–177
- With no live endpoint a frame verb reads its instance directory after the strict-modes check: a directory is `instance-dead`; absent, a regular file or a symlink is `unwrapped-name`.
### surface#17.3 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 178–178
- A failed connect is `instance-dead`; a wrapper lost with the call open is `endpoint-lost`.
### surface#17.4 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 179–179
- A frame verb against a stale instance reads `instance-dead`: stale heartbeat is exit 1's cause alone.
