### Bootstrap phases (derive for route / setup-project)

_[ALL tiers — explicit derivation hint for downstream consumers
per D26 chain. route reads this to plan bootstrap phase ordering;
setup-project reads this to materialize phase scaffolding.]_

- **otel-sdk-install:** **no-op in v1.** No `opentelemetry*` crate is added. The phase records only the deferral (D-12) and adds `cargo deny` `[[bans]]` entries for `opentelemetry-otlp`, `opentelemetry-stdout`, `sentry` and `tracing-appender` on the workspace, so they cannot arrive transitively without a Decisions Log entry.
- **logger-stack-install:**
  - Add `tracing = "0.1.44"` to the root bin, viola-pty, viola-channel, viola-state, viola-agent-claude, viola-mcp and viola-ui.
  - Add `tracing-subscriber 0.3.23` (`default-features = false`, `fmt,json,registry,std`; no `chrono` feature, D-26) to the root bin only.
  - Add tower-http 0.7.1 feature `trace` to viola-ui.
  - viola-core (`viola_core::obs`): `obs_event!` and the `ObsEvent` / `ObsProcess` enums. The macro expands to `::tracing::event!` at the caller, so viola-core itself gains no `tracing` dependency.
  - Root bin (`viola::obs`), the only crate that depends on tracing-subscriber: `MillisUtc`, `viola_obs_init` and `viola_panic_hook`.
    - Add a direct `chrono = { version = "0.4.45", default-features = false, features = ["clock", "std"] }` (already in the tree). `MillisUtc` calls `chrono::Utc::now()` itself, and tracing-subscriber's `chrono` feature does not re-export chrono.
  - Raw-tracing ban. The mechanism is split, because no allow inside `obs_event!` can exempt its inner `::tracing::event!`. As measured on clippy 1.98.1 at chunk 2026-09-24-observability-gates (`.andromeda/runs/2026-09-24T12-21-11-implement/clippy-disallowed-macros-measurement.md`), clippy reports a disallowed macro expanded inside an exported macro at the caller crate's level. Only a crate-level `#![allow]` in the caller silences it.
    - One workspace `clippy.toml` `disallowed-macros` bans the level macros `tracing::{info,warn,error,debug,trace}` by path. `tracing::event` is not listed there, because `obs_event!` expands to it.
    - Raw `event!` is caught by a fail-closed grep in `scripts/lint-probes.sh`: `\bevent!\s*[({[]` in any `*.rs` outside `crates/viola-core/src/obs.rs`. `\b` does not match inside `obs_event!`. The inner `#[allow(clippy::disallowed_macros)]` in `obs_event!` stays in place but has no effect.
    - The gate is proven both ways by `scripts/lint-probes.sh`, in throwaway crates carrying the repo's `clippy.toml` and lint table:
      - a raw `tracing::info!` fails and a real `obs_event!` call lints clean;
      - the grep finds a planted raw `tracing::event!`, passes `obs_event!`, and is clean over the tree.
  - Verify with `cargo check` of the sync crates without Tokio, plus `cargo deny check` on all three targets.
- **service-identity-wire:** `viola_core::{SERVICE_NAME, VERSION}` as the single source for `process-start.version`, `sender`, `writer` and `/health.version`. Workspace `version.workspace = true`.
- **log-format-schema-emit:** commit `schemas/diag-line.v1.json`, a hand-maintained JSON Schema of the Section 3 / Section 6 line. Its `event` enum must equal the `ObsEvent` variants, and a tests-owned check asserts that equality. The harness greps the same fields.
- **trace-context-propagate-wire:** `corr` population per the tests table, plus the additive `conn` in viola-channel client `params` and server logging (D-10), plus `Span` hand-off into std threads and Tokio tasks.
- **heartbeat-tick-wire:** the `liveness-changed` transition emitter in viola-ui's sessions reader, and the `sse-opened` / `sse-closed` guard in the `/api/events` stream (D-02, D-04).
- **pii-scrubbing-wire:** merge with security's `logging-redaction-wire` bootstrap phase:
  - `#[instrument(skip_all, fields(...))]` everywhere;
  - veil 0.3.0 `#[derive(Redact)]` on payload types, plus `cargo deny` `[[bans.features]] crate = "veil" deny = ["toggle"]`;
  - the tower-http custom `make_span_with` using `uri.path()` only;
  - fixed thiserror `Display`s;
  - the instance-scoped `detail-<process>.ndjson` routing.
- **obs-ci-gate-wire:** CI steps per Section 9:
  - `clippy.toml` `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}` outside `viola_core::obs`), plus the fail-closed raw-`event!` grep in `scripts/lint-probes.sh`, which runs in the `ci.yml` `lint` job on the Linux leg (source text is OS-independent). The split is explained under logger-stack-install;
  - `[workspace.lints.clippy]` `print_stdout` / `print_stderr` / `dbg_macro` = `deny` (§11 Logs), plus `[lints] workspace = true` in **every product** member's `Cargo.toml` (the root bin `viola` and the product `crates/viola-*` crates). A product member without it inherits no workspace lints, so the ban silently does not apply there. **Exempt (overseer fix pass 2, B5):** `viola-harness` (`crates/viola-e2e`) and the fake agent (`viola-fake-agent`) must print: the harness prints exactly one JSON document per command on stdout, and the fake agent emulates the claude CLI on stdout/stderr (tests §3, §4). Cargo cannot override single lints under `workspace = true`, so `viola-e2e` omits it and carries its own `[lints.clippy]` table without `print_stdout` / `print_stderr`. The fake agent is a `[[bin]]` of the root `viola` package (feature `fake-agent`), and lints are set per package, so it inherits the root's `[lints]` and is exempted by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`. A CI assertion lists members: every product member has `workspace = true`, and only `viola-e2e` lacks it. Prove it both ways: a `println!` in a `hook` path fails clippy, and the output modules' local `#[allow]` passes;
  - the SHA-pinned PCRE2 ripgrep install step, then G1 (bare `#[instrument]`) and G3 (`panic = "abort"`);
  - G2 (zero `event:"panic"`), G4 (`id: schema-conformance`), the secret scan (`id: secret-scan`, `if: always()`) and the scan-gated uploads, in the §9 step order;
  - SHA-pinned `actions/upload-artifact` v7.0.1.

Route order: otel-sdk-install (no-op record) → logger-stack-install → service-identity-wire → log-format-schema-emit → trace-context-propagate-wire → heartbeat-tick-wire → pii-scrubbing-wire → obs-ci-gate-wire.
- **Merged phase position:** `pii-scrubbing-wire` + security's `logging-redaction-wire` run as **one** phase at the `pii-scrubbing-wire` slot. It must follow logger-stack-install, because the sinks, `obs_event!` and the detail-file routing target must exist. It must precede obs-ci-gate-wire, because G1, the secret-scan/canary step and the veil `toggle` ban verify it. `logging-redaction-wire` is security's only logging contribution at Minimal tier, so obs imposes no order on security's other, non-logging phases.
- **Ownership:** obs owns the merged phase and its mechanical gates: `#[instrument(skip_all)]` + G1, the veil `toggle` cargo-deny ban (D-19), the TraceLayer `make_span_with`, and the `detail-<process>.ndjson` routing. Security owns the NEVER-log floor list that those gates enforce. The secret-scan test body and canary value stay tests-owned.

---
