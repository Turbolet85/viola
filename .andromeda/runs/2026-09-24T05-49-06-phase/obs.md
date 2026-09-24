# obs extract

## Relevance
partial. Only the one-line diagnostics minimum (scope item 6), the panic-hook ordering, the harness `logs` read path, and the CI leg settings that later obs gates depend on (homes, artifact uploads, `panic = "unwind"`) fall in obs. The full sinks, schemas, gates and redaction belong to later Epoch 1 chunks.

## Constraints
- **Line shape.** obs-plan §3 "Log format JSON schema" (binding, reproduced from tests §3) and §6 "Required fields" set the per-role line.
  - Required keys: `timestamp` (RFC 3339 UTC, milliseconds, `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and a flattened `message`.
  - `message` is a static literal equal to `event`. `event` must come from the closed kebab-case enum.
  - `process` must be in `run|hook|mcp|ui|cli`. `instance` is a `ViolaName`.
  - A null `corr` or `instance` must be written as a missing key, never as a literal `null` (§3 "Obs extensions → Null encoding", D-12/D-26).
  - Fields may be added but never renamed or removed. The harness greps on them.
  - If the minimal line is `process-start`, its extra fields are limited to the §6 "Additive field catalog" row (`service_name`, `version`, `os`, `pid`, …). The allow-list is default-deny (§8 "Default-deny posture").
- **Writing the line.** Per obs-plan §3 "Logging stack" and "Log file location":
  - Each line is one `write_all` of one complete line on an append-mode handle.
  - The file lives at `<home>/diagnostics/<role-basename>.ndjson`. Unix permissions are 0600 for files and 0700 for dirs.
  - Nothing goes to stdout on any role.
  - Init order is fixed (§3 "OTel SDK init → Init order"): the home strict-modes check runs first, then `diagnostics/` is created and opened. If the open fails, the writer becomes `std::io::sink`.
  - How much of the strict-modes and DACL work lands in this chunk is a P4 call.
- **Panic hook.** obs-plan §7 "Panic hooks" and §3 "Init order" step 1 require:
  - `std::panic::set_hook(viola_panic_hook)` is the first statement of `main`.
  - The hook never calls the default hook and never writes to stderr. It writes one JSON line via one `write_all` to a pre-opened `OnceLock<Arc<File>>`, not through the subscriber.
  - Everything after `set_hook` runs inside one `catch_unwind` in `main`. The role's exit is decided at that catch site.
- **Unwinding.** obs-plan §7 ("Unwinding is required") and §9 G3 require `panic = "unwind"` in every Cargo profile. No `panic = "abort"` may appear in any `Cargo.toml`, cargo `config.toml` or workflow file: not as a profile key, not as a `-C panic` rustflag, not as a `CARGO_PROFILE_*_PANIC` env var.
- **Logger setup, if tracing-subscriber is used for the minimal line.** Per obs-plan §3 "OTel SDK init → Cargo entry / Init body sketch" and "Logging stack → Mandatory builder settings":
  - Versions and features: `tracing-subscriber 0.3.23`, `default-features = false`, features `fmt,json,registry,std`, root bin only. `tracing 0.1.44`.
  - Timer: a `MillisUtc` timer backed by a direct `chrono 0.4.45` dependency (`clock`, `std`).
  - Builder: `.with_writer(...)` always explicit, `.log_internal_errors(false)`, `.with_ansi(false)`, `.with_span_list(false)`, `.with_current_span(false)`, `.flatten_event(true)`.
  - Level: a `filter::Targets` layer only. No `env-filter` feature, no `RUST_LOG`.
  - Whether the line goes through the subscriber or is written by hand in this chunk is P4's decision against research.
- **Service identity.** obs-plan §3 "Service identity" says `service.name` is the hardcoded constant `"viola"` and the version comes from `env!("CARGO_PKG_VERSION")`, with `version.workspace = true` on every member. Neither is ever read from env vars.
  - The plan puts these constants in `viola-core`. This chunk creates no product crate without a consumer, so where the constant lives now is a P4 question.
- **CI legs and homes.**
  - Per obs-plan §9 "Step order" 1 (fix-pass B1) and the "Integration tests" row: every harness or fake-agent home must sit under `target/e2e-home/`, and `ci.yml` must set `AGENT_RUN_KEEP_HOMES=1` so that `cleanup` never deletes a home before the later gate steps read it.
  - Per §9 "Telemetry artifact handling", uploads use SHA-pinned `actions/upload-artifact` v7.0.1 with `retention-days: 7`.

## Patterns to follow
- **Harness `logs` merge shape**, per obs-plan §3 "Snapshot / paste-to-AI integration":
  - diagnostics records are wrapped as `{"src":"diag","file","record"}`;
  - events are wrapped as `{"src":"events","instance","offset","record"}`;
  - instance detail lines add `instance`;
  - a torn line becomes `{"torn":true,"offset":n}`;
  - the glob includes `instances/*/diagnostics/detail-*.ndjson` (D-08).

  This chunk needs only the diag slice, but the envelope should take this shape from the start.
- **Lint exemption for the harness and the fake agent**, per obs-plan §3 "Bootstrap phases → obs-ci-gate-wire" (fix-pass B5):
  - `viola-harness` (`crates/viola-e2e`) and `viola-fake-agent` must print, so they leave out `[lints] workspace = true` and carry their own `[lints.clippy]` table without `print_stdout` / `print_stderr`;
  - every product member, root `viola` bin included, carries `[lints] workspace = true`.

  The print-ban lints themselves belong to the Observability-gates chunk. Laying out the workspace members this way now avoids rework.
- **Panic line key set**, per obs-plan §7 "Panic hooks": `{"timestamp","level":"ERROR","target":"viola::panic","message":"panic","event":"panic","process","instance","panic_location","thread"}`.
  - `corr` is never written. `instance` is omitted when unresolved.
  - `panic_location` is workspace-relative.
- **Mutation coverage includes obs code**, per obs-plan §9 "Pipeline integration → Mutation" and §10 "Build / deploy failure conditions". Obs code (panic hook, `MillisUtc`, emit sites) is mutated like product code, and a surviving mutant in it fails the build. The minimal line and hook need tests that kill mutants under `--in-diff`.

## Anti-patterns to avoid
- **stdout, stderr and the default panic hook** (obs-plan §11 "Logs", "Error Reporting"):
  - no telemetry to stdout on any product role;
  - never the default tracing-subscriber writer, which is stdout;
  - never leave `log_internal_errors` at its default `true`;
  - never let the default panic hook run;
  - no multi-line stack traces.
- **Configuration channels** (obs-plan §11 "Logs", "Universal"):
  - no `RUST_LOG`, `EnvFilter` or `env-filter`;
  - no `tracing-appender` / `non_blocking`;
  - no `tracing_log::LogTracer`;
  - service identity is never read from `OTEL_*` or other env vars;
  - CI env metadata (`ci.run.id`, `git.commit.sha`) is never injected into product lines (§11 "CI", §9 "CI-specific resource attributes").
- **CI artifacts** (obs-plan §11 "CI"): no unpinned Actions, and no `diagnostics/` upload before the secret-scan step passes.
  - This chunk's per-OS upload of `target/agent-run/artifacts/` must not carry `diagnostics/*.ndjson`, merged `logs` output or detail lines before the secret scan exists. That scan belongs to the Log-redaction / Observability-gates chunks.
  - Whether the artifact contents are diagnostics-free is a P3/P4 check.

## Contract bindings
- **obs ↔ tests §3, log format.** obs aligns to tests, not the other way round (obs-plan §3 "Log format JSON schema"). The harness `logs` / `status` implementation in this chunk reads the per-role line by the field names above. Absent and `null` count as the same value for `corr` / `instance`.
- **obs ↔ tests §3, `cleanup` and CI.** Per obs-plan §9 "Step order" 1 (B1), `AGENT_RUN_KEEP_HOMES=1` must be honoured by harness `cleanup`. Homes under `target/e2e-home/` are what later gates G2, G4, the secret scan and `diag-<os>` read.
  - G2 fails closed when no home-level role file exists (§9 G2).
  - So this chunk's real per-role line is what makes the future G2 scope non-empty.
- **obs ↔ tests §9, artifact names (reconcile needed).**
  - obs-plan §9 names the harness capture artifact `harness-${{ matrix.os }}`, with `path: target/agent-run/`, gated on `steps.secret-scan.outcome == 'success'`.
  - The scope names it `agent-run-${{ matrix.os }}`, with path `target/agent-run/artifacts/`.
  - Artifact names must be unique per workflow run, and v4+ artifacts are immutable.
  - P4 must reconcile the name and path with test-plan §9. Otherwise the later obs gating step will collide or duplicate.
- **obs ↔ security.** The home strict-modes check comes before any `diagnostics/` create or open (obs-plan §3 "Init order" step 4). A SHA-pinned `upload-artifact` sits under zizmor (§11 "CI"), and zizmor belongs to the Supply-chain chunk.

## Acceptance criteria contributions
- Every line that a product role writes to `<home>/diagnostics/<role>.ndjson` meets all of the following:
  - it is exactly one JSON object per physical line;
  - `timestamp` matches RFC 3339 UTC with milliseconds and a trailing `Z`;
  - `level` is in `DEBUG|INFO|WARN|ERROR`;
  - `target` is present and `message == event`;
  - `event` is a closed-enum value and `process` is in `run|hook|mcp|ui|cli`;
  - `corr` / `instance` are absent rather than literal `null` when unset;
  - `agent-run logs` streams the line in a `{"src":"diag","file","record"}` envelope.

  (per obs-plan §3 "Log format JSON schema", §3 "Snapshot / paste-to-AI integration")
- A forced panic after the role file opens produces exactly one single-line `level:"ERROR", event:"panic"` record in that role file. Nothing reaches stderr or stdout, and the process exits through the `catch_unwind` site, not with 101. (per obs-plan §7 "Panic hooks", §10 "Zero unlogged panics")
- The G3 command `rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' "(panic|_PANIC)\s*[:=]\s*[\"']?abort" .` exits 1 on the chunk's tree, and every Cargo profile states `panic = "unwind"`. (per obs-plan §9 "Gate commands → G3", §7)
- In `ci.yml`:
  - every `uses:` is pinned by full commit SHA;
  - `AGENT_RUN_KEEP_HOMES=1` is set on all three OS legs;
  - all harness homes resolve under `target/e2e-home/`;
  - every upload uses `actions/upload-artifact` v7.0.1 with `retention-days: 7`;
  - no uploaded path includes `diagnostics/*.ndjson` or merged `logs` output while no secret-scan step exists.

  (per obs-plan §9 "Telemetry artifact handling" / "Step order", §11 "CI")

## Relevant amendment history
(none). `D:/dev/projects/viola/.andromeda/obs-plan-amendments.md` does not exist, which is normal on a fresh project. For context, overseer fix-pass rulings B1 (keep homes via `AGENT_RUN_KEEP_HOMES=1`) and B5 (harness and fake-agent print-lint exemption) are already folded into the plan text (obs-plan §12 Decisions Log, fix pass 2) and are cited above.
