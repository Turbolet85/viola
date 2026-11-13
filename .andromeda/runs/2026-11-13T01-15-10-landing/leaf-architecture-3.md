@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/stack.md line=38 class=stale
leaf-text: plus `0BSD` only per crate for interprocess's `doctest-file` and `recvmsg`.
source: dec-vdvrv4l4 «0BSD only through the per-crate exception for doctest-file, never through the allowlist.»
replace-with: plus `0BSD` only through the per-crate exception for interprocess's `doctest-file`, never through the allowlist.
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=41 class=tree
leaf-text: `Ctx::budget_paused`; `wait.rs` the `WaitFeed`
source: dec-utofgtfh «The root bin depends on all members and holds the subcommand dispatch, the run pump, the wheel and the budget governor, all of which consume only normalised events.»
replace-with: `Ctx::budget_paused`; and beside `send.rs` in `src/run/`: `wait.rs` the `WaitFeed`
