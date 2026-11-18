@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope, 
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=6 class=stale
leaf-text: `run` and `revive` write no trust record
source: dec-wx7tk6ur «A start resolves the program, checks collision, pins the copy and plugin, gates the version, on a verified CLI writes the trust record of an untrusted spawn folder, binds the endpoint»
replace-with: `run`'s and `revive`'s start, on a verified CLI, write the trust record of an untrusted spawn folder
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=20 class=omission
leaf-text: nothing printed) → `socket_dir_ready`
source: dec-wx7tk6ur «A start resolves the program, checks collision, pins the copy and plugin, gates the version, on a verified CLI writes the trust record of an untrusted spawn folder, binds the endpoint»
replace-with: nothing printed) → on a verified CLI, the trust record of an untrusted spawn folder written into the child's Claude config (located through `CLAUDE_CONFIG_DIR` only when `claude_env_keep` keeps it) → `socket_dir_ready`
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=21 class=omission
leaf-text: spawned in the snapshot's recorded `cwd`
source: ctr-ywc67zqh «its one reader is `viola revive`, which uses it as the revived child's spawn directory and as the folder its start's trust record is keyed by.»
replace-with: spawned in the snapshot's recorded `cwd`, the folder its start's trust record is keyed by
