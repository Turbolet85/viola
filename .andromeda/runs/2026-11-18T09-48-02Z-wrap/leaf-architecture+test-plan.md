@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: an unclean recording is refused naming the file and check code.
source: ctr-vsm4qn3e «viola verify takes [--record <DIR>] [-- <program> [args…]], the program defaulting to claude; a hidden --record-first-screens, absent from --help, is passed only by the live rig.»
replace-with: an unclean recording is refused naming the file and check code. A hidden `--record-first-screens`, absent from `--help` and passed only by the live rig, makes a failed `--record` verify write a first-screen file of whole scrubbed rows per run short of the input box (Run A's trust excepted), refused whole when unclean, kept locally and never committed.
