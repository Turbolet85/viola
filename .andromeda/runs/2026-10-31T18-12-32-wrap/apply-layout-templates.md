# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · applied · .andromeda/layout-templates.md:102#1
- fanout-layout-templates.md 2 · applied · .andromeda/layout-templates.md:288#1
- fanout-layout-templates.md 3 · applied · .andromeda/layout-templates.md:329#1
- fanout-layout-templates.md 4 · applied · .andromeda/layout-templates.md:331#1
- fanout-layout-templates.md 5 · applied · .andromeda/layout-templates.md:332#1
- fanout-layout-templates.md 6 · applied · .andromeda/layout-templates.md:331#2
- fanout-layout-templates.md 7 · applied · .andromeda/layout-templates.md:329#2
- fanout-layout-templates.md 8 · applied · .andromeda/layout-templates.md:146#1
- fanout-layout-templates.md 9 · applied · .andromeda/layout-templates.md:288#2
- fanout-layout-templates.md 10 · applied · .andromeda/layout-templates.md:262#1
- fanout-layout-templates.md 11 · applied · .andromeda/layout-templates.md:262#2

## Edits
- line: .andromeda/layout-templates.md:102
  old: «prints instead `{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «prints instead `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`»
- line: .andromeda/layout-templates.md:288
  old: «or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`»
- line: .andromeda/layout-templates.md:329
  old: «left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`»
- line: .andromeda/layout-templates.md:331
  old: «it prints the exit-21 document `{"v":1,"error":"instance-unreachable","detail":null}`»
  new: «it prints the exit-21 document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`»
- line: .andromeda/layout-templates.md:332
  old: «The machine view carries the cause as a detail code once arch names those codes; until then it keeps arch's shape (`"detail":null` for exit 21).»
  new: «The machine view of exit 21 carries the cause in its document's `detail`, `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`.»
- line: .andromeda/layout-templates.md:331
  old: «mirror with the same hint).»
  new: «mirror with the same hint). A frame verb's exit 21 on a name with no directory at its instance path takes `hint: <name> is not a wrapped session; only sessions started with viola run take commands`, and with its wrapper lost while the call is open `hint: <name>'s wrapper went away during the call; viola list shows whether it is still running`, each the last stderr line in plain ASCII.»
- line: .andromeda/layout-templates.md:329
  old: «from the one catch site, with no hint (for»
  new: «from the one catch site, with no hint, and with `--json` among the arguments before a `--` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in its place with stderr empty (for»
- line: .andromeda/layout-templates.md:146
  old: «once, exit 1, no hint (Primary content block 2).»
  new: «once, exit 1, no hint, or under `--json` `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty (Primary content block 2).»
- line: .andromeda/layout-templates.md:288
  old: «catch site's `error: internal error`, exit 1.»
  new: «catch site's `error: internal error`, exit 1, or under `--json` `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty.»
- line: .andromeda/layout-templates.md:262
  old: «A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`) prints no document as landed; its document is owed to the working-route entry "Exit-cause code catalogue".»
  new: «Under `--json` a refusal of the start `revive` shares with `run`, after a passed preflight, prints one exit-1 document too, with stderr empty: `<code>` is `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, or `internal-error` for a program not found and for an error of the start itself, whose chain the catch site still logs. A `pump_child` error prints no document, the child having held the terminal.»
- line: .andromeda/layout-templates.md:262
  old: «`<code>` one of `strict-modes-failed`, `already-live`, `no-session` (both of its pairs)»
  new: «`<code>` one of `strict-modes-failed`, `already-live` (a live holder or a fresh replayed beat), `stale-heartbeat` (a stale holder), `no-session` (both of its pairs)»

## Patterns
[[pattern]]
id = "lt-01"
fixed = '"error":"instance-unreachable","detail":null'

[[pattern]]
id = "lt-02"
regex = 'strict-modes.{0,200}under `--json` `\{"v":1,"error":"instance-unreachable"'

[[pattern]]
id = "lt-03"
fixed = 'left as found; under `--json` `{"v":1,"error":"instance-unreachable"'

[[pattern]]
id = "lt-04"
fixed = 'it prints the exit-21 document `{"v":1,"error":"instance-unreachable"'

[[pattern]]
id = "lt-05"
fixed = 'carries the cause as a detail code once arch names'

[[pattern]]
id = "lt-06"
regex = 'prints no document as landed|owed to the working-route entry "Exit-cause code catalogue"'

[[pattern]]
id = "lt-07"
fixed = 'one of `strict-modes-failed`, `already-live`, `no-session`'
