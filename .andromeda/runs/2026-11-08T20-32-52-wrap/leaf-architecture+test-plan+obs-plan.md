@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_edit n=1 leaf=.claude/docs/commands.md line=16 class=stale
leaf-text: no TypeScript indexer in 0.1.0: the ts plane has no input (its one input, the tracked `e2e-web/tsconfig.json`, left with the browser leg), so `scripts/code-graph.py` detects no ts plane
source: fct-2nyzhkqj «- **ts:** none: the tree tracks no `tsconfig.json`, so `scripts/code-graph.py` detects no ts plane.»
replace-with: no TypeScript indexer: the tree tracks no `tsconfig.json`, so `scripts/code-graph.py` detects no ts plane
@leaf_edit n=2 leaf=.claude/docs/commands.md line=22 class=stale
leaf-text: no `viola ui` subcommand: the frontend is deferred to 0.2.0; its technology is not chosen.
source: fct-jn2hd4ac «viola has no web or graphical frontend. Its view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14), owed by the 0.2.0 Epoch 6 route entries»
replace-with: no `viola ui` subcommand: viola has no web or graphical frontend; its view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14), owed by the 0.2.0 Epoch 6 route entries «Console scaffold», «Fleet console view» and «Console actions».
