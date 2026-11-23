@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=19 class=omission
leaf-text: - `viola run <name> -- claude [args]` — wrap a live session (Linux only; first live on the Linux dev host);
source: ctr-jtlmr6fa «contract/viola-cli-arguments/run-parent · viola CLI arguments · run --parent — viola run --parent <NAME> declares the instance's parent; a parent equal to the instance's own name is a usage error, exit 2,»
replace-with: - `viola run <name> [--parent <NAME>] [--role <WORD>] -- claude [args]` — wrap a live session (Linux only; first live on the Linux dev host); `--parent` declares the instance's parent (equal to its own name: usage error, exit 2, before anything is read; with no `--parent` an inherited `VIOLA_NAME` other than the own name is taken as parent), `--role` its role, a code word;
@leaf_stands n=1 leaf=.claude/docs/commands.md line=70
because: The line's `v1` is the schema file names `diag-line.v1.json` / `diag-detail.v1.json`, not the `in v1` qualifier ctr-q5dspexy dropped about `viola list` columns, which the line never speaks of.
@leaf_stands n=2 leaf=.claude/docs/commands.md line=77
because: The line's `v1` speaks of the deploy stage and the v1.x release workflow, not of the `viola list` link column whose `in v1` qualifier ctr-q5dspexy dropped, so the row's change leaves it true.
