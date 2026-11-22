@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-pty.md line=31 class=stale
leaf-text: OPEN — whether the real `claude` on Linux loses a key typed right after a resize · owner: the measurement of the route entry «Key after a resize» (0.2.0, Epoch 4).
source: dec-zyofwgok «decision/human-key-never-held-behind-resize · Human key never held behind a resize — viola never holds, queues or re-orders a human key behind a resize. · why: On Linux, real claude kept a key typed 0, 20 or 100 ms after a resize»
replace-with: On Linux the real `claude` kept a key typed 0, 20 or 100 ms after a resize of its own pty in every measured round, so viola adds no wait; the key-loss reading is a Windows-ConPTY measurement.
