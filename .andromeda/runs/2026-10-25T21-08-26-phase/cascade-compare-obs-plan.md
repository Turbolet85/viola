# Cascade compare — obs-plan · phase · 2026-10-25-windows-endpoint-admission
E items 133 · held 41 · lost at classify 3 · lost at strong 2 · beside a placed fact - · cascade only 40 · body 77 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.3 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:12
- (.andromeda/obs-plan-detail/ci-integration.md:12) The Windows-leg evidence for the chunk's assertions is read from the nextest JUnit artifact (`junit-windows-2025`) and the gate exit codes, uploaded on failing runs too.
  > cargo-nextest 0.9.146 JUnit XML (`[profile.ci.junit] path = "junit.xml"` in `.config/nextest.toml` → `target/nextest/ci/junit.xml`) | every test job, `if: always() && steps.secret-scan.outcome == 'success'`.

### error-capture-reporting#2.1 · lost at classify · .andromeda/obs-plan-detail/error-capture-reporting.md:16
- (.andromeda/obs-plan-detail/error-capture-reporting.md:16) Any change to the `viola-channel` listener (`server.rs`, `server/win.rs`) must keep a panic in one connection worker contained: the `channel.dispatch` `Drop` guard still logs the `-32603` response, the connection closes, and `run` continues.
  > A panic in one channel server connection worker is contained: the `channel.dispatch` `Drop` guard logs `channel-response{result_class:"error", error_code:-32603}`, the connection closes, and `run` continues.

### error-capture-reporting#2.1 · lost at classify · .andromeda/obs-plan-detail/error-capture-reporting.md:15
- (.andromeda/obs-plan-detail/error-capture-reporting.md:15) The listener bind and the squatted-name start refusal run inside `run`'s start, so they must stay under the main-thread catch site: no new listener or first-instance path may end `run` through an uncaught unwind.
  > everything after `set_hook` (clap parse, init and role dispatch) runs inside one `std::panic::catch_unwind` in the root bin's `main`. An `Err` maps to the role's exit below, so no role ends through an uncaught unwind (Rust's default exit 101).

### slo-invariants-telemetry-budgets#2.2 · lost at classify · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:28
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:28) A change to the Windows pipe listener's security attributes or open mode must keep the spine and `pre-tool-use` hooks' hyperfine `max` under the spine deadline in the per-OS `perf` job, the Windows leg included.
  > hyperfine `max <` spine deadline in `target/agent-run/artifacts/perf-<hook>.json`, judged by `gate --require perf` (each spine row and the `pre-tool-use` row required by name; the perf session boots unstamped, so `pre-tool-use` runs on an unverified CLI). The deadline value is tests-owned (provisionally 1.0 s; the hook's own provisional 750 ms connect deadline sits below it); obs requires only that the max sample is gated

### span-trace-coverage#8.2 · lost at strong · .andromeda/obs-plan-detail/span-trace-coverage.md:137
- (.andromeda/obs-plan-detail/span-trace-coverage.md:137) The send-arrival budget evaluation behind the CARRY case must keep its span shape: declared once on `BudgetGate::decide_at`, `skip_all`, with `trigger`, `paused` and `window` only when paused.
  > `run.budget_eval`: `trigger` (`start|send|turn-ended`), `paused`, and `window`, recorded only when paused. As landed (chunk 2026-10-10-budget-governor) it is declared once, on `BudgetGate::decide_at`, `skip_all`, and opened once per evaluation: at start, at a `send`'s arrival, and after a `turn-ended` hook line is appended.

### telemetry-strategy#1.4 · lost at strong · .andromeda/obs-plan-detail/telemetry-strategy.md:13
- (.andromeda/obs-plan-detail/telemetry-strategy.md:13) The chunk must add no in-process metric instrument or exporter for admission outcomes; any count is derived by `jq` over the ndjson.
  > | Metrics | `duration_ms` samples, counts of `event`/`detail` values, `skipped` counts | No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson

## Held

- ci-integration#2.1
- ci-integration#2.3
- ci-integration#4.1
- log-coverage#1.5
- log-coverage#2.1
- log-coverage#2.6
- metric-coverage#1.1
- metric-coverage#2.11
- metric-coverage#2.4
- obs-anti-patterns#1.1
- obs-scope-summary#14.3
- obs-scope-summary#15.3
- obs-scope-summary#18.5
- obs-scope-summary#23.1
- obs-scope-summary#25.0
- obs-scope-summary#27.2
- obs-scope-summary#27.3
- obs-scope-summary#32.2
- obs-scope-summary#32.3
- obs-scope-summary#44.1
- obs-scope-summary#47.1
- obs-scope-summary#48.1
- obs-scope-summary#50.1
- obs-scope-summary#6.3
- pii-scrubbing-compliance#1.10
- pii-scrubbing-compliance#1.12
- pii-scrubbing-compliance#1.8
- pii-scrubbing-compliance#1.9
- pii-scrubbing-compliance#2.1
- slo-invariants-telemetry-budgets#1.1
- span-trace-coverage#2.4
- span-trace-coverage#3.1
- span-trace-coverage#3.2
- span-trace-coverage#3.3
- span-trace-coverage#4.3
- span-trace-coverage#8.3
- telemetry-strategy#1.1
- telemetry-strategy#1.2
- telemetry-strategy#1.3
- telemetry-strategy#3.1
- telemetry-strategy#3.12

## Cascade only

- ci-integration#1.1
- ci-integration#3.1
- ci-integration#3.2
- ci-integration#3.3
- log-coverage#1.1
- log-coverage#1.2
- log-coverage#1.8
- obs-scope-summary#14.2
- obs-scope-summary#18.1
- obs-scope-summary#19.1
- obs-scope-summary#19.3
- obs-scope-summary#19.4
- obs-scope-summary#19.5
- obs-scope-summary#21.1
- obs-scope-summary#21.3
- obs-scope-summary#21.4
- obs-scope-summary#21.5
- obs-scope-summary#24.1
- obs-scope-summary#26.1
- obs-scope-summary#27.1
- obs-scope-summary#28.3
- obs-scope-summary#3.2
- obs-scope-summary#3.3
- obs-scope-summary#34.1
- obs-scope-summary#4.3
- obs-scope-summary#40.1
- obs-scope-summary#41.1
- obs-scope-summary#45.1
- obs-scope-summary#6.2
- pii-scrubbing-compliance#1.2
- pii-scrubbing-compliance#1.4
- span-trace-coverage#2.2
- telemetry-strategy#2.1
- telemetry-strategy#3.11
- telemetry-strategy#3.13
- telemetry-strategy#3.15
- telemetry-strategy#3.16
- telemetry-strategy#3.2
- telemetry-strategy#3.5
- telemetry-strategy#3.8
