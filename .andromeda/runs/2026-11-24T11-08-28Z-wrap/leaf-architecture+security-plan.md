@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=42
last: - 2026-11-24: A verb that opens its log on a home it resolve
@leaf_edit n=1 leaf=.claude/rules/api.md line=28 class=stale
leaf-text: `run`, `revive`'s start, `mcp` and `hook` staying silent there
source: ctr-y4fupb3w «without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `adopt`'s start, `mcp` and `hook` stay silent.»
replace-with: `run`, `revive`'s start, `adopt`'s start, `mcp` and `hook` staying silent there
@leaf_edit n=2 leaf=.claude/rules/api.md line=29 class=omission
leaf-text: a `revive` strict-modes, home-tree, preflight, socket-directory or start refusal,
source: ctr-yzm3hmbz «`viola adopt --json` refusal codes — `<code>` is one of `strict-modes-failed` · `session-live` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
replace-with: a `revive` strict-modes, home-tree, preflight, socket-directory or start refusal, a `viola adopt` refusal (`strict-modes-failed` before any log, `session-live`, `already-live`, `stale-heartbeat`, `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused` or `internal-error`, each under `--json` its refusal document),
@leaf_edit n=3 leaf=.claude/rules/api.md line=29 class=omission
leaf-text: and a `viola run --parent` equal to the instance's own name,
source: ctr-asykvpei «a `--parent` equal to `--name` is a usage error, exit 2.»
replace-with: and a `viola run --parent` equal to the instance's own name, and a `viola adopt --parent` equal to its `--name`,
@leaf_edit n=4 leaf=.claude/rules/api.md line=29 class=omission
leaf-text: and on `run` or `revive` a `--model` or `--effort` failing its check
source: dec-yklnwv7k «viola run's, viola revive's and viola adopt's --model must be a model word, 1 to 64 bytes, a lowercase letter first, then lowercase letters, digits, dots and dashes, and --effort one of the five effort»
replace-with: and on `run`, `revive` or `adopt` a `--model` or `--effort` failing its check
@leaf_edit n=5 leaf=.claude/rules/api.md line=30 class=omission
leaf-text: `run`'s and `revive`'s `--model` a model word
source: dec-yklnwv7k «viola run's, viola revive's and viola adopt's --model must be a model word, 1 to 64 bytes, a lowercase letter first, then lowercase letters, digits, dots and dashes, and --effort one of the five effort»
replace-with: `run`'s, `revive`'s and `adopt`'s `--model` a model word
