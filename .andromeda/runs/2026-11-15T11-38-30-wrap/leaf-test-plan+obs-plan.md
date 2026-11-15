@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=20 class=stale
leaf-text: the role writer ends at `std::io::sink` and the detail line is dropped
source: dec-3yboxb3z «When the role file cannot be opened, viola_obs_init returns the error and installs no subscriber, so no line is written; hook carries on and the other roles, mcp included, return the error.»
replace-with: `viola_obs_init` returns the error and installs no subscriber (`hook` carries on writing no line, every other role, `mcp` included, returns the error) and the detail line is dropped
@leaf_edit n=2 leaf=.claude/rules/observability.md line=27 class=omission
leaf-text: Channel lines carry `conn = "<process>-<pid>-<t0>-<n>"`;
source: ctr-47amdxiq «A server receiving no conn, or one outside ^[a-z]+-\d+-\d+-\d+$ or over 64 bytes, logs srv_conn = "srv-<accept counter>" instead.»
replace-with: Channel lines carry `conn = "<process>-<pid>-<t0>-<n>"` (a server receiving no `conn`, or one outside `^[a-z]+-\d+-\d+-\d+$` or over 64 bytes, logs `srv_conn = "srv-<accept counter>"` instead);
