@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=81
last: - 2026-11-24: A live-reading script prints verdict words fro
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=40 class=omission
leaf-text: Receipt ndjson `"v":1` + kebab `kind`: `start`, `env` (names only),
source: dec-d5iw22pb «decision/child-argv-read-from-proc · Child argv read from the live child's cmdline — A test claim about the child's argv is read from the live child's /proc cmdline, never from the fake agent's receipt.»
replace-with: Receipt ndjson `"v":1` + kebab `kind`: `start` (it records no argv: a test claim about the child's argv is read from the live child's `/proc` cmdline, never from the receipt), `env` (names only),
