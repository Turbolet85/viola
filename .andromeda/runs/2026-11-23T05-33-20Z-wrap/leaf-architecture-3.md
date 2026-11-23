@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=59
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=45 class=stale
leaf-text: the `--json` `{items, skipped}` document
source: ctr-t5xneaha «The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, links?, skipped}`; `list` is not a channel method and needs no live wrapper.»
replace-with: the `--json` `{items, links?, skipped}` document
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=21 class=omission
leaf-text: It logs as process `run`, replays no launch, and its first record is `wheel{driver, start}`.
source: dec-zmmfuujl «A revive resumes the `gone` instance in place through the same start as `run`, its `events.ndjson` appended to, never truncated, and keeps the parent and role its real snapshot records.»
replace-with: It logs as process `run`, replays no launch, keeps the parent and role its real snapshot records, and its first record is `wheel{driver, start}`.
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=20 class=omission
leaf-text: first snapshot (with `endpoint`) + heartbeat
source: ctr-waf6f2tr «lineage — `lineage` is `{parent?, role?}`, the declared parent and role the start recorded; a replay rebuilds it from the log, the last `lineage` line replacing the held value.»
replace-with: first snapshot (with `endpoint`, and `lineage` `{parent?, role?}` from `--parent` / `--role`, with no `--parent` an inherited `VIOLA_NAME` other than the own name taken as parent) + heartbeat
@leaf_stands n=1 leaf=.claude/docs/stack.md line=16
because: The line's `v` is the record version field and bears on no `viola list` column, so the columns row losing `in v1` leaves it as the sources say.
@leaf_stands n=2 leaf=.claude/docs/stack.md line=29
because: The line's `Local-only v1` speaks of deployment, not of `viola list` columns, so the columns row dropping `in v1` leaves it true.
@leaf_stands n=3 leaf=.claude/docs/stack.md line=32
because: The line's `v1.x, not v1` release scope is no claim about `viola list` columns, which the changed row now states without `in v1`.
@leaf_stands n=4 leaf=.claude/docs/stack.md line=39
because: The line's `v1.x binary scans` is the audit deferral and bears on no `viola list` column, so the columns row losing `in v1` leaves it true.
@leaf_stands n=5 leaf=.claude/docs/workflow.md line=58
because: The line's `v1 has no release stage` concerns the release process, not the `viola list` columns the changed row now fixes without `in v1`.
