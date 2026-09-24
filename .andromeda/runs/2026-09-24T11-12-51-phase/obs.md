# obs extract

## Relevance
Relevant. This chunk is the merged obs `pii-scrubbing-wire` + security `logging-redaction-wire` phase, which obs owns (per obs-plan §3 Bootstrap phases → pii-scrubbing-wire / Merged phase position / Ownership).

## Constraints
- **Every span is declared in one form.** Each `#[instrument]` must be `#[instrument(skip_all, name = "<area>.<operation>", fields(..))]`. `skip_all` must be the first argument. Span names are static `<area>.<operation>` snake_case strings, with no instance names, dialog ids or paths. `err` / `ret` are banned on anything that can carry a serde / serde_path_to_error source or user content (per obs-plan §4 Span naming convention; §11 Spans / Traces). Whether bare or `err`/`ret` instruments exist at HEAD in `src/**` / `viola-core` is research's question.
- **Error scrubbing has three layers** (per obs-plan §7 `before_send` scrubbing pattern; §8 Integration points #4):
  1. Every thiserror `Display` is a fixed message, with no fields holding paths or payloads.
  2. At the root-bin dispatch edge, an anyhow chain with a `serde_json::Error` / `serde_path_to_error::Error` source becomes the fixed `internal error` before it reaches stderr, `--json`, MCP `isError`, Problem Details or `error.data`.
  3. The full chain is written only as a one-line JSON array `chain:[...]` to `instances/<name>/diagnostics/detail-<process>.ndjson`, and only when an instance resolves.
- **Sink separation** (per obs-plan §3 Logging stack → Sink; §8 Integration points #5; D-30):
  - Content-bearing records (chain, `drift_report`) never pass through the tracing subscriber.
  - No `obs_event!` call may carry `chain`, `drift_report`, `panic_payload` or `backtrace`.
  - The detail writer formats each line itself, with the same `timestamp` (MillisUtc) / `level` / `target` / `message` / `event` / `process` / `instance` (+ `corr`/`conn`) keys as the home-level line, and writes it with one `write_all`.
- **Detail file handling** (per obs-plan §3 OTel SDK init → Init order step 4; §3 Logging stack → Format):
  - The file is opened lazily by the first content-bearing record: append mode, 0600 in 0700, same owner/mode/symlink checks as the home.
  - A failed detail open drops only the detail line. It never drops the home-level line and never writes to stderr.
  - Detail lines can exceed 4 KiB, so they rely on a single `write_all` plus torn-line tolerance.
- **The NEVER-log floor** (per obs-plan §8 PII Scrubbing table rows Critical/High; §11 PII Scrubbing):
  - GUI token, launch URL / `ui/<port>.url`, `Cookie`, `?t=`, `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and every R8-stripped `CLAUDE*` value are never logged at any level in any file, detail files included.
  - Send text, tool `input`, `last_assistant_message` and `statusline_command` never appear in any home-level file. They may reach `detail-<process>.ndjson` only inside a chain or drift report.
- **Default-deny, and no switch can widen redaction** (per obs-plan §8 Default-deny posture; §3 Logging stack → Agent-mode flag; §11 PII Scrubbing):
  - A field is logged only if §6 catalogs it.
  - No `config.json` key, `VIOLA_*` env var or CLI flag may widen the allow-list or disable redaction. `diagnostics_level` changes volume only.
- **veil and paths** (per obs-plan §8 Scrubbing libraries; D-19; §11 PII Scrubbing):
  - veil 0.3.0 `#[derive(Redact)]` is the type-level second layer, and `toggle` stays banned.
  - `detail` codes must never contain paths or pids (Founder Direction 4).

## Patterns to follow
- **Panic-hook model for the catch site.** The shipped panic hook already pairs a codes-only home-level line with a companion detail line through `viola::obs` (per obs-plan §7 Panic hooks). The `main` catch-site chain routing should follow the same shape:
  - home-level `process-exit{subject:"self", exit_code:1, detail:"internal-error"}`, plus the fixed stderr `error: internal error` for `cli`;
  - a detail line carrying the same `event`/`corr`/`conn`, so `jq` joins the two (per obs-plan §7 Error → span correlation; §6 `detail` code catalog).
  - Which `event` the chain detail line carries is not pinned by the plan. Research/design must pick one that `diag-detail.v1.json`'s inlined enum admits.
- **Detail-line schema.** Content fields are exactly `panic_payload`, `backtrace`, `chain` and `drift_report`, under a top-level `unevaluatedProperties: false` (per obs-plan §8 Detail-file scope).
- **Where veil goes.** `#[redact]` targets are named payload fields: `SendParams.text`, `LastResult.last_assistant_message`, `HookEventParams.data`, `HookDialogParams.data`, `AnswerParams.response`. The logged proxy is `text_bytes` (per obs-plan §8 PII Scrubbing table). Most of those types live in crates that are OUT for this chunk. Whether any payload type exists at HEAD in `viola-core` / the root bin to derive `Redact` on is research's question.
- **`config.json` read fallbacks.** An unreadable or missing `config.json` falls back to `diagnostics_level:"info"`. The subscriber is not yet installed during the parse, so the failure is held in memory and then emitted as `parse-rejected{parser:"config-json", detail:"unreadable"|"malformed"|"unknown-keys", count}` (per obs-plan §3 Init order steps 5–6; §6 `parse-rejected` detail catalog; D-28). This is the `read_diagnostics_level` / `ConfigRejection::Unreadable` surface named in the folded CI verdict.

## Anti-patterns to avoid
- NEVER use a bare `#[instrument]`, or `#[instrument(err)]` / `#[instrument(ret)]` on serde- or content-bearing functions (per obs-plan §11 Spans / Traces).
- NEVER print an anyhow chain that still holds a serde_json / serde_path_to_error source to stderr, `--json`, MCP `isError`, Problem Details or `error.data`. NEVER let the default panic hook run (per obs-plan §11 Error Reporting).
- NEVER enable veil `toggle`. NEVER let a config, env or flag value widen the allow-list. NEVER put paths or pids in `detail` codes (per obs-plan §11 PII Scrubbing).

## Contract bindings
- **obs ↔ security:** obs owns the mechanism: `skip_all`, veil and the `toggle` ban, fixed `Display`s, and detail routing. Security owns the NEVER-log floor list, security-plan §logging-redaction-wire (per obs-plan §3 Bootstrap phases → Ownership).
- **obs ↔ tests:**
  - The secret-scan test body and canary value are tests-owned. The scan asserts the canary never appears in a home-level `diagnostics/*.ndjson`; it may appear only in `detail-*.ndjson` (per obs-plan §8 Integration points #6).
  - The G4 check body is tests-owned. The schemas it checks against are obs-owned: `schemas/diag-line.v1.json` rejects `chain` on home-level lines, and `schemas/diag-detail.v1.json` admits it (per obs-plan §9 G4; D-30).
  - The harness `logs` glob covers `instances/*/diagnostics/detail-*.ndjson` (per obs-plan §3 Snapshot / paste-to-AI integration).
- **obs ↔ CI gates (OUT, route entry "Observability gates"):** G1, the secret-scan step, `disallowed-macros` and the print bans verify this chunk's floor afterwards. The code this chunk lands must already be clean against them (per obs-plan §3 Merged phase position; §9 Gate commands).

## Acceptance criteria contributions
- Every `#[instrument` in product code at HEAD matches the G1 form. `rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .` exits 1, and no instrument carries `err` / `ret` (per obs-plan §9 G1; §11 Spans / Traces).
- **Serde error case** (per obs-plan §7 before_send scrubbing pattern layers 2–3; §3 Logging stack → Sink; D-30). A forced dispatch error with a serde / serde_path_to_error source and a resolved instance must produce all of:
  - stderr exactly `error: internal error`, exit 1;
  - a codes-only home-level `process-exit{detail:"internal-error"}` with no `chain` key;
  - one `detail-cli.ndjson` line with `chain:[...]` that validates against `schemas/diag-detail.v1.json`.
  - With no instance resolved, no detail file is created and nothing else is written.
- **Canary check** (per obs-plan §8 Integration points #6; §11 PII Scrubbing): a synthetic canary embedded in an error source or payload appears in no home-level `diagnostics/*.ndjson` and no stderr / `--json` output. It appears only in `detail-*.ndjson`. No floor secret (token, `Cookie`, `?t=`, `CLAUDE*` value) appears in any file, detail files included.
- cargo-mutants reports zero surviving mutants in `src/obs.rs` on the CI runner (ubuntu included). This covers the `read_diagnostics_level` NotFound guard: the `Unreadable` branch must be reached by an OS-independent test (per obs-plan §10 Build / deploy failure conditions "A surviving cargo-mutants mutant in obs code"; §9 Pipeline integration → Mutation).

## Relevant amendment history
- **2026-09-24-diagnostics-plane** (§3 Logging stack, §8 Default-deny + Detail-file scope, §11 Logs):
  - What changed:
    - `obs_event!` attaches `event` + `process` + `instance`. `corr` is a caller-supplied typed field with no macro arm.
    - Default-deny is enforced by a top-level `unevaluatedProperties: false`, not per-event `additionalProperties`.
    - `diag-detail.v1.json` is self-contained, with the event enum inlined.
  - Why: the chunk's shipped mechanism disproved the plan's claims. The `corr`-required tightening was CARRIED to the wrapper-channel chunk, not this one.
  - Bearing on this chunk: detail lines added here must satisfy that self-contained, `unevaluatedProperties`-closed schema. Any content field beyond `panic_payload` / `backtrace` / `chain` / `drift_report` needs a schema and plan amendment.
- **2026-09-24-three-os-ci-headless-harness-skeleton** (§3 obs-ci-gate-wire, §11 Logs): the fake agent (`src/bin/viola-fake-agent.rs`, a root-package bin) is exempt from print bans through a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`. Why: lints are per package, so a separate table is impossible. Bearing on this chunk: that bin is a stdout/stderr sink at HEAD that is exempt by design. Scope the "every sink at HEAD honours the floor" claim to product paths accordingly.
