@answer n=1 entity=obs-scope-summary#31.3 v=no
why: the human-taking-the-wheel must-trace scenario and its span and log fields are not needed (task item 3), no Linux path among them moves

@answer n=2 entity=obs-scope-summary#31.4 v=no
why: source line of the human-taking-the-wheel must-trace scenario, not needed (task item 3)

@answer n=3 entity=obs-scope-summary#32.1 v=no
why: budget governor surfaces (statusline shell-out, budget.json, budget-gate) are not needed (task items 3 and 13)

@answer n=4 entity=obs-scope-summary#32.2 v=no
why: budget governor must-trace spans are not needed (task item 3)

@answer n=5 entity=obs-scope-summary#32.3 v=no
why: budget-paused log fields and exit 11 belong to the budget governor and statusline, not needed (task items 3 and 13)

@answer n=6 entity=obs-scope-summary#32.4 v=no
why: source line of the budget governor scenario, not needed (task item 3)

@answer n=7 entity=obs-scope-summary#33.1 v=yes
why: it states the closed -32602 detail words and the fixed -32603 text of channel logging, closed words the scope keeps unchanged (task item 11 and the task prose)

@answer n=8 entity=obs-scope-summary#34.1 v=yes
why: it states the NEVER-log floor for send text and the send-refused detail words, which the scope keeps as they stand (task item 14)

@answer n=9 entity=obs-scope-summary#35.1 v=yes
why: it states the hook fail-open paths with exit 0 and empty stderr, and the macOS euid carve-out the chunk removes (task items 10 and 14)

@answer n=10 entity=obs-scope-summary#36.1 v=yes
why: it states the instance-unreachable, server-verify-failed and invalid-params codes on the wait, last and MCP paths that the scope keeps on Linux (task item 10)

@answer n=11 entity=obs-scope-summary#37.1 v=yes
why: it states the NEVER-log floor for the messaging token and CLAUDE* values, which the scope keeps as it stands (task item 14)

@answer n=12 entity=obs-scope-summary#38.1 v=yes
why: it states the 0600/0700 owner-only diagnostics rule with its Windows home-DACL arm the chunk removes, and the sideload_fallback companion (task items 8, 9 and 15)

@answer n=13 entity=obs-scope-summary#39.1 v=yes
why: it states process-start and process-exit for the claude child on the PTY seam, which the scope keeps unchanged (task item 12)

@answer n=14 entity=obs-scope-summary#40.1 v=yes
why: the obs dependencies must pass cargo deny on all targets, and the deny.toml entries of the removed Windows dependencies are edited (task item 7)

@answer n=15 entity=obs-scope-summary#41.1 v=no
why: the corr and dialog_id join keys across CLI, MCP, wrapper and hook calls are the corr plus conn join key the not-needed list drops (task item 10)

@answer n=16 entity=obs-scope-summary#42.1 v=yes
why: the session-end max under 1.0 s and the spine deadline are the 1.0 s hook budgets the perf job times (task item 5), the heartbeat flip is not needed (task item 7)

@answer n=17 entity=obs-scope-summary#43.1 v=yes
why: the child exit detected on the process handle without EOF on the PTY seam and the endpoint-lost exit 21 during wait bind (task items 10 and 12), the torn-line, snapshot replay and heartbeat bullets are not needed

@answer n=18 entity=obs-scope-summary#44.1 v=yes
why: the per-OS rows and the same-schema-on-every-OS rule are the expected amendments the wrap writes, naming the ConPTY, named pipe, DACL and macOS arms the chunk removes (task item 8)

@answer n=19 entity=obs-scope-summary#45.1 v=yes
why: it states the zero-unlogged-panics invariant for all four process roles, which the scope keeps as it stands (task item 14)

@answer n=20 entity=obs-scope-summary#46.1 v=no
why: operational retention and the absence of rotation for events.ndjson and diagnostics files are not needed (the task's not-needed list)

@answer n=21 entity=obs-scope-summary#47.1 v=yes
why: it states the closed event list adopted verbatim and the process-log roles, which the scope keeps fixed (task item 11)

@answer n=22 entity=obs-scope-summary#48.1 v=yes
why: it states the exit 21 and exit 1 causes with their distinct detail codes, including server-verify-failed and socket-dir-refused, which the scope keeps on Linux (task items 10 and 11)

@answer n=23 entity=obs-scope-summary#49.1 v=yes
why: it states the typed exit 14 RefusalReason fallback and the closed -32602 detail words, which the scope keeps (task item 11)

@answer n=24 entity=obs-scope-summary#50.1 v=no
why: deterministic emission, no screen-derived telemetry and no log-based sleeps are generic rules the chunk neither reaches nor names
