@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-state.md line=6 class=retired
leaf-text: by six product readers, each after the instance's strict-modes check: `viola revive`'s preflight (one read for both its collision check and its recorded directory), `viola list`'s `board::read`, `liveness::live_endpoint`, `viola hook`'s `connect`, `hook statusline`'s `serve` and `run`'s collision check
source: ctr-zgw4cnv3 «contract/hook-to-kind-map/hook-statusline · Hook to kind map · hook statusline — The retired hook word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0 and reads»
replace-with: by five product readers, each after the instance's strict-modes check: `viola revive`'s preflight (one read for both its collision check and its recorded directory), `viola list`'s `board::read`, `liveness::live_endpoint`, `viola hook`'s `connect` and `run`'s collision check
@leaf_edit n=2 leaf=.claude/docs/services/viola-state.md line=22 class=retired
leaf-text: Strict-modes runs in every process that reads `snapshot.json`, `ledger/stamps.json` or `statusline_command`
source: ctr-z3efu4mu «contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.»
replace-with: Strict-modes runs in every process that reads `snapshot.json` or `ledger/stamps.json`
@leaf_edit n=3 leaf=.claude/docs/services/viola-state.md line=22 class=retired
leaf-text: `hook`'s event, dialog and statusline paths through `trusted`
source: ctr-zgw4cnv3 «contract/hook-to-kind-map/hook-statusline · Hook to kind map · hook statusline — The retired hook word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0 and reads»
replace-with: `hook`'s event and dialog paths through `trusted`
@leaf_edit n=4 leaf=.claude/docs/services/viola-state.md line=28 class=retired
leaf-text: whose six product callers are `viola revive`'s preflight, `viola list`'s `board::read`, `liveness::live_endpoint`, `viola hook`'s `connect` (a replayed or absent snapshot unreachable, so the hook fails open), `hook statusline`'s `serve` (no `statusline_command` in a replayed snapshot) and `run`'s collision check
source: ctr-zgw4cnv3 «contract/hook-to-kind-map/hook-statusline · Hook to kind map · hook statusline — The retired hook word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0 and reads»
replace-with: whose five product callers are `viola revive`'s preflight, `viola list`'s `board::read`, `liveness::live_endpoint`, `viola hook`'s `connect` (a replayed or absent snapshot unreachable, so the hook fails open) and `run`'s collision check
@leaf_edit n=5 leaf=.claude/docs/services/viola-state.md line=30 class=retired
leaf-text: `endpoint`, `pinned_bin`, `cwd`, `statusline_command`, `pid`, `started_at` or `links`
source: dec-unwpeqm6 «decision/board-read-model-shape · Board read model is sync and Claude-free — The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd,»
replace-with: `endpoint`, `pinned_bin`, `cwd`, `pid`, `started_at` or `links`
@leaf_edit n=6 leaf=.claude/docs/services/viola-state.md line=30 class=retired
leaf-text: The snapshot holds an optional `statusline_command`; `SnapshotRead::Present`
source: ctr-z3efu4mu «contract/instance-snapshot/data · instance snapshot · data — `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?}`.»
replace-with: The snapshot holds no `statusline_command`; `SnapshotRead::Present`
