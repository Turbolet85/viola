# obs extract

## Relevance
Relevant. obs-plan owns several of the deny.toml entries this chunk writes: the telemetry-crate bans, the veil `toggle` feature ban, and the tracing-subscriber `env-filter` feature ban, which the scope leaves out. It also fixes the pass/fail semantics for the cargo-deny and Tokio-free gates.

## Constraints
- obs-plan §3 "Bootstrap phases → otel-sdk-install" requires cargo-deny bans on `opentelemetry-otlp`, `opentelemetry-stdout`, `sentry` and `tracing-appender` across the whole workspace, so none of them can arrive transitively without a Decisions Log entry. It also says no `opentelemetry*` crate is added in v1. The phase is otherwise a no-op that only records deferral D-12.
- obs-plan §9 "Pipeline integration" (Lint / typecheck row) lists the `cargo deny check` step on ubuntu. That step must cover the veil `toggle` ban, the four crate bans above, and a ban on the tracing-subscriber `env-filter` feature. The scope's `[[bans.features]]` list (veil, rmcp, axum) has no `env-filter` entry. §11 Logs and D-15 back this: levels come only from `config.json` through in-code `filter::Targets`, never from `EnvFilter` or `RUST_LOG`. Planning must add the entry or record why it is deferred.
- obs-plan §8 "PII Scrubbing" and D-19 require `[[bans.features]] crate = "veil" deny = ["toggle"]`. The `toggle` feature enables `VEIL_DISABLE_REDACTION` / `veil::disable()`, and no knob may disable redaction. Per §3 "Bootstrap phases → Ownership", obs owns this gate and security owns the NEVER-log floor it enforces.
- obs-plan §1 (telemetry trigger for security Vector 9) and §3 "Logging stack" require three things:
  - the logger dependency is tracing-subscriber `>=0.3.20` (RUSTSEC-2025-0055), with 0.3.23 pinned;
  - obs dependencies stay Tokio-free in the sync crates and add no C-building crates;
  - obs dependencies pass `cargo deny` on all three target triples.

  §3 "Bootstrap phases → logger-stack-install" names "`cargo check` of the sync crates without Tokio, plus `cargo deny check` on all three targets" as its verification. Whether tracing and tracing-subscriber are already in the graph is research's question.
- obs-plan §7 and §11 "Error Reporting" (also D-31) rule out sentry / sentry-tracing. Beyond the name ban, their default `reqwest` + `native-tls` + `tokio` transport is exactly what the C-crate (`openssl-sys`) and Tokio-in-sync-crates bans must catch. The C-crate class ban and the Tokio graph ban are therefore a second line of defence and need to be real, not just listed.
- obs-plan §3 "Log file location" rejects tracing-appender 0.2.5 for two reasons: its `non_blocking` writer drops lines, and it pulls in `time` in the RUSTSEC-2026-0009 range. The deny `reason` strings should cite a cause rather than be left bare.
- obs-plan §10 "Build / deploy failure conditions" makes any `cargo deny` failure (Tokio, C-crate, veil `toggle`, banned obs crates) a build failure. §9 "CI-specific resource attributes" bans injecting CI env metadata (`ci.run.id`, `git.commit.sha`, `deployment.environment`) into product log lines. The new workflow `env:` plumbing must not feed product configuration.

## Patterns to follow
- obs-plan §9 "Gate commands" sets the fail-closed semantics: each gate is its own `run:` step with `shell: bash`, and only the expected exit code passes. A missing tool (exit 127) or a tool error (exit 2) must fail, never pass. This applies to the cargo-deny, zizmor and Tokio-free `cargo check` steps, and to the local-absence premise.
- obs-plan §9 G1 and §3 "obs-ci-gate-wire" install a tool through a SHA-pinned step, then run a separate version or capability self-check (`rg --pcre2-version`) before the gate. The same shape fits installing cargo-deny 0.20.2 and zizmor 1.30.1.
- obs-plan §9 G2 checks first that the scope is non-empty (`test -n "$(find …)"`) so an empty scope cannot pass. The scope premise about the Tokio ban and sync-crate check needs the same guard: no vacuous pass over packages that don't exist yet.
- obs-plan §3 "logger-stack-install" requires a gate to be "proven both ways". Its example: a known-good case lints clean, and a known-bad case fails the same gate. This matches the scope's need for a negative probe per ban on crates absent from today's graph.

## Anti-patterns to avoid
- Per obs-plan §11 "Telemetry Strategy", "Error Reporting" and "Logs": never admit an OTel SDK, OTLP exporter, `opentelemetry-stdout`, sentry or any other reporter, tracing-appender (`non_blocking`), or the tracing-subscriber `env-filter` feature without a Decisions Log entry. A deny.toml skip or exception for any of these counts as admitting it.
- Per obs-plan §11 "PII Scrubbing": never let veil's `toggle` feature through. It must not be weakened by a `[[bans.skip]]` or `allow` entry, a wrapper, or a scoped config that leaves it out.
- Per obs-plan §11 "CI": never use unpinned Actions (zizmor checks this), and never inject CI env metadata into product log lines. The same section says every CI verdict must be an assertion that sets an exit code, never a human-reviewed log.

## Contract bindings
- **obs ↔ security:** the veil `toggle` ban and the tracing-subscriber `>=0.3.20` floor (Vector 9). Obs owns the mechanical gate; security owns the floor list (obs-plan §3 "Bootstrap phases → Ownership", §8, D-19).
- **obs ↔ arch:** the Tokio-free sync-crate graph and the C-crate class ban are arch build policy that every obs dependency must pass (obs-plan §1 harness spec "OTel SDK init", §3 "OTel SDK init" and "logger-stack-install").
- **obs ↔ the later Observability gates chunk:** obs-plan §3 "obs-ci-gate-wire" and §9 add SHA-pinned `actions/upload-artifact` v7.0.1 steps and a ripgrep install step to `ci.yml`. The least-privilege posture this chunk sets (`permissions: {}`, per-job `contents: read`, `persist-credentials: false`, zizmor) is what those steps will inherit and must pass. Whether an artifact-upload job needs more than `contents: read` is for that chunk, but the zizmor config set here must not block it.
- **obs ↔ tests:** obs-plan §9 "Platform" names a single workflow, `.github/workflows/ci.yml`. If the weekly advisory run goes into a separate `nightly.yml` (the scope's open premise), that departs from obs-plan's single-workflow statement and needs a note.

## Acceptance criteria contributions
- `deny.toml` `[bans]` denies `opentelemetry-otlp`, `opentelemetry-stdout`, `sentry` and `tracing-appender` workspace-wide. For each one, a negative probe that adds the crate makes `cargo deny check bans` exit non-zero (per obs-plan §3 Bootstrap phases → otel-sdk-install).
- `deny.toml` has `[[bans.features]] crate = "veil" deny = ["toggle"]`. A probe that enables `veil/toggle` makes `cargo deny check bans` fail (per obs-plan §8 PII Scrubbing / D-19).
- `deny.toml` bans the tracing-subscriber `env-filter` feature, or the chunk records an explicit deferral against obs-plan §9. Either way, a probe that enables `env-filter` must fail the gate once the ban exists (per obs-plan §9 Pipeline integration; §11 Logs).
- The `cargo deny check` and Tokio-free `cargo check` steps are fail-closed. A missing tool, config error or tool error gives a red step, never a vacuous pass. The deny check is evaluated for the Windows, macOS and Linux triples (per obs-plan §10 Build / deploy failure conditions; §3 logger-stack-install).

## Relevant amendment history
- `2026-09-24-three-os-ci-headless-harness-skeleton`, which amended §3 "obs-ci-gate-wire" and §11 "Logs". The fake agent (`viola-fake-agent`) is a `[[bin]]` of the root `viola` package, not its own package, and only `viola-e2e` carries its own lint table. This is the reason: lints are set per package, so the bin could not get a separate table. It matters here because the Tokio-graph exclusions key on packages. The fake agent falls under the excluded root `viola` package and `viola-e2e` is the separate test-only package, so neither needs its own entry. No amendment touches the cargo-deny, zizmor or permissions content itself.
