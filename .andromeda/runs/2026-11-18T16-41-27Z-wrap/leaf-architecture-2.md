@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPT
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rule
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=6 class=omission
leaf-text: prints the step lines and `stamped` summary
source: ctr-w3mgefkp «contract/viola-verify-output/summary-line · viola verify stdout · summary line — stamped <ver>  N pass  M fail, N counting every relied row that counts toward verified, an absent browser-tools-prompt among them.»
replace-with: prints the step lines (each `pass`, `absent` or `fail`) and `stamped` summary (its pass count including an absent `browser-tools-prompt`, the one relied row whose `absent` counts toward verified)
