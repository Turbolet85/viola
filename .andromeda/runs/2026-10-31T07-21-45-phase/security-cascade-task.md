# Task — security · 2026-10-31-sanitised-error-surfaces

## The chunk
Makes every error a `viola` process gives a machine reader a closed code and a fixed message, on CLI `--json` error documents, `viola mcp` `isError` results and wrapper JSON-RPC errors, and makes every product `Display` that can reach them fixed.
Proves it with one witness per surface, driving the failure with a path, an upstream text and a planted stripped `CLAUDE*` value in play, and an enumeration of the three surfaces' error bodies, so a new free-text body is visible.
Takes two carries: the `Refusal` enum of `viola-agent-claude` folded into `AgentError` or ratified as an exception, and the `channel-response` line telling apart the causes of `-32602`, so `viola.compat.v_rejected` stops over-counting.
Adds tests owed by the bounded-inputs chunk: a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes, and a cap-driving case for five `BOUNDARIES` rows.
Re-reads at HEAD, for candidate claim `v1-47` with its web clauses out, whether content and drift reports reach only owner-readable detail files and whether a secret scan after every end-to-end test proves the floor.

## Reached files
- crates/viola-agent-claude/src/lib.rs · edits the `Refusal` enum (`BatchScriptChild`, `NotFound`, the `resolve_program` refusal), the `AgentError` enum
- crates/viola-channel/src/lib.rs · edits `ProtocolError::InvalidParams`
- crates/viola-channel/src/client.rs · edits `fault_of`
- tests/contract_bounded_readers.rs · edits the `BOUNDARIES` rows of the status-line command's stdout, the child pipe drain, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep`
- .andromeda/obs-plan-detail/metric-coverage.md · re-reads the `viola.compat.v_rejected` row, the contract the `channel-response` line's change is read against

## Needs
- CLI `--json` error documents (`v`, `error`, `detail`): each holds a closed code and a code-shaped or null `detail`, and a failure the verb cannot name is no free text; external-response sanitisation under Error Handling binds them.
- `revive --json` as a `--json` surface: codes, closed words and session-shaped ids only, never a path or pid; no refusal line or hint holds the recorded directory, a pid or a logged id; its start-refusal documents stay with the exit-cause catalogue.
- MCP `isError` results of `send`, `wait`, `last`, `answer` and `list`: `structuredContent` carries only `refusal`/`detail` or `error`, the text is `refused: <code>`, `instance-unreachable` has a null `detail`, and `invalid-params` has a closed field word and writes no frame.
- MCP `wrapper-fault` carries the JSON-RPC fault's code, message and data, so the channel's fixed messages and data are what reach the driver through it.
- A handler fault, a reply of no known shape or a panicked MCP call is JSON-RPC `-32603`, `internal error`, `data: null`.
- Channel JSON-RPC errors: each has a fixed message and `error.data` a code or null, `-32603` being `internal error` with `data: null`; this covers `-32600` for a frame over `MAX_FRAME` and the `-32602` causes, plus the `result.refusal` pairs `not-delivered`/`control-character` and `not-delivered`/`empty-text` as bodies to enumerate.
- The `Display` impls of `PtyError` (hand-written), `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError`: fixed messages, with variant fields holding paths or payloads excluded (`error-sanitization-wire`).
- anyhow chains stop at the `viola` bin's human-mode stderr; a serde_json, serde_path_to_error or other upstream-quoting source is mapped to a fixed message before it joins a chain, and the full error goes only to `instances/<name>/diagnostics/` when an instance is resolved.
- The values no error body may hold, each a thing the witnesses and enumeration check: absolute paths, internal type names, user content, tool `input`, the `statusline_command` string, drift-report paths and values, the snapshot's recorded `cwd`, the terminal colour facts, and pids, `agent_session_id`, `started_at` and `budget.json` values outside `diagnostics/`.
- The stripped `CLAUDE*` floor, as the witness's planted value must prove it: no R8-stripped value in any log, diagnostic, snapshot, `events.ndjson`, fixture or error output, and the strip handles names and never reads a value.
- The carve-out from the floor: `events.ndjson`, `wait`/`last` results, `hook.dialog` and `statusline_command` in `snapshot.json` carry content by design, so a witness or the enumeration reads error bodies and never bans content from these payloads.
- The re-read of the owner-only detail leg: drift reports and other full detail go only to `diagnostics/`, its files 0600 inside 0700 directories, and never to stderr or the three surfaces.
- The re-read of the secret-scan leg: `viola-harness secret-scan`, its classes, the twelve `claude-stripped` needles, what it reads under `target/agent-run/`, its scan-gated uploads and its never printing matched bytes; a planted value reaching those files must be accounted for by the witness.
- `Refusal` fold or ratified exception: the `.cmd`/`.bat` child refusal and the `resolve_program` refusal keep refusing as they do, and whichever enum holds them displays a fixed message with no path; a refusal never blocks the human.
- `viola hook` keeps failing open while `Display` and refusal shapes change: no stderr, exit 0, no body, its hidden capture arms included.
- The `channel-response` line's change: `-32602` causes `InvalidParams`, unsupported-version and `release-from-driver` told apart in closed words only, holding no param value; `from` stays unauthenticated and `release-from-driver` still refuses before any budget is acted on.
- `viola mcp` stdin past `MAX_FRAME` before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11): the line is refused whole, never truncated or skipped, no byte released, one `parse-rejected` line holding no byte of it, and the new test drives this.
- Cap-driving case for the status-line command's stdout: Unix only, the output capped and printed byte for byte, never parsed, the wait bounded, no log line or span carrying the command.
- Cap-driving case for the child pipe drain shared by `claude --version` and `claude agents --json`: read under `MAX_FRAME`, and a failed read fails open or becomes `unknown`.
- Cap-driving cases for the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep`: the read capped at `MAX_FRAME`, a bad value taking its key's default or rejected whole and reported `parse-rejected`; no value switches the budget gate off or makes the hold zero or unbounded, and an identity-floor name has no effect.
- `MAX_FRAME` is 16 MiB, one const in `viola-core`, and the cap-driving cases drive past that value.

## Standing conventions
- S1 · Each new test uses its own `--home`, and the local gate's children get no host environment value beyond `HOME` and `PATH` · Secret Management body, Development
- S2 · Each new test's artefacts under `target/agent-run/` pass the canary scan, with synthetic inputs only and matched bytes never printed or written · Secret Management body, Secret scanning in CI
- S3 · A test-side removal under `target/e2e-home` takes only a dir the test side itself created · Security Anti-Patterns body, Code Patterns
- S4 · A test-only feature stays out of the release graph, refused by `scripts/release-check.sh`, and shared test helpers come only through root `[dev-dependencies]` · Dependency Security body, CI integration
- S5 · CI's G2 zero-panics gate keeps its one exemption, and a further one needs its own Decisions Log entry · Dependency Security body
- S6 · A new test seam or environment read needs its own Decisions Log entry, is compiled out of release builds and switches off no control · Security Anti-Patterns body, Universal
- S7 · A new or changed log line holds nothing from the NEVER-log floor, its format owned by obs · Bootstrap phases body, `logging-redaction-wire`

## Not needed
- Listener and socket enforcement (Windows DACL and SQOS, the 0700 socket directory, chmod after bind, the peer euid check, abstract namespace, `try_overwrite`): the scope leaves these to "Unix endpoint and home hardening" and "Windows-only live measurements".
- DLL search restriction, ConPTY sideloading and the vendored ConPTY audit: Windows launch hardening with no error body of the three surfaces in it.
- The paste and dialog text character set and how the wrapper types a text: the chunk reads only the refusal codes that come out.
- vt100 feeding, the screen model's poisoning and the stdin classifier of the human's terminal: these are terminal paths, not machine-read errors.
- Human-mode escaping of `wait`, `last` and `list` rows through `escape_row`: the chunk's surfaces are the machine readers.
- Recording `fixtures/claude/` through `viola verify --record`, probe dirs and the accepted residuals for verify's runs, scratch footer readings and live plugin installs: the chunk records nothing live.
- The verify-stamp gate on dialog answers, `read_stamps_strict` and who may write `ledger/stamps.json`: the chunk changes no decision about answering.
- Pinned binary re-hash, plugin file rewriting and absolute exec-form commands: start-up integrity the chunk does not change.
- `events.ndjson` retention and its local-availability accepted risk: an open item owned by arch.
- zizmor, action SHA pinning, `windows-mutants.yml`, the nightly advisory run and the v1.x release and signing prerequisites: the scope names no workflow or release change.
- TLS, HSTS, hosting, compliance triggers, account flows and SQL: not applicable to a local tool.
- Problem Details, the GUI token, cookie, launch URL and a11y artifact clauses, and any frontend surface: deferred to 0.2.0.
