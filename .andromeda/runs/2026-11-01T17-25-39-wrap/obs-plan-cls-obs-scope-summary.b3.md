# Classified — obs-plan · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · the arch CI/CD source citation is untouched by a session that changes no platform fact
obs-scope-summary#17.2 · no · not-instrumentable still holds, the session adds no instrumented surface
obs-scope-summary#17.3 · no · the fixture exclusion holds, the SocketRoot helper in tests/support/home.rs is not a telemetry fixture
obs-scope-summary#18.1 · no · the tracing facade, dispatch spans and panic hook are unchanged by the session
obs-scope-summary#18.2 · no · terminal-only output still holds, the new socket-dir hint is plain terminal text
obs-scope-summary#18.3 · yes · stale · the frame verbs' socket-directory refusal after the instance check writes one WARN process-exit line (F3), so the reading that a refused check writes no line moved
obs-scope-summary#18.4 · no · service identity fields are unchanged
obs-scope-summary#18.5 · yes · contradict · the socket-directory cause now has its own detail code socket-dir-refused (F1), so it is no longer a named carve-out, and macOS's pid gap is owned by no route entry
obs-scope-summary#19.1 · yes · stale · the run start gains a socket-directory step with no span before any snapshot, child or bind (F2), so the start-sequence spans reading moved
obs-scope-summary#19.2 · no · frontend N/A still holds, the child's screen is the terminal
obs-scope-summary#19.3 · no · the run log path holds, and the refusal happens before the child spawns, so no terminal write while the child runs
obs-scope-summary#19.4 · no · service identity for run is unchanged
obs-scope-summary#19.5 · no · the panic hook, heartbeat and handle-wait exit detection are untouched by the session
obs-scope-summary#20.1 · no · the hook spans and required hook-invoked and hook-decision lines hold, since the refused socket directory writes its hook-decision inside hook.handle (F4)
obs-scope-summary#20.2 · no · frontend N/A still holds
obs-scope-summary#20.3 · no · the hook-<name>.ndjson sink and empty-stdout rule hold, the refusal writes its line to the same sink with exit 0 and no frame
obs-scope-summary#20.4 · yes · stale · a refused socket directory on the dialog path writes one hook-decision after obs init (F4), so the silent-no-op reading for hooks moved, while the refused-instance silence itself holds
obs-scope-summary#20.5 · no · the perf band and the cited 2026-10-26 gate values stay a true record, and this run's measured values move none of them
obs-scope-summary#21.1 · no · client and server spans still hold, a dropped peer never reaches dispatch and makes no channel call
obs-scope-summary#21.2 · no · frontend N/A still holds
obs-scope-summary#21.3 · no · the per-process log file holds, a dropped peer writes no line, and the row does not claim every connection logs
obs-scope-summary#21.4 · no · params.sender as the peer version is unchanged
obs-scope-summary#21.5 · no · the corr uniqueness gap and the unauthenticated from rule still hold, since the euid gate authenticates no self-reported name
obs-scope-summary#22.1 · no · per-call spans wrapping the Tokio client still hold for calls that reach it, and the refused frame call returns before the client
obs-scope-summary#22.2 · no · frontend N/A still holds
