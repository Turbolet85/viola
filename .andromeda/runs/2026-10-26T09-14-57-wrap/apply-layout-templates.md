# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · applied · .andromeda/layout-templates.md:329#1

## Edits
- line: .andromeda/layout-templates.md:329
  old: «`wait` / `last`'s exit 21 `unable  <name>  instance-unreachable` takes `hint: <name> is not running; viola list shows the live instances` (`viola send`'s exit 21 keeps its `[/ ] unable` mirror with the same hint).»
  new: «`wait` / `last`'s exit 21 `unable  <name>  instance-unreachable` takes `hint: <name> is not running; viola list shows the live instances` when the instance is not running (`viola send`'s exit 21 keeps its `[/ ] unable` mirror with the same hint). On Linux and Windows (macOS on the recorded gap, its server check making no comparison) a frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) whose server check finds that the process on the endpoint is not the recorded wrapper, or cannot make the check, writes no frame and exits 21 with the same `unable  <name>  instance-unreachable` line (`send` in its `[/ ] unable` mirror) and `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` it prints the exit-21 document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.»

## Patterns
[[pattern]]
id = "lt-01"
fixed = 'instance-unreachable` takes `hint: <name> is not running'
