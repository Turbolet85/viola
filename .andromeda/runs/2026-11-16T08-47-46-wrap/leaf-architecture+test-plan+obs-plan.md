@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: with six compiled pastes (the probe prompt, a long text, a tag-like text, a 113-byte text holding one inner LF, `/compact`, `/clear`) and one `claude agents --json` read once its first turn has settled, before the five added pastes
source: dec-lzixnc6x «The local-command-echo relied ledger row is probed by Run B's seventh paste, /mcp disable viola-probe-absent, and passes on a new stdout tag with no UserPromptSubmit within the confirmation window»
replace-with: with seven compiled pastes (the probe prompt, a long text, a tag-like text, a 113-byte text holding one inner LF, `/compact`, `/clear`, then `/mcp disable viola-probe-absent`, the local-command-echo row's probe, skipped with that row failed when no transcript is readable) and one `claude agents --json` read once its first turn has settled, before the six added pastes
@leaf_edit n=2 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: CI runs it only against the fake agent's `-p/--print` mode and its `--screens --turn-stop --trusted-root --dialogs --framing` interactive modes
source: dec-pxitz5rr «Every stamping viola verify against the fake agent, in the harness boot and the stamped-home helpers, passes the fake agent --transcript beside --framing.»
replace-with: CI runs it only against the fake agent's `-p/--print` mode and its `--screens --turn-stop --trusted-root --dialogs --framing --transcript` interactive modes (every stamping verify passes `--transcript` beside `--framing`, or Run B's echo probe is skipped and the local-command-echo row fails)
