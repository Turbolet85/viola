# Cascade compare — obs-plan · phase · 2026-10-26-home-and-code-bearing-file-integrity
E items 168 · held 59 · lost at classify 1 · lost at strong 1 · beside a placed fact - · cascade only 55 · listed 7 · body 73 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### error-capture-reporting#1.1 · lost at strong · .andromeda/obs-plan-detail/error-capture-reporting.md:8
- (.andromeda/obs-plan-detail/error-capture-reporting.md:8) The panic hook's detail line lands in the instance's `diagnostics/detail-<process>.ndjson`, one of the diagnostics files the chunk's DACL / strict-modes check must precede before it is opened.
  > When an instance resolves, it also writes `{"event":"panic", ..., "panic_payload":"<escaped>", "backtrace":[...frames]}` as one line to `detail-<process>.ndjson`.

### metric-coverage#1.1 · lost at classify · .andromeda/obs-plan-detail/metric-coverage.md:10
- (.andromeda/obs-plan-detail/metric-coverage.md:10) A refusal this chunk adds, such as a strict-modes refusal or a pinned-exe hash mismatch, is countable only through a closed `event` / `detail` / `refusal` value in a home-level role file. A refusal made before any role file is opened leaves no line for this counter to count.
  > | Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#3.3 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:41
- (.andromeda/obs-plan-detail/ci-integration.md:41) Every line the new checks write (a refusal's `process-exit{detail}` on a verb that gains `strict-modes-failed`, a hook's fail-open `hook-decision`) is checked against the schemas, so each new `detail` code must be in `schemas/diag-line.v1.json` or G4 is red.
  > Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).

### ci-integration#3.2 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:39
- (.andromeda/obs-plan-detail/ci-integration.md:39) The hook client's new strict-modes check, which fails open, adds no G2 exemption: the seam line stays the only exemption.
  > One exemption, ratified by the founder live on 2026-09-28 (security-plan Decisions Log): a line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` — the `fake-agent` seam's forced panic — is not counted; the file is compared as a whole string, never a prefix, suffix, regex or home path.

### ci-integration#2.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:22
- (.andromeda/obs-plan-detail/ci-integration.md:22) The new check, re-hash and rewrite code in `viola-state`, the hook client, `client::start`, `verify` and `run` must pass the lint stage. It logs only through `obs_event!`, prints nothing, and keeps the sync crates Tokio-free.
  > clippy `-D warnings` incl. `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}!` outside `viola_core::obs`) and the `print_stdout` / `print_stderr` / `dbg_macro` bans; the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh`, Linux leg (mechanism in §3 logger-stack-install); bare-`#[instrument]` gate G1 (below); `cargo check` of sync crates without Tokio; `cargo deny check` (ubuntu)

### ci-integration#3.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:34
- (.andromeda/obs-plan-detail/ci-integration.md:34) Any span the chunk adds on a check or re-hash function must put `skip_all` first, or G1 fails.
  > `-U` lets `\s*` span lines, so an `#[instrument(` with `skip_all` on the next line passes. `skip_all` must be the first argument (the §4 form).

### metric-coverage#2.11 · listed (lost at strong) · .andromeda/obs-plan-detail/metric-coverage.md:30
- (.andromeda/obs-plan-detail/metric-coverage.md:30) The new checks, re-hash and plugin rewrite must not add a panic. The panic count must stay at 0.
  > | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |

### pii-scrubbing-compliance#1.10 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) The snapshot `pid` / `started_at` that the strict-modes check now guards may appear only as typed fields, never inside a refusal's `detail` code.
  > | `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |

### telemetry-strategy#1.1 · listed (lost at strong) · .andromeda/obs-plan-detail/telemetry-strategy.md:10
- (.andromeda/obs-plan-detail/telemetry-strategy.md:10) Any new `viola-state` strict-modes / liveness call site this chunk adds before a snapshot or stamps read must be declared with the `skip_all` instrument form and a static `<area>.<operation>` name.
  > `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`.

## Held

- ci-integration#2.3
- ci-integration#4.1
- error-capture-reporting#2.1
- error-capture-reporting#3.1
- log-coverage#1.1
- log-coverage#1.10
- log-coverage#2.1
- log-coverage#2.2
- log-coverage#2.4
- log-coverage#2.5
- log-coverage#2.7
- metric-coverage#2.1
- metric-coverage#2.6
- metric-coverage#3.1
- obs-anti-patterns#1.1
- obs-scope-summary#15.3
- obs-scope-summary#18.3
- obs-scope-summary#2.1
- obs-scope-summary#20.5
- obs-scope-summary#22.3
- obs-scope-summary#23.1
- obs-scope-summary#26.1
- obs-scope-summary#27.2
- obs-scope-summary#27.3
- obs-scope-summary#35.1
- obs-scope-summary#38.1
- obs-scope-summary#43.1
- obs-scope-summary#44.1
- obs-scope-summary#48.1
- pii-scrubbing-compliance#1.7
- pii-scrubbing-compliance#1.8
- pii-scrubbing-compliance#1.9
- pii-scrubbing-compliance#2.1
- slo-invariants-telemetry-budgets#1.1
- slo-invariants-telemetry-budgets#2.1
- slo-invariants-telemetry-budgets#2.2
- slo-invariants-telemetry-budgets#2.4
- span-trace-coverage#13.1
- span-trace-coverage#15.1
- span-trace-coverage#16.1
- span-trace-coverage#2.1
- span-trace-coverage#3.1
- span-trace-coverage#3.2
- span-trace-coverage#3.3
- span-trace-coverage#3.4
- span-trace-coverage#4.1
- span-trace-coverage#4.2
- span-trace-coverage#4.3
- span-trace-coverage#5.4
- span-trace-coverage#6.3
- span-trace-coverage#7.3
- span-trace-coverage#8.3
- span-trace-coverage#9.3
- telemetry-strategy#1.2
- telemetry-strategy#1.3
- telemetry-strategy#2.1
- telemetry-strategy#3.12
- telemetry-strategy#3.3
- telemetry-strategy#3.6

## Cascade only

- log-coverage#1.11
- log-coverage#1.2
- log-coverage#1.7
- obs-scope-summary#1.4
- obs-scope-summary#11.2
- obs-scope-summary#11.3
- obs-scope-summary#13.3
- obs-scope-summary#14.2
- obs-scope-summary#14.3
- obs-scope-summary#15.2
- obs-scope-summary#18.1
- obs-scope-summary#18.5
- obs-scope-summary#19.1
- obs-scope-summary#19.3
- obs-scope-summary#19.5
- obs-scope-summary#2.2
- obs-scope-summary#20.1
- obs-scope-summary#20.3
- obs-scope-summary#20.4
- obs-scope-summary#22.1
- obs-scope-summary#22.5
- obs-scope-summary#24.1
- obs-scope-summary#25.0
- obs-scope-summary#29.2
- obs-scope-summary#29.3
- obs-scope-summary#3.2
- obs-scope-summary#3.3
- obs-scope-summary#30.2
- obs-scope-summary#30.3
- obs-scope-summary#31.2
- obs-scope-summary#36.1
- obs-scope-summary#42.1
- obs-scope-summary#7.2
- obs-scope-summary#7.3
- obs-scope-summary#9.2
- span-trace-coverage#1.1
- span-trace-coverage#1.2
- span-trace-coverage#1.3
- span-trace-coverage#2.2
- span-trace-coverage#2.3
- span-trace-coverage#2.5
- span-trace-coverage#5.1
- span-trace-coverage#5.3
- span-trace-coverage#6.1
- span-trace-coverage#6.4
- span-trace-coverage#7.1
- span-trace-coverage#7.4
- span-trace-coverage#8.1
- telemetry-strategy#1.4
- telemetry-strategy#3.10
- telemetry-strategy#3.11
- telemetry-strategy#3.13
- telemetry-strategy#3.4
- telemetry-strategy#3.5
- telemetry-strategy#3.7
