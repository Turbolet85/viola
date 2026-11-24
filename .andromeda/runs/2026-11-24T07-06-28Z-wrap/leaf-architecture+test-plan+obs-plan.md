@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: No `viola ui` subcommand:
source: ctr-uv2g64ih «`viola up [<NAME>...] [--dry-run] [--json]` re-opens, after a reboot, each instance gone before the boot through viola revive in a new terminal; names given narrow the fleet and a name not scanned reads not-taken;»
replace-with: `viola up [<NAME>...] [--dry-run] [--json]` — after a reboot, re-open each instance gone before the boot through `viola revive` in a new terminal, parents before children (a live or stale instance skipped; an unnamed one ended on purpose stays down; names given narrow the fleet, a name not scanned reads `not-taken`; `--dry-run` spawns nothing; CLI only, no MCP tool); exit 0 when every line reads `live`, `already-live`, `stale-heartbeat` or `would-open`, 1 for any other line or a refusal (`boot-time-unread` · `strict-modes-failed` · `pinned-hash-mismatch` · `terminal-not-found`, under `--json` its document), 21 when its board scan's strict-modes check refuses (`instance-unreachable`, `strict-modes-failed`, no line printed). No `viola ui` subcommand:
@leaf_edit n=2 leaf=.claude/docs/commands.md line=22 class=omission
leaf-text: `viola revive <name> --list` prints its logged sessions;
source: ctr-fgftcrwe «viola revive --origin <cli|agent> is a hidden flag, conflicting with --list and with no --place: under it the revive places its window on the workspace a real snapshot recorded, else none; without it there is no window step.»
replace-with: a hidden `--origin <cli|agent>` (conflicting with `--list`) places the revived window on the workspace its snapshot recorded, else none, with no window step without it; `viola revive <name> --list` prints its logged sessions;
