@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/r
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=54 class=omission
leaf-text: only `viola-agent-claude` knows Claude payload shapes;
source: dec-uun4jqhm «decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: The test-only live rig»
replace-with: only `viola-agent-claude` knows Claude payload shapes, including those of `claude agents --json` (except the test-only live rig in `viola-e2e`, which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value);
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=omission
leaf-text: The only crate that knows Claude Code.
source: dec-zypgxbrh «decision/claude-first-no-adapter-trait · Claude-first, one agent crate, no adapter trait — All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude»
replace-with: The only crate that knows Claude Code (one test-only exception: the live rig in `viola-e2e` reads the key names and the source word of its own captured UserPromptSubmit payload, never a value).
