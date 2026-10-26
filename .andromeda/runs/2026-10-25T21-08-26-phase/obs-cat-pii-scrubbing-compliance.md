# obs · PII Scrubbing & Compliance — category read

## Carried
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) The pipe's name and the owning user's SID (or account name) that the DACL tests read back are path- and username-class data: a home-level line, an external error or a refusal written by the admission work or its tests must stay path-free, with such values reaching only an instance detail file.
  > | Absolute paths (OS username = only PII in scope), internal type names | Medium | scrub-required in external output; permitted in `diagnostics/` but obs keeps home-level lines path-free | `file` fields are basenames; `panic_location` workspace-relative; `with_file(false)`; paths only in detail files |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18) Any line the listener-mode, first-instance or squat work adds must never log the snapshot `endpoint` (the pipe name); only the closed `endpoint_kind` may appear.
  > | Config values (thresholds), snapshot `endpoint` / `pinned_bin`, ledger versions | Low confidentiality | OK structured; path-valued ones follow the Medium rule | `cli_version`, thresholds logged; `endpoint` / `pinned_bin` never logged (only `endpoint_kind`) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) A squatted-name or second-listener failure may name the holder's pid only as a typed field, never inside a `detail` code.
  > | `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21) The `squatted-name` refusal and any rejection the listener reports stay closed kebab-case codes.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28) The new Windows tests' homes and their `diagnostics/` files fall under the CI secret scan before any upload; a test that writes a home outside the scanned root escapes it.
  > 5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:30) Any input the new tests (remote-client, second-listener, foreign-principal open, and the CARRY's `send_budget_paused` case) feed through a wrapper is synthetic and carries the canary, which must never reach a home-level file.
  > - Every E2E and integration input is synthetic and embeds a fixed canary string.
  > - The scan also asserts that the canary never appears in a home-level `diagnostics/*.ndjson`. It may appear only in `detail-*.ndjson`.
- (.andromeda/obs-plan.md:190) A channel-server change (the listener's admission path) keeps channel lines free of `params` and result bodies, and the self-reported `from` is never logged as an identity.
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan.md:195) The CARRY's acceptance criterion (the `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` log capture) reads a `send-refused` line that must carry no send text, whichever way its capture is fixed.
  > - **Required telemetry signal:** `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
- (.andromeda/obs-plan.md:311) The squatted-endpoint start refusal, kept as is (exit 1, `squatted-name`), writes exactly the two fixed stderr lines with no path or pid.
  > a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid)
- (.andromeda/obs-plan.md:345) On `run`'s start path, where the bind and the squat refusal sit, the endpoint name reaches no span and a failure's `detail` holds no path or pid.
  > - Never the endpoint name, the `pinned_bin` path or env values in a span, and never the sideload companions' paths or hashes; the child's `--settings` argument, the statusline source path and the command appear in no line or span; a sideload outcome is never a refusal or an exit change; a failure's `detail` carries no path or pid;
- (.andromeda/obs-plan.md:557) The body's broad rule over this chunk's surfaces: home-level lines path-free and the endpoint never logged.
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged;
- (.andromeda/obs-plan.md:576) Any field the chunk adds to a line (e.g. a pipe-mode or remote-reject reading) must be named in the §6 catalog and pass G4 against `schemas/diag-line.v1.json`, or it is a build failure.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:735) The ban the squat and listener-failure codes must honour.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.

## Bindings
- **endpoint never logged ↔ security-plan §Authentication & Authorization (Windows pipe SDDL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`)**: the SID and the pipe name the security clause builds the listener from are values obs keeps out of every home-level line, span and refusal; only `endpoint_kind` and closed codes cross (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18), (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17), (.andromeda/obs-plan.md:557)
- **closed refusal codes, no path or pid ↔ the body's §3 start-refusal writer and architecture §Conventions (refusals, exit codes)**: `squatted-name`, exit 1 stand unchanged and its stderr pair stays fixed (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21), (.andromeda/obs-plan.md:311), (.andromeda/obs-plan.md:735)
- **`send-refused` carries no text ↔ the chunk's CARRY acceptance criterion (test-plan's log-capture helper)**: the case's fixed capture still asserts a text-free `send-refused` line (.andromeda/obs-plan.md:195)
- **default-deny ↔ body §6 catalog and §9 G4 (`schemas/diag-line.v1.json`)**: a new field enters the catalog and schema or is not logged (.andromeda/obs-plan.md:576)
- **secret scan and synthetic canary ↔ body §9 step order and test-plan §3 (homes under `target/e2e-home/`)**: the Windows leg's new test homes are scanned before `diag-<os>` uploads (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28), (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:30)
