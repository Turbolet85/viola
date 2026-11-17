@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_read leaf=.claude/rules/events.md lines=41
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/docs/stack.md line=13 class=stale
leaf-text: the screen of `viola verify`'s five interactive probe runs
source: dec-uczlwirj «viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.»
replace-with: the screen of `viola verify`'s six interactive probe runs
@leaf_edit n=2 leaf=.claude/docs/workflow.md line=49 class=stale
leaf-text: all five interactive runs and the relied ledger rows
source: dec-uczlwirj «viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.»
replace-with: all six interactive runs and the relied ledger rows
@leaf_edit n=3 leaf=.claude/docs/workflow.md line=49 class=stale
leaf-text: all twenty-two on every build
source: dec-f67jf527 «Trust-record ledger row and its probe — The trust-record relied ledger row asks, per CLI version, whether a trust record written for an untrusted folder opens it with no modal; verify's Run F, after Run E»
replace-with: every relied row on every build, the trust-record row (Run F) among them
