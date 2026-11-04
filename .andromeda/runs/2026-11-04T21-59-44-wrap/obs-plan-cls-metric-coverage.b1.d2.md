@answer n=1 entity=metric-coverage#1.1 v=no
why: the session moved no event, detail or refusal value and no diagnostics file, so the Counter derivation over send-refused still holds.
@answer n=2 entity=metric-coverage#1.2 v=no
why: no duration_ms emitting site changed, so the raw histogram samples per event still hold.
@answer n=3 entity=metric-coverage#1.3 v=no
why: no liveness rule changed and the stale-row witness reads list, not the agent-run status gauge source.
@answer n=4 entity=metric-coverage#2.1 v=no
why: no hook-decision emission or hook code moved, so hook latency still derives from hook-decision.duration_ms.
@answer n=5 entity=metric-coverage#2.2 v=no
why: no channel-response emission or channel call path changed, so channel latency still derives from channel-response.duration_ms.
@answer n=6 entity=metric-coverage#2.3 v=no
why: no send path or send-confirmed line changed, and src/run/send.rs is not in the session's files.
@answer n=7 entity=metric-coverage#2.4 v=no
why: send-confirmed and send-refused values and their emission are unchanged by the session.
@answer n=8 entity=metric-coverage#2.5 v=no
why: channel-response{method:"wait"}.outcome is emitted as before, the session moved no wait outcome.
@answer n=9 entity=metric-coverage#2.6 v=no
why: no state-recovered emission changed, and the stale-heartbeat witness asserts a stale row rather than a recovery.
@answer n=10 entity=metric-coverage#2.7 v=no
why: list --json skipped is unchanged, the session touched only list's terminal colour layer and its tests.
@answer n=11 entity=metric-coverage#2.8 v=no
why: parse-rejected emission and the parsers are untouched by the session.
@answer n=12 entity=metric-coverage#2.9 v=no
why: send-refused unknown, unsupported-version and invalid-params emission are unchanged, no compat path moved.
@answer n=13 entity=metric-coverage#2.10 v=no
why: process-exit fields and exit_source values are unchanged because product code is unchanged, and the new Unix test reads pids and start time, not the line.
@answer n=14 entity=metric-coverage#2.11 v=no
why: no panic event, panic hook or catch_unwind site changed, so the zero-panic SLO row still holds.
@answer n=15 entity=metric-coverage#3.1 v=no
why: the macOS euid-only carve-out behaviour is unchanged, and the 43-case test count is test inventory this row does not state.
@answer n=16 entity=metric-coverage#3.2 v=no
why: the strict-modes and check_instance refusals write no line as before, and the session moved only the Frozen guard's home, not instance-state refusal code.
@answer n=17 entity=metric-coverage#4.0 v=no
why: the canonical quantile snippet reads hook-decision duration_ms, which the session did not change.
