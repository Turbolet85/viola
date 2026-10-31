# security · Threat Model Summary — category read

## Carried
- (.andromeda/security-plan-detail/threat-model-summary.md:11) This class is upstream text that may hold secrets, so no error body on the three surfaces may carry it, and the upstream text the witnesses drive must stand for it.
  > **Type:** user-content (tool-call arguments, plans, dialog questions and answers). This may incidentally include secrets or personal data the user's session touches.
- (.andromeda/security-plan-detail/threat-model-summary.md:28) This names the variable class the planted stripped `CLAUDE*` value stands for; the proof must show no value of the class on any of the three surfaces.
  > The `viola run` wrapper process's environment holds `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and other `CLAUDE*` parent-identity variables, which are stripped from the child's environment (R8 strip list, Occupied Resources: Environment variables).
- (.andromeda/security-plan-detail/threat-model-summary.md:28) The founder's two live crossings cover no later probe, so the planted value must be a synthetic one the witness sets itself, and a live read of a real `CLAUDE*` value needs its own answer.
  > No value was read into any file, no product code, `viola verify` run or capture arm took part, and neither answer covers a second message, a verify child or any other probe.
- (.andromeda/security-plan-detail/threat-model-summary.md:32) The recorded `cwd` must appear in no error body or stdout on any surface, and the enumeration must hold no body that can carry it.
  > (the start's spawn directory: an absolute host path that can carry the OS username; it lives in the 0600 snapshot only, on no log line, error body or stdout, and is omitted when absent)
- (.andromeda/security-plan-detail/threat-model-summary.md:44) The enumeration of channel error bodies must cover every method of this set, each with its fixed message and a code or null `error.data`.
  > Methods: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, and `hook.event` notifications.
- (.andromeda/security-plan-detail/threat-model-summary.md:84) A drift report is the source a fixed message must replace: no serde or serde_path_to_error text from this parse may reach an external error, including through an `AgentError` or `Refusal` `Display`.
  > Parsing is tolerant in `viola-agent-claude`: no `deny_unknown_fields`, serde_path_to_error for drift reports (Established Decisions: Validation).
- (.andromeda/security-plan-detail/threat-model-summary.md:86) Any change to the error types, the `Refusal` fold included, must leave the hook path failing open with no body.
  > Any failure fails open to the human: exit 0 with no body.
- (.andromeda/security-plan-detail/threat-model-summary.md:95) The MCP witness and enumeration must cover exactly these five tools.
  > Tools `send`, `wait`, `last`, `answer` and `list`, called by the driver session's model (Established Decisions: MCP).
- (.andromeda/security-plan-detail/threat-model-summary.md:98) A strict-modes refusal on an MCP tool must keep this exact body, with the home path in play and absent from it, and must write no frame and no line.
  > a refusal is `instance-unreachable` with `detail: null`, no frame and no line.
- (.andromeda/security-plan-detail/threat-model-summary.md:99) `wait` and `last` content is untrusted upstream text delivered by contract; it must not cross into an `isError` result, whose reader is the same driver model.
  > The driver model reads the driven session's output (`wait`/`last`), which arch classes as untrusted upstream text.
- (.andromeda/security-plan-detail/threat-model-summary.md:108) The owed pre-`initialize` over-long-line test must hold this boundary: refused whole, session ended, one `parse-rejected` WARN holding no byte of the line.
  > an over-long line from the driver is refused whole and ends the MCP session after the earlier lines are answered, never truncated, skipped or dispatched in part, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN holding no byte of it.
- (.andromeda/security-plan-detail/threat-model-summary.md:120) Each parser at this boundary can fail under `--json`, so a failure from clap, `ViolaName` or `config.json` that reaches a `--json` document must be a closed code, echoing no argument or value; the owed `config.json` cap-driving cases sit at this boundary too.
  > Same-user invocation, clap parsing, the `ViolaName` nutype, and tolerant `config.json` parsing.
- (.andromeda/security-plan-detail/threat-model-summary.md:121) These strict-modes refusals are driven by a chosen path, so their `--json` documents must name a closed code and no path, and their exit codes must stand.
  > Before its first read or open under the home, `run`, `revive`'s start and `verify` hold the home tree to the strict-modes check (`check_home_tree`; a refusal exit 1, no log opened), and the frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` hold the instance to it (`check_instance`; a refusal exit 21)
- (.andromeda/security-plan-detail/threat-model-summary.md:151) The owed cap-driving case for the `claude agents --json` pipe drain must leave this line free of row text, id, `cwd` and path.
  > each spawn pair `agents-probe` is logged only under an open log and holds no row text, id, `cwd` or path.
- (.andromeda/security-plan-detail/threat-model-summary.md:157) A child output past its cap must end in the tolerant failure outcome, and the owed cap-driving case must assert it, with no child text reaching an error body.
  > Output parsing is tolerant, and failure becomes `unknown`.
- (.andromeda/security-plan-detail/threat-model-summary.md:178) The witnesses and owed cases that run in CI must drive the fake agent, not the real CLI.
  > The real `claude` CLI runs only locally; `viola verify` runs in CI only against the fake agent, and its real-CLI probe and `--record` only locally.
- (.andromeda/security-plan.md:116) The `release-from-driver` cause must stay a code in `data` on `-32602`, distinct from the other causes of that code.
  > Driver-originated `release` — the wrapper refuses a `release` whose params carry a string `from` (`-32602`, `release-from-driver`), before any budget is acted on
- (.andromeda/security-plan.md:140) A failed MCP tool input must surface only a closed field word, never the argument's value.
  > a failure being `invalid-params` with a closed field word and no frame.
- (.andromeda/security-plan.md:144) The `revive --json` documents must keep this floor; the start-refusal documents owed to the exit-cause catalogue stay out of this chunk.
  > `revive --json` prints codes, closed words and session-shaped ids only, never a path or pid.
- (.andromeda/security-plan.md:146) The owed cap-driving cases for the thresholds, the dialog hold and `claude_env_keep` must assert this outcome past the cap.
  > a tolerant parse capped at `MAX_FRAME`, `v` checked; a bad value takes its key's default, or is rejected whole, and is reported as `parse-rejected`
- (.andromeda/security-plan.md:160) The owed case for the child pipe drain shared by `claude --version` and `claude agents --json` must assert the cap and the fail-open read.
  > read under `MAX_FRAME` and parsed tolerantly, a failed `claude agents --json` read failing open
- (.andromeda/security-plan.md:166) No refusal line or hint may hold these three values, whichever surface prints it.
  > no refusal line or hint holds the recorded directory, a pid or a logged id.
- (.andromeda/security-plan.md:170) The owed status-line stdout case must drive past the cap, and no error may carry the command.
  > its stdout is capped and printed byte for byte, never parsed, its wait bounded; no log line or span carries the command, and it remains the only shell-out.
- (.andromeda/security-plan.md:178) The owed cases must drive the one `MAX_FRAME` const these readers share, with no second cap value.
  > `MAX_FRAME` = 16 MiB, one const in `viola-core`, caps every external read (channel frames, hook stdin, the statusline source and command output, `answer` input, `viola mcp`'s stdin lines, sent text, child output, ndjson lines, `config.json` and the stamps read;
- (.andromeda/security-plan.md:342) A chain printed to human-mode stderr must hold no source that can quote upstream text, and a chain must reach no `--json`, MCP or channel body.
  > anyhow chains stop at the `viola` bin's human-mode stderr. A chain printed there never includes a serde_json or serde_path_to_error source, or any other source whose `Display` can quote upstream text (serde errors quote the offending value). Map it to a fixed message before it joins the chain, and write the full error only to `instances/<name>/diagnostics/` when an instance is resolved (NEVER-log floor, `logging-redaction-wire`).
- (.andromeda/security-plan.md:344) This is the floor the planted value proves, across every diagnostic and error output the witness drives.
  > Never write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value to any log, diagnostic, snapshot, `events.ndjson` or fixture.
- (.andromeda/security-plan.md:345) This is the floor the chunk's three surfaces are held to, and the owner-readable destination the candidate claim's detail-file leg must show.
  > Outside those payloads, user content (prompts, `last_assistant_message`, plan, question and option text), tool `input`, the `statusline_command` string, and serde_path_to_error drift reports (paths and values) go only to `instances/<name>/diagnostics/`. They never go to stderr log or error lines, CLI `--json` error output, MCP `isError` results or channel `error.data`.
- (.andromeda/security-plan.md:346) Paths and internal type names must be absent from all three surfaces, so a `Debug`-formatted type name is a failure just as a path is.
  > Operational metadata (TMS §1): absolute paths, which carry the OS username (the only PII in scope), and internal type names never go to CLI `--json` error output, MCP `isError` results or channel `error.data`.
- (.andromeda/security-plan.md:372) A witness planting a stripped value must not make the strip read a value; the strip must keep handling names only.
  > the strip handles names only and never reads a value.
- (.andromeda/security-plan.md:380) The candidate claim's secret-scan leg must read against this owned scan and its `claude-stripped` class.
  > What CI does run is the obs-owned artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`, `if: always()`, obs-plan §9 step 3). Its classes:
  > - the stripped `CLAUDE*` canaries (the twelve `claude-stripped` needles);
- (.andromeda/security-plan.md:409) This is the chunk's governing rule: its exact bodies, the listed `Display` rule that any second enum left standing must also meet, and a `wrapper-fault` whose `{code,message,data}` must itself hold only fixed values.
  > **External responses** (to channel clients, CLI `--json` and MCP drivers): sanitised.
  > - No anyhow context chains, no absolute paths, no upstream text (prompts, tool `input`, assistant output), no serde_path_to_error paths or values, and no internal type names.
  > - The `Display` impls on `PtyError` (hand-written), `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError` (`thiserror`) use fixed messages. Variant fields that hold paths or payloads are excluded from `Display`.
  > - Channel `-32603` uses a fixed `message` ("internal error") with `data: null`.
  > - MCP `isError` results carry only the `structuredContent` codes from Conventions (`refusal`/`detail`, or `error: instance-unreachable | wrapper-fault | invalid-params` — `instance-unreachable` with `detail: null`, `wrapper-fault` with the JSON-RPC fault's `{code,message,data}`, `invalid-params` with a closed field word), and every `isError` text is `refused: <code>`. Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error `-32603` `internal error` with `data: null` (chunk 2026-10-17-mcp-server-for-drivers).
- (.andromeda/security-plan.md:416) Full detail removed from an error body must have only these destinations, and stderr must stay under the floor.
  > Full error detail goes only to `instances/<name>/diagnostics/` (files 0600; format owned by obs) or, for short-lived CLI verbs, to stderr. stderr never carries upstream text, tool `input`, drift-report values or any other value on the NEVER-log floor (Bootstrap phases, `logging-redaction-wire`).
- (.andromeda/security-plan.md:418) The fixed messages must not move any hook failure onto stderr or a non-zero exit.
  > `hook` never writes to stderr and never exits non-zero: a security check failing in `hook` still ends in exit 0 with no body, so the dialog renders for the human;
- (.andromeda/security-plan.md:421) The channel enumeration must keep this split of protocol faults and refusals, and any way the `-32602` causes are told apart must stay within codes.
  > **Channel:** JSON-RPC `error` for protocol faults, including `-32600` for frames over `MAX_FRAME` and `-32602` for `release`-with-`from`, and `result.refusal` for `not-delivered`/`control-character` and `not-delivered`/`empty-text`.
- (.andromeda/security-plan.md:454) The owed over-long-line test must fail on a reader that truncates or skips.
  > and NEVER hand rmcp a `viola mcp` stdin reader that does not cap each line at `MAX_FRAME`, nor truncate or skip an over-long line instead of ending the session.
- (.andromeda/security-plan.md:482) This ban covers the MCP and channel bodies directly, and the enumeration must make a breach visible.
  > NEVER echo serde_path_to_error drift reports or anyhow chains into MCP `structuredContent` or channel `error.data`, and NEVER print an anyhow chain to stderr that still contains a serde_json or serde_path_to_error source (its `Display` quotes input values such as tool `input`).
- (.andromeda/security-plan.md:487) No external error on any surface may carry an absolute path.
  > NEVER expose absolute paths in any external error.
- (.andromeda/security-plan.md:519) The witnesses must plant their values through what the test side already sets, and must add no product seam or environment read without its own Decisions Log entry.
  > NEVER add another such seam or environment read without its own Decisions Log entry.

## Bindings
- **Threat data classes ↔ body §Bootstrap phases and §Error Handling**: the classes the threat model names (`CLAUDE*` credential, user content, the recorded `cwd`) are the values the NEVER-log floor and the external-response rule keep off the three surfaces, (.andromeda/security-plan-detail/threat-model-summary.md:28), (.andromeda/security-plan-detail/threat-model-summary.md:32), (.andromeda/security-plan.md:345), (.andromeda/security-plan.md:409)
- **MCP boundary refusal ↔ body §Error Handling**: the `instance-unreachable` body of the MCP boundary and the `isError` code list must read as one, (.andromeda/security-plan-detail/threat-model-summary.md:98), (.andromeda/security-plan.md:409)
- **MCP stdin cap ↔ test-plan §4 viola-mcp and §6 Contract suite**: the boundary's refusal and its `MAX_FRAME` rule bind the owed cases that test-plan names this entry to own, (.andromeda/security-plan-detail/threat-model-summary.md:108), (.andromeda/security-plan.md:454), (.andromeda/security-plan.md:178)
- **Secret-scan leg ↔ obs-plan §9 and `verification-matrix.json#v1-47`**: the claim's secret-scan leg reads against the obs-owned canary scan and its `claude-stripped` class, (.andromeda/security-plan.md:380), (.andromeda/security-plan.md:344)
- **`-32602` causes ↔ obs-plan §5 and the `channel-response` line**: telling the causes apart must stay within codes, as a contract read against obs-plan, (.andromeda/security-plan.md:116), (.andromeda/security-plan.md:421)
- **`Display` rule ↔ architecture [Error Handling] one-enum rule**: folding `Refusal` into `AgentError` or ratifying it as an exception must leave every enum that can reach a surface on fixed messages, (.andromeda/security-plan.md:409), (.andromeda/security-plan-detail/threat-model-summary.md:84)
- **Fake-agent witnesses ↔ tests plan CI**: a witness run in CI must drive the fake agent, (.andromeda/security-plan-detail/threat-model-summary.md:178)
