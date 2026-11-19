@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: with seven compiled pastes (the probe prompt, a long text, a tag-like text, a text holding one inner LF, `/compact`, `/clear`, then `/mcp disable viola-probe-absent`, the local-command-echo row's probe, skipped with that row failed when no transcript is readable)
source: dec-lbocs2om «verify probes it by Run B's eighth paste, /context, and a failing read of it is fail, never absent.»
replace-with: with eight compiled pastes (the probe prompt, a long text, a tag-like text, a text holding one inner LF, `/compact`, `/clear`, then `/mcp disable viola-probe-absent`, the local-command-echo row's probe, skipped with that row failed when no transcript is readable, then `/context`, the local-command-context row's probe, a failing read of it `fail`, never `absent`)
