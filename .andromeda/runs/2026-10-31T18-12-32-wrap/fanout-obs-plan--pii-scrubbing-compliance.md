proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'In the strict-modes refusal item, the --json document now reads exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` in place of the `"detail":null` form; the rest of the clause (no pid, path or endpoint, no line, the fixed instance pair) stands.'
    sidecar: 'PII verification: the strict-modes refusal document of the six frame verbs names its cause, `detail` `strict-modes-failed`, in place of null (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Retired names the exit-21 document `detail: null`; Symbols / APIs: every cause prints `detail` `<cause>` and `refuse_untrusted` writes `StrictModes` through `unreachable_document`.'
    basis: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'In the same item, the `viola mcp` frame tools refused by the same check return `instance-unreachable` with `detail: "strict-modes-failed"` in place of `detail: null`, and still write no line.'
    sidecar: 'PII verification: the MCP frame tools refused by the strict-modes check return `detail` `strict-modes-failed` in place of null, no line (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Retired, the exit-21 `detail: null`; Symbols / APIs, MCP: `tools.rs` `frame`, a refused `check_instance` is `StrictModes`, no frame and no line, `isError` the record.'
    basis: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'New item: - An exit-21 cause word carries no pid, path or upstream text: the CLI `--json` `detail` and the MCP `isError` body (the same less `v`) hold one of the five closed words of `viola_core::UnreachableCause` (`strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`), no serde path, each written by the two clients; MCP writes no line for any of them; asserted by `every_code_is_a_code_word`, `tests/cross_exit_causes.rs`, `tests/contract_error_bodies.rs`, `binary(handlers)` and `binary(mcp_tools)`.'
    sidecar: 'PII verification: new item, the five exit-21 cause words are closed and carry no pid, path or upstream text on CLI `--json` and MCP (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Coverage of new surfaces, exit-21 `detail` causes (PII no path, pid or upstream text) and MCP `detail` causes; Symbols / APIs, `viola_core::UnreachableCause`.'
    basis: crates/viola-core/src/lib.rs:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'New item: - The `unwrapped-name` and `endpoint-lost` hints carry no read-back text: `<name>` is the parsed `ViolaName`, the line plain ASCII and last on stderr; asserted by `send_hint_is_the_design_string` and `tests/cross_exit_causes.rs`.'
    sidecar: 'PII verification: new item, the two new exit-21 hints print only the parsed `ViolaName` (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Coverage of new surfaces, the `unwrapped-name` and `endpoint-lost` hints: validation `<name>` the parsed `ViolaName`, PII checked, a11y plain ASCII, last on stderr.'
    basis: src/human/hints.rs:50-55
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'New item: - The `revive --json` start documents and the `cli` catch site document `{"v":1,"error":"internal-error","detail":null}` under `--json` hold closed codes only and leave stderr empty; an anyhow error there prints `internal-error` and the chain stays with the catch site, never in the document; asserted by `cli_verbs_print_the_internal_error_document_under_json` and `tests/cross_exit_causes.rs`.'
    sidecar: 'PII verification: new item, the `revive --json` start documents and the catch site document under `--json` hold closed codes only (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Coverage of new surfaces, `revive --json` start documents and the catch site document (validation closed codes, PII checked, stderr empty); Symbols / APIs, `viola revive --json` and the `cli` catch site under `--json`.'
    basis: src/cmd/mod.rs:99-102
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'New item: - The `process-exit` codes `verify` writes under a parsed `VIOLA_NAME` (`batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`) are closed words admitted by the enum in `schemas/diag-line.v1.json`; the fixture `<file>` enters no code; asserted by `tests/cli_verify.rs` and `tests/cross_exit_causes.rs`.'
    sidecar: 'PII verification: new item, the `verify` refusal codes on `process-exit` are closed and the fixture `<file>` enters no code (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report Coverage of new surfaces, `verify` `process-exit` codes (PII the fixture `<file>` enters no code); Symbols / APIs, `verify` codes; Schema / config, the `process-exit` `detail` enum.'
    basis: schemas/diag-line.v1.json:78
note: 'D-obs-instrumentation, D-obs-stack and D-obs-pii fire nothing in this file: the chunk adds no raw user data to a line and no telemetry dependency; obs-plan.md:361 holds the retired doc-null form too, outside this file.'
