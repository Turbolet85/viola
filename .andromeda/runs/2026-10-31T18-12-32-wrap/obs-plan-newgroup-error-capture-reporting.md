# New groups — obs-plan · error-capture-reporting

## Verdicts
- fanout-obs-plan--cascade-g4.md 12 · contradiction · lines 16–16 · «exactly `error: internal error`» · the file gives `revive <name> --list`'s catch-site line as exactly this with no `--json` exception, the proposal prints the `internal-error` document in its place under `--json`, stderr empty
- fanout-obs-plan--cascade-g4.md 13 · distinct · no group states the `internal-error` document for `revive --json`'s anyhow start errors; line 16 holds only the refused check's stderr pair
- fanout-obs-plan--cascade-g4.md 14 · distinct · no group states that a `pump_child` error under `revive --json` prints no document
- fanout-obs-plan--cascade-g4.md 15 · distinct · no group names `refused_with` or the start refusals' documents under `--json`; line 50 holds only the fixed messages of two `AgentError` variants
- fanout-obs-plan--cascade-g4.md 19 · contradiction · lines 40–40 · «a refusal returns `instance-unreachable` with `detail: null`» · the file gives a refused `check_instance` in `frame` a null detail, the proposal maps it to the cause `StrictModes`
- cascade-compare-obs-plan.md 1 · contradiction · lines 27–27 · «{"v":1,"error":"instance-unreachable","detail":null}» · the file's exit-21 document for `link` / `unlink` carries a null detail, the proposal's `detail` is one of five closed cause words
- cascade-compare-obs-plan.md 2 · distinct · no group states `without_endpoint`'s reading; line 40 holds only the check before `live_endpoint` in `frame`
- cascade-compare-obs-plan.md 3 · twin · fanout-obs-plan--cascade-g4.md 15 · both state that `refused_with` prints the start refusal documents in place of the human pair, before `spawn_child`
