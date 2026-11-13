@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: signature-only `Screen.<phase>.json` screens when 0 rows fail
source: dec-wbpw2h6h «A recorded screen fixture keeps a row holding a footer or modal literal whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty»
replace-with: signature-only `Screen.<phase>.json` screens (a footer or modal row whole, the input box's rule row as its rule chars alone and its prompt row as the prompt char alone, every other row empty; a box whose live prompt row holds a home path, the username or an email refuses the recording) when 0 rows fail
