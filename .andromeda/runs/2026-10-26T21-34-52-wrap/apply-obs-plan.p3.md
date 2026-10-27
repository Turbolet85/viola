# Apply — obs-plan · part 3 of 6

## Verdicts
- fanout-obs-plan--cascade-g3.md 1 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:20#1, .andromeda/obs-plan-detail/telemetry-strategy.md:20#2
- fanout-obs-plan--cascade-g3.md 2 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:30#1
- fanout-obs-plan--cascade-g3.md 3 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:33#1
- fanout-obs-plan--cascade-g3.md 4 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:43#1

## Edits
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:20
  old: «a failed step-4 role-file open,»
  new: «a failed step-4 role-file open (a `diagnostics/` folder or file its strict-modes check refuses included),»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:20
  old: «or a refused instance check on `viola link` / `viola unlink`, made before their role file opens)»
  new: «or a refused instance check on `viola link` / `viola unlink`, made before their role file opens, or a refused instance check on the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`), a refused or absent instance on `viola hook`'s event and dialog paths (a `VIOLA_DIR` naming no existing instance directory included), or a refused home tree on `viola run`, `viola revive`'s start or `viola verify`, each made before its role file opens)»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:30
  old: «asserted by absence (chunk 2026-10-10-budget-governor). The "server-verify»
  new: «asserted by absence (chunk 2026-10-10-budget-governor). The event and dialog paths' refused or absent instance (a `VIOLA_DIR` naming no existing instance directory included) leaves none either: exit 0 at once, no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append. The "server-verify»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:33
  old: «§9 secret-scan-before-upload |»
  new: «§9 secret-scan-before-upload, §4 Scenario 1 / §6 `process-start.sideload_fallback` `strict-modes-failed` (a `conpty/` companion its owner/DACL check refuses: the child on the inbox ConPTY, exit unchanged), §10 exemptions (a strict-modes check made before a read or open — the six frame verbs, `viola hook`'s event and dialog paths, `viola mcp`'s frame tools, `verify`, `run` / `revive`'s start, the `diagnostics/` opens — writes no line) |»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:43
  old: «| §6 detail code catalog, §12 D-01 |»
  new: «| §6 detail code catalog, §12 D-01; a strict-modes refusal made before any process log opens (the six frame verbs and `link` / `unlink`, exit 21; `run`, `revive`'s start and `verify`, exit 1) writes no line and carries no `detail` code: its machine-readable form is the exit code, and under `--json` the closed `instance-unreachable` document (the frame verbs) or `strict-modes-failed` (`revive`) |»

## Patterns
(none)
