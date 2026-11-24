@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=25 class=stale
leaf-text: `setup:` holding `open, run, revive, up, verify, plugin install`
source: ctr-pbkoayxj «`viola --help` lists the served verbs as `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, verify, plugin install`, then `Options:`.»
replace-with: `setup:` holding `open, run, revive, adopt, up, verify, plugin install`
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: (the exit-1 refusals of `run` / `revive` / `open` / `focus` / `up` / `verify`,
source: ctr-pwvbecet «The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`»
replace-with: (the exit-1 refusals of `run` / `revive` / `adopt` / `open` / `focus` / `up` / `verify`,
@leaf_edit n=3 leaf=.claude/rules/frontend.md line=25 class=stale
leaf-text: `run`, `revive`'s start, `mcp` and `hook` stay silent at that site
source: ctr-y4fupb3w «`{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s»
replace-with: `run`, `revive`'s start, `adopt`'s start, `mcp` and `hook` stay silent at that site
@leaf_edit n=4 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: and every refusal of `viola open --json` and every exit-1 refusal of `viola focus --json`
source: ctr-utjna4le «A start refusal of `viola adopt` exits 1 with its fixed `unable:` / `hint:` pair, or under `--json` one stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty.»
replace-with: and every start refusal of `viola adopt --json` (one of `strict-modes-failed` · `session-live` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`) and every refusal of `viola open --json` and every exit-1 refusal of `viola focus --json`
@leaf_edit n=5 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: a `pump_child` error printing none
source: dec-mqy6ys7k «An error while pumping the child under `viola revive --json` or `viola adopt --json` prints no document.»
replace-with: a `pump_child` error of `revive` or `adopt` printing none
