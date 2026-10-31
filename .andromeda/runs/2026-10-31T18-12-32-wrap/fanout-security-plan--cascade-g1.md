proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 99
    section: '## Threat Model Summary'
    change: 'Replace «a refusal is `instance-unreachable` with `detail: null`, no frame and no line» with «a refusal is `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line».'
    sidecar: 'boundary-mcp: a strict-modes refusal on an MCP frame tool now names its cause, `strict-modes-failed`, in `detail`.'
    rationale: 'Report Retired names the exit-21 document''s `detail: null`; Symbols / APIs, MCP, has `frame` read a refused `check_instance` as `StrictModes`, no frame and no line; Expected amendments names this line.'
    basis: .andromeda/security-plan-detail/threat-model-summary.md:99
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Replace «`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":null}`» with «`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":"<cause>"}`, the cause `without_endpoint`''s reading, `instance-dead` or `unwrapped-name`».'
    sidecar: 'auth, client verification: an MCP frame tool with no live endpoint now returns `without_endpoint`''s cause in `detail`.'
    rationale: 'Report Retired names `detail: null`; Symbols / APIs, MCP: no `live_endpoint` is `without_endpoint`''s reading. The report lists this line, doc-null @c561, changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'In the **On mismatch** clause, replace «MCP returns `instance-unreachable` with `detail: null` and writes no line» with «MCP returns `instance-unreachable` with `detail` `server-verify-failed` and writes no line».'
    sidecar: 'auth, client verification: an MCP server-check mismatch now reports `server-verify-failed` in `detail`.'
    rationale: 'Report Retired names `detail: null`; Symbols / APIs, MCP: a failed `server_verified` is `ServerVerify`, no frame and no line. The report lists this line, detail-null @c1115, changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'In the **On a strict-modes refusal** clause, replace «(under `--json` the `instance-unreachable` document, `detail: null`)» with «(under `--json` the `instance-unreachable` document, `detail` `strict-modes-failed`)».'
    sidecar: 'auth, client verification: a CLI frame verb''s strict-modes document now names `strict-modes-failed` in `detail`.'
    rationale: 'Report Symbols / APIs: `refuse_untrusted` writes `StrictModes` through `unreachable_document`. The report lists this line, detail-null @c1596, changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'In the **On a strict-modes refusal** clause, replace «MCP returns `instance-unreachable` with `detail: null`.» with «MCP returns `instance-unreachable` with `detail` `strict-modes-failed`.».'
    sidecar: 'auth, client verification: an MCP strict-modes refusal now names `strict-modes-failed` in `detail`.'
    rationale: 'Report Symbols / APIs, MCP: a refused `check_instance` is `StrictModes`. The report lists this line, detail-null @c1688, changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'In the board''s refusal, replace «`--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty» with «`--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty».'
    sidecar: 'auth, `~/.viola/`: the board''s whole refusal under `--json` now names `strict-modes-failed` in `detail`.'
    rationale: 'Report Symbols / APIs: `src/cmd/list.rs`''s `refused_document` builds `strict-modes-failed`, the board''s only refusal. The report lists this line, doc-null @c89, changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:36
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'In the `link` / `unlink` and six frame verbs'' refusal, replace «`--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty» with «`--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` on stdout with stderr empty».'
    sidecar: 'auth, `~/.viola/`: the frame verbs'' strict-modes document now names `strict-modes-failed` in `detail`.'
    rationale: 'Report Symbols / APIs: `refuse_untrusted` writes `StrictModes` through `unreachable_document`. The report lists this line, doc-null @c1480, changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:36
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'In the `mcp` clause, replace «a refusal returning `instance-unreachable` with `detail: null`, no frame and no line» with «a refusal returning `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line».'
    sidecar: 'auth, `~/.viola/`: an MCP frame tool''s strict-modes refusal now names `strict-modes-failed` in `detail`.'
    rationale: 'Report Symbols / APIs, MCP: a refused `check_instance` is `StrictModes`. The report lists this line, detail-null @c2116, changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:36
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 73
    section: '## Authentication & Authorization'
    change: 'Replace «under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty» with «under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` on stdout with stderr empty».'
    sidecar: 'auth, the CLI refusal: a server-check mismatch under `--json` now names `server-verify-failed` in `detail`.'
    rationale: 'Report Symbols / APIs: the exit-21 document now carries its cause and `server-verify-failed` is one of the five; Expected amendments names this line.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:73
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 73
    section: '## Authentication & Authorization'
    change: 'Replace «or under `--json` the same `instance-unreachable` document, opening nothing under the home» with «or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, opening nothing under the home».'
    sidecar: 'auth, the CLI refusal: the strict-modes document is no longer the same as the mismatch document; each names its cause.'
    rationale: 'Report Symbols / APIs: one document per cause, `refuse_untrusted` writing `StrictModes` through `unreachable_document`, so «the same document» no longer holds.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:73
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 70
    section: '## Authentication & Authorization'
    change: 'Replace «(`live_endpoint`; none is exit 21 with the not-running hint)» with «(`live_endpoint`; none is exit 21 through `Unreachable::without_endpoint`, `instance-dead` with the not-running hint or `unwrapped-name` with the unwrapped-name hint)».'
    sidecar: 'auth, `viola link` / `unlink`: no live endpoint now reads by the instance directory, dead or unwrapped, each with its own hint.'
    rationale: 'Report Symbols / APIs: the no-endpoint arms read `Unreachable::without_endpoint(&instance_dir)`, `src/cmd/link.rs:103-105` among them with `unlink` through `link_call`; `hint_cause` gives `not-running` and `unwrapped-name` apart.'
    basis: src/cmd/link.rs:103-105
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 18
    section: '## Authentication & Authorization'
    change: 'After «without writing any frame.» add: «**Socket-directory failure at bind:** a failure at `bind_endpoint`''s `create_private_dir` has no refusal code: it is an anyhow error at the catch site, and its code is owed to the route entry "Unix endpoint and home hardening", by the delegate''s word, PROVISIONAL (chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I3`).»'
    sidecar: 'auth, IPC access control (Unix): records that a socket-directory failure at bind has no refusal code and that its code is owed, PROVISIONAL.'
    rationale: 'Report Spec claims disproved: `bind_endpoint`''s `create_private_dir` failure is an anyhow error at the catch site; the Beyond-the-plan row (`inputs#I3`) leaves the code to "Unix endpoint and home hardening".'
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 68
    section: '## Authentication & Authorization'
    change: 'Add a row after this entity: «- **Requirement:** IPC client-side server verification, the no-endpoint reading» «  - **Implementation:** With no `LiveEndpoint`, `viola_state::liveness::without_endpoint` tells a dead instance from an unwrapped name by `symlink_metadata` on the instance directory alone: a directory reads `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`. It opens nothing and writes no line, and the strict-modes check stays first at every caller. The reading is the delegate''s word, PROVISIONAL (chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q2).» «  - **Stack reference:** `viola-state` liveness (`without_endpoint`), called by the root bin''s `src/cmd/client.rs` (`Unreachable::without_endpoint`) and `crates/viola-mcp/src/tools.rs` (`frame`)»'
    sidecar: 'auth, client verification: adds `without_endpoint`, the no-endpoint reading of dead against unwrapped, PROVISIONAL.'
    rationale: 'Report Symbols / APIs: `without_endpoint`, its reading, that it opens nothing and writes no line, its strict-modes-first order and its two callers; Decisions: `inputs#I2` Q2, the delegate''s word, PROVISIONAL.'
    basis: crates/viola-state/src/liveness.rs:107-115
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 74
    section: '## Authentication & Authorization'
    change: 'Add a row after this entity: «- **Requirement:** IPC client-side server verification, the MCP refusal» «  - **Implementation:** `viola mcp`''s `frame` maps a refused `check_instance` to `strict-modes-failed`, no `live_endpoint` to `without_endpoint`''s reading and a failed `server_verified` to `server-verify-failed`, each the `instance-unreachable` result with its cause in `detail`, with no frame and no line on any of them, `isError` the only record (chunk 2026-10-31-exit-cause-code-catalogue, `inputs#I2` Q4).» «  - **Stack reference:** `crates/viola-mcp/src/tools.rs` (`frame`); `crates/viola-mcp/src/result.rs` (`unreachable`)»'
    sidecar: 'auth, client verification: adds the MCP refusal row, each failed check its cause, no frame and no line.'
    rationale: 'Report Symbols / APIs, MCP: `tools.rs` `frame`''s three mappings, no frame and no line, `isError` the record; Coverage: no line by `inputs#I2` Q4.'
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 79
    section: '## Authentication & Authorization'
    change: 'Replace «the per-user socket directory loosened to 0755 giving exit 21 and the client euid check, are a CARRY» with «the per-user socket directory loosened to 0755 giving exit 21, the client euid check and the socket-directory refusal''s code (left out of chunk 2026-10-31-exit-cause-code-catalogue by the delegate''s word, PROVISIONAL, `inputs#I3`), are a CARRY».'
    sidecar: 'auth, the macOS cases: the list owed to "Unix endpoint and home hardening" gains the socket-directory refusal''s code, PROVISIONAL.'
    rationale: 'Report Cross-project claims, the Beyond-the-plan row, and Outcome on `v1-18`: the socket-directory cause and macOS''s server check are left unmet by name, with "Unix endpoint and home hardening".'
note: '`.andromeda/security-plan.md:413` still reads `detail: null` but is the broad line of error-handling, group 2''s; D-security-input, D-security-auth and D-security-deps found no drift (closed causes, no auth or crypto change, no dependency added); `authentication-authorization.md:27` and `:36` stand over 2 000 chars as the report lists, and these edits do not split them.'
