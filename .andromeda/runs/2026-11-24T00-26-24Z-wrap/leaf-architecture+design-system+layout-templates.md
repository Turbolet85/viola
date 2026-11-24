@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=25 class=stale
leaf-text: the exit-1 refusals of `run` / `revive` / `verify`
source: ctr-pwvbecet «The exit-1 refusals of `run`, `revive`, `open` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.»
replace-with: the exit-1 refusals of `run` / `revive` / `open` / `verify`
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=25 class=stale
leaf-text: `setup:` holding `run, revive, verify, plugin install`
source: ctr-pbkoayxj «`viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, verify, plugin install`, then `Options:`.»
replace-with: `setup:` holding `open, run, revive, verify, plugin install`
@leaf_edit n=3 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: every refusal of `viola revive --json` (its strict-modes check, its preflight and the start it shares with `run`) is one stdout document
source: ctr-nh47bte2 «`error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` refusal of `run`, `revive`, `open` or `verify` with its hint; `--json` only for `revive`'s shared-start refusals, `open`'s refusals and the catch site's»
replace-with: every refusal of `viola revive --json` (its strict-modes check, its preflight and the start it shares with `run`) and every refusal of `viola open --json` is one stdout document
@leaf_edit n=4 leaf=.claude/rules/frontend.md line=20 class=omission
leaf-text: the readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`.
source: ctr-jezharna «0 — The session went live: the human line `<name>  live  <placement>`, or under --json `{"v":1,"ok":{"name","parent","window","placement"}}`, window `{address, workspace}` or null.»
replace-with: the readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`; a live `viola open` prints `<name>  live  <placement>` (under `--json` `{"v":1,"ok":{"name","parent","window","placement"}}`), placement one of the eight closed words `placed` · `refused` · `unplaced` · `unsupported` · `window-unseen` · `unreachable` · `malformed` · `oversize`.
