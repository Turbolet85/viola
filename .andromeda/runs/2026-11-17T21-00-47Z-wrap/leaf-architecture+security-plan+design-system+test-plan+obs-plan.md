@leaf_group group=architecture+security-plan+design-system+test-plan+obs-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=CLAUDE.md lines=134
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=25 class=omission
leaf-text: (save the test-only live rig's read of the key names and source word of its own captured UserPromptSubmit payload, never a value)
source: dec-uun4jqhm «Test-only: the live rig reads the key names and source word of its own captured UserPromptSubmit payload, never a value; the fake agent and the rig read a Claude config file's trust record.»
replace-with: (save the test-only live rig's read of the key names and source word of its own captured UserPromptSubmit payload, never a value, and the fake agent's and the rig's read of a Claude config file's trust record)
@leaf_edit n=2 leaf=CLAUDE.md line=38 class=stale
leaf-text: only `NO_COLOR` / `TERM` / `COLORTERM` for `list`'s colour, `XDG_RUNTIME_DIR`, `PATH` and two `fake-agent` test seams are read
source: dec-u3r3pgro «NO_COLOR, TERM, COLORTERM for list's colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read by verify only when kept.»
replace-with: only `NO_COLOR` / `TERM` / `COLORTERM` for `list`'s colour, `XDG_RUNTIME_DIR`, `PATH`, two `fake-agent` test seams and `CLAUDE_CONFIG_DIR`'s value, read by `viola verify` only when `claude_env_keep` keeps that name, are read
@leaf_edit n=3 leaf=CLAUDE.md line=43 class=omission
leaf-text: (save the test-only live rig in `viola-e2e`, which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value)
source: dec-uun4jqhm «Test-only: the live rig reads the key names and source word of its own captured UserPromptSubmit payload, never a value; the fake agent and the rig read a Claude config file's trust record.»
replace-with: (save the test-only live rig in `viola-e2e`, which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value, and the test-only fake agent and rig, which read a Claude config file's trust record)
