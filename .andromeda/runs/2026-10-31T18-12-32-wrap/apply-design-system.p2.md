# Apply — design-system · part 2 of 2

## Verdicts
- fanout-design-system--cascade.md 1 · applied · .andromeda/design-system-detail/surface.md:40#1
- fanout-design-system--cascade.md 2 · applied · .andromeda/design-system-detail/surface.md:65+#1
- fanout-design-system--cascade.md 3 · applied · .andromeda/design-system-detail/surface.md:66#1
- fanout-design-system--cascade.md 4 · applied · .andromeda/design-system-detail/surface.md:70+#1
- fanout-design-system--cascade.md 5 · applied · .andromeda/design-system-detail/surface.md:83#1, .andromeda/design-system-detail/surface.md:83#2
- fanout-design-system--cascade.md 6 · applied · .andromeda/design-system-detail/surface.md:84#1
- fanout-design-system--cascade.md 7 · applied · .andromeda/design-system-detail/surface.md:90#1
- fanout-design-system--cascade.md 8 · applied · .andromeda/design-system-detail/surface.md:135#1
- fanout-design-system--cascade.md 9 · applied · .andromeda/design-system-detail/surface.md:135+#1
- fanout-design-system--cascade.md 10 · applied · .andromeda/design-system-detail/surface.md:135+#2
- fanout-design-system--cascade.md 11 · applied · .andromeda/design-system-detail/surface.md:135+#3
- fanout-design-system--cascade.md 12 · applied · .andromeda/design-system-detail/surface.md:144#1
- fanout-design-system--cascade.md 13 · applied · .andromeda/design-system-detail/surface.md:152#1, .andromeda/design-system-detail/surface.md:152#2
- fanout-design-system--cascade.md 14 · applied · .andromeda/design-system-detail/surface.md:162+#1
- fanout-design-system--cascade.md 15 · applied · .andromeda/design-system-detail/surface.md:162+#2
- fanout-design-system--cascade.md 16 · applied · .andromeda/design-system-detail/surface.md:162+#3
- fanout-design-system--cascade.md 17 · applied · .andromeda/design-system-detail/surface.md:168#1
- fanout-design-system--cascade.md 18 · folded into fanout-design-system--cascade.md 13
- fanout-design-system--cascade.md 19 · folded into fanout-design-system--cascade.md 13
- fanout-design-system--cascade.md 20 · applied · .andromeda/design-system-detail/surface.md:168+#1
- fanout-design-system--cascade.md 21 · applied · .andromeda/design-system-detail/surface.md:168+#2
- fanout-design-system--cascade.md 22 · applied · .andromeda/design-system-detail/surface.md:168+#3

## Edits
- line: .andromeda/design-system-detail/surface.md:40
  old: «of their home-tree check, exit 1).»
  new: «of their home-tree check, exit 1); under `--json` the refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.»
- after: .andromeda/design-system-detail/surface.md:65
  anchor: «    - the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`»
  add: «    - a wrapper lost with the call open (`endpoint-lost`): `<name>'s wrapper went away during the call; viola list shows whether it is still running` (last on stderr, plain ASCII). OPEN — this wording is PROVISIONAL · owner: the wording of the `endpoint-lost` hint»
- line: .andromeda/design-system-detail/surface.md:66
  old: «under `--json` the `instance-unreachable` document»
  new: «under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`»
- after: .andromeda/design-system-detail/surface.md:70
  anchor: «    - `unable: <name> is still running but not answering` (stale heartbeat, live pid) → `hint: viola list shows it as stale; stop that process before starting <name> again`»
  add: «    - These two print and log their own codes: a stale holder `stale-heartbeat`; a live holder and a fresh replayed beat `already-live`.»
- line: .andromeda/design-system-detail/surface.md:83
  old: «**Pending an arch amendment:** arch currently fixes `{"v":1,"error":"instance-unreachable","detail":null}` for exit 21, and for exit 1 only `viola revive`'s `{"v":1,"error":"<code>","detail":null}` (its strict-modes and preflight refusals; chunk 2026-10-17-cli-machine-contract). Until arch names the `detail` codes, `--json` keeps arch's shape. obs-plan D-20 already names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`,»
  new: «Exit 21's document names its cause in `detail`, one of five closed words (exit-code row 21); exit 1's, `detail` null, is printed by `viola revive` for every refusal of the start it shares with `run` and, as `internal-error`, by the `cli` catch site (exit-code row 1). obs-plan D-20 names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `squatted-name`,»
- line: .andromeda/design-system-detail/surface.md:83
  old: «whose `--json` document stays `detail:null`, as the not-running cause's does. The stale-heartbeat and unwrapped-name causes have no code yet.»
  new: «whose `--json` document reads the same word. The socket-directory cause and macOS's server check stay without these codes; owner: the route entry "Unix endpoint and home hardening".»
- line: .andromeda/design-system-detail/surface.md:84
  old: «the `--json` document's `detail` is row 20's closed object»
  new: «the `--json` document's `detail` is, for the wrapper fault, row 20's closed object and, for the internal error, null»
- line: .andromeda/design-system-detail/surface.md:90
  old: «as the hint cause `server-verify` beside `not-running`»
  new: «as the hint cause `server-verify` beside `not-running`, `unwrapped-name` and `endpoint-lost` (`not-running` is the hint key of the log's `instance-dead`)»
- line: .andromeda/design-system-detail/surface.md:135
  old: «a refused strict-modes check or a preflight refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) and no hint, stderr empty, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`; a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to the working-route entry "Exit-cause code catalogue").»
  new: «a refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty, its codes in the item below, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`.»
- after: .andromeda/design-system-detail/surface.md:135
  anchor: «- **`revive`:** prints nothing once the child starts, as `run`. `viola revive builder --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>` with two spaces between fields: no header, no colour, plain ASCII. A refusal (exit 1) prints its cause's fixed-message line and then that cause's hint, listed under the send hints above. Under `--json` (both arms; chunk 2026-10-17-cli-machine-contract) a refused strict-modes check or a preflight refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) and no hint, stderr empty, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`; a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to the working-route entry "Exit-cause code catalogue").»
  add: «- **`revive --json`'s documents:** one exit-1 document for every refusal of the start it shares with `run`, its closed codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; `run` itself prints none of these documents.»
- after: .andromeda/design-system-detail/surface.md:135
  anchor: «- **`revive`:** prints nothing once the child starts, as `run`. `viola revive builder --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>` with two spaces between fields: no header, no colour, plain ASCII. A refusal (exit 1) prints its cause's fixed-message line and then that cause's hint, listed under the send hints above. Under `--json` (both arms; chunk 2026-10-17-cli-machine-contract) a refused strict-modes check or a preflight refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) and no hint, stderr empty, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`; a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to the working-route entry "Exit-cause code catalogue").»
  add: «- **`revive --json`'s `internal-error`:** a program not found prints `internal-error`, a cause with no human pair; an anyhow error of its start prints the same document, and the catch site logs its chain and its line.»
- after: .andromeda/design-system-detail/surface.md:135
  anchor: «- **`revive`:** prints nothing once the child starts, as `run`. `viola revive builder --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>` with two spaces between fields: no header, no colour, plain ASCII. A refusal (exit 1) prints its cause's fixed-message line and then that cause's hint, listed under the send hints above. Under `--json` (both arms; chunk 2026-10-17-cli-machine-contract) a refused strict-modes check or a preflight refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) and no hint, stderr empty, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`; a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to the working-route entry "Exit-cause code catalogue").»
  add: «- **A `pump_child` error under `revive --json`:** prints no document: the child has held the terminal and viola writes nothing past it.»
- line: .andromeda/design-system-detail/surface.md:144
  old: «`viola revive --json`'s refusals of its strict-modes check and its preflight: `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; arch §Standard Contracts); every other exit 1 has no `--json` document (a `run` start refusal reached through `revive` included, owed to "Exit-cause code catalogue")»
  new: «`viola revive --json`'s refusals of the start it shares with `run`: `{"v":1,"error":"<code>","detail":null}` (its closed codes in the `revive --json` item above; arch §Standard Contracts); the `cli` catch site's `{"v":1,"error":"internal-error","detail":null}` (the items below); a `pump_child` error and every other exit 1 have no `--json` document»
- line: .andromeda/design-system-detail/surface.md:152
  old: «(not running / unwrapped / server verification)»
  new: «(not running / unwrapped / endpoint lost / server verification)»
- line: .andromeda/design-system-detail/surface.md:152
  old: «`{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), the `list`, `link` and `unlink` refusals and the six frame verbs' strict-modes refusals included, with stderr empty»
  new: «`{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; the `list`, `link`, `unlink` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`; stderr empty»
- after: .andromeda/design-system-detail/surface.md:162
  anchor: «- Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.»
  add: «- Under `--json` the `cli` catch site prints row 1's `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.»
- after: .andromeda/design-system-detail/surface.md:162
  anchor: «- Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.»
  add: «- The catch site counts `--json` only among the arguments before a `--`, read from argv.»
- after: .andromeda/design-system-detail/surface.md:162
  anchor: «- Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.»
  add: «- `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.»
- line: .andromeda/design-system-detail/surface.md:168
  old: «or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty»
  new: «or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` with stderr empty»
- after: .andromeda/design-system-detail/surface.md:168
  anchor: «- Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.»
  add: «- With no live endpoint a frame verb reads its instance directory after the strict-modes check: a directory is `instance-dead`; absent, a regular file or a symlink is `unwrapped-name`. OPEN — this reading is PROVISIONAL · owner: whether an unwrapped name reads by its instance directory»
- after: .andromeda/design-system-detail/surface.md:168
  anchor: «- Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.»
  add: «- A failed connect is `instance-dead`; a wrapper lost with the call open is `endpoint-lost`.»
- after: .andromeda/design-system-detail/surface.md:168
  anchor: «- Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.»
  add: «- A frame verb against a stale instance reads `instance-dead`: stale heartbeat is exit 1's cause alone.»

## Patterns
[[pattern]]
id = "ds2-01"
fixed = 'Until arch names the `detail` codes'

[[pattern]]
id = "ds2-02"
fixed = 'document stays `detail:null`'

[[pattern]]
id = "ds2-03"
fixed = 'causes have no code yet'

[[pattern]]
id = "ds2-04"
fixed = '`detail` is row 20'

[[pattern]]
id = "ds2-05"
fixed = 'prints no document as landed'

[[pattern]]
id = "ds2-06"
fixed = 'reached through `revive` included'

[[pattern]]
id = "ds2-07"
fixed = 'code awaits an arch amendment'

[[pattern]]
id = "ds2-08"
fixed = '"error":"instance-unreachable","detail":null'
