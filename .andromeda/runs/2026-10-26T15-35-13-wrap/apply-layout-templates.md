# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · applied · .andromeda/layout-templates.md:219#1, .andromeda/layout-templates.md:219#2
- fanout-layout-templates.md 2 · applied · .andromeda/layout-templates.md:61#1
- fanout-layout-templates.md 3 · applied · .andromeda/layout-templates.md:301#1
- fanout-layout-templates.md 4 · applied · .andromeda/layout-templates.md:168+#1
- fanout-layout-templates.md 5 · applied · .andromeda/layout-templates.md:329#1
- fanout-layout-templates.md 6 · applied · .andromeda/layout-templates.md:329#2
- fanout-layout-templates.md 7 · applied · .andromeda/layout-templates.md:327#1
- fanout-layout-templates.md 8 · applied · .andromeda/layout-templates.md:327#2, .andromeda/layout-templates.md:327#3
- fanout-layout-templates.md 9 · applied · .andromeda/layout-templates.md:330#1
- fanout-layout-templates.md 10 · applied · .andromeda/layout-templates.md:48#1

## Edits
- line: .andromeda/layout-templates.md:219
  old: «the four rows `board:    list` · `traffic:  send, wait, last, answer` · `wheel:    pause, release` · `setup:    run, revive, verify, plugin install`»
  new: «the five rows `board:    list` · `traffic:  send, wait, last, answer` · `wheel:    pause, release` · `handoff:  link, unlink` · `setup:    run, revive, verify, plugin install`»
- line: .andromeda/layout-templates.md:219
  old: «Served verbs only: the wireframe's `handoff:  link, unlink` row joins with "Explicit session links"; `help_lists_only_served_verbs` reads»
  new: «Served verbs only: `help_lists_only_served_verbs` reads»
- line: .andromeda/layout-templates.md:61
  old: «the board, traffic, wheel and setup rows, `setup:` holding `plugin install`; the `handoff:` row joins when "Explicit session links" serves its verbs.»
  new: «the board, traffic, wheel, handoff and setup rows, `handoff:` holding `link, unlink` and `setup:` holding `plugin install`.»
- line: .andromeda/layout-templates.md:301
  old: «it prints board / traffic / wheel / setup over the served verbs, `setup:` holding `plugin install`, the handoff group joining when its route entry serves its verbs.»
  new: «it prints all five groups over the served verbs, `handoff:` holding `link, unlink` and `setup:` holding `plugin install`.»
- after: .andromeda/layout-templates.md:168
  anchor: «`viola release <name> --budget` prints one of two stdout lines, exit 0, with the wheel left where it is. `<name>  budget gate lifted until <end>  wheel <holder>` when a pause stood: `<end>` is the reply's `override_until` as `HH:MM:SSZ` on the current UTC date and as `YYYY-MM-DD HH:MM:SSZ` on any other date, and a value that is no RFC 3339 instant prints escaped. `<name>  budget gate open  wheel <holder>` when no pause stood. The date form and the `budget gate open` line are the founder's wording: planning forms as built (chunk 2026-10-10-budget-governor, on the operator's word) that he confirmed as they were shown to him, his own answer, 2026-10-13, relayed verbatim by the Viola overseer (the operator's word superseded 2026-10-13) (design-system §Surface: cli, Component Patterns 4).»
  add: «»
  add: «`viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` act on the driven instance and each print one stdout line in one write, plain in every mode, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, the reply's `since` as UTC clock time in whole seconds (a `since` that is no RFC 3339 instant, such as the restore's `unknown`, prints escaped); `link    <driver> -> <driven>  already linked` for a pair already held; `unlink  <driver> -> <driven>` whether or not a pair was removed. The names printed are the verb's own arguments, never names read back from the reply. Under `--json` each prints the reply's `ok` as `{"v":1,"ok":<ok>}`: `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}` for `link`, `{"v":1,"ok":{}}` for `unlink`. A wrapper error, a `driver` equal to `driven` (`-32602`) included, is the wrapper-fault line, exit 20; the exit-21 refusals are under Primary content block 2.»
- line: .andromeda/layout-templates.md:329
  old: «a frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) whose server check»
  new: «a frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) whose server check»
- line: .andromeda/layout-templates.md:329
  old: «`wait` / `last`'s exit 21 `unable  <name>  instance-unreachable` takes»
  new: «`wait`, `last`, `link` and `unlink`'s exit 21 `unable  <name>  instance-unreachable` (for `link` / `unlink`, `<name>` is the driven instance) takes»
- line: .andromeda/layout-templates.md:327
  old: «a `marketplace/` tree dir, nothing created and no log opened. A failed»
  new: «a `marketplace/` tree dir, nothing created and no log opened; and `viola link` / `viola unlink`'s exit-21 strict-modes refusal of the driven instance, made before any role file or log opens, stdout empty: `revive`'s instance pair (`unable: <driven>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`), no line written to any log and the directory's mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty. A failed»
- line: .andromeda/layout-templates.md:327
  old: «`revive --list`, `plugin install`; a `revive` start stays outside it)»
  new: «`revive --list`, `plugin install`, `link`, `unlink`; a `revive` start stays outside it)»
- line: .andromeda/layout-templates.md:327
  old: «; for `list` an `instances/` that cannot be listed»
  new: «; for `link` / `unlink` a refusal reply or an `ok` of another shape; for `list` an `instances/` that cannot be listed»
- line: .andromeda/layout-templates.md:330
  old: «(for `viola list` and `viola plugin install` the fixed state pair above instead)»
  new: «(for `viola list`, `viola plugin install` and the strict-modes refusal of `viola link` / `viola unlink` the fixed state pair above instead)»
- line: .andromeda/layout-templates.md:48
  old: «and `viola wait` / `viola last` for exit 21)»
  new: «and `viola wait` / `viola last` / `viola link` / `viola unlink` for exit 21)»

## Patterns
[[pattern]]
id = "lt-01"
fixed = 'the four rows `board:'

[[pattern]]
id = "lt-02"
fixed = 'row joins with "Explicit session links"'

[[pattern]]
id = "lt-03"
fixed = 'row joins when "Explicit session links" serves its verbs'

[[pattern]]
id = "lt-04"
fixed = 'the handoff group joining when its route entry serves its verbs'

[[pattern]]
id = "lt-05"
fixed = '`pause`, `release`) whose server check'

[[pattern]]
id = "lt-06"
fixed = 'for `viola list` and `viola plugin install` the fixed state pair'
