@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=42 class=stale
leaf-text: `console-key` (once per key read: required `key`, one of `quit|interrupt|unbound`; the key's bytes never reach a line)
source: ctr-pspx3lhv «contract/diag-line-fields/console-key · diagnostic line additive fields · console-key — Logged once per key read with key, a word of its closed set, required; on the editing line one read's keys write at m»
replace-with: `console-key` (once per key read: required `key`, a word of its closed set; on the editing line one read's keys write at most one line a class (`text`, `backspace`, `unbound`) with a `count`, required for `text`; the key's bytes and no typed text ever reach a line)
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=42 class=omission
leaf-text: · `trust-record` (one INFO line per wrapped
source: ctr-ayehpgr4 «contract/diag-line-fields/console-action · diagnostic line additive fields · console-action — One line per finished action child, with verb (a closed enum of the nine CLI verb words the console runs)»
replace-with: · `console-action` (one line per finished action child, in `console.ndjson`: required `verb`, a closed enum of the nine CLI verb words the console runs, and `exit_code` (integer); no name, argument or output reaches the line; the action opens no span of its own, the child verb logging in its own `cli` process log) · `trust-record` (one INFO line per wrapped
