proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 58
    section: "## Threat Model Summary"
    change: After the sentence that input validation is limited to JSON-RPC parsing, the `v` check (`-32602`) and the `ViolaName` nutype, add «That limit is the wrapper's read of a frame. A client's read of the answer is screened on its own, `reply_of` and `of_reply` taking a `refusal` or a string `detail` only as a code word (`viola_core::is_code_word`) and a fault only through `viola_channel::sanitised_fault`, so a process answering on the endpoint puts no free text or path in a CLI `--json` document or an MCP result; a code word outside the closed list stays visible by design.»
    sidecar: threat-model-summary, boundary-ipc's trust boundary gains the client's screen of a peer's reply (`is_code_word`, `sanitised_fault`) beside the wrapper's frame validation.
    rationale: The report's Coverage of new surfaces lists `reply_of` / `of_reply`'s screen of a peer's reply as validation by `is_code_word` and `sanitised_fault`, and its Outcome holds that a peer's message, data, refusal or detail reaches no document or text, a code word outside the closed list visible by design; the entity's «limited to» no longer covers the whole boundary.
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 30
    section: "## Authentication & Authorization"
    change: After «logs one `release-from-driver` line (`from_trust:"self-reported"`)» add «; the answer's `channel-response` line also carries `detail:"release-from-driver"`, from `ProtocolError::cause`, on the server's and the client's line, level WARN, no param name or value on it, a `from` of another type reading `invalid-params` there; the wire body is unchanged, and a client keeps it through `viola_channel::sanitised_fault` only for `data.reason` exactly `release-from-driver`».
    sidecar: authentication-authorization, driver-originated `release` names the `release-from-driver` word on `channel-response` and the client's `sanitised_fault` read of that body.
    rationale: The report's Symbols / APIs has `ResponseLine`'s `Drop` write `detail = fault.and_then(ProtocolError::cause)` on both lines, level unchanged (WARN), `sanitised_fault` keep the body for `data.reason` exactly `release-from-driver`, and `fault_of` read `data` for `-32602`; its Schema entry closes `detail` to the three words and leaves the wire form unchanged.
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 19
    section: "## Input Validation"
    change: Add, after the `hook.event` re-validation line and within this row's What to validate, one line «**A reply, read by a client:** `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`) take a `refusal` or a string `detail` only as a code word (`viola_core::is_code_word`), and a fault only through `viola_channel::sanitised_fault` (§ Error Handling). **The refusals on the log:** the three `-32602` faults read `channel-response`'s `detail` `unsupported-version`, `invalid-params` and `release-from-driver`, on both sides, level WARN, no param name or value; every other `channel-response` line has no `detail`.»
    sidecar: input-validation, Channel frames gains the client's screen of a peer's reply and the three closed `detail` words of its `-32602` refusals.
    rationale: The report's Coverage of new surfaces lists `reply_of` / `of_reply`'s screen and `channel-response.detail` as validated surfaces, the first by `is_code_word` and `sanitised_fault`, the second a closed enum with no param value or name; the row states only the wrapper's checks. A new line, since the row's first line stands near the 2 000-char bound.
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 31
    section: "## Input Validation"
    change: In On the cut, read the close as «and returns what the session gave, exit 0 for an initialized session; when the cut lands before `initialize` completes `serve` returns `ServeError::Initialize` and the `viola mcp` process exits 1, both streams empty, with that one line (`src/main.rs` `exit_code`, `Role::Mcp` + `Failed`), 11 being only the unit child's own code (`serve_child_entry`)», with «(exit 11)» gone.
    sidecar: input-validation, MCP stdin's cut before `initialize` exits the process 1, not 11, which is the unit child's own code.
    rationale: The report's Spec claims disproved measures it, `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` reading exit 1, both streams empty, one `parse-rejected{mcp-stdin, oversize, count:1}`, and `src/main.rs` `exit_code` giving `Role::Mcp` + `Failed` exit 1; pattern `exit-eleven` names this site.
    basis: .andromeda/security-plan-detail/input-validation.md:31
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 74
    section: "## Input Validation"
    change: Name the refusal `AgentError::BatchScriptChild`, in place of `Refusal::BatchScriptChild`, keeping «closed» and the rest of the sentence; the exit 1, the `process-exit` line and the two fixed stderr lines stay as written.
    sidecar: input-validation, Child executable resolution's refusal is `AgentError::BatchScriptChild`, `viola_agent_claude::Refusal` retired with its two messages unchanged.
    rationale: The report's Retired bullet names `viola_agent_claude::Refusal`, its Symbols / APIs folds the variant into `AgentError` with messages unchanged, and pattern `refusal-enum` names this site as changed.
    basis: .andromeda/security-plan-detail/input-validation.md:74
note: F1 to F13 are all error-handling, no picked entity of this group's categories carries one, so none is proposed here; the Input Validation broad sentence that shared validators live in `viola-core` now meets `sanitised_fault` shared from `viola-channel`, for the orchestrator to weigh with error-handling; threat-model-summary's line calling a driver-originated `release` still open predates this session and was left alone.
