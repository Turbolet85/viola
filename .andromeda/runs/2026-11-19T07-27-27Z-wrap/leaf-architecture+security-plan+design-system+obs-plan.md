@leaf_group group=architecture+security-plan+design-system+obs-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=CLAUDE.md lines=134
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=25 class=omission
leaf-text: key name there, never a value, and the fake agent's and the rig's read of a Claude config file's trust record)
source: dec-uun4jqhm «Test-only, no value: rig seeds private config, reads its UserPromptSubmit key names, source word, cachedChromeExtensionInstalled name; VH real-config key names, needle; fake agent, rig: trust records.»
replace-with: key name there, never a value, the rig's VH arm's read of the real config's key names and one needle word, never a value, and the fake agent's and the rig's read of a Claude config file's trust record)
@leaf_edit n=2 leaf=CLAUDE.md line=43 class=omission
leaf-text: and reads its `cachedChromeExtensionInstalled` key name, never a value, and the test-only fake agent and rig
source: dec-zypgxbrh «Test-only, no value: rig seeds private config, reads its UserPromptSubmit key names, source word, cachedChromeExtensionInstalled name; VH real-config key names, needle; fake agent, rig: trust records.»
replace-with: and reads its `cachedChromeExtensionInstalled` key name, never a value, and whose VH arm reads the real config's key names and one needle word, never a value, and the test-only fake agent and rig
