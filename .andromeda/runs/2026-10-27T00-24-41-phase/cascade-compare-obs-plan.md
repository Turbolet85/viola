# Cascade compare — obs-plan · phase · 2026-10-27-pinned-copy-race-and-reproducible-builds
E items 125 · held 30 · lost at classify 3 · lost at strong 4 · beside a placed fact - · cascade only 32 · listed 6 · body 60 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### metric-coverage#1.1 · lost at strong · .andromeda/obs-plan-detail/metric-coverage.md:10
- (.andromeda/obs-plan-detail/metric-coverage.md:10) A boot-failure or refusal count the two-sided witness derives from `diagnostics/` must count over the home-level role files only and skip a torn line. A count over detail files would double it.
  > over home-level role files only (detail files repeat lines; a torn line is skipped as in G2)

### metric-coverage#1.2 · lost at classify · .andromeda/obs-plan-detail/metric-coverage.md:11
- (.andromeda/obs-plan-detail/metric-coverage.md:11) A latency the chunk reports, such as the measured 5–11 ms `ERROR_SHARING_VIOLATION` wait-out or the window that shows the race is reachable, must come from raw `duration_ms` samples with no bucketing.
  > Raw `duration_ms` samples per event, with no bucketing; quantiles by sort/index in `jq`

### slo-invariants-telemetry-budgets#2.2 · lost at classify · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:29
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:29) The race lives on unstamped boots, and so does the perf session; any wait or change the fix adds to the pinned path's write or spawn must keep each spine row and the `pre-tool-use` row under the gated hyperfine `max`.
  > | Spine hooks (`session-start`, `user-prompt-submit`, `stop`) and `pre-tool-use` on an unverified CLI | report-only | report-only | hyperfine `max <` spine deadline in `target/agent-run/artifacts/perf-<hook>.json`, judged by `gate --require perf` (each spine row and the `pre-tool-use` row required by name; the perf session boots unstamped, so `pre-tool-use` runs on an unverified CLI).

### span-trace-coverage#4.4 · lost at strong · .andromeda/obs-plan-detail/span-trace-coverage.md:60
- (.andromeda/obs-plan-detail/span-trace-coverage.md:60) A wait the fix adds inside the pin step still closes inside `run.start`, which closes only when `pty.spawn` returns.
  > `run.start` closes when `pty.spawn` returns.

### telemetry-strategy#1.1 · lost at strong · .andromeda/obs-plan-detail/telemetry-strategy.md:10
- (.andromeda/obs-plan-detail/telemetry-strategy.md:10) Any span the race fix adds or changes on the `viola-state` pin seam (`pin_bytes` / `pin_exe` / `replace_private_shared`), reached from `run`'s start and `verify`, must be declared with `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`, never a bare or serialized span.
  > every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`.

### telemetry-strategy#1.4 · lost at classify · .andromeda/obs-plan-detail/telemetry-strategy.md:13
- (.andromeda/obs-plan-detail/telemetry-strategy.md:13) The two-sided race witness and the release-build comparison take their counts from harness or ndjson output (or the build's own files), never from an in-process instrument added to the product.
  > No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson

### telemetry-strategy#3.12 · lost at strong · .andromeda/obs-plan-detail/telemetry-strategy.md:39
- (.andromeda/obs-plan-detail/telemetry-strategy.md:39) A Windows-specific arm of the exe pin (for example a sharing-violation wait), read on the CI runner only, must keep the same file sink and line schema on every OS.
  > | multi-platform-exporter-compat | §3 Log file location (0600/0700 vs home DACL), §3 Logging stack (concurrent-append check on windows-2025 / macos-latest / ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 per-OS artifacts |

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#2.3 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:24
- (.andromeda/obs-plan-detail/ci-integration.md:24) The pin-race witness (a concurrent two-instance boot or a narrower concurrent-pin control) must put every viola home it makes under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it; the chaos home is the only carve-out and this chunk adds none.
  > Their viola homes must live under `target/e2e-home/` (the directory layout is tests-owned), so G2, G4, the secret scan and the `diag-<os>` upload cover them. A `hook` panic exits 0, so a home outside that root would hide it. The one carve-out: `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the secret scan and the upload

### ci-integration#4.1 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:59
- (.andromeda/obs-plan-detail/ci-integration.md:59) The witness's failing runs (a hook spawn failing with `ran:false`) must still reach G2 and G4: both run `if: always()` after the tests, so a red witness step never skips them.
  > 1. Integration and E2E tests (every viola home under `target/e2e-home/` but the one `viola-chaos-*` home of `tests/chaos_feed_panic.rs`, outside G2, G4 and the secret scan by the founder's ruling of 2026-10-04), then G2 and G4, **both** `if: always()`.

### ci-integration#4.1 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:59
- (.andromeda/obs-plan-detail/ci-integration.md:59) A witness that boots two instances on one fresh home (or a Path 7 returned to two instances, if P4 decides so) must leave its homes in place until the gate steps have read them.
  > **Homes are kept until the gate steps have run** (overseer fix pass 2, B1): `ci.yml` sets `AGENT_RUN_KEEP_HOMES=1`, so neither harness `cleanup` nor a test home's drop deletes a home before G2, G4, the secret scan and the scan-gated uploads have read it (test-plan §3 `cleanup` step 6).

### metric-coverage#2.11 · listed (lost at classify) · .andromeda/obs-plan-detail/metric-coverage.md:30
- (.andromeda/obs-plan-detail/metric-coverage.md:30) The concurrent-first-start tests and the race reproduction must write zero `event:"panic"` lines in any home G2 counts. The two-instance boot reaching that state does not exempt it.
  > | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |

### pii-scrubbing-compliance#1.10 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) A pid read in the race (the other start's, the agent's hook spawn, the `ETXTBSY` reproduction) may be logged only as a typed field, never inside a `detail` code.
  > | `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |

### pii-scrubbing-compliance#1.13 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) Any new outcome code for the pin race (or a kernel error the fix reads, such as the hypothesised `ETXTBSY`) must be a closed kebab-case enum value, never raw OS error text.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |

### telemetry-strategy#2.1 · listed (lost at strong) · .andromeda/obs-plan-detail/telemetry-strategy.md:20
- (.andromeda/obs-plan-detail/telemetry-strategy.md:20) A panic on the pin path after init step 4 must still produce its one line; the exemption list is closed, so moving the pin or a new lock or wait ahead of the role file's open adds no unlisted no-line window.
  > Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance.

## Held

- ci-integration#3.2
- ci-integration#3.3
- error-capture-reporting#1.1
- error-capture-reporting#2.1
- log-coverage#1.1
- log-coverage#2.1
- log-coverage#2.7
- obs-anti-patterns#1.1
- obs-scope-summary#14.3
- obs-scope-summary#15.3
- obs-scope-summary#18.3
- obs-scope-summary#23.1
- obs-scope-summary#27.2
- obs-scope-summary#27.3
- obs-scope-summary#38.1
- obs-scope-summary#44.1
- obs-scope-summary#48.1
- obs-scope-summary#50.1
- pii-scrubbing-compliance#1.8
- pii-scrubbing-compliance#1.9
- pii-scrubbing-compliance#2.1
- slo-invariants-telemetry-budgets#1.1
- span-trace-coverage#16.1
- span-trace-coverage#4.1
- span-trace-coverage#4.2
- span-trace-coverage#4.3
- telemetry-strategy#1.2
- telemetry-strategy#1.3
- telemetry-strategy#3.16
- telemetry-strategy#3.6

## Cascade only

- ci-integration#2.1
- error-capture-reporting#3.1
- log-coverage#1.2
- log-coverage#1.7
- log-coverage#1.8
- metric-coverage#1.3
- obs-scope-summary#10.2
- obs-scope-summary#10.3
- obs-scope-summary#11.2
- obs-scope-summary#11.3
- obs-scope-summary#14.2
- obs-scope-summary#15.2
- obs-scope-summary#18.4
- obs-scope-summary#18.5
- obs-scope-summary#19.1
- obs-scope-summary#19.3
- obs-scope-summary#20.3
- obs-scope-summary#20.4
- obs-scope-summary#3.2
- obs-scope-summary#3.3
- obs-scope-summary#35.1
- obs-scope-summary#39.1
- obs-scope-summary#40.1
- obs-scope-summary#47.1
- obs-scope-summary#7.2
- obs-scope-summary#7.3
- span-trace-coverage#2.1
- span-trace-coverage#2.2
- span-trace-coverage#2.3
- telemetry-strategy#3.11
- telemetry-strategy#3.18
- telemetry-strategy#3.8
