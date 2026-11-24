@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=25 class=stale
leaf-text: (the exit-1 refusals of `run` / `revive` / `open` / `verify`,
source: ctr-pwvbecet «The exit-1 refusals of `run`, `revive`, `open`, `focus` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.»
replace-with: (the exit-1 refusals of `run` / `revive` / `open` / `focus` / `verify`,
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=25 class=stale
leaf-text: the exit-21 strict-modes refusal `send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance
source: ctr-3r7jbq6j «`viola focus` exits 21 with `instance-dead` or `unwrapped-name` for a name not live, and with `strict-modes-failed` for a home or instance failing strict modes, each before any compositor read.»
replace-with: the exit-21 strict-modes refusal `send`, `wait`, `last`, `answer`, `pause`, `release` and `focus` (of its home or instance, before any compositor read) make of their instance
@leaf_edit n=3 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: every refusal of `viola open --json` is one stdout document
source: ctr-nh47bte2 «`error: internal error` (fixed; no chain, path or hint) or a fixed `unable:` refusal of `run`, `revive`, `open`, `focus` or `verify` and its hint; `--json` only for revive's shared-start, open's and focus's refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`; stderr empty.»
replace-with: every refusal of `viola open --json` and every exit-1 refusal of `viola focus --json` (one of `refused` · `unsupported` · `window-not-found` · `unreachable` · `malformed` · `oversize`) is one stdout document
@leaf_edit n=4 leaf=.claude/rules/frontend.md line=25 class=stale
leaf-text: (`handoff:` holding `link, unlink`, `setup:` holding
source: ctr-mt5jspdm «The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `board:` holding list and focus, `handoff:` holding link, unlink and `setup:` holding open, run, revive, verify and plugin install.»
replace-with: (`board:` holding `list, focus`, `handoff:` holding `link, unlink`, `setup:` holding
@leaf_edit n=5 leaf=.claude/rules/frontend.md line=20 class=omission
leaf-text: `malformed` · `oversize`.
source: ctr-x63m6t5f «`viola focus <name>` prints `<name>  focused` on stdout, or under `--json` `{"v":1,"ok":{"name":"<name>"}}`; no window address, pid or workspace is ever printed or logged.»
replace-with: `malformed` · `oversize`; a `viola focus <name>` prints `<name>  focused` (under `--json` `{"v":1,"ok":{"name":"<name>"}}`), and no window address, pid or workspace is ever printed or logged.
