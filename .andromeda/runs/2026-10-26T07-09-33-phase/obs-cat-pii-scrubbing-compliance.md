# obs · PII Scrubbing & Compliance — category read

## Carried
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18) The verification connects to the snapshot's recorded endpoint and must never log the endpoint (or `pinned_bin`) in any line it adds to a client, only `endpoint_kind`.
  > | Config values (thresholds), snapshot `endpoint` / `pinned_bin`, ledger versions | Low confidentiality | OK structured; path-valued ones follow the Medium rule | `cli_version`, thresholds logged; `endpoint` / `pinned_bin` never logged (only `endpoint_kind`) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) The recorded and observed server `pid` and `started_at` the check compares may be logged only as typed fields, never folded into a `detail` code of the mismatch line.
  > | `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) The `snapshot.json` path the clients read before the check stays out of every home-level line; any path goes to a detail file only.
  > | Absolute paths (OS username = only PII in scope), internal type names | Medium | scrub-required in external output; permitted in `diagnostics/` but obs keeps home-level lines path-free | `file` fields are basenames; `panic_location` workspace-relative; `with_file(false)`; paths only in detail files |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21) A verification failure's refusal or detail word (CLI exit 21, MCP `instance-unreachable`, the hook's fail-open) must be a closed kebab-case enum value.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13) The `send` client (CLI and MCP) gains a pre-frame step; its payload must stay under `skip_all` and `#[redact]`, with only `text_bytes` logged.
  > | Prompt text, assistant output (`prompt-submitted.data.text`, `last_assistant_message`) | High | never in any home-level process-log line; payloads stay in contract payloads and reach an instance `detail-<process>.ndjson` only inside a chain or drift report | `#[instrument(skip_all)]`; veil 0.3.0 `#[derive(Redact)]` with `#[redact]` on `SendParams.text`, `LastResult.last_assistant_message`, `HookEventParams.data`; only `text_bytes` logged |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:14) The `hook.dialog` and `answer` clients gain the same step; their dialog payload and answer response must stay redacted, with only codes-only dialog events.
  > | Tool `input`, plan text, dialog questions/answers | High | never in any home-level process-log line; instance `detail-<process>.ndjson` only inside a chain or drift report | `#[redact]` on `HookDialogParams.data`, `AnswerParams.response`; codes-only dialog events (`dialog_kind`) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28) The chunk's new mismatch and fail-open tests write `diagnostics/` files that must pass the CI secret scan, and the content canary must never reach a home-level file.
  > 5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
  >    - **High classes** (prompt and send text, `last_assistant_message`, tool `input`, dialog answers, drift-report echoes):
  >      - Every E2E and integration input is synthetic and embeds a fixed canary string.
  >      - The scan also asserts that the canary never appears in a home-level `diagnostics/*.ndjson`. It may appear only in `detail-*.ndjson`.
- (.andromeda/obs-plan.md:557) The broad rule over every line the verification adds: path-free home-level lines, no endpoint, and low-class metadata as typed fields only.
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged; `claude agents --json` rows are never logged verbatim, only `parse-rejected` counts; low-class metadata is logged as typed fields, never inside `detail` codes.
- (.andromeda/obs-plan.md:565) Any span or event the verification adds must use the source allow-list and pass the three gates.
  > 1. **Source allow-list (primary):** `#[instrument(skip_all, fields(...))]` plus `obs_event!` with explicit typed fields only. Enforced by three gates: clippy `disallowed-macros` on the tracing level macros, the fail-closed raw-`event!` grep in `scripts/lint-probes.sh` (§3 logger-stack-install), and a grep gate on bare `#[instrument]`.
- (.andromeda/obs-plan.md:566) Any new payload or verification-result type on the channel / hook / MCP path must take veil `Redact`.
  > 2. **Type-level redaction:** veil `Redact` on every channel / hook / MCP payload type, so `Debug` never reveals content.
- (.andromeda/obs-plan.md:567) The new verification error (in `viola-channel` or the client) must have a fixed `Display` and add no serde-sourced chain to external output.
  > 3. **Error layer:** fixed thiserror `Display`, and serde-sourced chains mapped to fixed messages at dispatch.
- (.andromeda/obs-plan.md:568) Any full chain from a failed verification (for example a sysinfo or snapshot read fault) goes only to the instance detail file.
  > 4. **Sink separation:** content-bearing records go only to instance-scoped `detail-<process>.ndjson`.
- (.andromeda/obs-plan.md:576) Any new field or `detail` value the verification logs must first be named in the §6 catalog and pass G4 against `schemas/diag-line.v1.json`.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:200) The hook client's fail-open on a mismatch must log only to its role file, with exit 0 and stderr empty.
  > - **Required telemetry signal:** fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
- (.andromeda/obs-plan.md:205) The MCP tools' `instance-unreachable` on a failed check must be logged with codes only.
  > - **Required telemetry signal:** rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan.md:731) A verification line carries no field outside the catalog.
  > - NEVER log any field not named in the Section 6 catalog (default-deny).
- (.andromeda/obs-plan.md:735) A mismatch `detail` code must hold neither the recorded nor the observed pid, nor the snapshot or endpoint path.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.

## Bindings
- **pid / `started_at` as typed fields ↔ the body §6 `detail` code catalog**: the check's mismatch outcome is a closed `detail` code (§6, "closed, kebab-case, never a path or pid"), and any pid it reports is a separate typed field named in the §6 additive catalog (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19), (.andromeda/obs-plan.md:735)
- **endpoint never logged ↔ security-plan NEVER-log floor and External errors**: the verification's CLI stderr, `--json`, MCP `isError` and log lines carry codes only, with no endpoint, snapshot path or pid (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18), (.andromeda/obs-plan.md:557)
- **default-deny ↔ the body §4 scenario `wait` / `last` and §9 G4**: the body names `strict-modes-failed` / `server-verify-failed` as values that cannot occur on `wait` / `last` until this route entry lands; a value made live here has to be in the §6 catalog and in `schemas/diag-line.v1.json` before G4 passes (.andromeda/obs-plan.md:576)
- **secret-scan verification ↔ test-plan §3 / §9 step order**: the new client-mismatch E2E homes sit under `target/e2e-home/`, so the scan and the gated `diag-<os>` upload cover them; the canary rule is tests-owned (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28)
- **hook fail-open ↔ the universal invariant (hook exits 0, no stderr)**: a mismatch on the hook client logs only to `hook-<name>.ndjson`, and writes no frame and nothing on stdout or stderr (.andromeda/obs-plan.md:200)
