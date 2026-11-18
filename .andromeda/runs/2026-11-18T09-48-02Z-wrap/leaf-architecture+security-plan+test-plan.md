@leaf_group group=architecture+security-plan+test-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=CLAUDE.md lines=134
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=141 class=stale
leaf-text: (30 s, 30 s and 60 s under `mutants`)
source: dec-5eznx6xr «Tests named verify_window_ take an 80 s per-test kill, in both the ci and mutants profiles, through an override that stands first.»
replace-with: (30 s, 30 s and 80 s under `mutants`)
@leaf_edit n=2 leaf=CLAUDE.md line=25 class=omission
leaf-text: save the test-only live rig's read of the key names and source word of its own captured UserPromptSubmit payload, never a value, and the fake agent's and the rig's read of a Claude config file's trust record
source: dec-uun4jqhm «Test-only: the rig reads its captured UserPromptSubmit payload's key names and source word, never a value, and seeds its private Claude config; the fake agent and the rig read a config's trust record.»
replace-with: save the test-only live rig's read of the key names and source word of its own captured UserPromptSubmit payload, never a value, its seeding of its private Claude config, and the fake agent's and the rig's read of a Claude config file's trust record
@leaf_edit n=3 leaf=CLAUDE.md line=43 class=omission
leaf-text: which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value, and the test-only fake agent and rig
source: dec-zypgxbrh «Test-only: the rig reads its captured UserPromptSubmit payload's key names and source word, never a value, and seeds its private Claude config; the fake agent and the rig read a config's trust record.»
replace-with: which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value, and seeds its private Claude config, and the test-only fake agent and rig
