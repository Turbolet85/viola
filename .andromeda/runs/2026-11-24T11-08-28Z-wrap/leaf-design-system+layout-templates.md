@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=50 class=stale
leaf-text: while `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site
source: ctr-nll6i4jd «`run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent at the catch site, under `--json` too.»
replace-with: while `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent at the catch site
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=59 class=stale
leaf-text: the exit-1 refusals of `run`, `revive`, `open`, `focus`, `up` and `verify`
source: ctr-pwvbecet «The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`»
replace-with: the exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify`
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: ), the `viola run` / `viola revive` passthrough (
source: ctr-yj7zgy45 «contract/cli-verbs/adopt · CLI verbs · viola adopt <SESSION_ID> --name <NAME> — Resumes a plain claude session by its session id under viola with the given name and parent, in the current folder, with no»
replace-with: ), `viola adopt <SESSION_ID> --name <NAME>` (resumes a plain claude session by its session id under viola with the given name and parent, in the current folder, with no window; prints nothing once the child starts, and a `pump_child` error under `--json` prints no document), the `viola run` / `viola revive` passthrough (
@leaf_edit n=4 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: a `pump_child` error prints none, the child having held the terminal)
source: ctr-utjna4le «contract/cli-exit-codes/1-adopt-start · CLI exit codes · 1 of viola adopt — A start refusal of `viola adopt` exits 1 with its fixed `unable:` / `hint:` pair, or under `--json` one stdout document `{"v":1,»
replace-with: a `pump_child` error prints none, the child having held the terminal; a start refusal of `viola adopt` exits 1 with its fixed `unable:` / `hint:` pair, or under `--json` one stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty)
