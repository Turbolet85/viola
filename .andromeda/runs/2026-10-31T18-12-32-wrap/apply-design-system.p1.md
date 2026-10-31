# Apply — design-system · part 1 of 2

## Verdicts
- fanout-design-system--cascade.md 23 · applied · .andromeda/design-system.md:263#1
- fanout-design-system--cascade.md 24 · applied · .andromeda/design-system.md:292#1
- fanout-design-system--cascade.md 25 · applied · .andromeda/design-system.md:303#1
- fanout-design-system.md 1 · folded into fanout-design-system--cascade.md 25
- fanout-design-system--cascade.md 26 · applied · .andromeda/design-system.md:334#1
- fanout-design-system.md 7 · folded into fanout-design-system--cascade.md 26
- fanout-design-system--cascade.md 27 · applied · .andromeda/design-system.md:338#1
- fanout-design-system.md 6 · folded into fanout-design-system--cascade.md 27
- fanout-design-system.md 8 · folded into fanout-design-system--cascade.md 27
- fanout-design-system--cascade.md 28 · applied · .andromeda/design-system.md:338#2
- fanout-design-system.md 2 · folded into fanout-design-system--cascade.md 28
- fanout-design-system--cascade.md 29 · applied · .andromeda/design-system.md:340#1
- fanout-design-system.md 3 · folded into fanout-design-system--cascade.md 29
- fanout-design-system--cascade.md 30 · applied · .andromeda/design-system.md:342#1
- fanout-design-system.md 4 · folded into fanout-design-system--cascade.md 30
- fanout-design-system.md 5 · applied · .andromeda/design-system.md:263#2

## Edits
- line: .andromeda/design-system.md:263
  old: «which exit 2 with their fixed stderr line and no document.»
  new: «which exit 2 with their fixed stderr line and no document, and a `pump_child` error under `viola revive --json`, which prints none, the child having held the terminal. At the catch site `run`, `revive`'s start, `mcp` and `hook` stay silent.»
- line: .andromeda/design-system.md:263
  old: «the typed `refusal` / `error` fields are the whole answer»
  new: «the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`»
- line: .andromeda/design-system.md:292
  old: «in the fixed `unable:` / `hint:` form.»
  new: «in the fixed `unable:` / `hint:` form, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`.»
- line: .andromeda/design-system.md:303
  old: «a per-cause `detail` code is pending, owner: the arch amendment naming them.»
  new: «exit 21's document names its cause in `detail`, one of five closed words (Exit-code phraseology below).»
- line: .andromeda/design-system.md:334
  old: «under `--json` a refused strict-modes check or preflight prints one stdout document and no hint. A refusal `run`'s start makes after a passed preflight prints no `--json` document: pending, owner: the working-route entry "Exit-cause code catalogue".»
  new: «under `--json` every refusal of the start it shares with `run` prints one stdout document and no hint, stderr empty, its code one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; a `pump_child` error prints none, the child having held the terminal.»
- line: .andromeda/design-system.md:338
  old: «(a `--json` document only for `viola revive`'s strict-modes and preflight refusals)»
  new: «(a `--json` document for every refusal of `viola revive`'s start, and in place of the `error: internal error` line the catch site's `{"v":1,"error":"internal-error","detail":null}` document on stdout, stderr empty)»
- line: .andromeda/design-system.md:338
  old: «its `--json` document with stderr empty»
  new: «its `--json` document `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, one per cause, the cause one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, with stderr empty»
- line: .andromeda/design-system.md:340
  old: «or under `--json` the `instance-unreachable` document, stderr empty.»
  new: «or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty.»
- line: .andromeda/design-system.md:342
  old: «or under `--json` the `instance-unreachable` document, stderr empty; no frame»
  new: «or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty; no frame»

## Patterns
[[pattern]]
id = "ds1-01"
fixed = 'fields are the whole answer'

[[pattern]]
id = "ds1-02"
fixed = 'a per-cause `detail` code is pending'

[[pattern]]
id = "ds1-03"
fixed = 'after a passed preflight prints no'

[[pattern]]
id = "ds1-04"
fixed = 'document only for `viola revive`'

[[pattern]]
id = "ds1-05"
fixed = 'its `--json` document with stderr empty'

[[pattern]]
id = "ds1-06"
fixed = 'the `instance-unreachable` document, stderr empty.'

[[pattern]]
id = "ds1-07"
fixed = 'the `instance-unreachable` document, stderr empty; no frame'
