@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=109 class=stale
leaf-text: **The product window — open and unmeasured for `claude`:**
source: dec-zyofwgok «On Linux, real claude kept a key typed 0, 20 or 100 ms after a resize of its own pty in every measured round, so viola adds no wait; the key-loss reading is a Windows-ConPTY measurement.»
replace-with: **The product window — measured on Linux for `claude`:**
@leaf_edit n=2 leaf=.claude/docs/gotchas.md line=109 class=stale
leaf-text: The open question is now whether the real `claude` on Linux loses a key typed right after a resize; owner: the measurement of the route entry
source: dec-zyofwgok «On Linux, real claude kept a key typed 0, 20 or 100 ms after a resize of its own pty in every measured round, so viola adds no wait; the key-loss reading is a Windows-ConPTY measurement.»
replace-with: On Linux the real `claude` kept a key typed 0, 20 or 100 ms after a resize of its own pty in every measured round, so viola adds no wait (the key-loss reading is a Windows-ConPTY measurement); measured by the route entry
