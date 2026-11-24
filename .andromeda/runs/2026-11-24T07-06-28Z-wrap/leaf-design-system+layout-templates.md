@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=74
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=59 class=stale
leaf-text: the exit-1 refusals of `run`, `revive`, `open`, `focus` and `verify`
source: ctr-pwvbecet «The exit-1 refusals of `run`, `revive`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.»
replace-with: the exit-1 refusals of `run`, `revive`, `open`, `focus`, `up` and `verify`
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: the `viola run` / `viola revive` passthrough (on a verified CLI
source: ctr-z7n7sjhl «`viola up` prints one stdout line per instance, `<name>  <word>` or `<name>  live  <placement>` with a closed placement word, or under `--json` one document `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`.»
replace-with: `viola up [<NAME>...] [--dry-run] [--json]` (re-opens the fleet after a reboot, each candidate revived in its own terminal, parents before children, CLI-only; one stdout line per instance, `<name>  <word>` or `<name>  live  <placement>` with a closed placement word, the word one of nine closed outcome words, or under `--json` `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`), the `viola run` / `viola revive` passthrough (on a verified CLI
