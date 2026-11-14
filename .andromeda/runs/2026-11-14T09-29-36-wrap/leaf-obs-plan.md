@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=55 class=stale
leaf-text: or use print/`dbg!` macros in product crates (clippy `print_stdout`/`print_stderr`/`dbg_macro` = deny).
source: dec-4gs34zbe «print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints.»
replace-with: or use print/`dbg!` macros in the hook, run or mcp code paths of the product crates (clippy `print_stdout`/`print_stderr`/`dbg_macro` = deny; exempt: viola-harness, the fake agent and the output modules that own `--json`, human CLI stderr and the hook decision body).
