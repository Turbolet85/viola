@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=35 class=omission
leaf-text: `tools/list` carries `ttlMs` 0 and `cacheScope` `private`
source: ctr-swzmtrjb «The tools/list and server/discover results each carry cacheScope private, because claude drops every tool of a list result without it.»
replace-with: `tools/list` and `server/discover` each carry `ttlMs` 0 and `cacheScope` `private`
@leaf_edit n=2 leaf=.claude/rules/api.md line=14 class=omission
leaf-text: no path and no upstream text
source: dec-xxc5ggaf «CLI --json documents, MCP error results and channel error data carry codes and fixed messages only, no path, pid or upstream text; a peer's message, data, refusal or detail reaches them only as a closed value.»
replace-with: no path, pid or upstream text
