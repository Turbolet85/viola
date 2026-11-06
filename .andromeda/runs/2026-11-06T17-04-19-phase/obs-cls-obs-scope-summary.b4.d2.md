@answer n=1 entity=obs-scope-summary#22.2 v=no
why: frontend instrumentation hooks are N/A and the 0.2.0 frontend is out of this chunk
@answer n=2 entity=obs-scope-summary#22.3 v=no
why: the mcp file sink and stdout rule govern the mcp process, which the chunk does not touch
@answer n=3 entity=obs-scope-summary#22.4 v=no
why: service identity of the mcp process, untouched by the chunk
@answer n=4 entity=obs-scope-summary#22.5 v=yes
why: names the socket-dir-refused detail code under instance-unreachable on mcp results, the code the item-14 wording ratifies or re-words
@answer n=5 entity=obs-scope-summary#23.1 v=yes
why: its cargo deny bans on Tokio in the sync crates' graph and on C-building crates are what the portable-pty bump's new graph must pass
@answer n=6 entity=obs-scope-summary#24.1 v=yes
why: the security Vector 9 floor of tracing-subscriber >=0.3.20 is a Cargo.lock version the portable-pty bump re-resolves
@answer n=7 entity=obs-scope-summary#25.0 v=yes
why: the process-start and process-exit lines the PTY seam emits (item 6) must carry these required fields with corr null
@answer n=8 entity=obs-scope-summary#26.1 v=yes
why: fixes the diagnostics/run-<name>.ndjson location and its 0600/0700 modes, which the live run evidence (item 10) and its upload rely on
@answer n=9 entity=obs-scope-summary#27.1 v=no
why: a surface list for the run critical path with no rule of its own; the spans and events below carry the rules
@answer n=10 entity=obs-scope-summary#27.2 v=yes
why: the pty.spawn span and the child's process-start are what the live viola run (item 10) and the portable-pty seam must emit
@answer n=11 entity=obs-scope-summary#27.3 v=yes
why: names the run start's socket-dir-refused exit-1 detail code, part of the wording item 14 ratifies or re-words
@answer n=12 entity=obs-scope-summary#27.4 v=no
why: a source citation only, carrying no rule of its own
@answer n=13 entity=obs-scope-summary#28.1 v=no
why: a send surface list only, with no rule the chunk's code or tests must meet
@answer n=14 entity=obs-scope-summary#28.2 v=yes
why: the send spans and events are the witnesses of the overseer-driving-a-builder live session named in item 10
@answer n=15 entity=obs-scope-summary#28.3 v=yes
why: the send line fields and the rule that sent text never appears are the live session's evidence contract and item 11's no-send-text rule
@answer n=16 entity=obs-scope-summary#28.4 v=no
why: a source citation only
@answer n=17 entity=obs-scope-summary#29.1 v=no
why: a wait surface list only, with no rule of its own
@answer n=18 entity=obs-scope-summary#29.2 v=yes
why: the wait spans and channel events are how the overseer-driving-a-builder live session's waits are evidenced (item 10)
@answer n=19 entity=obs-scope-summary#29.3 v=yes
why: the rule that last_assistant_message content never appears is item 11's rule on home-level files, and the wait line fields are the evidence contract
@answer n=20 entity=obs-scope-summary#29.4 v=no
why: a source citation only
@answer n=21 entity=obs-scope-summary#30.1 v=no
why: a dialog surface list only, with no rule of its own
@answer n=22 entity=obs-scope-summary#30.2 v=yes
why: the dialog spans and events (hook-invoked, dialog-raised, dialog-answered, hook-decision) are the evidence of the dialog hold read live (item 12)
@answer n=23 entity=obs-scope-summary#30.3 v=yes
why: the dialog line fields and the rule that question, answer and tool input never appear are the read's evidence contract (item 12)
@answer n=24 entity=obs-scope-summary#30.4 v=no
why: a source citation only
@answer n=25 entity=obs-scope-summary#31.1 v=no
why: a wheel and release surface list only, with no rule of its own
