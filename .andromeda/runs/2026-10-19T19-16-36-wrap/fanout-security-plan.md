proposals:
  - detector: D-security-input
    severity: escalate
    section: Input Validation
    change: "In the Hook stdin row add the two new data keys read from the payload under the existing take(MAX_FRAME + 1) and one-object rule: the `question` data gains per-question `option_details`, one `{description, preview}` per option parallel to `options`, each field the option's string or `null` (a missing or non-string value is `null` and never refuses the dialog; `options` keeps its `[label]` shape), and the Stop's `turn-ended` data gains `background_tasks`, the payload array's length only (elements read as `serde::de::IgnoredAny`, none read or carried; absent, `null` or a non-array is `null`, a non-array also writing the drift entry `{\"path\":\"background_tasks\",\"expected\":\"array\"}`); the option description and preview text crossing into `events.ndjson`, `hook.dialog`, CLI `wait --json` and the MCP `wait` result is a boundary widening the founder answered himself (P4 fork Q4, `inputs#I14` of chunk 2026-10-19-overseer-switch-to-viola), and `background_tasks` is a count only by his same answer."
    sidecar: "security-plan §Input Validation Hook stdin row: `question` `option_details` (description/preview, string or null) and `turn-ended` `background_tasks` (count only, IgnoredAny) recorded with their validation; the option-text crossing recorded as the founder's own P4 Q4 widening (chunk 2026-10-19-overseer-switch-to-viola, inputs#I14)."
    rationale: "Report Changes, Symbols / APIs and Schema / config: two new external-input fields parsed from hook stdin (`dialog.rs` 151-156, `hook.rs` 104-105, 147-152, 170-173) and upstream option text now carried into events, `hook.dialog`, `wait --json` and MCP `wait` (Schema / config: 'upstream text crossing a boundary: the Boundary widening the founder answered himself at P4 fork Q4'); Coverage of new surfaces states the validation (type check to null, MAX_FRAME on stdin; array length only). The Hook stdin row states neither the fields nor the ratified crossing; Expected amendments names security-plan §Input Validation / §Data Protection for the same widening."
    basis: "viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/report.md (Symbols / APIs; Schema / config; Coverage of new surfaces); crates/viola-agent-claude/src/dialog.rs:151-156; crates/viola-agent-claude/src/hook.rs:170-173"
  - detector: D-security-input
    severity: escalate
    section: Threat Model Summary
    change: "In Data classification, the dialog user-content bullet's 'Where': `question` carries questions, options and each option's description and preview text (`option_details`), and that content goes out over CLI `wait --json` and the MCP `wait` result as well as `events.ndjson` and `hook.dialog` (the founder's own P4 Q4 widening, chunk 2026-10-19-overseer-switch-to-viola); `turn-ended` additionally carries `background_tasks`, a count only, no task text."
    sidecar: "security-plan §Threat Model Summary data classification: `question` content now includes option description/preview text; `turn-ended` adds a background-task count (no content)."
    rationale: "Report Schema / config: `option_details` carries the CLI's option description / preview text into `events.ndjson`, `wait --json` and the MCP `wait`; `background_tasks` is a count and no task text crosses. The classification's 'question carries questions and options' understates what the payload now holds."
    basis: "viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/report.md (Schema / config); .andromeda/security-plan.md"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: Bootstrap phases (derive for route / setup-project) → logging-redaction-wire
    change: "In the floor's Scope bullet, name option description and preview text among the user content: 'user content (prompts, `last_assistant_message`, plan, question and option text)', the option text carried by design only in the content payloads already listed (`events.ndjson`, `wait`/`last` results CLI and MCP, `hook.dialog`) and otherwise only in `diagnostics/`."
    sidecar: "security-plan logging-redaction-wire scope: option description/preview text named as user content inside the by-design content payloads (chunk 2026-10-19-overseer-switch-to-viola)."
    rationale: "Report Expected amendments names `security-plan.md:390` (the NEVER-log floor's scope) as a candidate site of the option-text widening; Coverage of new surfaces, PII: the upstream option text lands in `events.ndjson` / `wait` / MCP `wait`, content payloads, never a log line (security-plan floor scope)."
    basis: ".andromeda/security-plan.md:390"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: Security Anti-Patterns → Code Patterns
    change: "In the R1 ban, list option text among the upstream text never interpreted as a viola command or config: 'upstream text (prompts, `last_assistant_message`, plan text, question and option text, tool `input`)'."
    sidecar: "security-plan Anti-Patterns Code Patterns R1: question option description/preview text named as upstream text (chunk 2026-10-19-overseer-switch-to-viola)."
    rationale: "Report Expected amendments names `security-plan.md:527` as the second candidate site; Schema / config calls the newly carried option text 'upstream text crossing a boundary'."
    basis: ".andromeda/security-plan.md:527"
    dependent-of: D-security-input
note: "D-security-auth and D-security-deps found no drift (no identity/token/key change; Dependencies none). Retired is none. Outside these detectors: report Spec claims disproved 3 names an omission (not a falsehood) at security-plan.md:261 — each real-CLI verify also leaves the CLI's `-home-<user>--viola-ledger-probes-<pid>` project dir (empty `memory/`, no transcript) for the print probe's cwd; the orchestrator may route it to the accepted-risk paragraph if it wants the residual stated."
