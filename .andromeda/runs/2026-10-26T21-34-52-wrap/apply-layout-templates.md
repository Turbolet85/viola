# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · applied · .andromeda/layout-templates.md:329#1
- fanout-layout-templates.md 2 · applied · .andromeda/layout-templates.md:332#1
- fanout-layout-templates.md 3 · applied · .andromeda/layout-templates.md:329#2
- fanout-layout-templates.md 4 · applied · .andromeda/layout-templates.md:260#1

## Edits
- line: .andromeda/layout-templates.md:329
  old: «and `viola link` / `viola unlink`'s exit-21 strict-modes refusal of the driven instance, made before any role file or log opens, stdout empty: `revive`'s instance pair (`unable: <driven>'s state files can be written by another user`»
  new: «and the exit-21 strict-modes refusal of the frame verbs `viola send` (after its text read), `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (for these two `<name>` is the driven instance), made before any role file or log opens, stdout empty: `revive`'s instance pair (`unable: <name>'s state files can be written by another user`»
- line: .andromeda/layout-templates.md:332
  old: «(for `viola list`, `viola plugin install` and the strict-modes refusal of `viola link` / `viola unlink` the fixed state pair above instead)»
  new: «(for `viola list`, `viola plugin install` and the strict-modes refusal of `viola send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` the fixed state pair above instead)»
- line: .andromeda/layout-templates.md:329
  old: «(a run stopped by Ctrl-C or TERM, stdout empty); plus run's pinned-copy refusal;»
  new: «(a run stopped by Ctrl-C or TERM, stdout empty); `the viola home is not private to you` / `the viola home must be readable only by you; viola does not change its permissions` (the home pair: its strict-modes check refuses the home, `ledger/`, `bin/` or `plugin/` before its log opens, or refuses the stamps write; stdout empty, no step line, no summary); plus run's pinned-copy refusal; and the home-tree refusal of the `viola run` and `viola revive` starts, checked before the wrapper log opens: the home pair for the home, `ledger/`, `bin/`, `plugin/` or the pinned copy's folders and file, `revive`'s instance pair for the instance's state files, no log opened, no line, no child;»
- line: .andromeda/layout-templates.md:260
  old: «`--list` has two of the refusals (state files, no logged session). The start reads its preflight»
  new: «`--list` has two of the refusals (state files, no logged session). After the state-files check and before its wrapper log opens, the start checks its home tree as `run`'s start does: a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints the home pair `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair, each exit 1 with no log opened and no child (under `--json` the `strict-modes-failed` document). The start reads its preflight»

## Patterns
[[pattern]]
id = "lt-01"
fixed = "strict-modes refusal of `viola link` / `viola unlink` the fixed state pair"

[[pattern]]
id = "lt-02"
fixed = "`viola unlink`'s exit-21 strict-modes refusal of the driven instance"
