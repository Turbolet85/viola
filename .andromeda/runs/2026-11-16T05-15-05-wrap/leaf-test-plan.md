@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules
@leaf_read leaf=.claude/rules/testing.md lines=118
last: - 2026-11-16: A shared case table whose assertions pin one 
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: `send-issued` then `prompt-submitted{driver}`;
source: ctr-ih3pmwbj «The records after the send's cursor are send-issued{cursor, from} then prompt-submitted{origin:"driver"} holding the submitted text: the typed text, or the typed text less the trailing run the CLI trimmed.»
replace-with: `send-issued` then `prompt-submitted{driver}` holding the submitted text (the typed text, or the typed text less the trailing run of space, TAB, CR and LF the CLI trimmed);
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: `--report-version`, `--framing`
source: ctr-j2qcfglq «Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text, or under --trim-prompt-end with its trailing run of space, TAB, CR and LF removed.»
replace-with: `--report-version`, `--trim-prompt-end` (a bracketed paste plus Enter fires UserPromptSubmit with its trailing run of space, TAB, CR and LF removed), `--framing`
