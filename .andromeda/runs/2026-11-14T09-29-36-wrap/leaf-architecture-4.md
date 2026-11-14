@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=34 class=stale
leaf-text: with serde_path_to_error drift reports going to the instance detail file only
source: dec-u7smqsbl «Hook JSON, `claude agents --json` and statusline payloads are parsed with no `deny_unknown_fields`, with defaults and options; agents rows keep the raw value for the fixture recorder, and serde_path_to_error gives drift reports for hook JSON only.»
replace-with: with serde_path_to_error drift reports for hook JSON only, going to the instance detail file only
