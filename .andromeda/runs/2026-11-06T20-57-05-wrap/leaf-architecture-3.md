@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `.andromeda/architecture.md` · `.claude/rules/api.md` · `.cl
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/stack.md line=12 class=stale
leaf-text: portable-pty `=0.8.1` (pinned)
source: fct-kxiazeyf «| PTY layer | portable-pty `=0.9.0` (pinned) behind viola's own `pty` seam»
replace-with: portable-pty `=0.9.0` (pinned)
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=3 class=tree
leaf-text: Primary source: `.andromeda/architecture.md` §Crate dependency direction
source: fct-p5s5gpab «- The `viola` root bin → all members, clap, anyhow, tempfile (`=3.27.0`, workspace-pinned: `viola verify`'s Run A temp dir only)»
replace-with: Primary source: `doc:architecture` §Crate dependency direction
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=58 class=tree
leaf-text: - `.andromeda/architecture.md` · `.claude/rules/api.md`
source: fct-p5s5gpab «- The `viola` root bin → all members, clap, anyhow, tempfile (`=3.27.0`, workspace-pinned: `viola verify`'s Run A temp dir only)»
replace-with: - `doc:architecture` · `.claude/rules/api.md`
@leaf_edit n=4 leaf=.claude/docs/stack.md line=3 class=tree
leaf-text: _Extracted from `.andromeda/architecture.md` §Stack and Technologies
source: fct-kxiazeyf «| PTY layer | portable-pty `=0.9.0` (pinned) behind viola's own `pty` seam»
replace-with: _Extracted from `doc:architecture` §Stack and Technologies
@leaf_edit n=5 leaf=.claude/docs/stack.md line=57 class=tree
leaf-text: see `.andromeda/architecture.md` Established Decisions section
source: fct-hcu7z4ke «- **[Scale / Product] Personal v1, Linux only.**»
replace-with: see `doc:architecture` Established Decisions section
@leaf_edit n=6 leaf=.claude/docs/stack.md line=62 class=tree
leaf-text: 1. Update `.andromeda/architecture.md` Stack table first
source: fct-kxiazeyf «| PTY layer | portable-pty `=0.9.0` (pinned) behind viola's own `pty` seam»
replace-with: 1. Update `doc:architecture` Stack table first
@leaf_edit n=7 leaf=.claude/docs/workflow.md line=55 class=tree
leaf-text: read `.andromeda/architecture.md` Established Decisions to check alignment
source: fct-hcu7z4ke «- **[Scale / Product] Personal v1, Linux only.**»
replace-with: read `doc:architecture` Established Decisions to check alignment
