# Apply — architecture · part 2 of 2

## Verdicts
- fanout-architecture--cascade.md 8 · applied · .andromeda/architecture-detail/conventions.md:26#1, .andromeda/architecture-detail/conventions.md:26#2
- fanout-architecture--cascade.md 9 · applied · .andromeda/architecture-detail/conventions.md:33#1
- fanout-architecture--cascade.md 10 · applied · .andromeda/architecture-detail/conventions.md:34#1
- fanout-architecture--cascade.md 11 · applied · .andromeda/architecture-detail/conventions.md:40#1, .andromeda/architecture-detail/conventions.md:40#2
- fanout-architecture--cascade.md 20 · applied · .andromeda/architecture-detail/occupied-resources.md:14#1
- fanout-architecture--cascade.md 21 · applied · .andromeda/architecture-detail/occupied-resources.md:97#1, .andromeda/architecture-detail/occupied-resources.md:97#2, .andromeda/architecture-detail/occupied-resources.md:97#3
- fanout-architecture--cascade.md 22 · applied · .andromeda/architecture-detail/occupied-resources.md:105#1
- fanout-architecture--cascade.md 23 · applied · .andromeda/architecture-detail/occupied-resources.md:106#1, .andromeda/architecture-detail/occupied-resources.md:106#2
- cascade-compare-architecture.md 1 · applied · .andromeda/architecture-detail/occupied-resources.md:31+#1
- fanout-architecture.md 4 · applied · .andromeda/registries/contracts/architecture/project-directory-structure.md:49#1

## Edits
- line: .andromeda/architecture-detail/conventions.md:26
  old: «`pause`, `release`, `verify`, `revive --list` and `plugin install`)»
  new: «`pause`, `release`, `link`, `unlink`, `verify`, `revive --list` and `plugin install`)»
- line: .andromeda/architecture-detail/conventions.md:26
  old: «(those are exit 21); for `viola list`»
  new: «(those are exit 21); for `viola link` / `viola unlink` exit 1 also covers a refusal reply and an `ok` of another shape than the verb's (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object); for `viola list`»
- line: .andromeda/architecture-detail/conventions.md:33
  old: «`20` protocol fault from the wrapper»
  new: «`20` protocol fault from the wrapper, a `-32602` from `link` / `unlink` included (a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven`: the self-pair is refused by the wrapper only, never client-side)»
- line: .andromeda/architecture-detail/conventions.md:34
  old: «(human mode the home-state pair; nothing created, no log opened)»
  new: «(human mode the home-state pair; nothing created, no log opened); `viola link` / `viola unlink`'s refused instance check of the driven instance (`check_instance` on the home and `instances/<driven>`, before any role file or log opens) is the same exit and document (human mode the instance-state `unable:` / `hint:` pair; no log opened, no line written, the mode left as found): exit 21 as for `list` and `plugin install`, not `revive`'s exit 1»
- line: .andromeda/architecture-detail/conventions.md:40
  old: «on `send`, `wait`, `last`, `answer`, `pause` and `release`, a server»
  new: «on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, a server»
- line: .andromeda/architecture-detail/conventions.md:40
  old: «The client's own refusals (exit 13) come first, and the cause»
  new: «The client's own refusals (exit 13), and for `link` / `unlink` the instance check of the driven instance (exit 21), come first, before any connect, and the cause»
- line: .andromeda/architecture-detail/occupied-resources.md:14
  old: «`pause` / `release`, and `mcp`'s four frame tools»
  new: «`pause` / `release` / `link` / `unlink`, and `mcp`'s four frame tools»
- line: .andromeda/architecture-detail/occupied-resources.md:97
  old: «`pause`, `release`, `revive --list` and `plugin install` have producers today»
  new: «`pause`, `release`, `link`, `unlink`, `revive --list` and `plugin install` have producers today»
- line: .andromeda/architecture-detail/occupied-resources.md:97
  old: «`send` / `wait` / `last` / `answer` / `pause` / `release` — `<name>` the target instance — write»
  new: «`send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink` — `<name>` the target instance, the driven one for `link` / `unlink` — write»
- line: .andromeda/architecture-detail/occupied-resources.md:97
  old: «exit 20 with `detail:"wrapper-fault"`;»
  new: «exit 20 with `detail:"wrapper-fault"`; a refused `link` / `unlink` instance check opens no role file and writes no line;»
- line: .andromeda/architecture-detail/occupied-resources.md:105
  old: «Three readers check these files before they use them.»
  new: «Four readers check these files before they use them (`viola list`, `viola revive`, `hook statusline`, and `viola link` / `viola unlink` sharing one check).»
- line: .andromeda/architecture-detail/occupied-resources.md:106
  old: «on any line. Both checks come first»
  new: «on any line. `viola link` / `viola unlink` run `check_instance` on the home and `instances/<driven>` before `client::start`, so before any role file or log opens and before any snapshot value is read; a refusal is exit 21 (`--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document on stdout, stderr empty; human the instance-state `unable:` / `hint:` pair on stderr), no line written and the mode left as found. Each of these checks comes first»
- line: .andromeda/architecture-detail/occupied-resources.md:106
  old: «`check_instance` has these two product call sites (the rule: `check_instance(` under `src/`).»
  new: «`check_instance` has these three product call sites (`src/cmd/hook/statusline.rs`, `src/cmd/revive.rs` and `src/cmd/link.rs`, the one site both link verbs share; the rule: `check_instance(` under `src/`).»
- after: .andromeda/architecture-detail/occupied-resources.md:31
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · res-binaries -->»
  add: «»
  add: «- `link` and `unlink` each take `<driver> <driven> [--json]` (one shared `LinkArgs`), the verb's instance being the driven one; each exits 0 with its one result line, or `{"v":1,"ok":<ok>}` under `--json` (`link`: `{since, already_linked}`; `unlink`: `{}`), 21 when the instance check refuses, no live endpoint is recorded or the serving process is not the recorded wrapper, 20 for a wrapper error (a `-32602` included, such as `driver` equal to `driven`), and 1 for a refusal reply or an `ok` of another shape.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/registries/contracts/architecture/project-directory-structure.md:49
  old: «`record`, and `remove`, which has no product caller yet)»
  new: «`record`, `remove` (its product caller the served `unlink` method), and the served `link` / `unlink` channel methods, sharing `driver_of`)»

## Patterns
[[pattern]]
id = "arch2-01"
fixed = '`release`, `verify`, `revive --list` and `plugin install`'

[[pattern]]
id = "arch2-02"
fixed = '`answer`, `pause` and `release`, a server that is not the recorded wrapper'

[[pattern]]
id = "arch2-03"
fixed = '`answer` / `pause` / `release`, and'

[[pattern]]
id = "arch2-04"
fixed = '`release`, `revive --list` and `plugin install` have producers'

[[pattern]]
id = "arch2-05"
fixed = '`pause` / `release` — `<name>` the target instance'

[[pattern]]
id = "arch2-06"
fixed = 'Three readers check these files'

[[pattern]]
id = "arch2-07"
fixed = 'these two product call sites'

[[pattern]]
id = "arch2-08"
fixed = 'which has no product caller yet'
