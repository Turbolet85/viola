@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=65 class=stale
leaf-text: A switch (provisional) ends only the wrapper's own claude child
source: dec-vxcpgd5u «decision/switch-ends-only-its-own-child · Switch ends only the wrapper's own child — A switch ends only the wrapper's own claude child, the one its own PTY handle holds, with no pid lookup»
replace-with: A switch ends only the wrapper's own claude child
