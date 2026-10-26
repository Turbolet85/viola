# Apply — obs-plan · part 1 of 2

## Verdicts
- fanout-obs-plan.md 1 · applied · .andromeda/obs-plan.md:392#1
- fanout-obs-plan--cascade.md 14 · folded into fanout-obs-plan.md 1
- fanout-obs-plan.md 2 · applied · .andromeda/obs-plan.md:311#1
- fanout-obs-plan.md 3 · applied · .andromeda/obs-plan.md:418#1
- fanout-obs-plan--cascade.md 16 · folded into fanout-obs-plan.md 3
- fanout-obs-plan--cascade.md 1 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:158#1
- fanout-obs-plan--cascade.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:397#1
- fanout-obs-plan--cascade.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:488#1
- fanout-obs-plan--cascade.md 4 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:487#1
- fanout-obs-plan--cascade.md 5 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:497#1
- cascade-compare-obs-plan.md 1 · folded into fanout-obs-plan--cascade.md 12
- fanout-obs-plan--cascade.md 6 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:20#1
- fanout-obs-plan--cascade.md 7 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:8#1
- fanout-obs-plan--cascade.md 8 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:18#1
- fanout-obs-plan--cascade.md 9 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:28#1
- fanout-obs-plan--cascade.md 10 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:177#1, .andromeda/obs-plan-detail/span-trace-coverage.md:177#2
- fanout-obs-plan--cascade.md 11 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:179+#1
- fanout-obs-plan--cascade.md 12 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:179+#2
- fanout-obs-plan--cascade.md 13 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:179+#3
- fanout-obs-plan--cascade.md 15 · applied · .andromeda/obs-plan-detail/metric-coverage.md:28#1

## Edits
- line: .andromeda/obs-plan.md:392
  old: «only the implicit link exists, recorded by the driven wrapper inside an issued `send` or a taken `answer`; it has no process-log line, no `event` value and no span, and the explicit `link` / `unlink` methods are not built.»
  new: «the implicit link (recorded by the driven wrapper inside an issued `send` or a taken `answer`) and the explicit `link` / `unlink` methods the driven wrapper serves have no process-log line, no `event` value and no span of their own; their trace is the call's `channel-request` / `channel-response` pair, the product `link` / `unlink` line in `events.ndjson` and the snapshot's `links`; the `viola link` / `viola unlink` verbs log into the driven instance's `cli-<driven>.ndjson`, their request inside `cli.link` / `cli.unlink`.»
- line: .andromeda/obs-plan.md:311
  old: «`revive` (1) and `plugin install` (1), as re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-17-optional-plugin-install; that chunk's `list` colour layer»
  new: «`revive` (1), `plugin install` (1) and `link` (1, the one site `viola link` / `viola unlink` share), as re-derived by `grep -rcE 'human::refuse\(' src` at chunk 2026-10-26-explicit-session-links; `list`'s colour layer»
- line: .andromeda/obs-plan.md:418
  old: «`send`'s invalid params and `release` carrying `from`, until»
  new: «`send`'s invalid params, `release` carrying `from` and a bad `link` / `unlink` name or pair, until»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:158
  old: «a refused check opens none; chunk 2026-10-17-cli-machine-contract). `viola revive`'s start arm»
  new: «a refused check opens none; chunk 2026-10-17-cli-machine-contract). `viola link` / `viola unlink` log as `cli` into the driven instance's `cli-<driven>.ndjson` (the instance their `<driven>` argument names, never the caller's `VIOLA_NAME`) once their instance check of it has passed; a refused check opens none. `viola revive`'s start arm»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:397
  old: «Four strict-modes refusals leave no line: `viola revive`'s instance check, `viola hook statusline`'s instance check,»
  new: «Five strict-modes refusals leave no line: `viola revive`'s instance check, `viola hook statusline`'s instance check, `viola link` / `viola unlink`'s instance check of the driven instance (made before any role file opens; exit 21),»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:488
  old: «by `send`, `wait`, `last`, `answer`, `pause` and `release`, distinct»
  new: «by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (for `link` / `unlink` the driven instance's file), distinct»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:487
  old: «`{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor).»
  new: «`{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor). `viola link` / `viola unlink`'s strict-mode fail (their instance check of the driven instance) carries no logged `detail` either (no line): its output is the fixed instance-state `unable` / `hint:` pair on stderr, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, exit 21.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:497
  old: «the code also carries `send`'s invalid params and `release` carrying `from`, so»
  new: «the code also carries `send`'s invalid params, `release` carrying `from`, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`, so»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:20
  old: «or a refused `viola list` scan, which opens none either)»
  new: «or a refused `viola list` scan, which opens none either, or a refused instance check on `viola link` / `viola unlink`, made before their role file opens)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:8
  old: «`release.client`, `channel.request` (client side)»
  new: «`release.client`, `cli.link`, `cli.unlink`, `channel.request` (client side)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:18
  old: «opens no process log and writes no line. Client-side»
  new: «opens no process log and writes no line. `viola link` / `viola unlink` (spans `cli.link` / `cli.unlink` on their one request) run their instance check of the driven instance before any role file opens, then open the driven instance's `cli-<driven>.ndjson` and write `process-start` / `process-exit`; a refused check (exit 21) opens no process log and writes no line. Client-side»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:28
  old: «each of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) verifies the wrapper through `cmd::client::connect` inside its dispatch span, after its own client-side refusals and before any frame»
  new: «each of the eight frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) verifies the wrapper through `cmd::client::connect` inside its dispatch span, after its own client-side refusals (for `link` / `unlink`, their instance check of the driven instance, then the liveness pre-check) and before any frame»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:177
  old: «As landed (chunk 2026-10-10-session-links) only the implicit link exists. The driven wrapper records it inside»
  new: «The driven wrapper records the implicit link inside»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:177
  old: «belongs to the explicit methods, which are not built: neither is served (both answer `-32601`), and no product path appends an `unlink` line.»
  new: «belongs to the explicit methods, which the driven wrapper serves (the group below).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:179
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · paths -->»
  add: «»
  add: «- **E3**, the explicit methods: the driven wrapper serves `link` / `unlink`, reading `driver` and `driven` as strings through `ViolaName::try_new`; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null` and no line written, a `record` / `remove` error is `-32603`, and neither method reads `from`.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:179
  anchor: «<!-- /detail -->»
  add: «  - Neither method opens a span or writes a process-log line of its own: its trace is the driven wrapper's `channel-request` / `channel-response` pair with `method:"link"|"unlink"`, which holds neither the `driver` nor the `driven` name, then the product line and the snapshot's `links`.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:179
  anchor: «<!-- /detail -->»
  add: «  - `link` appends one `link` product line `{driver, driven}` (`source: wrapper`) for a new pair and nothing for a pair the set holds; `unlink`, `LinkSet::remove`'s one product caller, appends one `unlink` product line `{driver, driven}` (`source: wrapper`) only when it removes a pair and nothing for an absent one; each answers after its line and the snapshot write.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/obs-plan-detail/metric-coverage.md:28
  old: «`"invalid params"`) and `release` carrying `from`, and the client's»
  new: «`"invalid params"`), `release` carrying `from` and a `link` / `unlink` with a missing, non-string or invalid name, a `driven` that is not the serving wrapper or a `driver` equal to `driven`, and the client's»

## Patterns
[[pattern]]
id = "obs1-01"
fixed = "the explicit `link` / `unlink` methods are not built"

[[pattern]]
id = "obs1-02"
fixed = "`revive` (1) and `plugin install` (1)"

[[pattern]]
id = "obs1-03"
fixed = "`send`'s invalid params and `release` carrying `from`, until"

[[pattern]]
id = "obs1-04"
fixed = "Four strict-modes refusals leave no line"

[[pattern]]
id = "obs1-05"
fixed = "`answer`, `pause` and `release`, distinct from"

[[pattern]]
id = "obs1-06"
fixed = "invalid params and `release` carrying `from`, so it is not"

[[pattern]]
id = "obs1-07"
fixed = "each of the six frame verbs"

[[pattern]]
id = "obs1-08"
fixed = "only the implicit link exists"

[[pattern]]
id = "obs1-09"
fixed = "which are not built: neither is served"

[[pattern]]
id = "obs1-10"
fixed = '`"invalid params"`) and `release` carrying `from`'
