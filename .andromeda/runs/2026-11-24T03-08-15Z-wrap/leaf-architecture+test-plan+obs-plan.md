@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select(
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: No `viola ui` subcommand: viola has no web or graphical frontend;
source: ctr-tb7gbduz «contract/cli-verbs/focus · CLI verbs · focus — `viola focus <name> [--json]` takes one ViolaName and `--json` only.»
replace-with: `viola focus <name> [--json]` — raise the named session's window (takes one ViolaName and `--json` only; CLI only, no MCP tool raises a window); exit 1 with one process-exit detail: `window-not-found` (no window of the session, nothing dispatched), `unsupported` (not on Hyprland, nothing spawned), `unreachable` (no compositor answering) or the hyprctl seam's `oversize`, `malformed` or `refused`; exit 21 on a refused strict-modes check (under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, nothing opened, no line written). No `viola ui` subcommand: viola has no web or graphical frontend;
