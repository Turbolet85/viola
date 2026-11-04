@answer n=1 entity=obs-scope-summary#17.1 v=no
why: the CI/CD platform source reading stands, since the session adds no telemetry or sink there
@answer n=2 entity=obs-scope-summary#17.2 v=no
why: the not-instrumentable reading still holds, since the session adds no instrumentation
@answer n=3 entity=obs-scope-summary#17.3 v=no
why: fixtures/ did not move and the session's test files are not fixtures, so the reason stands
@answer n=4 entity=obs-scope-summary#18.1 v=no
why: the CLI tracing spans and panic hook are product code the session did not change
@answer n=5 entity=obs-scope-summary#18.2 v=no
why: CLI output stays stdout and stderr with no interactive surface, and the list colour witness adds no output surface
@answer n=6 entity=obs-scope-summary#18.3 v=no
why: the exporter sinks and logging verbs are product code the session did not change, and list adds no stderr writer
@answer n=7 entity=obs-scope-summary#18.4 v=no
why: service identity is compile-time and unchanged, and the session touches no identity field
@answer n=8 entity=obs-scope-summary#18.5 v=yes mode=stale
why: the session now counts the macOS no-peer-pid carve-out as 43 ubuntu-not-macos cases, beside a euid-only rule that stays unchanged
@answer n=9 entity=obs-scope-summary#19.1 v=no
why: the wrapper's tracing spans and panic hook are product code the session did not change
@answer n=10 entity=obs-scope-summary#19.2 v=no
why: the wrapper terminal stays the child's screen, and the outer-PTY witness observes list rather than run output
@answer n=11 entity=obs-scope-summary#19.3 v=no
why: run's file sink and its no-terminal rule are unchanged, and the session adds no run output
@answer n=12 entity=obs-scope-summary#19.4 v=no
why: run's service identity fields are product code the session did not change
@answer n=13 entity=obs-scope-summary#19.5 v=yes mode=stale
why: run_child_ends_when_its_wrapper_is_terminated now witnesses on Unix that the child ends after a wrapper SIGKILL, a reading of the child boundary this entity states only for the handle wait
@answer n=14 entity=obs-scope-summary#20.1 v=no
why: hook spans and the catch_unwind panic line are product code the session did not change
@answer n=15 entity=obs-scope-summary#20.2 v=no
why: the hook has no frontend and the session adds none
@answer n=16 entity=obs-scope-summary#20.3 v=no
why: the hook sink path and stderr ban are unchanged, and the session edits no hook code
@answer n=17 entity=obs-scope-summary#20.4 v=no
why: the FAKE_AGENT_HOOK_PANIC seam keeps its place after obs init, and the session changed only the fake agent's test reader
@answer n=18 entity=obs-scope-summary#20.5 v=no
why: the hook latency budget and its measured rows are unchanged, and the session runs no perf measurement
@answer n=19 entity=obs-scope-summary#21.1 v=no
why: channel correlation and span propagation are product code the session did not change
@answer n=20 entity=obs-scope-summary#21.2 v=no
why: the channel has no frontend and the session adds none
@answer n=21 entity=obs-scope-summary#21.3 v=no
why: the per-process log sinks are unchanged
@answer n=22 entity=obs-scope-summary#21.4 v=no
why: service identity and the peer version field are unchanged
@answer n=23 entity=obs-scope-summary#21.5 v=no
why: the peer euid drop on the accept thread is product code the session did not change, and it holds on macOS as well
@answer n=24 entity=obs-scope-summary#22.1 v=no
why: MCP tool-call spans are product code the session did not change
@answer n=25 entity=obs-scope-summary#22.2 v=no
why: the MCP surface has no frontend and the session adds none
