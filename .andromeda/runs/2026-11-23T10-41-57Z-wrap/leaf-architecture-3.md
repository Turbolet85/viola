@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=59
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rules
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectur
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=21 class=stale
leaf-text: replays no launch, keeps the parent and role its real snapshot records
source: dec-zmmfuujl «A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated, and keeps the parent, the role and the model and effort pair its real snapshot records.»
replace-with: replays no launch spec but the recorded model and effort, keeps the parent, the role and the model and effort pair its real snapshot records
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=21 class=omission
leaf-text: child arguments `--plugin-dir <dir>`, `--resume <id>`
source: ctr-zhermwwv «`--model <M> --effort <E>` follows `--plugin-dir` and precedes the caller's arguments at every start and every resume.»
replace-with: child arguments `--plugin-dir <dir>`, then `--model <M> --effort <E>` (each half the `--model` / `--effort` override, else the recorded half while it still parses, else the start's default), `--resume <id>`
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=20 class=omission
leaf-text: pinned folder first on PATH, `--plugin-dir`)
source: ctr-zhermwwv «`--model <M> --effort <E>` follows `--plugin-dir` and precedes the caller's arguments at every start and every resume.»
replace-with: pinned folder first on PATH, `--plugin-dir`, then `--model <M> --effort <E>` before the caller's arguments)
@leaf_edit n=4 leaf=.claude/docs/services/viola.md line=20 class=omission
leaf-text: first snapshot (with `endpoint`, and `lineage` `{parent?, role?}` from `--parent` / `--role`,
source: ctr-wjadxiye «`launch` is `{model, effort}`, the pair the start handed, omitted when absent with no snapshot `v` move; a replay rebuilds it from the log, the last `launch` line replacing the held value and a line»
replace-with: first snapshot (with `endpoint`, `launch` `{model, effort}` (the pair the start hands: `--model` / `--effort`, else the `config.json` default, else the compiled `DEFAULT_PAIR`; a `launch` line appended once after `lineage`), and `lineage` `{parent?, role?}` from `--parent` / `--role`,
