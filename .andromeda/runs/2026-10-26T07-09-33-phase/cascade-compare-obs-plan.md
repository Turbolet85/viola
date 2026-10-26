# Cascade compare — obs-plan · phase · 2026-10-26-server-verification-before-any-frame
E items 177 · held 59 · lost at classify 6 · lost at strong 2 · beside a placed fact - · cascade only 57 · listed 10 · body 88 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#2.2 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:23
- (.andromeda/obs-plan-detail/ci-integration.md:23) A new log line or `detail` code the verification emits (a client's refusal or fail-open on a mismatch) must be covered by the in-memory writer tests that parse emitted lines against the line schema.
  > In-memory `Vec<u8>` writer tests parse emitted lines with serde_json against `schemas/diag-line.v1.json` (tests-owned bodies)

### ci-integration#3.2 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:38
- (.andromeda/obs-plan-detail/ci-integration.md:38) G2 must read zero panics over the home-level role files of every new test home; the only exemption is the seam's forced panic, so no verification path may panic, the hook's included.
  > Scope is the home-level role files only. `detail-*.ndjson` repeats the panic line and is excluded (§10).

### ci-integration#3.2 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:39
- (.andromeda/obs-plan-detail/ci-integration.md:39) The chunk must add no G2 exemption: the seam line is the one exemption, compared as a whole string.
  > One exemption, ratified by the founder live on 2026-09-28 (security-plan Decisions Log): a line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` — the `fake-agent` seam's forced panic — is not counted

### ci-integration#2.5 · lost at classify · .andromeda/obs-plan-detail/ci-integration.md:26
- (.andromeda/obs-plan-detail/ci-integration.md:26) The `obs_event!` call sites the chunk adds are obs code, scored by cargo-mutants like product code at the next `run --mutants` (no push or PR job).
  > obs code (`obs_event!` call sites, `MillisUtc`, panic hook) under cargo-mutants like product code

### error-capture-reporting#2.1 · lost at strong · .andromeda/obs-plan-detail/error-capture-reporting.md:15
- (.andromeda/obs-plan-detail/error-capture-reporting.md:15) The verification code added to every role's client path runs inside the one main-thread catch site, so no client ends through an uncaught unwind.
  > **Main-thread catch site (every role):** everything after `set_hook` (clap parse, init and role dispatch) runs inside one `std::panic::catch_unwind` in the root bin's `main`. An `Err` maps to the role's exit below, so no role ends through an uncaught unwind (Rust's default exit 101).

### error-capture-reporting#3.1 · lost at classify · .andromeda/obs-plan-detail/error-capture-reporting.md:23
- (.andromeda/obs-plan-detail/error-capture-reporting.md:23) A verification or strict-modes check placed before any role file opens (the way `hook statusline`'s instance check is) falls in the no-file window, where a panic writes nothing. Any such new pre-log check must be added to this window's list by amendment.
  > - Before step 4 of init, no file exists, so the panic hook writes nothing. This window contains clap parsing and home resolution, and the instance check that `viola revive`'s start arm and `viola hook statusline` run before any role file is opened (chunk 2026-10-10-budget-governor), and `viola mcp`'s check of its home and `diagnostics/`, also run before any role file is opened (chunk 2026-10-17-mcp-server-for-drivers).

### metric-coverage#2.5 · lost at strong · .andromeda/obs-plan-detail/metric-coverage.md:24
- (.andromeda/obs-plan-detail/metric-coverage.md:24) A `wait` refused on a failed server verification is a `wait` outcome, and this counter reads it only from `channel-response{method:"wait"}.outcome`, so the new outcome must land in that field.
  > | `wait` outcome (design CLI `viola wait`) | `viola.wait.outcome` ← `channel-response{method:"wait"}.outcome` | counter | 1 | n/a |

### metric-coverage#2.11 · lost at classify · .andromeda/obs-plan-detail/metric-coverage.md:30
- (.andromeda/obs-plan-detail/metric-coverage.md:30) The new client paths, the hook's fail-open on mismatch included, must leave the panic count at zero.
  > | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |

### obs-scope-summary#45.1 · lost at classify · .andromeda/obs-plan-detail/obs-scope-summary.md:462
- (.andromeda/obs-plan-detail/obs-scope-summary.md:462) A panic in the new verification code must yield exactly one single-line panic record, and the hook must still exit 0.
  > - **Required telemetry signal:** A zero-unlogged-panics invariant for all four process roles. Each panic produces exactly one single-line `event:"panic"` record and no multi-line backtrace, and `hook` still exits 0. No formal latency or availability SLO beyond the tests perf gates.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#2.3 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:24
- (.andromeda/obs-plan-detail/ci-integration.md:24) Every integration test this chunk adds (a mismatched or unverifiable server for each client, the hook's fail-open on a mismatch) must keep its viola home under `target/e2e-home/`, with no new carve-out beside the one chaos home, so a swallowed `hook` panic on the fail-open path stays visible to G2, G4, the scan and the upload.
  > `diagnostics/*.ndjson` from fake-agent runs. Their viola homes must live under `target/e2e-home/` (the directory layout is tests-owned), so G2, G4, the secret scan and the `diag-<os>` upload cover them. A `hook` panic exits 0, so a home outside that root would hide it. The one carve-out: `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the secret scan and the upload, whose deliberate panic line and `parse-rejected` line the test asserts present itself (the founder's ruling, live, 2026-10-04, relayed by the overseer).

### ci-integration#3.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:34
- (.andromeda/obs-plan-detail/ci-integration.md:34) Any span the chunk adds around a client's verification step must be declared with `skip_all` as its first argument, or G1 fails the commit.
  > `-U` lets `\s*` span lines, so an `#[instrument(` with `skip_all` on the next line passes. `skip_all` must be the first argument (the §4 form).

### ci-integration#3.3 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:41
- (.andromeda/obs-plan-detail/ci-integration.md:41) Every line the new tests' processes write must conform to its schema under G4, so a new event value, field or `detail` code that the verification writes must be in the obs-owned schemas first.
  > Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).

### ci-integration#3.3 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:42
- (.andromeda/obs-plan-detail/ci-integration.md:42) The schemas G4 checks are obs-owned, so the chunk's schema additions are obs's, while the check body stays tests-owned.
  > It exits non-zero on any failing line and prints `file`, line number and the failing schema keyword, never the line's content (detail lines carry the canary). The check body and the JSON Schema validator crate are tests-owned; obs owns both schemas.

### ci-integration#4.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:59
- (.andromeda/obs-plan-detail/ci-integration.md:59) A test home the chunk adds must be kept until the gate steps have read it: no test home's drop and no harness `cleanup` may delete it under `AGENT_RUN_KEEP_HOMES=1`.
  > **Homes are kept until the gate steps have run** (overseer fix pass 2, B1): `ci.yml` sets `AGENT_RUN_KEEP_HOMES=1`, so neither harness `cleanup` nor a test home's drop deletes a home before G2, G4, the secret scan and the scan-gated uploads have read it (test-plan §3 `cleanup` step 6).

### pii-scrubbing-compliance#1.8 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) The `snapshot.json` path the clients read before the check stays out of every home-level line; any path goes to a detail file only.
  > | Absolute paths (OS username = only PII in scope), internal type names | Medium | scrub-required in external output; permitted in `diagnostics/` but obs keeps home-level lines path-free | `file` fields are basenames; `panic_location` workspace-relative; `with_file(false)`; paths only in detail files |

### pii-scrubbing-compliance#1.12 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21) A verification failure's refusal or detail word (CLI exit 21, MCP `instance-unreachable`, the hook's fail-open) must be a closed kebab-case enum value.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |

### pii-scrubbing-compliance#1.4 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13) The `send` client (CLI and MCP) gains a pre-frame step; its payload must stay under `skip_all` and `#[redact]`, with only `text_bytes` logged.
  > | Prompt text, assistant output (`prompt-submitted.data.text`, `last_assistant_message`) | High | never in any home-level process-log line; payloads stay in contract payloads and reach an instance `detail-<process>.ndjson` only inside a chain or drift report | `#[instrument(skip_all)]`; veil 0.3.0 `#[derive(Redact)]` with `#[redact]` on `SendParams.text`, `LastResult.last_assistant_message`, `HookEventParams.data`; only `text_bytes` logged |

### pii-scrubbing-compliance#1.5 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:14
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:14) The `hook.dialog` and `answer` clients gain the same step; their dialog payload and answer response must stay redacted, with only codes-only dialog events.
  > | Tool `input`, plan text, dialog questions/answers | High | never in any home-level process-log line; instance `detail-<process>.ndjson` only inside a chain or drift report | `#[redact]` on `HookDialogParams.data`, `AnswerParams.response`; codes-only dialog events (`dialog_kind`) |

### pii-scrubbing-compliance#2.1 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28) The chunk's new mismatch and fail-open tests write `diagnostics/` files that must pass the CI secret scan, and the content canary must never reach a home-level file.
  > 5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
  >    - **High classes** (prompt and send text, `last_assistant_message`, tool `input`, dialog answers, drift-report echoes):
  >      - Every E2E and integration input is synthetic and embeds a fixed canary string.
  >      - The scan also asserts that the canary never appears in a home-level `diagnostics/*.ndjson`. It may appear only in `detail-*.ndjson`.

### telemetry-strategy#2.1 · listed (lost at classify) · .andromeda/obs-plan-detail/telemetry-strategy.md:20
- (.andromeda/obs-plan-detail/telemetry-strategy.md:20) A verification step placed before or after init step 4 decides whether a panic inside it is logged or falls in a bounded exemption; the new code must keep every post-init panic to exactly one line and add no exemption outside Section 10's list.
  > Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance. The bounded exemptions (the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`, a failed step-4 role-file open,

## Held

- ci-integration#2.1
- log-coverage#1.10
- log-coverage#1.2
- log-coverage#1.7
- log-coverage#2.2
- log-coverage#2.4
- log-coverage#2.7
- metric-coverage#1.1
- metric-coverage#2.1
- metric-coverage#2.4
- metric-coverage#2.6
- obs-anti-patterns#1.1
- obs-scope-summary#14.3
- obs-scope-summary#15.3
- obs-scope-summary#18.3
- obs-scope-summary#18.5
- obs-scope-summary#20.3
- obs-scope-summary#20.5
- obs-scope-summary#22.5
- obs-scope-summary#29.3
- obs-scope-summary#35.1
- obs-scope-summary#36.1
- obs-scope-summary#38.1
- obs-scope-summary#43.1
- obs-scope-summary#44.1
- obs-scope-summary#48.1
- obs-scope-summary#6.3
- obs-scope-summary#7.3
- pii-scrubbing-compliance#1.10
- pii-scrubbing-compliance#1.9
- slo-invariants-telemetry-budgets#1.1
- slo-invariants-telemetry-budgets#2.1
- slo-invariants-telemetry-budgets#2.2
- slo-invariants-telemetry-budgets#2.3
- slo-invariants-telemetry-budgets#2.4
- span-trace-coverage#1.1
- span-trace-coverage#11.1
- span-trace-coverage#2.1
- span-trace-coverage#2.3
- span-trace-coverage#2.4
- span-trace-coverage#2.5
- span-trace-coverage#4.1
- span-trace-coverage#4.2
- span-trace-coverage#4.4
- span-trace-coverage#5.3
- span-trace-coverage#6.1
- span-trace-coverage#6.4
- span-trace-coverage#7.1
- telemetry-strategy#1.1
- telemetry-strategy#1.2
- telemetry-strategy#1.3
- telemetry-strategy#1.4
- telemetry-strategy#3.1
- telemetry-strategy#3.10
- telemetry-strategy#3.12
- telemetry-strategy#3.16
- telemetry-strategy#3.3
- telemetry-strategy#3.6
- telemetry-strategy#3.9

## Cascade only

- ci-integration#1.4
- log-coverage#1.3
- log-coverage#1.4
- log-coverage#1.5
- log-coverage#2.1
- metric-coverage#1.2
- metric-coverage#3.0
- obs-scope-summary#11.2
- obs-scope-summary#11.3
- obs-scope-summary#14.2
- obs-scope-summary#15.2
- obs-scope-summary#2.2
- obs-scope-summary#2.4
- obs-scope-summary#20.1
- obs-scope-summary#21.1
- obs-scope-summary#21.3
- obs-scope-summary#21.5
- obs-scope-summary#22.1
- obs-scope-summary#23.1
- obs-scope-summary#25.0
- obs-scope-summary#27.3
- obs-scope-summary#28.1
- obs-scope-summary#28.2
- obs-scope-summary#28.3
- obs-scope-summary#29.1
- obs-scope-summary#29.2
- obs-scope-summary#3.2
- obs-scope-summary#30.1
- obs-scope-summary#30.2
- obs-scope-summary#30.3
- obs-scope-summary#31.1
- obs-scope-summary#31.2
- obs-scope-summary#33.1
- obs-scope-summary#34.1
- obs-scope-summary#40.1
- obs-scope-summary#41.1
- obs-scope-summary#42.1
- obs-scope-summary#47.1
- obs-scope-summary#6.2
- obs-scope-summary#7.2
- obs-scope-summary#9.2
- span-trace-coverage#1.2
- span-trace-coverage#1.3
- span-trace-coverage#3.2
- span-trace-coverage#3.3
- span-trace-coverage#4.3
- span-trace-coverage#5.1
- span-trace-coverage#5.4
- span-trace-coverage#6.2
- span-trace-coverage#6.3
- span-trace-coverage#7.3
- telemetry-strategy#1.5
- telemetry-strategy#3.11
- telemetry-strategy#3.15
- telemetry-strategy#3.2
- telemetry-strategy#3.4
- telemetry-strategy#3.8
