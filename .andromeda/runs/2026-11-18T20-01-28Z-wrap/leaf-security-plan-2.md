@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/rules/security.md line=19 class=stale
leaf-text: owned by the 2.1.325 recorded fixture set
source: dec-t4wfjgzz «StopFailure is registered with no capability ledger row and no verify probe, owned by the entry A live StopFailure payload.»
replace-with: owned by the entry A live StopFailure payload
@leaf_edit n=2 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: its `viola mcp` a direct spawn with no shell and no PATH lookup)
source: dec-pvrejrf6 «MCP exchange fixtures are recorded by a test-only tap, started live by the standing rig under its private CLI config, that spawns viola mcp directly with no shell, caps each line»
replace-with: its `viola mcp` a direct spawn with no shell and no PATH lookup, the tap started live by the standing rig under its private CLI config)
@leaf_edit n=3 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: the test-only `viola-hypr-tap` keeps a reply string
source: dec-w5vqy6om «The live rig's MCP recording arm keeps no CLI output, stdout drained and dropped and stderr kept only as a byte count and a SHA, and its MCP config is a private 0600 file inside the scratch, removed with it.»
replace-with: the live rig's MCP recording arm keeps no CLI output (stdout drained and dropped, stderr only as a byte count and a SHA; its MCP config a private 0600 file inside the scratch, removed with it); the test-only `viola-hypr-tap` keeps a reply string
