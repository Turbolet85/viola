@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=42
last: - 2026-11-24: A verb that opens its log on a home it resolved
@leaf_edit n=1 leaf=.claude/rules/api.md line=20 class=omission
leaf-text: `link`, `unlink`, `hook.dialog` (requests, integer ids
source: ctr-qd4d52i7 «A request: `switch` `{model?, effort?}`, at least one, reads no `from` and answers `{model, effort}` or a refusal human-typing, unverified-cli, or not-delivered with turn-running»
replace-with: `link`, `unlink`, `switch` (`{model?, effort?}`, at least one, reading no `from`, answered `{model, effort}`; a bad or missing half `-32602` with `data: null`), `hook.dialog` (requests, integer ids
@leaf_edit n=2 leaf=.claude/rules/api.md line=34 class=omission
leaf-text: `release`, `pause`, `link`, `unlink`, `focus` and `up` stay CLI-only
source: dec-y7x5wcbv «viola switch is offered on the CLI only: no MCP tool switches a session's model or effort.»
replace-with: `release`, `pause`, `link`, `unlink`, `focus`, `up` and `switch` stay CLI-only
@leaf_edit n=3 leaf=.claude/rules/api.md line=30 class=stale
leaf-text: `run`'s, `revive`'s and `adopt`'s `--model` a model word
source: dec-yklnwv7k «viola run's, revive's, adopt's and switch's --model must be a model word, 1 to 64 bytes, a lowercase letter first»
replace-with: `run`'s, `revive`'s, `adopt`'s and `switch`'s `--model` a model word
@leaf_edit n=4 leaf=.claude/rules/api.md line=29 class=omission
leaf-text: and on `run`, `revive` or `adopt` a `--model` or `--effort` failing its check or either flag
source: ctr-d6isq7rt «`viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]` takes at least one of `--model` and `--effort`, else a usage error, exit 2»
replace-with: and on `run`, `revive`, `adopt` or `switch` a `--model` or `--effort` failing its check, and a `viola switch` with neither flag, and on `run`, `revive` or `adopt` either flag
@leaf_edit n=5 leaf=.claude/rules/api.md line=23 class=omission
leaf-text: (CLI `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink`, `viola hook`
source: dec-od2mxr3r «The wrapper's switch method, which ends and resumes the driven child, is served only behind the IPC controls (socket-directory, peer euid and server checks)»
replace-with: (CLI `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink` / `switch`, `viola hook`
@leaf_edit n=6 leaf=.claude/rules/api.md line=23 class=stale
leaf-text: for the eight CLI verbs
source: dec-pmdeyzjd «CLI send, wait, last, answer, pause, release, link, unlink, focus and switch run the instance strict-modes check before any frame or compositor read»
replace-with: for the nine CLI verbs
@leaf_edit n=7 leaf=.claude/rules/api.md line=29 class=omission
leaf-text: on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` also a server
source: dec-od2mxr3r «The wrapper's switch method, which ends and resumes the driven child, is served only behind the IPC controls (socket-directory, peer euid and server checks)»
replace-with: on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink` and `switch` also a server
@leaf_edit n=8 leaf=.claude/rules/api.md line=29 class=omission
leaf-text: the target instance for `send`, `wait`, `last`, `answer`, `pause` and `release`, the driven
source: dec-pmdeyzjd «CLI send, wait, last, answer, pause, release, link, unlink, focus and switch run the instance strict-modes check before any frame or compositor read; a refusal exits 21»
replace-with: the target instance for `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch`, the driven
