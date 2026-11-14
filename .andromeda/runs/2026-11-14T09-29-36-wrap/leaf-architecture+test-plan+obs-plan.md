@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: (`record` / `fixture`, the same scrub and closed refusal codes; architecture §Occupied Resources → Binary)
source: dec-d5bfbqzd «the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.»
replace-with: (`record` / `fixture`, the same scrub and closed refusal codes; architecture §Occupied Resources → Binary); the recorded Hyprland replies likewise by the test-only `viola-hypr-tap`, under the same scrub
