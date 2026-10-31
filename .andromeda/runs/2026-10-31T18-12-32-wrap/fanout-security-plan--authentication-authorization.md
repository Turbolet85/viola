proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: 'At @c561, the MCP no-endpoint arm, replace `else {"error":"instance-unreachable","detail":null}` with `else {"error":"instance-unreachable","detail":"<cause>"}`, the cause `viola_state::liveness::without_endpoint`''s reading, `instance-dead` or `unwrapped-name`.'
    sidecar: MCP's no-endpoint result names its cause, `instance-dead` or `unwrapped-name`, in place of `detail` null (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Retired bullet names the exit-21 document's `detail` null; Symbols / APIs, MCP, has no `live_endpoint` read as `without_endpoint`'s reading; report Counts places doc-null at `authentication-authorization.md:27` @c561; the cause in `detail` is the founder's word, `inputs#I2` Q1, the unwrapped reading the delegate's, `inputs#I2` Q2, PROVISIONAL.
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: 'At @c1115, the **On mismatch** clause, replace `MCP returns instance-unreachable with detail: null and writes no line` with MCP returning `instance-unreachable` with `detail` `server-verify-failed` and writing no line.'
    sidecar: MCP's server-verification refusal reads `detail` `server-verify-failed` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Retired bullet and Symbols / APIs, MCP, a failed `server_verified` is `ServerVerify`, no frame and no line; report Counts places detail-null at `authentication-authorization.md:27` @c1115.
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: 'At @c1596, the **On a strict-modes refusal** clause, replace `under --json the instance-unreachable document, detail: null` with the `instance-unreachable` document under `--json` with `detail` `strict-modes-failed`.'
    sidecar: The CLI frame verbs' strict-modes document reads `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Retired bullet and Symbols / APIs, `refuse_untrusted` writes `StrictModes` through `unreachable_document`; report Counts places detail-null at `authentication-authorization.md:27` @c1596.
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: 'At @c1688, the same strict-modes clause, replace `MCP returns instance-unreachable with detail: null` with MCP returning `instance-unreachable` with `detail` `strict-modes-failed`.'
    sidecar: MCP's strict-modes refusal reads `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Retired bullet and Symbols / APIs, MCP, a refused `check_instance` is `StrictModes`; report Counts places detail-null at `authentication-authorization.md:27` @c1688.
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: "## Authentication & Authorization"
    change: 'At @c89, the whole-board refusal, the exact `--json` document becomes `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`.'
    sidecar: The board's refusal document reads `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Retired bullet and Symbols / APIs, `src/cmd/list.rs`'s `refused_document` builds `strict-modes-failed`, the board's only refusal; report Counts places doc-null at `authentication-authorization.md:36` @c89.
    basis: .andromeda/security-plan-detail/authentication-authorization.md:36
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: "## Authentication & Authorization"
    change: 'At @c1480, the `link` / `unlink` and six frame verbs'' `refuse_untrusted` refusal, the exact `--json` document becomes `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`.'
    sidecar: The `refuse_untrusted` document reads `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Retired bullet and Symbols / APIs, `refuse_untrusted` writes `StrictModes` through `unreachable_document`; report Counts places doc-null at `authentication-authorization.md:36` @c1480.
    basis: .andromeda/security-plan-detail/authentication-authorization.md:36
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: "## Authentication & Authorization"
    change: 'At @c2116, `mcp`''s frame tools, replace `a refusal returning instance-unreachable with detail: null` with a refusal returning `instance-unreachable` with `detail` `strict-modes-failed`, still no frame and no line.'
    sidecar: The `viola mcp` frame tools' strict-modes refusal reads `detail` `strict-modes-failed` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Retired bullet and Symbols / APIs, MCP, a refused `check_instance` is `StrictModes`; report Counts places detail-null at `authentication-authorization.md:36` @c2116.
    basis: .andromeda/security-plan-detail/authentication-authorization.md:36
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 73
    section: "## Authentication & Authorization"
    change: 'The mismatch document under `--json` becomes `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`.'
    sidecar: The CLI refusal on a failed server check reads `detail` `server-verify-failed` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Retired bullet and Symbols / APIs, the closed cause `server-verify-failed` and `Unreachable::ServerUnverified`; Expected amendments names `authentication-authorization.md:73`.
    basis: .andromeda/security-plan-detail/authentication-authorization.md:73
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 73
    section: "## Authentication & Authorization"
    change: 'In the strict-modes clause, replace `under --json the same instance-unreachable document` with the `instance-unreachable` document under `--json` with `detail` `strict-modes-failed`, since the two refusals no longer share one document.'
    sidecar: The frame verbs' strict-modes document is told apart from the server-check document by its `detail` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Symbols / APIs, one document per cause, `refuse_untrusted` writing `StrictModes`; the word "same" on this line retires with the single exit-21 document the Retired bullet names.
    basis: .andromeda/security-plan-detail/authentication-authorization.md:73
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 70
    section: "## Authentication & Authorization"
    change: 'Replace `none is exit 21 with the not-running hint` with none being exit 21 with the hint of `without_endpoint`''s reading, the not-running hint for an instance directory, the `unwrapped-name` hint for anything else.'
    sidecar: With no live endpoint, `viola link` / `viola unlink` hint by the instance directory's reading (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Symbols / APIs, the six no-endpoint arms read `Unreachable::without_endpoint(&instance_dir)`, `src/cmd/link.rs:103-105` among them, `unlink` reaching it through `link_call`; `hint_cause()` now reads `not-running` or `unwrapped-name`; the reading is the delegate's word, `inputs#I2` Q2, PROVISIONAL.
    basis: src/cmd/link.rs:103-105
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 74
    section: "## Authentication & Authorization"
    change: 'Add a record after this line. **Requirement:** IPC client-side server verification, the cause an unreachable instance names. **Implementation:** `error` stays `instance-unreachable` and no exit moves; `detail` names one closed `viola_core::UnreachableCause` word on the CLI `--json` document and MCP''s `isError` body (the same less `v`, MCP''s text still `refused: instance-unreachable`), holding no path, pid or upstream text. `strict-modes-failed` is a refused `check_instance`, the first check. `instance-dead` or `unwrapped-name` is no `live_endpoint`, read by `viola_state::liveness::without_endpoint`, whose `symlink_metadata` reads a directory `instance-dead` and anything else (absent, a regular file, a symlink) `unwrapped-name`, opening nothing and writing no line (the unwrapped reading the delegate''s word, `inputs#I2` Q2, delegate the overseer, PROVISIONAL). A refused connect is `instance-dead`, a wrapper lost with the call open `endpoint-lost`, a mismatch or a check that cannot be made `server-verify-failed`. `pub(crate) unreachable_document` is the CLI''s one writer of the document; MCP writes no frame and no line on any of the causes, `isError` the record. The cause in `detail` is the founder''s word, `inputs#I2` Q1. **Stack reference:** `viola-core` `UnreachableCause`; `viola-state` liveness (`without_endpoint`); root bin `src/cmd/client.rs` (`Unreachable`, `unreachable_document`); `viola-mcp` `result.rs` (`unreachable`), `tools.rs` (`frame`).'
    sidecar: New record, the closed exit-21 causes and `without_endpoint`'s reading of a name with no live endpoint (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Symbols / APIs bring `UnreachableCause`, `without_endpoint` and `unreachable_document`; Counts `unreach-cause` says no master site holds them, and Expected amendments carries §Authentication & Authorization with the unwrapped reading PROVISIONAL.
    basis: crates/viola-core/src/lib.rs:181-190
note: '.andromeda/security-plan.md:413 (body, §Error Handling) still reads `instance-unreachable` with `detail: null`, outside this file; .andromeda/security-plan-detail/authentication-authorization.md:18 describes a socket-directory refusal of `run` as standing, while the report says the code holds no such refusal and calls it no master site, so no proposal is written and it is left to the directive''s socket-directory row.'
