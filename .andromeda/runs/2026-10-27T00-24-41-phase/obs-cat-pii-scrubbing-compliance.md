# obs · PII Scrubbing & Compliance — category read

## Carried
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18) Any line the pin-race fix adds on `run`'s start or `verify`'s `pin_exe` (a wait, a retry, a same-bytes acceptance, a failed replace) must never log the pinned path `pinned_bin`, and a `bin/<key>/` hash key, if logged, is a Low structured value only under the path rule.
  > | Config values (thresholds), snapshot `endpoint` / `pinned_bin`, ledger versions | Low confidentiality | OK structured; path-valued ones follow the Medium rule | `cli_version`, thresholds logged; `endpoint` / `pinned_bin` never logged (only `endpoint_kind`) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) The concurrent-pin and `plugin/<key>/` rewrite paths (`replace_private_shared` → `replace_private`, the temp file and its rename) must keep home-level lines path-free: no temp-file, `bin/<key>/` or `plugin/<key>/` path in a home-level line, paths only in detail files.
  > | Absolute paths (OS username = only PII in scope), internal type names | Medium | scrub-required in external output; permitted in `diagnostics/` but obs keeps home-level lines path-free | `file` fields are basenames; `panic_location` workspace-relative; `with_file(false)`; paths only in detail files |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) A pid read in the race (the other start's, the agent's hook spawn, the `ETXTBSY` reproduction) may be logged only as a typed field, never inside a `detail` code.
  > | `pid`, `child_pid`, `agent_session_id`, `started_at`, budget percentages, `resets_at` | Low | OK to log structured | Logged as typed fields; never inside `detail` codes (Founder Direction 4) |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) Any new outcome code for the pin race (or a kernel error the fix reads, such as the hypothesised `ETXTBSY`) must be a closed kebab-case enum value, never raw OS error text.
  > | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums |
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:29) The two-sided race witness's homes and harness capture stay inside the secret scan run before any CI upload.
  > 5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:36) The race reproduction (the unstamped two-instance harness boot or a narrower concurrent-pin control) must use synthetic input only, or first drop the detail and harness uploads.
  > - Any non-synthetic input source must first drop `detail-*.ndjson` and `target/agent-run/` from the upload paths.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:44) If P4 reads the reproducible-build measurement on CI's three OSes, any artifact it uploads (build hashes, a section or note listing) is admissible only once its content has been read for host paths, and is otherwise scanned before upload or dropped.
  > The guard stays binding: a member that could carry a host path or log content is scanned before upload, or dropped (operator ratification 2026-09-24, logged for founder review).
- (.andromeda/obs-plan.md:345) `run`'s start pin (`pin_bytes`) and the Windows `pin_companions` arm sit on this scenario: no span or line may carry the `pinned_bin` path or a companion's path or hash, and a failure's `detail` carries no path or pid.
  > - Never the endpoint name, the `pinned_bin` path or env values in a span, and never the sideload companions' paths or hashes; the child's `--settings` argument, the statusline source path and the command appear in no line or span; a sideload outcome is never a refusal or an exit change; a failure's `detail` carries no path or pid; the child's `process-exit` comes from the handle-wait thread, never on reader EOF.
- (.andromeda/obs-plan.md:215) The hash-before-reuse rule stands, so the fix must keep the pinned-exe hash-mismatch refusal telemetry as it is and keep `diagnostics/` 0600 in 0700.
  > - **Required telemetry signal:** `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL), plus refusal telemetry for pinned-exe hash mismatches and ignored symlinks;
- (.andromeda/obs-plan.md:557) The body's broad rule for this chunk's surface: home-level lines path-free and `pinned_bin` never logged.
  > Home-level lines stay path-free; `endpoint` / `pinned_bin` are never logged; `claude agents --json` rows are never logged verbatim, only `parse-rejected` counts; low-class metadata is logged as typed fields, never inside `detail` codes.
- (.andromeda/obs-plan.md:576) Any new field or `detail` value for the pin race must first be named in the §6 catalog and admitted by `schemas/diag-line.v1.json`, or G4 fails the build.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:735) No path or pid inside any new `detail` code.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.

## Bindings
- **`pinned_bin` never logged ↔ body §4 scenario `viola run` start sequence to child spawn**: the §8 table row (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:18) and the scenario's span rule (.andromeda/obs-plan.md:345) bind the same pin surface; security.md's Home rule (re-hash before reuse, never overwritten) stays unchanged beside both.
- **New pin-race field or code ↔ body §6 Log Coverage catalog and §9 gate G4**: default-deny (.andromeda/obs-plan.md:576) means the §6 catalog and `schemas/diag-line.v1.json` both gain any added value; closed enum (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22), no pid or path in it (.andromeda/obs-plan.md:735).
- **Race witness ↔ tests §3 secret scan and body §9 step order**: synthetic input and the pre-upload scan (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:29, .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:36) bind the reproduction rig wherever it runs in CI.
- **Reproducible-build measurement on CI (P4's) ↔ body §9 artifact handling**: an uploaded measurement member is read for host paths first, or scanned or dropped (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:44).
