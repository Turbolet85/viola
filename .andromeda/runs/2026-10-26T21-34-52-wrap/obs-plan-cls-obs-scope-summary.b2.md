# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · the reached diagnostics writers keep diagnostics/ as the only destination and the R8 veil is untouched
obs-scope-summary#9.1 · no · the rmcp stack and workspace are unchanged, with no crate or module added or removed
obs-scope-summary#9.2 · no · the MCP server stays instrumentable, its tool-call handling in the reached tools.rs still the boundary
obs-scope-summary#9.3 · no · the reached tools.rs refusal returns the MCP result and no line, so stdout carries only MCP frames
obs-scope-summary#10.1 · no · the portable-pty source and the hosted unmodified claude are unchanged
obs-scope-summary#10.2 · no · the child stays boundary-only and the session adds no instrumentation inside it
obs-scope-summary#10.3 · no · spawn and exit, paste writes and the readiness gate are unchanged in the reached run.rs, and the conpty refusal only sets a process-start sideload word, which this row does not list
obs-scope-summary#11.1 · no · the hook contract source is unchanged and the reached hook.rs still exits 0 on a refused instance, as the contract requires
obs-scope-summary#11.2 · no · the hook and MCP surface stays boundary-only, the caller still being Claude Code
obs-scope-summary#11.3 · yes · add-to · F5 the hook event and dialog paths take canonical then trusted before obs init, so a refused or absent instance writes no hook line, and F6 a VIOLA_DIR naming no existing instance directory canonicalises to no session and writes nothing
obs-scope-summary#12.1 · no · the sysinfo and claude agents --json source is unchanged and the agents read in src/run/agents.rs is not reached
obs-scope-summary#12.2 · no · the external CLI stays boundary-only and the session changes no agents-read instrumentation
obs-scope-summary#12.3 · no · the agents read's spawn, exit and parse outcome are unchanged, and the reached revive.rs and verify.rs gain only a tree check ahead of their log
obs-scope-summary#13.1 · no · viola hook statusline is still the only shell-out and the reached statusline.rs runs the same /bin/sh -c command
obs-scope-summary#13.2 · no · the statusline stays boundary-only, the user command still being the one thing it runs
obs-scope-summary#13.3 · no · the shell-out duration and exit stay the instrumented fields and the command string is still never logged, and a refused or replayed statusline runs no command, so nothing new is instrumented
obs-scope-summary#14.1 · no · the Stack source is unchanged and no dependency moved
obs-scope-summary#14.2 · no · the OS call sites (bind, connect, lock, rename, kill-fallback) stay the instrumented boundary in the reached client.rs and strict.rs
obs-scope-summary#14.3 · yes · add-to · F4 the six frame verbs first run refuse_untrusted and a refused instance exits 21 and writes no line, and F13 the founder's ruling that every new refused check writes no line joins the per-surface failure logging listed here
obs-scope-summary#15.1 · no · the ndjson and snapshot sources and the event, snapshot envelope and heartbeat contracts are unchanged, and the reached stamps.rs and strict.rs add refusals and no store field
obs-scope-summary#15.2 · no · the stores stay instrumentable through the viola-state call sites, whose reached changes add refusals and no new store or log
obs-scope-summary#15.3 · yes · add-to · F12 a replayed snapshot carries no endpoint, pid, started_at or statusline_command, a field note the snapshot.json record must carry for replays
obs-scope-summary#16.1 · no · the config source is unchanged and no config reader or key is among the reached files
obs-scope-summary#16.2 · no · configuration stays not-instrumentable with no config file or reader among the reached files
obs-scope-summary#16.3 · no · the load-time parse and its skipped-key counts are unchanged, and a refused hook reads no config, which leaves this claim true
