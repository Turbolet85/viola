@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: `.
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project prese
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=22 class=omission
leaf-text: `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`, and
source: dec-2ttgqz2j «decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive.»
replace-with: `TERM=dumb` / `--json` / a passthrough role / `viola run` / a passed `viola revive`, and
