# arch extract

## Relevance
Partial. The arch plan sets where the code lives, the error-type split (thiserror/anyhow), the diagnostic output channels, the dependency and CI rules, and the config and env-var registry. The redaction rules themselves (the NEVER-log list, the veil semantics, the span-field form) belong to obs and security.

## Constraints
- **Error-type split.** Per architecture §Established Decisions [Error Handling] and §Conventions → Error handling schema ("Rust error types"), there is one thiserror enum per crate, named `<Crate>Error` (at HEAD that means `CoreError` in `viola-core`). anyhow may appear only in the root `viola` bin's `main` and dispatch. The chunk's fixed-message `Display` impls and its "chain stops at the root bin" rule must stay inside this split. No anyhow in `viola-core`, and no new per-crate error enum under a different name. Whether the current `Display` impls already use fixed messages is research's question.
- **Where each kind of output goes.** Per architecture §Cross-cutting Patterns → Diagnostic output channels:
  - Codes-only process lines go to the home-level role files `diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`.
  - Content-bearing detail belongs only in `instances/<ViolaName>/diagnostics/detail-<role>.ndjson`.
  - While the child runs, `run` writes nothing to the terminal except the child's own output.
  - `hook` writes nothing to stderr.
  - Short-lived CLI verbs may use stderr only for their human output.

  Routing the anyhow chain and drift reports to the instance detail sink must respect these destinations. The chain must not go to stderr from `run` while the child is running. The detail files' rules (0700/0600, one `write` per line, format fixed by `schemas/diag-detail.v1.json`) come from architecture §Occupied Resources → Filesystem (the `instances/<ViolaName>/…` row).
- **Exit codes.** Per architecture §Conventions → CLI exit codes and "`viola hook` exit codes", the `main` catch site keeps exit `1` (internal error, anyhow edge). `viola hook` always exits 0, even on caught panics. Recording the chain must not change the exit-code taxonomy.
- **No redaction switch in config or env.** Per architecture §Cross-cutting Patterns → Config management (Settings, and the "environment variables are not a configuration channel" bullet):
  - `diagnostics_level` (`info`|`debug`) is the only source of the log level, and `RUST_LOG` has no effect.
  - `VIOLA_*` variables are wrapper-to-child plumbing only.
  - §Occupied Resources → Environment variables registers no knob for redaction.

  So the "no env var, flag or `config.json` key can widen or disable redaction" rule is also an arch rule: add no env var, flag or config key for redaction, and `debug` must not widen the NEVER-log floor.
- **Dependency and ban rules.** Per architecture §Stack and Technologies and §Infrastructure Patterns → Build system:
  - Every third-party version is pinned once in `[workspace.dependencies]`.
  - `deny.toml` carries the `veil/toggle` feature ban.
  - The sync crates are checked by the sole-root `deny-sync.toml` tokio ban (per `scripts/sync-crates.txt`), plus the C-crate and telemetry bans.

  veil 0.3.0 is not a row in §Stack and Technologies. Under §Infrastructure Patterns → Crate dependency direction, `viola-core`'s only listed third-party dependency is nutype (plus the shared serde, serde_json, chrono and thiserror). Adding veil to `viola-core` (or to the root bin) is a new dependency edge that the wrap must amend in. It must not pull tokio or a C build into a sync crate.
- **Mutation gate on the CI runner.** Per architecture §Infrastructure Patterns → CI/CD approach, the `mutants` job runs on ubuntu, on push and PR, over the chunk diff. Per §Cross-cutting Patterns → Cross-platform discipline, every OS-specific branch must compile and be tested on its CI runner. The folded `read_diagnostics_level` MISSED mutant (`src/obs.rs:193`) must therefore be killed by a test that reaches the open-error `Unreadable` branch on Linux, not only on Windows.
- **Scope of the code.** Per architecture §Established Decisions [Module Boundaries], each product crate is created by its first consumer. The chunk must not create `viola-channel`, `viola-mcp`, `viola-ui` or `viola-agent-claude` just to host redaction types. The mechanism lives in `viola-core` and the root bin (`src/main.rs`, `src/cmd/*`, `src/run/*`, `src/obs.rs`) per architecture §Infrastructure Patterns → Project directory structure.

## Patterns to follow
- **Detail lines have no `v`.** Process-log and detail lines carry no `v`; their version lives in the schema filename, per architecture §Stack and Technologies (ORM / migrations row). New detail records such as `chain` and `drift_report` follow `diag-detail.v1.json`, not the `v`-carrying event or snapshot rule.
- **ndjson line discipline.** One complete JSON object plus `\n` per single `write`, with multi-line text (anyhow chains, backtraces) carried as an escaped JSON string, per architecture §Conventions → Data model conventions.
- **Tolerant external parsing with drift reports.** Keep serde_path_to_error for path-precise drift reports and keep parsing tolerant, per architecture §Established Decisions [Validation]. Mapping the source to a fixed message before it joins a chain must not turn tolerant parsing into hard failures. The `config.json` path reports `parse-rejected{parser:"config-json"}` and falls back to the default, per architecture §Occupied Resources → Filesystem (the `config.json` row).
- **Upstream text is content.** Per architecture §Cross-cutting Patterns → Untrusted upstream text, a driven session's Stop text and dialog payloads are content: forwarded as escaped strings, never interpreted. These are the payloads the redacted types wrap.
- **Fail open toward the human.** Per architecture §Cross-cutting Patterns → Fail open toward the human and §Established Decisions [Hook Contract], a failure to write a diagnostics or detail file is ignored and never changes the command's outcome.

## Anti-patterns to avoid
- Do not let anyhow leak outside the root bin. Do not put upstream text or paths into `<Crate>Error` `Display` (architecture §Conventions → Error handling schema, "Rust error types").
- Do not add an env var, CLI flag or `config.json` key that changes the level or redaction beyond `diagnostics_level`. Do not honour `RUST_LOG` (architecture §Cross-cutting Patterns → Config management).
- Do not add a dependency by version outside `[workspace.dependencies]`. Do not enable `veil/toggle` (architecture §Infrastructure Patterns → Build system).

## Contract bindings
- **arch ↔ obs:** obs owns the logger and the line and detail formats (architecture §Cross-cutting Patterns → Diagnostic output channels, "The logger and the format are owned by obs"). The arch plan fixes only the destinations and file rules. The `#[instrument(skip_all, …)]` form and the veil derive semantics are obs-plan §6 and §3.
- **arch ↔ security:** the NEVER-log list and the error-sanitization rules are security-plan's (§logging-redaction-wire, §error-sanitization-wire). The arch plan provides the R8 strip-list names (architecture §Occupied Resources → Environment variables) that the floor must keep out of every sink.
- **Open question for obs and security to settle (not arch):** architecture §Standard Contracts → "Event `data` per kind" makes `events.ndjson` carry content by design (`prompt-submitted.text`, `permission.input`, `plan.plan`, `turn-ended.last_assistant_message`) as the authoritative audit trail. The floor's "user content and tool `input` only in instance detail files" presumably covers process-log sinks only, not this data log. The researcher should confirm the floor's intended scope, and the chunk must not redact or strip `events.ndjson` payloads.
- **arch ↔ tests:** the mutation gate and the ubuntu `mutants` job (architecture §Infrastructure Patterns → CI/CD approach) tie to test-plan's harness `run --mutants`. Test homes are isolated per test through `--home` (architecture §Cross-cutting Patterns → Config management), and the fixtures that check the detail file use that isolation.
- **arch ↔ all (wrap-time):** architecture §Stack and Technologies needs a veil 0.3.0 row. §Infrastructure Patterns → Crate dependency direction needs the new edge to veil from `viola-core` and/or the root bin.

## Acceptance criteria contributions
- **Error-type split.** anyhow appears only in the root `viola` bin, error enums are named `<Crate>Error`, and their `Display` output contains no path, upstream text or payload (per architecture §Established Decisions [Error Handling] and §Conventions → Error handling schema).
- **Catch site.** A failing verb still exits `1` (and `viola hook` still exits 0). When an instance resolves, its anyhow chain appears only in `instances/<name>/diagnostics/detail-<process>.ndjson` (0600, one line per write). The chain does not appear in the home-level role file, on the terminal while `run`'s child is running, or in `--json` output (per architecture §Conventions → CLI exit codes and §Cross-cutting Patterns → Diagnostic output channels).
- **No redaction knob.** No new `VIOLA_*` env var, CLI flag or `config.json` key is added. Setting `diagnostics_level: "debug"` or `RUST_LOG` does not surface any NEVER-log value (per architecture §Occupied Resources → Environment variables and §Cross-cutting Patterns → Config management).
- **Dependencies and CI.** veil is pinned in `[workspace.dependencies]` without `toggle`. `cargo deny check` and the sole-root `deny-sync.toml` ban pass for every sync crate. The ubuntu `mutants` job is green on the chunk diff, with the `src/obs.rs` `read_diagnostics_level` NotFound-guard mutant caught on the CI runner (per architecture §Infrastructure Patterns → Build system and → CI/CD approach).

## Relevant amendment history
- **2026-09-24-diagnostics-plane (diagnostics roots, config key, diag schemas, v exception).** This amendment:
  - set up the home-level role files and the owner-only instance `detail-<role>.ndjson`, with `hook` detail going only to `detail-hook.ndjson`;
  - registered `diagnostics_level` as the only level source (`RUST_LOG` has no effect) and the `MAX_FRAME`-capped `config.json` read with `parse-rejected`;
  - registered `schemas/diag-line.v1.json` and `diag-detail.v1.json`;
  - added the "process-log lines carry no `v`" exception.

  Why: the chunk before this one shipped those facts. This chunk builds directly on that detail sink and config path, including the `read_diagnostics_level` code the folded CI red points at.
- **2026-09-24-three-os-ci-headless-harness-skeleton (run's process log location, logging and serialization rows).** This amendment moved `run`'s codes-only log to the home-level `run-<name>.ndjson`, with content-bearing detail staying in the instance's `diagnostics/` (obs D-08). It added the tracing 0.1.44 and tracing-subscriber 0.3.23 row (root only) and serde_json `preserve_order`. Why: shipped facts. It fixes the tracing stack that the `#[instrument(skip_all…)]` form sits on.
- **2026-09-24-three-os-ci-headless-harness-skeleton (CI setup and wired jobs).** This amendment wired the `mutants` job (ubuntu, push and PR, base passed through `env:`). Why: shipped CI. That job produced the MISSED mutant folded into this chunk.
- **2026-09-24-supply-chain-and-workflow-gates.** This amendment gave `deny.toml` four families, including the feature bans such as `veil/toggle`, and moved the tokio ban to the sole-root `deny-sync.toml` per `scripts/sync-crates.txt`. Why: the excluded-graph mechanism false-failed under feature unification. Any veil dependency this chunk adds must pass both configs.
