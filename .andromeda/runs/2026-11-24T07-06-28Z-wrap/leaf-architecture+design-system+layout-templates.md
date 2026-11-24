@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pr
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=25 class=stale
leaf-text: `setup:` holding `open, run, revive, verify, plugin install`
source: ctr-pbkoayxj «`viola --help` lists the served verbs as `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, up, verify, plugin install`»
replace-with: `setup:` holding `open, run, revive, up, verify, plugin install`
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: the exit-1 refusals of `run` / `revive` / `open` / `focus` / `verify`,
source: ctr-pwvbecet «The exit-1 refusals of `run`, `revive`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` print»
replace-with: the exit-1 refusals of `run` / `revive` / `open` / `focus` / `up` / `verify`,
@leaf_edit n=3 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: `list`'s exit-21 strict-modes refusal,
source: ctr-nrpvfuyp «The `list`, `link`, `unlink`, `focus`, `up` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`.»
replace-with: `list`'s and `up`'s exit-21 strict-modes refusal,
@leaf_edit n=4 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: every exit-1 refusal of `viola focus --json` (one of `refused` · `unsupported` · `window-not-found` · `unreachable` · `malformed` · `oversize`)
source: ctr-qprhd2jl «Any other line; or a refusal with a document under --json: boot-time-unread, strict-modes-failed, pinned-hash-mismatch or terminal-not-found.»
replace-with: every exit-1 refusal of `viola focus --json` (one of `refused` · `unsupported` · `window-not-found` · `unreachable` · `malformed` · `oversize`) and every exit-1 refusal of `viola up --json` (one of `boot-time-unread` · `strict-modes-failed` · `pinned-hash-mismatch` · `terminal-not-found`)
@leaf_edit n=5 leaf=.claude/rules/frontend.md line=20 class=omission
leaf-text: a `viola focus <name>` prints `<name>  focused` (under `--json` `{"v":1,"ok":{"name":"<name>"}}`),
source: ctr-z7n7sjhl «`viola up` prints one stdout line per instance, `<name>  <word>` or `<name>  live  <placement>` with a closed placement word, or under `--json` one document `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`.»
replace-with: a `viola focus <name>` prints `<name>  focused` (under `--json` `{"v":1,"ok":{"name":"<name>"}}`); `viola up` prints one stdout line per instance, `<name>  <word>` or `<name>  live  <placement>` (under `--json` `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`),
