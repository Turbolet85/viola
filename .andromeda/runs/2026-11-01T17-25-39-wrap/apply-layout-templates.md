# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · applied · .andromeda/layout-templates.md:329#1
- fanout-layout-templates.md 2 · applied · .andromeda/layout-templates.md:331#1
- fanout-layout-templates.md 3 · applied · .andromeda/layout-templates.md:331#2
- fanout-layout-templates.md 4 · applied · .andromeda/layout-templates.md:332#1
- fanout-layout-templates.md 5 · applied · .andromeda/layout-templates.md:262#1

## Edits
- line: .andromeda/layout-templates.md:329
  old: «no line, no child; and `viola list`'s exit-21»
  new: «no line, no child; and, on Unix, the socket-directory refusal of the `viola run` and `viola revive` starts, exit 1, made before any snapshot, child or bind, stdout empty and the directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` (under `revive --json` its document, Output structure — `viola revive`); and `viola list`'s exit-21»
- line: .andromeda/layout-templates.md:331
  old: «with stderr empty. There is no hint for the `unknown` refusal»
  new: «with stderr empty. On Unix a frame verb meeting a refused socket directory writes no frame and exits 21 with the same `unable  <name>  instance-unreachable` line (`send` in its `[/ ] unable` mirror) and `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` as the last stderr line; under `--json` it prints the exit-21 document `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` with stderr empty. There is no hint for the `unknown` refusal»
- line: .andromeda/layout-templates.md:331
  old: «On Linux and Windows (macOS on the recorded gap, its server check making no comparison) a frame verb»
  new: «On Linux, Windows and macOS (on macOS an euid mismatch alone, the pid and start-time comparison absent, the named carve-out, owned by no route entry) a frame verb»
- line: .andromeda/layout-templates.md:332
  old: «`endpoint-lost`, `server-verify-failed`.»
  new: «`endpoint-lost`, `server-verify-failed`, `socket-dir-refused`.»
- line: .andromeda/layout-templates.md:262
  old: «`squatted-name`, or `internal-error` for a program not found»
  new: «`squatted-name`, `socket-dir-refused` (without `--json` the socket-directory pair of Primary content block 2), or `internal-error` for a program not found»

## Patterns
[[pattern]]
id = "lt-01"
fixed = 'macOS on the recorded gap, its server check making no comparison'

[[pattern]]
id = "lt-02"
fixed = '`endpoint-lost`, `server-verify-failed`.'

[[pattern]]
id = "lt-03"
fixed = '`squatted-name`, or `internal-error`'
