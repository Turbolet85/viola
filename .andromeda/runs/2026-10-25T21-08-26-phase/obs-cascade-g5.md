# obs · cascade read · group 5 of 5

## Carried
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11) The `run` start path the chunk edits (`refuse_squatted`, up to channel bind) must pass no env map to a span or event and write no `CLAUDE*` value in any line.
  > | `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET`, other R8-stripped `CLAUDE*` values | Critical | never-log anywhere, including detail files | R8 strip logs `env_stripped_count` + `env_stripped_known` (stripped names) + `env_kept` (kept names), names only, values never read (D-34); env maps never passed to a span or event; secret-scan test with one canary per identity-floor name plus an unknown name |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13) The CARRY case_4 re-asserts the budget-paused `send-refused` line, which must carry no send text (only `text_bytes` may be logged), and a capture fix must not route text into a home-level line.
  > | Prompt text, assistant output (`prompt-submitted.data.text`, `last_assistant_message`) | High | never in any home-level process-log line; payloads stay in contract payloads and reach an instance `detail-<process>.ndjson` only inside a chain or drift report | `#[instrument(skip_all)]`; veil 0.3.0 `#[derive(Redact)]` with `#[redact]` on `SendParams.text`, `LastResult.last_assistant_message`, `HookEventParams.data`; only `text_bytes` logged |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) The squatted refusal's two stderr lines, the exit-1 `process-exit` line and any new or changed `ChannelError` `Display` must carry no absolute path or pipe name; paths may reach only instance detail files.
  > | Absolute paths (OS username = only PII in scope), internal type names | Medium | scrub-required in external output; permitted in `diagnostics/` but obs keeps home-level lines path-free | `file` fields are basenames; `panic_location` workspace-relative; `with_file(false)`; paths only in detail files |
  (.andromeda/obs-plan.md:567)
  > 3. **Error layer:** fixed thiserror `Display`, and serde-sourced chains mapped to fixed messages at dispatch.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18) The pipe name is the snapshot `endpoint`: no line or span the chunk adds on the listener bind, the remote-client or first-instance refusal, or `run`'s start may log it or `pinned_bin`, only `endpoint_kind`.
  > | Config values (thresholds), snapshot `endpoint` / `pinned_bin`, ledger versions | Low confidentiality | OK structured; path-valued ones follow the Medium rule | `cli_version`, thresholds logged; `endpoint` / `pinned_bin` never logged (only `endpoint_kind`) |
  (.andromeda/obs-plan.md:557)
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged; `claude agents --json` rows are never logged verbatim, only `parse-rejected` counts; low-class metadata is logged as typed fields, never inside `detail` codes.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) A pid (the squatter's or the wrapper's) may appear only as a typed field, never inside the exit-1 `detail` code or the `squatted-name` refusal.
  > | `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |
  (.andromeda/obs-plan.md:735)
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21) `squatted-name`, the exit-1 `detail` code and the budget refusal code the chunk re-asserts must stay closed kebab-case values, and a listener or bind failure crossing the `viola-channel` seam is logged once with a closed code.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |
  (.andromeda/obs-plan.md:643)
  > - **Module-boundary error logging:** every typed error that crosses a crate seam is logged once at `WARN`/`ERROR` with a closed `detail` / `error_code`. The full chain goes only to the instance detail file.
  (.andromeda/obs-plan.md:715)
  > - NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28) The secret scan must cover every `diagnostics/` file the new Windows listener tests write, and every input those tests feed must be synthetic.
  > 5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:30)
  > Every E2E and integration input is synthetic and embeds a fixed canary string.
- (.andromeda/obs-plan-detail/ci-integration.md:10) The new Windows tests' homes must fall under the `diag-<os>` upload path so a failing Windows leg leaves its lines.
  > | `diagnostics/*.ndjson` + `instances/*/diagnostics/detail-*.ndjson` from `target/e2e-home/**` | every E2E job, `if: always() && steps.secret-scan.outcome == 'success'`. A bare `always()` would also upload after a failed scan | `actions/upload-artifact` v7.0.1 (SHA-pinned), `name: diag-${{ matrix.os }}`, `path: target/e2e-home/**/diagnostics/*.ndjson`, `retention-days: 7` | `gh run download` → `jq` |
- (.andromeda/obs-plan-detail/ci-integration.md:22) Any line the chunk adds in `viola-channel` or `run.rs` must go through `obs_event!` (no raw level macro, no raw `event!`, no print macro), and `viola-channel` must still check without Tokio; the level stays code-set, never `env-filter`.
  > | Lint / typecheck | clippy `-D warnings` incl. `disallowed-macros` (raw `tracing::{info,warn,error,debug,trace}!` outside `viola_core::obs`) and the `print_stdout` / `print_stderr` / `dbg_macro` bans; the fail-closed raw-`event!` grep and both-ways ban probes in `scripts/lint-probes.sh`, Linux leg (mechanism in §3 logger-stack-install); bare-`#[instrument]` gate G1 (below); `cargo check` of sync crates without Tokio; `cargo deny check` (ubuntu) incl. veil `toggle` ban and bans on `opentelemetry-otlp` / `opentelemetry-stdout` / `sentry` / `tracing-appender` / tracing-subscriber `env-filter` feature | CI annotations + job exit code |
  (.andromeda/obs-plan.md:565)
  > 1. **Source allow-list (primary):** `#[instrument(skip_all, fields(...))]` plus `obs_event!` with explicit typed fields only.
  (.andromeda/obs-plan.md:709)
  > - NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`. Only `obs_event!` guarantees `event`, `process` and `instance`; the caller sets `corr` explicitly as a typed field on the events that carry one.
  (.andromeda/obs-plan.md:711)
  > - NEVER read the level from `RUST_LOG`, enable the `env-filter` feature, or call `EnvFilter::from_default_env`. Env vars are not a configuration channel.
- (.andromeda/obs-plan-detail/ci-integration.md:24) Every new integration test (the Windows listener negatives, the start refusal) must put its viola home under `target/e2e-home/`, so G2, G4, the secret scan and the upload read it.
  > | Integration tests | `diagnostics/*.ndjson` from fake-agent runs. Their viola homes must live under `target/e2e-home/` (the directory layout is tests-owned), so G2, G4, the secret scan and the `diag-<os>` upload cover them. A `hook` panic exits 0, so a home outside that root would hide it.
  (.andromeda/obs-plan.md:600)
  > Every integration-test viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it
- (.andromeda/obs-plan-detail/ci-integration.md:34) Any span the chunk adds on the listener or `run`'s start must take `skip_all` as its first `#[instrument]` argument.
  > `-U` lets `\s*` span lines, so an `#[instrument(` with `skip_all` on the next line passes. `skip_all` must be the first argument (the §4 form).
- (.andromeda/obs-plan-detail/ci-integration.md:36) G2 must read the home-level role files the new listener tests and `run`'s start write on the Windows leg, and count their panics.
  > - **G2, zero panics.** It runs as `scripts/g2-zero-panics.sh` (fail-closed; `tool-missing: jq` without jq), `--probe` first, in the `test` and `perf` jobs; the snippet below is its logic.
  (.andromeda/obs-plan-detail/ci-integration.md:38)
  > Scope is the home-level role files only. `detail-*.ndjson` repeats the panic line and is excluded (§10).
- (.andromeda/obs-plan-detail/ci-integration.md:40) Every line the chunk's code and tests emit (the exit-1 `process-exit`, any listener or bind failure line, the `send-refused` line case_4 reads) must conform to the default-deny schema G4 checks.
  > - **G4, schema conformance.** §10 counts schema-conformance failures as a build failure. G1–G3 do not measure them, and the Unit tests row checks only in-memory lines, so G4 checks real process output.
  (.andromeda/obs-plan-detail/ci-integration.md:41)
  > Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).
  (.andromeda/obs-plan.md:576)
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan-detail/ci-integration.md:59) The new tests run in the `test` job, and their homes must be inside the gated scope (not a second carve-out beside `tests/chaos_feed_panic.rs`), so a red Windows test step still leaves G2 and G4 reading them.
  > 1. Integration and E2E tests (every viola home under `target/e2e-home/` but the one `viola-chaos-*` home of `tests/chaos_feed_panic.rs`, outside G2, G4 and the secret scan by the founder's ruling of 2026-10-04), then G2 and G4, **both** `if: always()`.
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:8) A panic in the new listener tests or in `run`'s start after init step 4 must leave exactly one home-level `panic` line, and the CARRY case_4 red is closed by its cause or a capture fix, never by a retry.
  > - **Zero unlogged panics:** every panic in `run`, `hook`, `mcp` and `cli` **after init step 4** produces **exactly one** single-line `level:"ERROR", event:"panic"` record in its home-level role file, with no multi-line backtrace in home-level files, and `hook` still exits 0. Retry-once policies for panic handling are forbidden, in parallel with the tests' `retries = 0`.
  (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:19)
  > The failure condition "a panic evidenced by an exit code without its line" applies only to processes whose `process-start` is in a home-level role file.
  (.andromeda/obs-plan.md:662)
  > **Cross-input parallel:** the tests' zero-flakiness budget (`retries = 0`) ↔ obs' zero-unlogged-panics invariant. Retry policies mask real signals in both cases.
  (.andromeda/obs-plan.md:748)
  > - NEVER add retry-once policies for panics or flaky telemetry assertions.
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) `refuse_squatted` must keep writing its refusal through `src/human.rs` `refuse` with no print macro and no new `#[allow]`, and the case_4 capture fix must not print to stdout or stderr; a changed `run` caller count is restated here.
  > NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates (`viola-harness` and the fake agent are exempt, §3 obs-ci-gate-wire). They bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing. Enforced by `[workspace.lints.clippy]` `print_stdout`, `print_stderr`, `dbg_macro` = `deny` under the §9 clippy step. Only the output modules that own `--json`, human CLI stderr and the hook decision body may carry a local `#[allow]`, and only where they use a print macro — `run`'s and `revive`'s pre-spawn refusals, `revive --list`'s, `verify`'s, `list`'s and `plugin install`'s refusals write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1, chunk 2026-10-17-optional-plugin-install), which uses no print macro and carries no `#[allow]`
  (.andromeda/obs-plan.md:717)
  > - NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates: they bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing;

## Bindings
- **pii (path-, pid- and endpoint-free lines) ↔ ci (G4 default-deny schema)**: the exit-1 `process-exit` `detail` code and any listener-failure line must carry neither the pipe name nor a pid, and G4 is the gate that fails a field outside the catalog (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18) (.andromeda/obs-plan-detail/ci-integration.md:41)
- **ci (homes under `target/e2e-home/`) ↔ slo-gates (zero unlogged panics) ↔ pii (secret scan)**: the one home rule for the new Windows tests is what puts their lines under G2's panic count, G4 and the secret scan at once (.andromeda/obs-plan-detail/ci-integration.md:24) (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:8) (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28)
- **slo-gates (no retry-once) ↔ test-plan (`retries = 0`)**: the CARRY case_4 red closes only on a known cause or a fixed capture, never a green re-run (.andromeda/obs-plan.md:662)
- **logs (print ban, `src/human.rs` `refuse`) ↔ pii (path-free refusal lines)**: `refuse_squatted`'s two fixed stderr lines go through the one `refuse` writer and carry no path or pid (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17)
- **ci (sync crates checked without Tokio) ↔ architecture (Tokio only in `viola-mcp`)**: any instrumentation the chunk adds in `viola-channel`'s listener stays sync (.andromeda/obs-plan-detail/ci-integration.md:22)
