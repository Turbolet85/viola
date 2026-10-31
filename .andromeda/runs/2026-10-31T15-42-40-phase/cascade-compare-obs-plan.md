# Cascade compare — obs-plan · phase · 2026-10-31-exit-cause-code-catalogue
E items 239 · held 71 · lost at classify 0 · lost at strong 1 · beside a placed fact - · cascade only 57 · listed 4 · body 121 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#2.5 · lost at strong · .andromeda/obs-plan-detail/ci-integration.md:26
- (.andromeda/obs-plan-detail/ci-integration.md:26) New `obs_event!` call sites count as obs code under mutation at the boundary audit, so each new log form needs a test that catches its removal or change; this chunk is Epoch 6's last entry.
  > obs code (`obs_event!` call sites, `MillisUtc`, panic hook) under cargo-mutants like product code | no push or pull-request job since 2026-09-28: `agent-run run --mutants` at the epoch boundary through `/andromeda-code-audit`, and on demand.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#2.3 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:24
- (.andromeda/obs-plan-detail/ci-integration.md:24) Every home `tests/cross_exit_causes.rs` opens must live under `target/e2e-home/`, and the table takes no second carve-out, so its logged causes reach G2, G4, the secret scan and the upload.
  > Their viola homes must live under `target/e2e-home/` (the directory layout is tests-owned), so G2, G4, the secret scan and the `diag-<os>` upload cover them. A `hook` panic exits 0, so a home outside that root would hide it. The one carve-out: `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the secret scan and the upload, whose deliberate panic line and `parse-rejected` line the test asserts present itself (the founder's ruling, live, 2026-10-04, relayed by the overseer).

### ci-integration#4.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:59
- (.andromeda/obs-plan-detail/ci-integration.md:59) The table's homes must stay on disk until the gate steps have read them, so no case deletes its home outside the keep rule.
  > **Homes are kept until the gate steps have run** (overseer fix pass 2, B1): `ci.yml` sets `AGENT_RUN_KEEP_HOMES=1`, so neither harness `cleanup` nor a test home's drop deletes a home before G2, G4, the secret scan and the scan-gated uploads have read it (test-plan §3 `cleanup` step 6).

### ci-integration#3.2 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:39
- (.andromeda/obs-plan-detail/ci-integration.md:39) The table's internal-error case must not leave a counted panic line in a scanned home: the only exempt panic location is the `fake-agent` seam's, so a case proving that cause through a forced panic elsewhere reads red at G2.
  > One exemption, ratified by the founder live on 2026-09-28 (security-plan Decisions Log): a line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` — the `fake-agent` seam's forced panic — is not counted; the file is compared as a whole string, never a prefix, suffix, regex or home path.

### ci-integration#2.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:22
- (.andromeda/obs-plan-detail/ci-integration.md:22) New log forms must go through `obs_event!` and new hint or document output through the owning output modules, with any new span carrying `skip_all` first, or the lint stage fails the job.
  > clippy `-D warnings` incl. `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}!` outside `viola_core::obs`) and the `print_stdout` / `print_stderr` / `dbg_macro` bans; the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh`, Linux leg (mechanism in §3 logger-stack-install); bare-`#[instrument]` gate G1 (below)

## Held

- ci-integration#2.2
- ci-integration#3.3
- error-capture-reporting#2.1
- error-capture-reporting#3.1
- error-capture-reporting#5.1
- error-capture-reporting#5.2
- error-capture-reporting#5.3
- error-capture-reporting#6.1
- error-capture-reporting#6.2
- error-capture-reporting#6.3
- log-coverage#1.2
- log-coverage#2.1
- log-coverage#2.2
- log-coverage#2.4
- log-coverage#2.8
- log-coverage#3.1
- metric-coverage#1.1
- metric-coverage#3.1
- metric-coverage#3.2
- obs-anti-patterns#1.1
- obs-scope-summary#14.3
- obs-scope-summary#15.3
- obs-scope-summary#18.3
- obs-scope-summary#18.5
- obs-scope-summary#22.3
- obs-scope-summary#22.5
- obs-scope-summary#23.1
- obs-scope-summary#25.0
- obs-scope-summary#27.3
- obs-scope-summary#29.3
- obs-scope-summary#30.3
- obs-scope-summary#35.1
- obs-scope-summary#36.1
- obs-scope-summary#38.1
- obs-scope-summary#43.1
- obs-scope-summary#44.1
- obs-scope-summary#48.1
- pii-scrubbing-compliance#1.10
- pii-scrubbing-compliance#1.11
- pii-scrubbing-compliance#1.12
- pii-scrubbing-compliance#1.13
- pii-scrubbing-compliance#1.8
- pii-scrubbing-compliance#1.9
- pii-scrubbing-compliance#2.1
- slo-invariants-telemetry-budgets#1.1
- slo-invariants-telemetry-budgets#2.2
- slo-invariants-telemetry-budgets#2.3
- span-trace-coverage#14.1
- span-trace-coverage#17.1
- span-trace-coverage#2.1
- span-trace-coverage#3.1
- span-trace-coverage#3.2
- span-trace-coverage#3.3
- span-trace-coverage#3.4
- span-trace-coverage#3.6
- span-trace-coverage#3.7
- span-trace-coverage#4.2
- span-trace-coverage#4.3
- span-trace-coverage#5.1
- span-trace-coverage#6.3
- span-trace-coverage#6.4
- span-trace-coverage#7.3
- span-trace-coverage#7.4
- span-trace-coverage#8.3
- span-trace-coverage#9.3
- telemetry-strategy#1.2
- telemetry-strategy#1.3
- telemetry-strategy#2.1
- telemetry-strategy#3.16
- telemetry-strategy#3.3
- telemetry-strategy#3.6

## Cascade only

- ci-integration#1.3
- error-capture-reporting#6.4
- log-coverage#1.1
- log-coverage#1.11
- log-coverage#1.4
- log-coverage#1.7
- log-coverage#2.5
- obs-scope-summary#11.3
- obs-scope-summary#19.1
- obs-scope-summary#19.3
- obs-scope-summary#2.2
- obs-scope-summary#20.1
- obs-scope-summary#20.3
- obs-scope-summary#20.4
- obs-scope-summary#20.5
- obs-scope-summary#21.3
- obs-scope-summary#21.5
- obs-scope-summary#26.1
- obs-scope-summary#27.2
- obs-scope-summary#27.4
- obs-scope-summary#28.2
- obs-scope-summary#28.3
- obs-scope-summary#29.2
- obs-scope-summary#3.2
- obs-scope-summary#3.3
- obs-scope-summary#30.2
- obs-scope-summary#31.2
- obs-scope-summary#33.1
- obs-scope-summary#34.1
- obs-scope-summary#37.1
- obs-scope-summary#4.2
- obs-scope-summary#4.3
- obs-scope-summary#42.1
- obs-scope-summary#49.1
- obs-scope-summary#6.2
- obs-scope-summary#6.3
- obs-scope-summary#9.3
- pii-scrubbing-compliance#1.4
- pii-scrubbing-compliance#1.5
- pii-scrubbing-compliance#1.6
- span-trace-coverage#1.1
- span-trace-coverage#1.2
- span-trace-coverage#16.1
- span-trace-coverage#2.2
- span-trace-coverage#2.3
- span-trace-coverage#2.4
- span-trace-coverage#2.5
- span-trace-coverage#4.1
- span-trace-coverage#6.1
- span-trace-coverage#7.1
- span-trace-coverage#8.1
- span-trace-coverage#9.1
- telemetry-strategy#3.1
- telemetry-strategy#3.11
- telemetry-strategy#3.15
- telemetry-strategy#3.4
- telemetry-strategy#3.5
