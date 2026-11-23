@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=59 class=omission
leaf-text: NEVER put a path or pid inside a `detail` code.
source: dec-3tlgpfr6 «A start's parent and role names appear only in events.ndjson, the snapshot, the viola list board, list --json and the MCP list result, each parent re-parsed as a viola name and each role as a code word»
replace-with: NEVER put a path or pid inside a `detail` code; NEVER write a start's parent or role name into a process-log or detail line (they appear only in `events.ndjson`, the snapshot, the `viola list` board, `list --json` and the MCP `list` result).
@leaf_stands n=1 leaf=.claude/docs/obs-summary.md line=16
because: The line's "v1" names the OTel deferral and the schema version, not the `viola list` columns row, which still keeps no link column and six fixed columns.
@leaf_stands n=2 leaf=.claude/docs/obs-summary.md line=20
because: The line's "v1" is the schema file names of gate G4, untouched by the columns row's dropped "in v1".
@leaf_stands n=3 leaf=.claude/docs/obs-summary.md line=42
because: The line's "v1" is only the `schemas/diag-line.v1.json` file name, which the columns row's change does not bear on.
@leaf_stands n=4 leaf=.claude/docs/obs-summary.md line=22
because: The line only names `events.ndjson` as a file the `logs` wrapper merges, which every row named still holds as the instance's appended events log.
@leaf_stands n=5 leaf=.claude/docs/obs-summary.md line=70
because: The line keeps link names out of every process-log line and traces links by the name-free channel pair, the product `link` / `unlink` line in `events.ndjson` and the snapshot's `links`, all of which the rows still state.
