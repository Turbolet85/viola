# obs · PII Scrubbing & Compliance — category read

## Carried
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) The `link` / `unlink` verbs' human result lines, `--json` documents, stderr refusals and their `cli-<name>.ndjson` lines must carry no absolute path; a path may reach only an instance detail file.
  > | Absolute paths (OS username = only PII in scope), internal type names | Medium | scrub-required in external output; permitted in `diagnostics/` but obs keeps home-level lines path-free | `file` fields are basenames; `panic_location` workspace-relative; `with_file(false)`; paths only in detail files |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18) The verbs read `snapshot.json`'s `endpoint` to reach the driven wrapper, and that endpoint must never be logged or printed (only `endpoint_kind` may be).
  > | Config values (thresholds), snapshot `endpoint` / `pinned_bin`, ledger versions | Low confidentiality | OK structured; path-valued ones follow the Medium rule | `cli_version`, thresholds logged; `endpoint` / `pinned_bin` never logged (only `endpoint_kind`) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) The snapshot `pid` / `started_at` the verbs' liveness pre-check and server check read may be logged only as typed fields, never inside a `detail` code.
  > | `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21) The two methods' refusals (`-32602` for a `driven` that is not the serving wrapper's name or a `driver` equal to `driven`) and the verbs' exit refusals must be logged and returned as closed kebab-case codes only.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28) Every `diagnostics/` file the new verbs, methods and `tests/channel_links.rs` homes write must pass the CI secret-scan test before artifact upload.
  > 5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:37) When the verbs take the server identity check through `client::connect`, their server-verification refusal must carry no pid, start time, path or endpoint, and the `digit_runs` scan of `tests/security_negatives_server_identity.rs` must cover their outputs and `cli-<name>.ndjson` line on Linux and Windows.
  > - A server-verification refusal carries no pid, start time, path or endpoint: `tests/security_negatives_server_identity.rs` scans every output and the `cli-<name>.ndjson` line with `digit_runs` (whole numbers only, itself tested by `digit_runs_reads_whole_numbers_only`), on Linux and Windows (the file compiles out on macOS).
- (.andromeda/obs-plan.md:190) The `link` / `unlink` channel lines must never carry the methods' `params` (`driver`, `driven`) or result bodies (`since`, `already_linked`), and a self-reported `from` is logged as unauthenticated.
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan.md:557) Any line the verbs or methods add must stay path-free, never log `endpoint`, and keep low-class metadata out of `detail` codes.
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged; `claude agents --json` rows are never logged verbatim, only `parse-rejected` counts; low-class metadata is logged as typed fields, never inside `detail` codes.
- (.andromeda/obs-plan.md:565) Each new span on the verbs' client path and the wrapper's `link` / `unlink` dispatch must be `#[instrument(skip_all, fields(...))]`, and each new line must go through `obs_event!` with explicit typed fields.
  > 1. **Source allow-list (primary):** `#[instrument(skip_all, fields(...))]` plus `obs_event!` with explicit typed fields only. Enforced by three gates: clippy `disallowed-macros` on the tracing level macros, the fail-closed raw-`event!` grep in `scripts/lint-probes.sh` (§3 logger-stack-install), and a grep gate on bare `#[instrument]`.
- (.andromeda/obs-plan.md:566) The `link` / `unlink` params and result types must derive veil `Redact`, as every channel payload type does.
  > 2. **Type-level redaction:** veil `Redact` on every channel / hook / MCP payload type, so `Debug` never reveals content.
- (.andromeda/obs-plan.md:567) The verbs' and methods' errors must use fixed thiserror `Display` messages, and a serde-sourced chain must become a fixed message at dispatch.
  > 3. **Error layer:** fixed thiserror `Display`, and serde-sourced chains mapped to fixed messages at dispatch.
- (.andromeda/obs-plan.md:576) A field the chunk logs must be named in the §6 catalog and admitted by `schemas/diag-line.v1.json`, or G4 fails the build.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:731) No field outside the §6 catalog may be logged by the new verbs or methods.
  > - NEVER log any field not named in the Section 6 catalog (default-deny).
- (.andromeda/obs-plan.md:735) No `detail` code the verbs or methods write may hold a path or pid.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.

## Bindings
- **Server-verification refusal scan ↔ security plan (server check before the first frame) and test-plan**: if the founder's P4 form has the verbs go through `client::connect`, their exit-21 refusal joins the scanned set of `tests/security_negatives_server_identity.rs` (tests-owned file, obs-owned no-pid/path/endpoint rule), (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:37)
- **Default-deny ↔ §4 Edge flow E3 and §6 Log Coverage catalog**: E3 states today that a link has no process-log line, no `event` value and no span and that the explicit methods are not built (.andromeda/obs-plan.md:392); any line or field the served methods add is admitted only through the §6 catalog and `schemas/diag-line.v1.json`, (.andromeda/obs-plan.md:576)
- **Channel lines carry no params ↔ §6 IPC invocation boundary wrapper**: the `link` / `unlink` `channel-request` / `channel-response` lines carry method, `corr`, `conn`, result class and error code only, (.andromeda/obs-plan.md:190)
- **Closed refusal codes ↔ architecture §Conventions (refusals, exit codes)**: the `-32602` refusals and the verbs' exit codes are the architecture's closed codes; obs logs them as closed enums, (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21)
- **Secret scan ↔ test-plan (secret-scan test, canary) and §9 CI**: the new tests' homes sit under `target/e2e-home/` so the scan and uploads cover them, (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28)
