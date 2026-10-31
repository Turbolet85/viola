# Cascade compare — obs-plan · phase · 2026-10-31-sanitised-error-surfaces
E items 262 · held 89 · lost at classify 0 · lost at strong 1 · beside a placed fact - · cascade only 50 · listed 7 · body 125 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#2.5 · lost at strong · .andromeda/obs-plan-detail/ci-integration.md:26
- (.andromeda/obs-plan-detail/ci-integration.md:26) A new or changed `obs_event!` call site must be coverable under cargo-mutants, scored at a later `run --mutants` and not in this chunk's CI, since no boundary falls inside Epoch 6.
  > obs code (`obs_event!` call sites, `MillisUtc`, panic hook) under cargo-mutants like product code | no push or pull-request job since 2026-09-28: `agent-run run --mutants` at the epoch boundary through `/andromeda-code-audit`, and on demand.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#2.3 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:24
- (.andromeda/obs-plan-detail/ci-integration.md:24) Every home this chunk's tests open (the per-surface witnesses, the `ServeError::Initialize` case, the cap-driving `BOUNDARIES` cases) must live under `target/e2e-home/`, and the one chaos carve-out admits no second home.
  > Their viola homes must live under `target/e2e-home/` (the directory layout is tests-owned), so G2, G4, the secret scan and the `diag-<os>` upload cover them. A `hook` panic exits 0, so a home outside that root would hide it. The one carve-out: `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the secret scan and the upload, whose deliberate panic line and `parse-rejected` line the test asserts present itself (the founder's ruling, live, 2026-10-04, relayed by the overseer).

### ci-integration#3.2 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:39
- (.andromeda/obs-plan-detail/ci-integration.md:39) A witness that drives the `"internal error"` handler fault must leave no panic line G2 counts: only the `fake-agent` seam's location is exempt, so a forced panic at any other location is red.
  > One exemption, ratified by the founder live on 2026-09-28 (security-plan Decisions Log): a line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` — the `fake-agent` seam's forced panic — is not counted; the file is compared as a whole string, never a prefix, suffix, regex or home path.

### ci-integration#2.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:22
- (.andromeda/obs-plan-detail/ci-integration.md:22) Product code this chunk changes (error mapping, a folded `Refusal`, any new log site) must pass the lint stage's bans, with no raw tracing macro, raw `event!` or print macro and no bare `#[instrument]`.
  > clippy `-D warnings` incl. `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}!` outside `viola_core::obs`) and the `print_stdout` / `print_stderr` / `dbg_macro` bans; the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh`, Linux leg (mechanism in §3 logger-stack-install); bare-`#[instrument]` gate G1 (below)

### ci-integration#3.1 · listed (lost at classify) · .andromeda/obs-plan-detail/ci-integration.md:34
- (.andromeda/obs-plan-detail/ci-integration.md:34) Any `#[instrument(` this chunk adds or edits must hold `skip_all` as its first argument.
  > `skip_all` must be the first argument (the §4 form).

### metric-coverage#2.11 · listed (lost at classify) · .andromeda/obs-plan-detail/metric-coverage.md:30
- (.andromeda/obs-plan-detail/metric-coverage.md:30) A witness that drives a handler fault must leave this count at 0 in the gated scope: a fault proven through the `"internal error"` body must not add a panic line G2 counts.
  > | Panics (SLO) | `viola.panic` ← `event:"panic"` | counter | 1 | n/a (must be 0) |

### obs-anti-patterns#1.1 · listed (lost at strong) · .andromeda/obs-plan-detail/obs-anti-patterns.md:8
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) Every error writer the chunk adds or changes on the `hook`, `run` or `mcp` paths must use no print macro, human refusals must go through `src/human.rs`, and a chunk that adds or removes a `human::refuse(` call site must move this census with it.
  > NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates (`viola-harness` and the fake agent are exempt, §3 obs-ci-gate-wire). They bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing. Enforced by `[workspace.lints.clippy]` `print_stdout`, `print_stderr`, `dbg_macro` = `deny` under the §9 clippy step. Only the output modules that own `--json`, human CLI stderr and the hook decision body may carry a local `#[allow]`, and only where they use a print macro — `run`'s and `revive`'s pre-spawn refusals, `revive --list`'s, `verify`'s, `list`'s and `plugin install`'s refusals, and the instance-check refusal of `link` / `unlink` and of `send` / `wait` / `last` / `answer` / `pause` / `release`, write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 7 · `verify` 6 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1, the one `refuse_untrusted` site `link` / `unlink` and the six frame verbs share; the rule: the `human::refuse(` call sites under `src/`), which uses no print macro and carries no `#[allow]` — plus the fake agent's crate-level `#![allow]` in `src/bin/viola-fake-agent.rs` (a root-package bin, §3 obs-ci-gate-wire).

### telemetry-strategy#3.13 · listed (lost at strong) · .andromeda/obs-plan-detail/telemetry-strategy.md:40
- (.andromeda/obs-plan-detail/telemetry-strategy.md:40) A sanitised handler fault must keep the zero-unlogged-panics invariant whole; a `-32603` or `"internal error"` body must not stand in for its panic line.
  > | error-budget-SLO | §7, §10 zero-unlogged-panics |

## Held

- ci-integration#1.1
- ci-integration#2.2
- ci-integration#3.3
- ci-integration#4.1
- error-capture-reporting#1.1
- error-capture-reporting#1.2
- error-capture-reporting#2.1
- error-capture-reporting#3.1
- error-capture-reporting#5.1
- error-capture-reporting#5.2
- log-coverage#1.1
- log-coverage#1.11
- log-coverage#1.4
- log-coverage#1.8
- log-coverage#1.9
- log-coverage#2.1
- log-coverage#2.2
- log-coverage#2.3
- log-coverage#2.4
- log-coverage#2.5
- log-coverage#2.6
- log-coverage#2.7
- log-coverage#2.8
- log-coverage#3.1
- metric-coverage#1.1
- metric-coverage#2.4
- metric-coverage#2.9
- metric-coverage#3.1
- metric-coverage#3.2
- obs-scope-summary#11.3
- obs-scope-summary#12.3
- obs-scope-summary#13.3
- obs-scope-summary#15.3
- obs-scope-summary#18.3
- obs-scope-summary#18.5
- obs-scope-summary#21.5
- obs-scope-summary#22.3
- obs-scope-summary#22.5
- obs-scope-summary#25.0
- obs-scope-summary#26.1
- obs-scope-summary#3.3
- obs-scope-summary#33.1
- obs-scope-summary#34.1
- obs-scope-summary#36.1
- obs-scope-summary#37.1
- obs-scope-summary#48.1
- obs-scope-summary#49.1
- obs-scope-summary#8.3
- pii-scrubbing-compliance#1.10
- pii-scrubbing-compliance#1.11
- pii-scrubbing-compliance#1.13
- pii-scrubbing-compliance#1.2
- pii-scrubbing-compliance#1.4
- pii-scrubbing-compliance#1.5
- pii-scrubbing-compliance#1.6
- pii-scrubbing-compliance#1.7
- pii-scrubbing-compliance#1.8
- pii-scrubbing-compliance#1.9
- pii-scrubbing-compliance#2.1
- slo-invariants-telemetry-budgets#1.1
- span-trace-coverage#10.4
- span-trace-coverage#11.1
- span-trace-coverage#13.1
- span-trace-coverage#17.1
- span-trace-coverage#2.1
- span-trace-coverage#2.4
- span-trace-coverage#2.5
- span-trace-coverage#3.1
- span-trace-coverage#3.3
- span-trace-coverage#3.5
- span-trace-coverage#4.2
- span-trace-coverage#4.3
- span-trace-coverage#6.1
- span-trace-coverage#6.3
- span-trace-coverage#6.4
- span-trace-coverage#7.3
- span-trace-coverage#8.1
- span-trace-coverage#8.2
- span-trace-coverage#8.3
- span-trace-coverage#9.1
- span-trace-coverage#9.3
- telemetry-strategy#1.2
- telemetry-strategy#1.3
- telemetry-strategy#1.4
- telemetry-strategy#2.1
- telemetry-strategy#3.1
- telemetry-strategy#3.16
- telemetry-strategy#3.4
- telemetry-strategy#3.5

## Cascade only

- ci-integration#1.2
- ci-integration#1.3
- error-capture-reporting#4.1
- log-coverage#1.2
- log-coverage#1.3
- log-coverage#1.7
- metric-coverage#2.2
- metric-coverage#2.5
- metric-coverage#2.8
- obs-scope-summary#14.3
- obs-scope-summary#16.3
- obs-scope-summary#2.1
- obs-scope-summary#2.2
- obs-scope-summary#2.3
- obs-scope-summary#20.3
- obs-scope-summary#20.5
- obs-scope-summary#23.1
- obs-scope-summary#27.3
- obs-scope-summary#28.2
- obs-scope-summary#28.3
- obs-scope-summary#29.2
- obs-scope-summary#29.3
- obs-scope-summary#30.3
- obs-scope-summary#31.2
- obs-scope-summary#32.3
- obs-scope-summary#35.1
- obs-scope-summary#38.1
- obs-scope-summary#39.1
- obs-scope-summary#4.3
- obs-scope-summary#43.1
- obs-scope-summary#44.1
- obs-scope-summary#47.1
- obs-scope-summary#5.3
- obs-scope-summary#6.1
- obs-scope-summary#6.3
- pii-scrubbing-compliance#1.1
- pii-scrubbing-compliance#1.12
- span-trace-coverage#10.2
- span-trace-coverage#10.3
- span-trace-coverage#16.1
- span-trace-coverage#2.2
- span-trace-coverage#4.1
- telemetry-strategy#1.5
- telemetry-strategy#3.12
- telemetry-strategy#3.15
- telemetry-strategy#3.17
- telemetry-strategy#3.2
- telemetry-strategy#3.3
- telemetry-strategy#3.6
- telemetry-strategy#3.7
