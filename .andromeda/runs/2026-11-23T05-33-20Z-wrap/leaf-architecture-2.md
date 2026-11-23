@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-state.md line=30 class=stale
leaf-text: no type holds `endpoint`, `pinned_bin`, `cwd`, `pid`, `started_at` or `links`
source: dec-unwpeqm6 «The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, pid or started_at.»
replace-with: no type holds `endpoint`, `pinned_bin`, `cwd`, `pid` or `started_at`
@leaf_edit n=2 leaf=.claude/docs/services/viola-state.md line=28 class=omission
leaf-text: the replay writes no file and recovers `links` from the logged `link` / `unlink` lines
source: ctr-waf6f2tr «`lineage` is `{parent?, role?}`, the declared parent and role the start recorded; a replay rebuilds it from the log, the last `lineage` line replacing the held value.»
replace-with: the replay writes no file, rebuilds `lineage` from the log (the last `lineage` line replacing the held value) and recovers `links` from the logged `link` / `unlink` lines
@leaf_stands n=1 leaf=.claude/docs/services/viola-state.md line=6
because: The line names `revive` among the verbs resolving the home and describes the replay and snapshot readers without listing the recovered fields, so nothing it says conflicts with revive-in-place, the unreadable-snapshot replay, the run-start order or the link-names rule as they now stand.
@leaf_stands n=2 leaf=.claude/docs/services/viola-state.md line=22
because: It says only that strict-modes runs in each reader of `snapshot.json` or `ledger/stamps.json` and that each existing `events.ndjson` is checked before it is opened, which none of the changed rows contradicts.
@leaf_stands n=3 leaf=.claude/docs/services/viola-state.md line=25
because: `end_offset` and `append_event_at` read and append `events.ndjson` under its lock, which still matches revive-in-place's append-never-truncate rule and states no start order or log name.
@leaf_stands n=4 leaf=.claude/docs/services/viola-state.md line=26
because: `try_append_event` declining while the lock is held concerns only the hook's SessionEnd fallback, untouched by any row this pass changed.
@leaf_stands n=5 leaf=.claude/docs/services/viola-state.md line=27
because: `events.ndjson` never truncated and instance dirs reused and appended to is exactly what revive-in-place still says: appended to, never truncated.
