@answer n=1 entity=obs-scope-summary#1.1 v=no
why: a count of telemetry surface entries that the chunk neither adds nor removes
@answer n=2 entity=obs-scope-summary#1.2 v=no
why: a count of instrumentable crates that the chunk does not change
@answer n=3 entity=obs-scope-summary#1.3 v=no
why: a count of must-trace paths and edge flows, none of which the chunk touches
@answer n=4 entity=obs-scope-summary#1.4 v=no
why: a count of triggers; the multi-OS compat trigger it includes is the wrap's named amendment, not a rule this chunk's code must honour
@answer n=5 entity=obs-scope-summary#2.1 v=yes
why: the kept Linux diag upload covers the homes under the e2e-home rule and must read the diagnostics directory root the tests write
@answer n=6 entity=obs-scope-summary#2.2 v=no
why: mcp stderr fallback logging is not OS-gated and the chunk does not touch mcp logging
@answer n=7 entity=obs-scope-summary#2.3 v=no
why: enum extensions the chunk does not make; the closed event enum keeps its Linux values unchanged
@answer n=8 entity=obs-scope-summary#2.4 v=no
why: hook-invoked corr timing is not named by the scope and the hook contract stays as it is
@answer n=9 entity=obs-scope-summary#3.1 v=no
why: a source line for the cli crate with no rule the chunk must honour
@answer n=10 entity=obs-scope-summary#3.2 v=no
why: an instrumentability classification of the cli crate that the chunk does not change
@answer n=11 entity=obs-scope-summary#3.3 v=no
why: the cli anyhow chain and stderr fixed-message rule is not named by the scope and the chunk adds no dispatch error path
@answer n=12 entity=obs-scope-summary#4.1 v=no
why: a source line for the core crate with no rule the chunk must honour
@answer n=13 entity=obs-scope-summary#4.2 v=no
why: a not-instrumentable classification; the rule that binds sits in entity 4.3
@answer n=14 entity=obs-scope-summary#4.3 v=yes
why: the six exit-21 cause words are the closed set the scope names as unchanged, and the macOS carve-out the chunk removes sits in the server-verification path that writes them
@answer n=15 entity=obs-scope-summary#5.1 v=no
why: a source line for the PTY seam with no rule the chunk must honour
@answer n=16 entity=obs-scope-summary#5.2 v=no
why: an instrumentability classification the chunk does not change
@answer n=17 entity=obs-scope-summary#5.3 v=no
why: the ConPTY and openpty boundary rule is a per-OS row the wrap amends, and the byte-stream no-log rule is unchanged by the removal
@answer n=18 entity=obs-scope-summary#6.1 v=no
why: a source line listing the IPC methods, stating no rule on wait or last behaviour
@answer n=19 entity=obs-scope-summary#6.2 v=no
why: an instrumentability classification the chunk does not change
@answer n=20 entity=obs-scope-summary#6.3 v=no
why: sync-only instrumentation rule over a crate that gains no instrumentation from the chunk
@answer n=21 entity=obs-scope-summary#7.1 v=no
why: a source line for the sync store crate with no rule the chunk must honour
@answer n=22 entity=obs-scope-summary#7.2 v=no
why: an instrumentability classification the chunk does not change
@answer n=23 entity=obs-scope-summary#7.3 v=no
why: sync-only instrumentation rule over a crate that gains no instrumentation from the chunk
@answer n=24 entity=obs-scope-summary#8.1 v=no
why: a source line for the schema crate naming drift reports the chunk does not change
@answer n=25 entity=obs-scope-summary#8.2 v=no
why: an instrumentability classification the chunk does not change
