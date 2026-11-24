@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=40 class=omission
leaf-text: [--effort E] [--json] -- claude`;
source: ctr-tb7gbduz «contract/cli-verbs/focus · CLI verbs · focus — `viola focus <name> [--json]` takes one ViolaName and `--json` only.»
replace-with: [--effort E] [--json] -- claude`; `viola focus <name> [--json]` takes one ViolaName and `--json` only;
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: `not-live` or `internal-error`, each under `--json` the `{"v":1,"error":"<code>","detail":null}` document)
source: ctr-tfxx4gxd «contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error, and the start, revive-preflight, open, focus and verify refusals, a refused home-tree check or socket directory of the start among them»
replace-with: `not-live` or `internal-error`, each under `--json` the `{"v":1,"error":"<code>","detail":null}` document; also `viola focus`'s refusals, each an `unable:` / `hint:` pair on stderr with no path, or under `--json` one refusal document)
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: which refuses the whole board, no log opened;
source: ctr-wzlokvfk «contract/cli-exit-21/strict-modes-failed · CLI exit 21 · strict-modes-failed — A refused strict-modes check of send, wait, last, answer, pause, release or focus exits 21: human mode prints the unable: / hint: pair»
replace-with: which refuses the whole board, no log opened; also `viola focus`'s refused strict-modes check: the `unable:` / `hint:` pair, or `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` under `--json` with stderr empty, nothing under the home opened and no line written;
