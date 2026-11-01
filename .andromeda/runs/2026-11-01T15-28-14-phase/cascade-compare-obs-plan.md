# Cascade compare — obs-plan · phase · 2026-11-01-unix-endpoint-and-home-hardening
E items 212 · held 66 · lost at classify 0 · lost at strong 5 · beside a placed fact - · cascade only 39 · listed 8 · body 99 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.4 · lost at strong · .andromeda/obs-plan-detail/ci-integration.md:13
- (.andromeda/obs-plan-detail/ci-integration.md:13) The hook is a Unix client, so the verification it gains before its first frame must stay inside the gated hyperfine rows of the per-OS `perf` job.
  > hyperfine `--export-json target/agent-run/artifacts/perf-<hook>.json`, one per row (`session-start`, `user-prompt-submit`, `stop`, `session-end`, `pre-tool-use`), from `run --perf` in the per-OS `perf` job

### ci-integration#2.5 · lost at strong · .andromeda/obs-plan-detail/ci-integration.md:26
- (.andromeda/obs-plan-detail/ci-integration.md:26) New `obs_event!` call sites for these refusals fall under the mutation gate, and a `#[cfg(unix)]` check or a negative compiled out on macOS must not leave a surviving mutant other than one the harness excludes as host-excluded.
  > A timed-out obs-code mutant, a missed one the harness does not leave out as host-excluded (test-plan §3 `run` step 4), or unviable outnumbering caught, is red at that run (test-plan §10 Mutation gate)

### error-capture-reporting#1.2 · lost at strong · .andromeda/obs-plan-detail/error-capture-reporting.md:9
- (.andromeda/obs-plan-detail/error-capture-reporting.md:9) Unix strict modes on the paths this entry reaches must leave the `diagnostics/` reading as written, refused before any narrowing, and a refusal exactly a failed open.
  > A role file or a detail file is opened only past `check_diagnostics` (`src/obs.rs`, in `open_role_file` and `write_detail`), which runs `strict::check_diagnostics` on the existing `diagnostics/` folder and on the file before `ensure_private_dir` narrows anything: a symlink is refused, a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows, and an absent path passes. A refusal is exactly a failed open: the role writer ends at `std::io::sink`, and the detail line is dropped.

### metric-coverage#2.1 · lost at strong · .andromeda/obs-plan-detail/metric-coverage.md:20
- (.andromeda/obs-plan-detail/metric-coverage.md:20) The directory check and the peer euid check on the hook's connect path, every Unix client's, must stay inside this metric's gate: hyperfine `max` is the number the added checks answer to.
  > | Hook latency per `hook_event` (Scenario 4/6; perf trigger) | `viola.hook.duration_ms` ← `hook-decision.duration_ms` | histogram | ms | raw samples; gate = hyperfine `max` |

### obs-scope-summary#26.1 · lost at strong · .andromeda/obs-plan-detail/obs-scope-summary.md:295
- (.andromeda/obs-plan-detail/obs-scope-summary.md:295) The Unix strict modes this chunk touches must not shift the `diagnostics/` rule unstated: a symlink refused, the folder and file held to the read rule on Unix, a refusal a failed open.
  > a symlink refused, a folder held to the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows — and a refusal is a failed open (no role file opened; a detail line dropped).

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#2.3 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:24
- (.andromeda/obs-plan-detail/ci-integration.md:24) Every viola home a socket-directory, peer-euid or strict-modes test uses must live under `target/e2e-home/`, even where the socket directory itself stands outside it, and this chunk gains no second carve-out beside the one named.
  > Their viola homes must live under `target/e2e-home/` (the directory layout is tests-owned), so G2, G4, the secret scan and the `diag-<os>` upload cover them. A `hook` panic exits 0, so a home outside that root would hide it. The one carve-out: `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the secret scan and the upload, whose deliberate panic line and `parse-rejected` line the test asserts present itself (the founder's ruling, live, 2026-10-04, relayed by the overseer).

### ci-integration#3.3 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:48
- (.andromeda/obs-plan-detail/ci-integration.md:48) The chunk's not-a-symlink rule and its tests must read the same on the Linux dev host, where the scope root is a link, as on a real directory, and must leave G2's descent of that link intact.
  > the start point ends in a slash, so a scope that is a link to a directory is descended (the Linux dev host's `target/e2e-home`; on a real directory the walk is the same)

### ci-integration#3.2 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:36
- (.andromeda/obs-plan-detail/ci-integration.md:36) A refused socket directory or a peer euid mismatch must end as its typed refusal and never as a panic line, because G2 reads the home-level role files of both the `test` and `perf` jobs.
  > **G2, zero panics.** It runs as `scripts/g2-zero-panics.sh` (fail-closed; `tool-missing: jq` without jq), `--probe` first, in the `test` and `perf` jobs; the snippet below is its logic.

### ci-integration#3.3 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:41
- (.andromeda/obs-plan-detail/ci-integration.md:41) Each new refusal's real process output, whichever closed `detail` code it logs, must validate against the schemas, since G4 checks every such line.
  > Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).

### ci-integration#3.3 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:42
- (.andromeda/obs-plan-detail/ci-integration.md:42) A closed code this chunk logs must land in the obs-owned schema in the same change, or G4 reds on the first line carrying it.
  > It exits non-zero on any failing line and prints `file`, line number and the failing schema keyword, never the line's content (detail lines carry the canary). The check body and the JSON Schema validator crate are tests-owned; obs owns both schemas.

### ci-integration#3.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:34
- (.andromeda/obs-plan-detail/ci-integration.md:34) Any span this chunk adds around the directory or peer euid checks must take `skip_all` as its first argument to pass G1.
  > `skip_all` must be the first argument (the §4 form).

### ci-integration#2.1 · listed (lost at strong) · .andromeda/obs-plan-detail/ci-integration.md:22
- (.andromeda/obs-plan-detail/ci-integration.md:22) The directory and peer euid checks must pass the lint stage: refusal text through the owned writers, no raw tracing macros, the sync crates still free of Tokio, and any crate added for peer credentials clear of `cargo deny`.
  > clippy `-D warnings` incl. `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}!` outside `viola_core::obs`) and the `print_stdout` / `print_stderr` / `dbg_macro` bans; the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh`, Linux leg (mechanism in §3 logger-stack-install); bare-`#[instrument]` gate G1 (below); `cargo check` of sync crates without Tokio; `cargo deny check` (ubuntu) incl. veil `toggle` ban and bans on `opentelemetry-otlp` / `opentelemetry-stdout` / `sentry` / `tracing-appender` / tracing-subscriber `env-filter` feature

### pii-scrubbing-compliance#1.10 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) A pid in a line of this chunk's checks must be a typed field and never part of a `detail` code; the same rule holds a euid out of the new codes.
  > Logged as typed fields; never inside `detail` codes (Founder Direction 4)

### pii-scrubbing-compliance#1.13 · listed (lost at strong) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) The new refusal codes for the socket directory and the peer euid must be closed kebab-case enum members.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums;

### telemetry-strategy#1.1 · listed (lost at strong) · .andromeda/obs-plan-detail/telemetry-strategy.md:10
- (.andromeda/obs-plan-detail/telemetry-strategy.md:10) Any span the directory and peer euid checks add at the `viola-channel` client or server seam must be declared in this form, unserialized, with no field beyond its allow-list.
  > Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`. They are **not serialized** in v1

## Held

- ci-integration#4.1
- error-capture-reporting#2.1
- error-capture-reporting#3.1
- error-capture-reporting#5.1
- error-capture-reporting#5.2
- error-capture-reporting#6.3
- error-capture-reporting#6.5
- log-coverage#1.2
- log-coverage#1.4
- log-coverage#2.1
- log-coverage#2.2
- log-coverage#2.4
- log-coverage#2.8
- metric-coverage#1.1
- metric-coverage#3.1
- metric-coverage#3.2
- obs-anti-patterns#1.1
- obs-scope-summary#14.3
- obs-scope-summary#18.3
- obs-scope-summary#18.5
- obs-scope-summary#20.4
- obs-scope-summary#20.5
- obs-scope-summary#22.5
- obs-scope-summary#23.1
- obs-scope-summary#27.2
- obs-scope-summary#27.3
- obs-scope-summary#30.3
- obs-scope-summary#35.1
- obs-scope-summary#36.1
- obs-scope-summary#38.1
- obs-scope-summary#4.3
- obs-scope-summary#44.1
- obs-scope-summary#48.1
- pii-scrubbing-compliance#1.8
- pii-scrubbing-compliance#1.9
- pii-scrubbing-compliance#2.1
- slo-invariants-telemetry-budgets#1.1
- slo-invariants-telemetry-budgets#2.1
- slo-invariants-telemetry-budgets#2.2
- slo-invariants-telemetry-budgets#2.4
- span-trace-coverage#2.1
- span-trace-coverage#2.3
- span-trace-coverage#2.4
- span-trace-coverage#3.1
- span-trace-coverage#3.2
- span-trace-coverage#3.3
- span-trace-coverage#3.4
- span-trace-coverage#3.6
- span-trace-coverage#4.1
- span-trace-coverage#4.2
- span-trace-coverage#4.3
- span-trace-coverage#6.1
- span-trace-coverage#6.3
- span-trace-coverage#6.4
- span-trace-coverage#7.1
- span-trace-coverage#7.4
- span-trace-coverage#8.1
- span-trace-coverage#8.3
- span-trace-coverage#9.1
- span-trace-coverage#9.3
- telemetry-strategy#1.2
- telemetry-strategy#1.3
- telemetry-strategy#2.1
- telemetry-strategy#3.16
- telemetry-strategy#3.3
- telemetry-strategy#3.6

## Cascade only

- log-coverage#1.3
- log-coverage#1.7
- metric-coverage#2.5
- obs-scope-summary#11.3
- obs-scope-summary#15.3
- obs-scope-summary#19.1
- obs-scope-summary#19.3
- obs-scope-summary#20.1
- obs-scope-summary#20.3
- obs-scope-summary#21.1
- obs-scope-summary#21.3
- obs-scope-summary#21.4
- obs-scope-summary#21.5
- obs-scope-summary#22.1
- obs-scope-summary#25.0
- obs-scope-summary#28.2
- obs-scope-summary#28.3
- obs-scope-summary#29.2
- obs-scope-summary#29.3
- obs-scope-summary#3.3
- obs-scope-summary#30.2
- obs-scope-summary#31.2
- obs-scope-summary#33.1
- obs-scope-summary#41.1
- obs-scope-summary#42.1
- obs-scope-summary#6.3
- obs-scope-summary#7.3
- span-trace-coverage#1.1
- span-trace-coverage#1.2
- span-trace-coverage#2.2
- span-trace-coverage#2.5
- span-trace-coverage#7.3
- span-trace-coverage#8.4
- telemetry-strategy#3.1
- telemetry-strategy#3.11
- telemetry-strategy#3.12
- telemetry-strategy#3.13
- telemetry-strategy#3.4
- telemetry-strategy#3.9
