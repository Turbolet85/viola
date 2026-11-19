@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: a modal start is killed with no key;
source: dec-tgxu6nua «viola verify's interactive children never get a byte written into any dialog, and a start showing a modal is killed with no key; a browser-tools prompt standing at start is left for the human to answer once in claude»
replace-with: a modal start is killed with no key, and a browser-tools prompt standing at start is left for the human to answer once in claude, as verify says before its summary;
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: its MCP config is a private 0600 file inside the scratch, removed with it.
source: dec-zmtgpxom «The live rig's CX arm reads the session transcript through MAX_FRAME, counting only tags past the kept length and keeping no byte of it, and sends only through viola send under the rig's private config.»
replace-with: its MCP config is a private 0600 file inside the scratch, removed with it. The live rig's CX arm reads the session transcript through `MAX_FRAME`, counting only tags past the kept length and keeping no byte of it, and sends only through `viola send` under the rig's private config.
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: removes the copy on every exit path, and writes no credential, host path or config value into any ledger, frame or summary.
source: dec-jhelkp3b «The live rig copies the claude OAuth credentials file owner-only into its private scratch, reads only its expiry, refuses a start with under 20 minutes of token, removes the copy on every exit path»
replace-with: removes the copy on every exit path, and writes no credential, host path or config value into any ledger, frame or summary; the VH arm, run under the real home, makes no copy and reads no credentials file.
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: searching it for rig-made needles, dropping the bytes and keeping one closed word.
source: dec-75ttngyv «The live rig runs its child under a private CLAUDE_CONFIG_DIR with an environment from a closed kept list of host names plus its own, and reads the user's ~/.claude.json only after a moved stamp»
replace-with: searching it for rig-made needles, dropping the bytes and keeping one closed word; the VH arm runs under the real home and config, reads no credentials, keeps only key names and one needle word, prints no value, and needs an explicit home, Claude dir and user config.
