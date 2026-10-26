# Apply — design-system

## Verdicts
- fanout-design-system--cascade.md 1 · applied · .andromeda/design-system-detail/surface.md:76+#1
- fanout-design-system--cascade.md 2 · applied · .andromeda/design-system-detail/surface.md:77+#1
- fanout-design-system--cascade.md 3 · applied · .andromeda/design-system-detail/surface.md:112+#1
- fanout-design-system--cascade.md 4 · applied · .andromeda/design-system-detail/surface.md:73#1

## Edits
- line: .andromeda/design-system-detail/surface.md:73
  old: «The stale-heartbeat and unwrapped-name causes have no code yet.»
  new: «Of them, `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause` and `release` at their exit-21 server-verification refusal, whose `--json` document stays `detail:null`, as the not-running cause's does. The stale-heartbeat and unwrapped-name causes have no code yet.»
- after: .andromeda/design-system-detail/surface.md:76
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cli-send -->»
  add: «»
  add: «- The server-verification hint of `instance-unreachable` is built for `send`, `wait`, `last`, `answer`, `pause` and `release`, the six verbs that write a frame, as the hint cause `server-verify` beside `not-running`: it prints the server-verification line above byte for byte, plain ASCII with no SGR, and it holds no pid, start time, path or endpoint.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/design-system-detail/surface.md:77
  anchor: «»
  next: «<!-- detail: values · cli-verbs -->»
  add: «<!-- detail: values · cli-send -->»
  add: «»
  add: «- On macOS no peer pid is available, so the server check passes with no comparison and the server-verification hint cannot print there (a recorded gap; owner: the route entry "Unix endpoint and home hardening").»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/design-system-detail/surface.md:112
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cli-exit-codes -->»
  add: «»
  add: «- Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause` and `release`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.»
  add: «»
  add: «<!-- /detail -->»

## Patterns
(none)
