# Cascade compare — obs-plan · phase · 2026-10-26-explicit-session-links
E items 169 · held 48 · lost at classify 3 · lost at strong 1 · beside a placed fact - · cascade only 26 · listed 8 · body 92 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### metric-coverage#1.1 · lost at classify · .andromeda/obs-plan-detail/metric-coverage.md:10
- (.andromeda/obs-plan-detail/metric-coverage.md:10) The new verbs' refusals and exits must land as closed `event` / `detail` / `refusal` values in the home-level role files, so the Counter derivation counts them by grouping alone.
  > | Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |

### metric-coverage#3.1 · lost at strong · .andromeda/obs-plan-detail/metric-coverage.md:36
- (.andromeda/obs-plan-detail/metric-coverage.md:36) The two verbs take the server check through `client::connect` like the other CLI frame verbs, so their server-verification refusal must be the same one `process-exit` line, counted by the Counter row's `detail` grouping.
  > The CLI's server-verification refusal (Linux and Windows) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, counted by the Counter row's `detail` grouping;

### slo-invariants-telemetry-budgets#2.3 · lost at classify · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:29
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:29) The verbs' liveness-only pre-check must use the same heartbeat liveness reading, with the same flip, as the other frame verbs; it must not define a staleness threshold of its own.
  > | Heartbeat staleness flip | n/a | n/a | 4.9 s → `live`, 5.1 s → `stale`/`gone`, read as `list --json` `liveness` |

### span-trace-coverage#4.1 · lost at classify · .andromeda/obs-plan-detail/span-trace-coverage.md:39
- (.andromeda/obs-plan-detail/span-trace-coverage.md:39) The start-time link set restore must keep appending and logging nothing once `unlink` lines exist in the log, so the start span chain and product order stay unchanged.
  > A third start step carries none either, as landed (chunk 2026-10-10-session-links): the link set's restore, one read pass over the instance's `events.ndjson` through `viola_state::events::read_from`, made beside the wait feed's rebuild and before the endpoint serves. It appends nothing and logs nothing, its result is the first snapshot's `links`, and the start appends no record for links, so the span chain above and the product order below are unchanged;

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#2.3 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:24
- (.andromeda/obs-plan-detail/ci-integration.md:24) Every viola home the chunk's new root cases make (`tests/channel_links.rs` and any test of the `link` / `unlink` verbs) must live under `target/e2e-home/`, with no second outside-scan home: the chaos panic test's home is the one carve-out.
  > `diagnostics/*.ndjson` from fake-agent runs. Their viola homes must live under `target/e2e-home/` (the directory layout is tests-owned), so G2, G4, the secret scan and the `diag-<os>` upload cover them. A `hook` panic exits 0, so a home outside that root would hide it. The one carve-out: `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the secret scan and the upload, whose deliberate panic line and `parse-rejected` line the test asserts present itself (the founder's ruling, live, 2026-10-04, relayed by the overseer).

### ci-integration#2.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:22
- (.andromeda/obs-plan-detail/ci-integration.md:22) The new verbs', the wrapper's two served methods' and the tests' code must pass the lint-stage gates: no raw tracing level macro outside `viola_core::obs`, no print / dbg macro, no raw `event!`, no bare `#[instrument]`.
  > clippy `-D warnings` incl. `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}!` outside `viola_core::obs`) and the `print_stdout` / `print_stderr` / `dbg_macro` bans; the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh`, Linux leg (mechanism in §3 logger-stack-install); bare-`#[instrument]` gate G1 (below);

### ci-integration#3.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:34
- (.andromeda/obs-plan-detail/ci-integration.md:34) Any span the chunk adds (the verbs' client span, the wrapper's `link` / `unlink` dispatch) must take `skip_all` as its first `#[instrument]` argument, or G1 fails.
  > `skip_all` must be the first argument (the §4 form).

### ci-integration#3.2 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:36
- (.andromeda/obs-plan-detail/ci-integration.md:36) The chunk's runs must leave zero `event:"panic"` lines in the home-level role files G2 counts, in the `test` and `perf` jobs.
  > **G2, zero panics.** It runs as `scripts/g2-zero-panics.sh` (fail-closed; `tool-missing: jq` without jq), `--probe` first, in the `test` and `perf` jobs; the snippet below is its logic.

### ci-integration#3.2 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:39
- (.andromeda/obs-plan-detail/ci-integration.md:39) No new panic exemption may be added for the chunk's tests: the only line G2 leaves out is the `fake-agent` seam's.
  > One exemption, ratified by the founder live on 2026-09-28 (security-plan Decisions Log): a line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` — the `fake-agent` seam's forced panic — is not counted;

### ci-integration#3.3 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:41
- (.andromeda/obs-plan-detail/ci-integration.md:41) Every process-log line the new verbs (`cli`) and the wrapper's `link` / `unlink` methods (`run`, `channel-request` / `channel-response`) write must conform to `schemas/diag-line.v1.json`, and any detail line to `schemas/diag-detail.v1.json`, or G4 fails.
  > Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).

### ci-integration#3.3 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:42
- (.andromeda/obs-plan-detail/ci-integration.md:42) If the new lines need a schema change (a `method` value, an `event` value or a field), the schema moves in the chunk, owned by obs, with the validator body tests-owned.
  > The check body and the JSON Schema validator crate are tests-owned; obs owns both schemas.

### ci-integration#4.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:59
- (.andromeda/obs-plan-detail/ci-integration.md:59) The chunk's test homes must survive until G2, G4, the secret scan and the uploads have read them: a test home's drop must honour `AGENT_RUN_KEEP_HOMES=1`.
  > **Homes are kept until the gate steps have run** (overseer fix pass 2, B1): `ci.yml` sets `AGENT_RUN_KEEP_HOMES=1`, so neither harness `cleanup` nor a test home's drop deletes a home before G2, G4, the secret scan and the scan-gated uploads have read it (test-plan §3 `cleanup` step 6).

### pii-scrubbing-compliance#1.10 · listed (lost at classify) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) The snapshot `pid` / `started_at` the verbs' liveness pre-check and server check read may be logged only as typed fields, never inside a `detail` code.
  > | `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |

### pii-scrubbing-compliance#1.12 · listed (lost at classify) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21) The two methods' refusals (`-32602` for a `driven` that is not the serving wrapper's name or a `driver` equal to `driven`) and the verbs' exit refusals must be logged and returned as closed kebab-case codes only.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |

## Held

- error-capture-reporting#2.1
- error-capture-reporting#3.1
- log-coverage#1.3
- log-coverage#1.4
- log-coverage#2.1
- log-coverage#2.2
- log-coverage#2.7
- metric-coverage#2.2
- metric-coverage#2.9
- obs-anti-patterns#1.1
- obs-scope-summary#14.3
- obs-scope-summary#15.3
- obs-scope-summary#18.3
- obs-scope-summary#18.4
- obs-scope-summary#18.5
- obs-scope-summary#21.1
- obs-scope-summary#21.5
- obs-scope-summary#25.0
- obs-scope-summary#33.1
- obs-scope-summary#38.1
- obs-scope-summary#44.1
- obs-scope-summary#48.1
- obs-scope-summary#49.1
- obs-scope-summary#6.1
- pii-scrubbing-compliance#1.8
- pii-scrubbing-compliance#1.9
- pii-scrubbing-compliance#2.1
- slo-invariants-telemetry-budgets#1.1
- span-trace-coverage#1.1
- span-trace-coverage#1.2
- span-trace-coverage#11.1
- span-trace-coverage#12.1
- span-trace-coverage#2.1
- span-trace-coverage#2.4
- span-trace-coverage#3.1
- span-trace-coverage#3.2
- span-trace-coverage#5.1
- span-trace-coverage#5.4
- span-trace-coverage#6.3
- telemetry-strategy#1.1
- telemetry-strategy#1.2
- telemetry-strategy#1.3
- telemetry-strategy#1.5
- telemetry-strategy#2.1
- telemetry-strategy#3.1
- telemetry-strategy#3.16
- telemetry-strategy#3.6
- telemetry-strategy#3.9

## Cascade only

- log-coverage#1.11
- log-coverage#1.2
- log-coverage#2.5
- obs-scope-summary#14.2
- obs-scope-summary#15.2
- obs-scope-summary#18.1
- obs-scope-summary#19.1
- obs-scope-summary#19.3
- obs-scope-summary#19.4
- obs-scope-summary#21.3
- obs-scope-summary#21.4
- obs-scope-summary#23.1
- obs-scope-summary#26.1
- obs-scope-summary#3.2
- obs-scope-summary#3.3
- obs-scope-summary#37.1
- obs-scope-summary#39.1
- obs-scope-summary#4.2
- obs-scope-summary#4.3
- obs-scope-summary#41.1
- span-trace-coverage#2.2
- span-trace-coverage#5.3
- telemetry-strategy#1.4
- telemetry-strategy#3.11
- telemetry-strategy#3.14
- telemetry-strategy#3.5
