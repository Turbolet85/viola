@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: its view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14), owed by
source: ctr-krgnhdj4 «viola console writes only its screen to stdout, in the alternate screen, plain ASCII with no colour; its refusals and its internal error go to stderr.»
replace-with: its view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14), the verb `viola console` (no `--json`; it writes only its screen to stdout, in the alternate screen, plain ASCII with no colour, its refusals and internal error to stderr; a home another user may read exits 1 with the home pair, writing no line; it logs to the home-level `diagnostics/console.ndjson` with no instance), owed by
