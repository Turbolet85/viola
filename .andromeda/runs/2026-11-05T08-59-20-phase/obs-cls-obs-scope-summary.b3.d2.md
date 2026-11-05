@answer n=1 entity=obs-scope-summary#17.1 v=no
why: source citation only, naming no rule the chunk's code or tests act on.
@answer n=2 entity=obs-scope-summary#17.2 v=no
why: CI/CD is classed not-instrumentable and the chunk adds no telemetry to ci.yml or nightly.yml.
@answer n=3 entity=obs-scope-summary#17.3 v=no
why: scope exclusion of CI and fixtures; the chunk adds no telemetry source and its harness figure is test-controlled, not recorded host data.
@answer n=4 entity=obs-scope-summary#18.1 v=yes
why: each process's panic hook writes one line per panic, which the zero-panic-lines budget read by G2 and the fake-agent forced panic both run through.
@answer n=5 entity=obs-scope-summary#18.2 v=no
why: frontend is N/A and the chunk adds no terminal surface; the stderr shape is carried by #18.5.
@answer n=6 entity=obs-scope-summary#18.3 v=yes
why: the Linux instance checks of the exit-21 and exit-1 verbs, and their refused-check no-log rule, stay unchanged under items 10 and 14; only the macOS carve-out leaves.
@answer n=7 entity=obs-scope-summary#18.4 v=no
why: service identity (name, CARGO_PKG_VERSION, process role) is not changed, and the Cargo.toml edit touches only cfg(windows) dependencies.
@answer n=8 entity=obs-scope-summary#18.5 v=yes
why: the Linux exit set, the fixed stderr refusal and fault strings, and the Linux detail codes (strict-modes-failed, socket-dir-refused) bind items 10, 11 and 13; only the macOS euid carve-out leaves.
@answer n=9 entity=obs-scope-summary#19.1 v=yes
why: spans on the start sequence and viola-pty seam operations sit in units the chunk edits, and the sync-only rule binds through the cargo deny Tokio ban in deny.toml.
@answer n=10 entity=obs-scope-summary#19.2 v=no
why: frontend is N/A and the run terminal carrying the child's screen is not changed by the chunk.
@answer n=11 entity=obs-scope-summary#19.3 v=yes
why: the run sink diagnostics/run-<name>.ndjson holds process-start, where the sideload_fallback field the chunk removes lives, and the no-terminal-output-while-child-runs rule holds on the run start path it edits.
@answer n=12 entity=obs-scope-summary#19.4 v=no
why: run's service identity (process="run", instance) is unchanged and no identity constant is edited.
@answer n=13 entity=obs-scope-summary#19.5 v=yes
why: process-exit must come from the process-handle wait and never reader EOF, and the panic hook is replaced before spawn, both kept on Linux by item 12.
@answer n=14 entity=obs-scope-summary#20.1 v=yes
why: hook-invoked and hook-decision lines and the single ERROR panic line from catch_unwind bind the zero-unlogged-panics invariant in item 14 and the G2 panic-line budget in item 6.
@answer n=15 entity=obs-scope-summary#20.2 v=no
why: frontend is N/A and the hook opens no terminal surface the chunk touches.
@answer n=16 entity=obs-scope-summary#20.3 v=yes
why: the hook sink, stderr ban, stdout decision-body-only rule and swallowed log-write failure with exit 0 are the hook contract that item 14 keeps.
@answer n=17 entity=obs-scope-summary#20.4 v=yes
why: the silent no-op without VIOLA_NAME and the strict-modes check ahead of obs init are the hook contract in item 14 and the instance check named in item 10.
@answer n=18 entity=obs-scope-summary#20.5 v=yes
why: the per-hook latency budget under 1.0 s, with the socket-dir and peer-euid checks in the timed path, binds the run --perf budgets in item 5 and the checks in item 10.
@answer n=19 entity=obs-scope-summary#21.1 v=no
why: channel spans and corr propagation are unchanged because the channel wire stays, and the wait and last edit removes only a macOS euid arm.
@answer n=20 entity=obs-scope-summary#21.2 v=no
why: frontend is N/A and the channel has no terminal surface.
@answer n=21 entity=obs-scope-summary#21.3 v=no
why: each side's log file location is unchanged and the chunk edits no channel log sink.
@answer n=22 entity=obs-scope-summary#21.4 v=no
why: each side's process identity and params.sender are unchanged and no version or sender constant is edited.
@answer n=23 entity=obs-scope-summary#21.5 v=yes
why: the Linux peer-euid refusal in accept_loop, dropped before any frame with no line, binds item 10, which keeps the peer-pid and euid comparison; the wire values stay under item 11.
@answer n=24 entity=obs-scope-summary#22.1 v=no
why: MCP tool-call spans wrap the channel client and the chunk changes no tool-call span; the wait, last and server-verify edits remove only the macOS arm.
@answer n=25 entity=obs-scope-summary#22.2 v=no
why: frontend is N/A and stdio MCP has no terminal surface the chunk touches.
