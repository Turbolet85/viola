# Apply — architecture · part 1 of 2

## Verdicts
- fanout-architecture--cascade.md 12 · applied · .andromeda/architecture.md:100#1
- fanout-architecture.md 5 · folded into fanout-architecture--cascade.md 12
- fanout-architecture--cascade.md 24 · applied · .andromeda/architecture.md:325#1
- fanout-architecture.md 6 · folded into fanout-architecture--cascade.md 24
- fanout-architecture.md 1 · applied · .andromeda/architecture.md:51#1
- fanout-architecture--cascade.md 7 · folded into fanout-architecture.md 1
- fanout-architecture.md 2 · applied · .andromeda/architecture.md:193#1
- fanout-architecture--cascade.md 19 · folded into fanout-architecture.md 2
- fanout-architecture.md 3 · applied · .andromeda/architecture.md:220#1
- fanout-architecture--cascade-new-standard-contracts.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:79+#1, .andromeda/architecture.md:459#1
- fanout-architecture--cascade.md 13 · applied · .andromeda/architecture-detail/standard-contracts.md:41+#1
- fanout-architecture--cascade.md 14 · applied · .andromeda/architecture-detail/standard-contracts.md:41+#2
- fanout-architecture--cascade.md 15 · applied · .andromeda/architecture-detail/standard-contracts.md:41+#3
- fanout-architecture--cascade.md 16 · applied · .andromeda/architecture-detail/standard-contracts.md:41+#4
- fanout-architecture--cascade.md 17 · applied · .andromeda/architecture-detail/standard-contracts.md:47#1
- fanout-architecture--cascade.md 18 · applied · .andromeda/architecture-detail/standard-contracts.md:61+#1
- cascade-compare-architecture.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:59#1
- fanout-architecture--cascade.md 1 · applied · .andromeda/architecture-detail/established-decisions.md:149#1
- fanout-architecture--cascade.md 2 · applied · .andromeda/architecture-detail/established-decisions.md:149#2
- fanout-architecture--cascade.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:150#1
- fanout-architecture--cascade.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:150+#1
- fanout-architecture--cascade.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:150+#2
- fanout-architecture--cascade.md 6 · applied · .andromeda/architecture-detail/established-decisions.md:150+#3

## Edits
- line: .andromeda/architecture.md:100
  old: «a refused strict-modes check of `viola list` or `viola plugin install`.»
  new: «a refused strict-modes check of `viola list`, `viola plugin install`, `viola link` or `viola unlink`.»
- line: .andromeda/architecture.md:325
  old: «`viola list`, `viola revive` and `hook statusline` check these files»
  new: «`viola list`, `viola revive`, `hook statusline`, `viola link` and `viola unlink` check these files»
- line: .andromeda/architecture.md:51
  old: «only the implicit link is built, the explicit verbs held.»
  new: «the implicit link and the explicit verbs are built, the verbs checking the driven instance's files before any log and its wrapper serving `link` / `unlink`.»
- line: .andromeda/architecture.md:193
  old: «`link` and `unlink` `{driver, driven}`, sent to the driven instance's wrapper → `{}`. Neither is built as of chunk 2026-10-10-session-links, which landed the implicit link alone: the wrapper answers both `-32601`, and the explicit verbs are held for the founder's word on the boundary widening they need ([Links]).»
  new: «`link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}`, each sent to the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null` ([Links]).»
- line: .andromeda/architecture.md:220
  old: «Since chunk 2026-10-10-session-links the driven instance's wrapper also appends `link` (`source: wrapper`, `{driver, driven}`), once per new `driver → driven` pair on an implicit link (an issued `send` or a taken `answer` from another instance, [Links]). It is log-only and never wakes `wait` either, and it is how the audit trail and a replay learn the link set. No product path appends `unlink` yet: its one writer, `LinkSet::remove`, has no caller until the `unlink` method is served.»
  new: «The driven instance's wrapper also appends `link` (`source: wrapper`, `{driver, driven}`) once per new `driver → driven` pair, on an implicit link (an issued `send` or a taken `answer` from another instance, [Links]) or a served `link` call, and `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair a served `unlink` call removes (`LinkSet::remove`). Both are log-only and never wake `wait` either, and they are how the audit trail and a replay learn the link set.»
- line: .andromeda/architecture.md:459
  old: «contract-event-data · contract-hook-statusline»
  new: «contract-event-data · contract-cli-json-output · contract-hook-statusline»
- after: .andromeda/architecture-detail/standard-contracts.md:79
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-cli-json-output -->»
  add: «»
  add: «- `viola link <driver> <driven> --json` prints `{"v":1,"ok":{"since":…,"already_linked":…}}` (`since` a string, `already_linked` a boolean) and `viola unlink <driver> <driven> --json` prints `{"v":1,"ok":{}}`, each with exit 0. Their refused strict-modes check on the home and `instances/<driven>`, made before any role file or log opens, prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21), with stderr empty and no line in any log.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/standard-contracts.md:41
  anchor: «- `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`. `since` is the `link` line's `ts`. As landed (chunk 2026-10-10-session-links) the wrapper holds the set in memory (`LinkSet`, `src/run/links.rs`, one per wrapper process, under one lock), restores it at every start, a revived one included, by one pass over `events.ndjson` through `events::read_from` and the rule `replay::apply_link_line` before `server.serve`, writes it as the first snapshot's `links` in place of an empty list, and rewrites `links` through `Snapshots::update` at each change, after the line has landed. The restore appends no line and logs nothing. The set follows the log: a failed append leaves it as it was, and a line that landed is in the set even when the snapshot write then fails.»
  add: «- The set changes on an implicit link and on a served `link` / `unlink` call: `LinkSet::link` adds the pair through `record`, and `LinkSet::unlink` removes it through `remove`.»
- after: .andromeda/architecture-detail/standard-contracts.md:41
  anchor: «- `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`. `since` is the `link` line's `ts`. As landed (chunk 2026-10-10-session-links) the wrapper holds the set in memory (`LinkSet`, `src/run/links.rs`, one per wrapper process, under one lock), restores it at every start, a revived one included, by one pass over `events.ndjson` through `events::read_from` and the rule `replay::apply_link_line` before `server.serve`, writes it as the first snapshot's `links` in place of an empty list, and rewrites `links` through `Snapshots::update` at each change, after the line has landed. The restore appends no line and logs nothing. The set follows the log: a failed append leaves it as it was, and a line that landed is in the set even when the snapshot write then fails.»
  add: «- A `link` of a pair the set holds appends nothing and answers that pair's `since` with `already_linked: true`; a new pair answers its new `link` line's `ts` as `since`, with `already_linked: false`.»
- after: .andromeda/architecture-detail/standard-contracts.md:41
  anchor: «- `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`. `since` is the `link` line's `ts`. As landed (chunk 2026-10-10-session-links) the wrapper holds the set in memory (`LinkSet`, `src/run/links.rs`, one per wrapper process, under one lock), restores it at every start, a revived one included, by one pass over `events.ndjson` through `events::read_from` and the rule `replay::apply_link_line` before `server.serve`, writes it as the first snapshot's `links` in place of an empty list, and rewrites `links` through `Snapshots::update` at each change, after the line has landed. The restore appends no line and logs nothing. The set follows the log: a failed append leaves it as it was, and a line that landed is in the set even when the snapshot write then fails.»
  add: «- `unlink` answers `{}` whether or not the set held the pair; an absent pair appends nothing and leaves the set as it was.»
- after: .andromeda/architecture-detail/standard-contracts.md:41
  anchor: «- `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}`. `since` is the `link` line's `ts`. As landed (chunk 2026-10-10-session-links) the wrapper holds the set in memory (`LinkSet`, `src/run/links.rs`, one per wrapper process, under one lock), restores it at every start, a revived one included, by one pass over `events.ndjson` through `events::read_from` and the rule `replay::apply_link_line` before `server.serve`, writes it as the first snapshot's `links` in place of an empty list, and rewrites `links` through `Snapshots::update` at each change, after the line has landed. The restore appends no line and logs nothing. The set follows the log: a failed append leaves it as it was, and a line that landed is in the set even when the snapshot write then fails.»
  add: «- A `link` or `unlink` answered `-32602` (its causes: §Standard Contracts, Channel methods) writes no line, and a failed record or remove is `-32603`.»
- line: .andromeda/architecture-detail/standard-contracts.md:47
  old: «`answer`, `pause`, `release`, `hook.event`»
  new: «`answer`, `pause`, `release`, `link`, `unlink`, `hook.event`»
- after: .andromeda/architecture-detail/standard-contracts.md:61
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-channel-methods -->»
  add: «»
  add: «- `link` and `unlink` read no `from`: a `from` of any shape changes nothing. Each answers only after its event line and the snapshot write.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/standard-contracts.md:59
  old: «(the wrapper's `send-*`, `link` and start records never wake)»
  new: «(the wrapper's `send-*`, `link`, `unlink` and start records never wake)»
- line: .andromeda/architecture-detail/established-decisions.md:149
  old: «As landed (chunk 2026-10-10-session-links) only the implicit link is built.»
  new: «Both are built: the driven instance's wrapper serves the `link` and `unlink` methods (`LinkSet::link` / `LinkSet::unlink`, sharing `driver_of`), sent by the verbs `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, and records the implicit link itself.»
- line: .andromeda/architecture-detail/established-decisions.md:149
  old: «of an existing pair adds none and returns `{}`, so»
  new: «of an existing pair adds none and answers its held `since` with `already_linked: true` (§Standard Contracts, Instance snapshot), so»
- line: .andromeda/architecture-detail/established-decisions.md:150
  old: «The explicit `viola link` / `viola unlink` and their two channel methods are not built: the wrapper answers `link` and `unlink` `-32601`, no subcommand exists, and no product path appends an `unlink` event (`LinkSet::remove` is unit-tested and has no caller). They are held: the boundary widening they carry is not ratified (the wrapper serving two more methods, the two verbs writing their frame after the liveness-only pre-check, and their snapshot read without the strict-modes check), and the founder's own answer of 2026-10-13, relayed verbatim by the Viola overseer, is that no explicit verb is built before Epoch 6, the three crossings staying unbuilt and unratified. The Epoch 6 working-route entry "Explicit session links", after "Windows endpoint admission" and "Server verification before any frame", carries the hold (moved there from the end of Epoch 5 by that answer; first moved from "The board: viola list" by the review card the founder approved, chunk 2026-10-11-the-board-viola-list).»
  new: «The explicit `viola link` / `viola unlink` and their two channel methods are built, with no dated gap: the wrapper serves `link` and `unlink`, and the `unlink` method is `LinkSet::remove`'s product caller, so a product path appends `unlink` event lines (`source: wrapper`, `{driver, driven}`).»
- after: .andromeda/architecture-detail/established-decisions.md:150
  anchor: « The explicit `viola link` / `viola unlink` and their two channel methods are not built: the wrapper answers `link` and `unlink` `-32601`, no subcommand exists, and no product path appends an `unlink` event (`LinkSet::remove` is unit-tested and has no caller). They are held: the boundary widening they carry is not ratified (the wrapper serving two more methods, the two verbs writing their frame after the liveness-only pre-check, and their snapshot read without the strict-modes check), and the founder's own answer of 2026-10-13, relayed verbatim by the Viola overseer, is that no explicit verb is built before Epoch 6, the three crossings staying unbuilt and unratified. The Epoch 6 working-route entry "Explicit session links", after "Windows endpoint admission" and "Server verification before any frame", carries the hold (moved there from the end of Epoch 5 by that answer; first moved from "The board: viola list" by the review card the founder approved, chunk 2026-10-11-the-board-viola-list).»
  add: «- The explicit verbs run `viola_state::strict::check_instance` on the home and `instances/<driven>` before `client::start`, so ahead of any role file, log or snapshot read, because opening a log narrows an existing home to 0700 and a check made after it passes a home at 0770; a refusal is exit 21. Then come the liveness-only pre-check (`live_endpoint`), the server identity check through `cmd::client::connect`, and one request.»
- after: .andromeda/architecture-detail/established-decisions.md:150
  anchor: « The explicit `viola link` / `viola unlink` and their two channel methods are not built: the wrapper answers `link` and `unlink` `-32601`, no subcommand exists, and no product path appends an `unlink` event (`LinkSet::remove` is unit-tested and has no caller). They are held: the boundary widening they carry is not ratified (the wrapper serving two more methods, the two verbs writing their frame after the liveness-only pre-check, and their snapshot read without the strict-modes check), and the founder's own answer of 2026-10-13, relayed verbatim by the Viola overseer, is that no explicit verb is built before Epoch 6, the three crossings staying unbuilt and unratified. The Epoch 6 working-route entry "Explicit session links", after "Windows endpoint admission" and "Server verification before any frame", carries the hold (moved there from the end of Epoch 5 by that answer; first moved from "The board: viola list" by the review card the founder approved, chunk 2026-10-11-the-board-viola-list).»
  add: «- The wrapper opens no span and writes no process-log line of its own for `link` / `unlink`: its `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name. The verbs run inside the `cli.link` / `cli.unlink` dispatch span.»
- after: .andromeda/architecture-detail/established-decisions.md:150
  anchor: « The explicit `viola link` / `viola unlink` and their two channel methods are not built: the wrapper answers `link` and `unlink` `-32601`, no subcommand exists, and no product path appends an `unlink` event (`LinkSet::remove` is unit-tested and has no caller). They are held: the boundary widening they carry is not ratified (the wrapper serving two more methods, the two verbs writing their frame after the liveness-only pre-check, and their snapshot read without the strict-modes check), and the founder's own answer of 2026-10-13, relayed verbatim by the Viola overseer, is that no explicit verb is built before Epoch 6, the three crossings staying unbuilt and unratified. The Epoch 6 working-route entry "Explicit session links", after "Windows endpoint admission" and "Server verification before any frame", carries the hold (moved there from the end of Epoch 5 by that answer; first moved from "The board: viola list" by the review card the founder approved, chunk 2026-10-11-the-board-viola-list).»
  add: «- `LinkSet::line` stamps the `link` line's `ts`, the pair's `since`, with `Utc::now()` directly, outside the `Clock` seam (`src/run/links.rs:92`).»

## Patterns
[[pattern]]
id = "arch1-01"
fixed = 'strict-modes check of `viola list` or `viola plugin install`'

[[pattern]]
id = "arch1-02"
fixed = '`viola revive` and `hook statusline` check these files'

[[pattern]]
id = "arch1-03"
fixed = 'the explicit verbs held'

[[pattern]]
id = "arch1-04"
fixed = 'the wrapper answers both `-32601`'

[[pattern]]
id = "arch1-05"
fixed = 'No product path appends `unlink` yet'

[[pattern]]
id = "arch1-06"
fixed = '`pause`, `release`, `hook.event` and `hook.dialog`'

[[pattern]]
id = "arch1-07"
fixed = '`send-*`, `link` and start records never wake'

[[pattern]]
id = "arch1-08"
fixed = 'only the implicit link is built'

[[pattern]]
id = "arch1-09"
fixed = 'of an existing pair adds none and returns `{}`'

[[pattern]]
id = "arch1-10"
fixed = 'channel methods are not built'
