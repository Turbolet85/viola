@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=48
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=50
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_read leaf=.claude/docs/services/viola.md lines=59
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rule
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=20 class=stale
leaf-text: → on a verified CLI, the trust record of an untrusted spawn folder written into the child's Claude config (located through `CLAUDE_CONFIG_DIR` only when `claude_env_keep` keeps it) → `socket_dir_ready`
source: dec-wx7tk6ur «A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, runs any automatic verify, on a verified verdict writes an untrusted spawn folder's»
replace-with: → `socket_dir_ready`
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=20 class=omission
leaf-text: exclusive endpoint bind (taken → exit 1 `squatted-name`) → first snapshot (with `endpoint`)
source: dec-zrumetff «With config.json verify_new_cli on, a viola run or revive start meeting a claude version with no attempted stamps entry runs viola verify once as a child after the endpoint bind, before the trust record»
replace-with: exclusive endpoint bind (taken → exit 1 `squatted-name`) → any automatic verify (with `config.json`'s `verify_new_cli` on, default off, and no attempted stamps entry for the claude version: `viola verify` run once as a child, only while holding the owner-only `ledger/verify.lock`, try-locked after the stamps strict-modes check and never the stamps lock, a held lock meaning no verify and an unverified start; an entry already attempted, a red one included, never verified again automatically; the start gating on the final verdict) → on a verified verdict, the trust record of an untrusted spawn folder written into the child's Claude config (located through `CLAUDE_CONFIG_DIR` only when `claude_env_keep` keeps it) → first snapshot (with `endpoint`)
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=20 class=stale
leaf-text: `cli_version` / `cli_verified` into the first snapshot and the `claude-child` `process-start`
source: ctr-yuyp6dlb «`cli_verified` is the start's final verdict: run's version gate, its stamps read again after the start's automatic verify where one ran.»
replace-with: `cli_version` / `cli_verified` (the start's final verdict: the gate's, its stamps read again after an automatic verify where one ran) into the first snapshot and the `claude-child` `process-start`
@leaf_edit n=4 leaf=.claude/docs/services/viola-state.md line=33 class=omission
leaf-text: Lock the `.lock` sibling, never the guarded file: a layout chosen while Windows (where append + exclusive lock fails) was a target, and kept.
source: dec-z7zvfnpe «ledger/verify.lock, the automatic verify's attempt lock, guards no file of its own name and stands apart from stamps.json.lock.»
replace-with: Lock the `.lock` sibling, never the guarded file: a layout chosen while Windows (where append + exclusive lock fails) was a target, and kept; the one exception, `ledger/verify.lock`, the automatic verify's owner-only attempt lock, guards no file of its own name and stands apart from `stamps.json.lock`.
