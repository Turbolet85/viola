@answer n=1 entity=obs-scope-summary#17.1 v=no
why: names only the source of the CI/CD platform entity and states no rule the chunk's code or tests must honour
@answer n=2 entity=obs-scope-summary#17.2 v=no
why: classifies CI/CD as not-instrumentable, the chunk adds no instrumentation to CI and the entity asks for no code or test action
@answer n=3 entity=obs-scope-summary#17.3 v=no standing=S5
why: fixtures are scrub-required data, the fixture-input convention every test and fixture owes (synthetic inputs)
@answer n=4 entity=obs-scope-summary#18.1 v=yes
why: the tracing JSON logger and the per-process panic hook writing one line bind the panic-hook and catch-site work (needs 10)
@answer n=5 entity=obs-scope-summary#18.2 v=no
why: frontend N/A, and the static fixed-column terminal text is the behaviour of human::refuse, which the scope does not name, so that component keeps it by its own code
@answer n=6 entity=obs-scope-summary#18.3 v=yes
why: the file-sink-only exporter, the per-verb cli logging, the refusals that write no line and the same file sink on every OS bind the CLI exit-21 and macOS carve-out work (needs 3, 6)
@answer n=7 entity=obs-scope-summary#18.4 v=yes
why: the process and instance fields are required on every line (needs 4), and the service.name part is not reached
@answer n=8 entity=obs-scope-summary#18.5 v=yes
why: fixed refusal strings with no path or pid, exit codes as a log field, and distinct detail codes including the macOS euid-only server-verify-failed bind the diagnostics and refusal work (needs 5, 6, 8)
@answer n=9 entity=obs-scope-summary#19.1 v=yes
why: the pty seam operations are instrumented at the seam, and the sync-only rule with no Tokio binds the viola-pty work and cargo deny (needs 1, 2, 17)
@answer n=10 entity=obs-scope-summary#19.2 v=yes
why: the terminal carries the child's screen, so the Unix pty relay must send the child's bytes to the terminal and log none of them (needs 1, 8)
@answer n=11 entity=obs-scope-summary#19.3 v=yes
why: the run file sink to diagnostics/run-<name>.ndjson and nothing on the terminal while the child runs (needs 8, 20)
@answer n=12 entity=obs-scope-summary#19.4 v=yes
why: process=run and instance=ViolaName on every run line, including the revive start arm (needs 4, 9)
@answer n=13 entity=obs-scope-summary#19.5 v=yes
why: the panic hook is replaced before the child spawns (need 10), the 1 s heartbeat is the liveness tick (need 18), and child exit comes from the handle wait, not reader EOF (need 2)
@answer n=14 entity=obs-scope-summary#20.1 v=yes
why: hook-invoked and hook-decision lines and one catch_unwind panic line bind the hook logging and panic catch sites (needs 4, 10, 19)
@answer n=15 entity=obs-scope-summary#20.2 v=no
why: frontend N/A, the hook has no frontend surface to instrument
@answer n=16 entity=obs-scope-summary#20.3 v=yes
why: the hook file sink with stdout carrying only the decision body, stderr empty, and failed log writes swallowed while exit stays 0 (need 19)
@answer n=17 entity=obs-scope-summary#20.4 v=yes
why: process=hook, a null instance with a silent no-op when VIOLA_NAME is missing, and the strict-modes refusal ahead of any log, config read or stdin read (need 19)
@answer n=18 entity=obs-scope-summary#20.5 v=yes
why: one write per line on an append handle, the 1.0 s latency budget and the hook perf rows with the socket-directory and peer-euid checks in the timed path (needs 7, 16)
@answer n=19 entity=obs-scope-summary#21.1 v=yes
why: corr on channel-request and channel-response, with no trace-context field, binds the channel telemetry (need 7)
@answer n=20 entity=obs-scope-summary#21.2 v=no
why: frontend N/A, the channel has no frontend surface to instrument
@answer n=21 entity=obs-scope-summary#21.3 v=yes
why: each side's log file is run, hook, mcp or cli under diagnostics (need 20)
@answer n=22 entity=obs-scope-summary#21.4 v=yes
why: each side logs its own process, the required field on every line (needs 4, 7)
@answer n=23 entity=obs-scope-summary#21.5 v=yes
why: corr is not unique across connections so the conn discriminator is required, and a peer refused by euid writes no line (need 7)
@answer n=24 entity=obs-scope-summary#22.1 v=yes
why: MCP tool-call spans wrap the Tokio client beside the macOS MCP refusal, which must return instance-unreachable with no line (needs 6, 17)
@answer n=25 entity=obs-scope-summary#22.2 v=no
why: frontend N/A for MCP, no frontend surface to instrument
