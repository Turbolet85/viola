# Apply — design-system

## Verdicts
- fanout-design-system--cascade.md 2 · applied · .andromeda/design-system.md:261#1
- fanout-design-system--cascade.md 9 · applied · .andromeda/design-system.md:315#1
- fanout-design-system--cascade.md 13 · applied · .andromeda/design-system.md:340+#1
- fanout-design-system.md 2 · folded into fanout-design-system--cascade.md 13
- fanout-design-system.md 1 · applied · .andromeda/design-system.md:348#1
- fanout-design-system--cascade.md 16 · folded into fanout-design-system.md 1
- fanout-design-system--cascade.md 1 · applied · .andromeda/design-system-detail/surface.md:21#1
- fanout-design-system--cascade.md 3 · applied · .andromeda/design-system-detail/surface.md:37#1
- fanout-design-system--cascade.md 4 · applied · .andromeda/design-system-detail/surface.md:73#1
- fanout-design-system--cascade.md 5 · applied · .andromeda/design-system-detail/surface.md:80#1
- fanout-design-system--cascade.md 6 · applied · .andromeda/design-system-detail/surface.md:94+#1
- fanout-design-system--cascade.md 7 · applied · .andromeda/design-system-detail/surface.md:94+#2
- fanout-design-system--cascade.md 8 · applied · .andromeda/design-system-detail/surface.md:94+#3
- fanout-design-system--cascade.md 10 · applied · .andromeda/design-system-detail/surface.md:124+#1
- fanout-design-system--cascade.md 11 · applied · .andromeda/design-system-detail/surface.md:124+#2
- fanout-design-system--cascade.md 12 · applied · .andromeda/design-system-detail/surface.md:122#1, .andromeda/design-system-detail/surface.md:122#2
- fanout-design-system--cascade.md 14 · applied · .andromeda/design-system-detail/surface.md:128#1
- fanout-design-system--cascade.md 15 · applied · .andromeda/design-system-detail/surface.md:136#1

## Edits
- line: .andromeda/design-system.md:261
  old: «and `list`'s exit-21 refusal), each refusal's»
  new: «and the exit-21 strict-modes refusals of `list`, `link` and `unlink`), each refusal's»
- line: .andromeda/design-system.md:315
  old: «- Message mode, the escaper both verbs share:»
  new: «- Message mode, the escaper `wait` and `last` share and `viola link` uses for a `since` that is no RFC 3339 instant:»
- after: .andromeda/design-system.md:340
  anchor: «`viola plugin install`'s refused strict-modes check (exit 21) prints the home-state `unable:` / `hint:` pair as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty.»
  add: «»
  add: «`viola link` / `viola unlink`'s refused strict-modes check of the driven instance (exit 21) prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty.»
- line: .andromeda/design-system.md:348
  old: «`wheel: pause, release` / `setup: run, revive, verify, plugin install`, then `Options:`; the `handoff: link, unlink` row is pending, owner: the route entry "Explicit session links".»
  new: «`wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.»
- line: .andromeda/design-system-detail/surface.md:21
  old: «superseded 2026-10-13), use the fixed message form»
  new: «superseded 2026-10-13), and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance, use the fixed message form»
- line: .andromeda/design-system-detail/surface.md:37
  old: «(`revive`'s pair, shared).»
  new: «(`revive`'s pair, shared; `viola link` / `viola unlink` print it, naming the driven instance, at their exit-21 strict-modes refusal).»
- line: .andromeda/design-system-detail/surface.md:73
  old: «by `send`, `wait`, `last`, `answer`, `pause` and `release` at their exit-21»
  new: «by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21»
- line: .andromeda/design-system-detail/surface.md:80
  old: «`pause` and `release`, the six verbs that write a frame»
  new: «`pause`, `release`, `link` and `unlink`, the eight verbs that write a frame»
- after: .andromeda/design-system-detail/surface.md:94
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cli-verbs -->»
  add: «»
  add: «- `viola link` / `viola unlink` print their one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0; the names printed are the verb's own parsed arguments, never names read back from the reply or the log.»
- after: .andromeda/design-system-detail/surface.md:94
  anchor: «<!-- /detail -->»
  add: «- The `since` of the `link` line is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through the message-mode escaper of pattern 3.»
- after: .andromeda/design-system-detail/surface.md:94
  anchor: «<!-- /detail -->»
  add: «- A second `viola link` of a held pair prints the `already linked` form, the wrapper keeping the first `since`; `viola unlink` of a pair that is not held prints the same `unlink  <driver> -> <driven>` line; both exit 0.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/design-system-detail/surface.md:122
  old: «`viola list`'s refused strict-modes check prints instead»
  new: «`viola list`'s refused strict-modes check, and `viola link` / `viola unlink`'s of the driven instance, print instead»
- line: .andromeda/design-system-detail/surface.md:122
  old: «`list`'s refusal included, with stderr empty»
  new: «the `list`, `link` and `unlink` refusals included, with stderr empty»
- after: .andromeda/design-system-detail/surface.md:124
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cli-exit-codes -->»
  add: «»
  add: «- Exit 0 under `--json`: `viola link`'s document is `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link, and `viola unlink`'s is `{"v":1,"ok":{}}`.»
- after: .andromeda/design-system-detail/surface.md:124
  anchor: «<!-- /detail -->»
  add: «- Exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/design-system-detail/surface.md:128
  old: «built for `send`, `wait`, `last`, `answer`, `pause` and `release`: a process»
  new: «built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process»
- line: .andromeda/design-system-detail/surface.md:136
  old: «`wheel: pause, release` / `setup: run, revive, verify, plugin install`, then `Options:`; the `handoff:` row joins when "Explicit session links" serves `link` and `unlink`.»
  new: «`wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.»

## Patterns
[[pattern]]
id = "ds-01"
regex = 'list.s exit-21 (strict-modes )?refusal'

[[pattern]]
id = "ds-02"
fixed = 'the escaper both verbs share'

[[pattern]]
id = "ds-03"
fixed = 'row is pending, owner: the route entry "Explicit session links"'

[[pattern]]
id = "ds-04"
fixed = 'only the exit-1 refusals of'

[[pattern]]
id = "ds-05"
fixed = '`pause` and `release` at their exit-21'

[[pattern]]
id = "ds-06"
fixed = 'six verbs that write a frame'
ignore_case = true

[[pattern]]
id = "ds-07"
fixed = 'refused strict-modes check prints instead'

[[pattern]]
id = "ds-08"
regex = 'list.s refusal included'

[[pattern]]
id = "ds-09"
fixed = '`answer`, `pause` and `release`'

[[pattern]]
id = "ds-10"
fixed = 'row joins when "Explicit session links"'
