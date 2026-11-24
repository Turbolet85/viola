@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-mcp.md line=20 class=omission
leaf-text: `release`, `pause`, `link`, `unlink` are CLI-only (an affordance, not enforcement).
source: dec-dzhhh3s3 «decision/focus-has-no-driver-form · Focus has no driver-facing form — viola focus is offered on the CLI only: no MCP tool raises a window, and adding one is a further boundary widening of its own. · provisional»
replace-with: `release`, `pause`, `link`, `unlink` are CLI-only (an affordance, not enforcement); `focus` is CLI-only too: no MCP tool raises a window, and adding one is a boundary widening of its own.
@leaf_edit n=2 leaf=.claude/docs/services/viola-state.md line=22 class=omission
leaf-text: the CLI frame verbs `send` / `wait` / `last` / `answer` / `pause` / `release` and `link` / `unlink` through `client::refuse_untrusted`
source: ctr-wzlokvfk «contract/cli-exit-21/strict-modes-failed · CLI exit 21 · strict-modes-failed — A refused strict-modes check of send, wait, last, answer, pause, release or focus exits 21: human mode prints the unable: / hint: pair, --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} with stderr empty; no»
replace-with: the CLI frame verbs `send` / `wait` / `last` / `answer` / `pause` / `release` and `link` / `unlink` through `client::refuse_untrusted`, `viola focus` (a refusal exits 21 `strict-modes-failed`)
