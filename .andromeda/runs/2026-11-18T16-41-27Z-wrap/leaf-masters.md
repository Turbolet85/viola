@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=CLAUDE.md lines=134
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=25 class=omission
leaf-text: its seeding of its private Claude config, and the fake agent's
source: dec-uun4jqhm «Test-only: the rig seeds its private config and reads, no value, its UserPromptSubmit key names, source word and the cachedChromeExtensionInstalled key name; the fake agent and rig read trust records.»
replace-with: its seeding of its private Claude config, its read of the `cachedChromeExtensionInstalled` key name there, never a value, and the fake agent's
@leaf_edit n=2 leaf=CLAUDE.md line=43 class=omission
leaf-text: and seeds its private Claude config, and the test-only fake agent
source: dec-zypgxbrh «Test-only: the rig seeds its private config and reads, no value, its UserPromptSubmit key names, source word and the cachedChromeExtensionInstalled key name; the fake agent and rig read trust records.»
replace-with: and seeds its private Claude config and reads its `cachedChromeExtensionInstalled` key name, never a value, and the test-only fake agent
