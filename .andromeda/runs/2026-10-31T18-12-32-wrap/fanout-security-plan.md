proposals:
  - detector: D-retired
    severity: warning
    section: Error Handling
    change: "In the External responses bullet on MCP `isError` results, replace `instance-unreachable` with `detail: null` by `instance-unreachable` with `detail` its closed cause word (`strict-modes-failed` | `instance-dead` | `unwrapped-name` | `endpoint-lost` | `server-verify-failed`), the rest of the bullet unchanged."
    sidecar: "Error Handling: the MCP `instance-unreachable` result names its cause in `detail`, one of five closed words, in place of `detail: null` (chunk 2026-10-31-exit-cause-code-catalogue)."
    rationale: "The report's Retired bullet names the exit-21 document's `detail: null`; its Symbols / APIs says the document now carries the cause, with MCP's `isError` result the same less `v` and the text still `refused: instance-unreachable`, and its Expected amendments lists security-plan.md:413. No other passage of the body states the null: security-plan.md's other `instance-unreachable` mention, in the Unix IPC access control row, names only the code, which stands."
    basis: "crates/viola-core/src/lib.rs:181-190"
note: "D-security-input, D-security-auth and D-security-deps found no drift: the report adds no unvalidated input surface (causes are a closed enum with no serde path, `<name>` stays the parsed ViolaName, `json_asked` only compares argv for `--json`), no auth library or secret source, and its Dependencies bullet reads none added or bumped. The other Retired items have no occurrence in the body; the detail files' sites listed in the report's Expected amendments are the cascade's."
