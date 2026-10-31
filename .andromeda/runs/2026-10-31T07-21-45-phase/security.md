# security extract

## Relevance
relevant: this chunk is the plan's error-sanitisation mandate applied across CLI `--json`, MCP `isError` and channel `error.data`.

## Constraints
- security-plan §Error Handling (External responses) requires every response to channel clients, CLI `--json` and MCP drivers to hold no anyhow chain, absolute path, upstream text, serde_path_to_error path or value, or internal type name; whether every verb's `--json` error document already holds to this is research's question.
- security-plan §Error Handling (External responses) requires fixed `Display` messages on the six named error enums, with path- and payload-holding variant fields excluded. `Refusal` in `viola-agent-claude` is not among the six: whichever way its CARRY lands (folded or ratified), a `Display` that reaches a surface owes the same rule, and whether it holds a path today is research's question.
- security-plan §Error Handling (External responses, Error format) requires channel `-32603` as the fixed `internal error` with `data: null`, MCP `isError` results holding only the listed `structuredContent` codes with text `refused: <code>`, and every other handler fault, unknown reply shape or panicked call as JSON-RPC `-32603`.
- security-plan §Error Handling (External responses) lets `wrapper-fault` carry the JSON-RPC fault's `{code,message,data}`, so the MCP surface inherits whatever the channel answers; whether any wrapper fault's `message` or `data` can hold free text is research's question.
- security-plan §Bootstrap phases (`error-sanitization-wire`) requires anyhow chains to stop at the `viola` bin's human-mode stderr, any source whose `Display` can quote upstream text to be mapped to a fixed message before it joins the chain, and the full error to go only to `instances/<name>/diagnostics/` when an instance is resolved.
- security-plan §Bootstrap phases (`logging-redaction-wire`) requires the consolidated NEVER-log floor to hold on all three surfaces: no R8-stripped `CLAUDE*` value, user content, tool `input`, `statusline_command` string, drift report, absolute path or internal type name. Its scope carve-out keeps contract payloads (`wait`/`last` results, `events.ndjson`, `hook.dialog`) out of the floor, so the enumeration covers error bodies, not those payloads.
- security-plan §Input Validation (MCP stdin, Constants) requires an over-long `viola mcp` stdin line to be refused whole, end the session and leave one `parse-rejected` line holding no byte of it, and `MAX_FRAME` to cap the statusline output, child output and `config.json` reads; whether each holds when driven past its cap, including before `initialize` completes, is research's question for the PREREQ cases.

## Patterns to follow
- The frame verbs' refusal document under `--json`, a closed code with `detail: null`, per security-plan §Error Handling detail (`.andromeda/security-plan-detail/error-handling.md`, the `client::refuse_untrusted` item).
- `StampsError` and `PinError::StrictModes` as the shape of a fixed `Display` with no path or payload, per security-plan §Error Handling detail (`.andromeda/security-plan-detail/error-handling.md`, the `viola_state::stamps::StampsError` item).
- MCP `invalid-params` with a closed field word and no frame as the code-shaped `detail` form, per security-plan §Input Validation (MCP tool inputs row).
- A refusal that names no path, and a scan that never prints or writes the matched bytes, per security-plan §Secret Management (the artifact canary scan) and security-plan §Security Anti-Patterns → Code Patterns detail (the `target/e2e-home` keepers).

## Anti-patterns to avoid
- NEVER echo drift reports or anyhow chains into MCP `structuredContent` or channel `error.data`, print a chain still holding a serde source to stderr, or expose an absolute path in any external error (per security-plan §Security Anti-Patterns → Logging).
- NEVER write an R8-stripped `CLAUDE*` value into any log, stderr, `diagnostics/` or fixture; the planted value in the proof is a synthetic canary, and the strip handles names only (per security-plan §Security Anti-Patterns → Secrets and security-plan §Secret Management, Production).
- NEVER let an error-path change give `viola hook` a stderr byte or a non-zero exit, a security failure included (per security-plan §Security Anti-Patterns → Logging).

## Contract bindings
- security ↔ obs: the NEVER-log floor (security-plan §Bootstrap phases, `logging-redaction-wire`) binds obs's `diagnostics/` format and obs-plan §9's artifact canary scan (security-plan §Secret Management); whether that scan, as a run over `target/agent-run/`, meets the acceptance's "a secret scan after every end-to-end test" is research's question.
- security ↔ obs: telling the `-32602` causes apart on the `channel-response` line (the scope's CARRY, obs-plan §5) stays a closed word under security-plan §Error Handling (Error format), holding no param value.
- security ↔ tests: the per-surface witnesses and PREREQ cap-driving cases are test-plan §4 and §6 bodies, and their inputs stay synthetic (security-plan §Bootstrap phases, `secret-scanning-ci-gate`); security owns only what must be absent from the bodies.
- security ↔ arch: the `Refusal` fold-or-ratify decision is architecture [Error Handling]'s; security-plan §Error Handling binds only its `Display`.

## Acceptance criteria contributions
- With a path, an upstream text and a planted synthetic `CLAUDE*` canary in play, each driven failure on CLI `--json`, MCP `isError` and channel `error.data` yields a body holding none of the three (per security-plan §Bootstrap phases, `logging-redaction-wire`).
- A checkable enumeration names every error body of the three surfaces, each a closed code with a code-shaped or null `detail`/`data`, and `-32603` is exactly `internal error` with `data: null` on channel and MCP (per security-plan §Error Handling).
- A serde parse failure driven on a CLI verb prints no serde source on stderr or under `--json`, its input value absent from both (per security-plan §Bootstrap phases, `error-sanitization-wire`).
- A `viola mcp` stdin line past `MAX_FRAME` landing before `initialize` completes dispatches nothing and its `parse-rejected` line holds no byte of it, and each of the five named `BOUNDARIES` rows has a case driving past its cap (per security-plan §Input Validation).
