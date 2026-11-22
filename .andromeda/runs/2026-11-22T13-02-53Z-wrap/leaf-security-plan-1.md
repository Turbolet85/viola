@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=61 class=omission
leaf-text: An empty typed text is refused `not-delivered` / `empty-text` (exit 13), by the client before any frame and by the wrapper before the first wheel read.
source: dec-nueeascz «A paste not finished within its compiled bound is cut: the bracket is closed with no Enter and the send is refused not-delivered paste-cut.»
replace-with: An empty typed text is refused `not-delivered` / `empty-text` (exit 13), by the client before any frame and by the wrapper before the first wheel read. A paste not finished within its compiled bound is cut: the bracket is closed with no Enter (no CR) and the send is refused `not-delivered` / `paste-cut` (exit 13).
