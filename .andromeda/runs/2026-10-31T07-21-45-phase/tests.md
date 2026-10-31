# tests extract

## Relevance
relevant — the chunk delivers a named security-vector trigger of this plan (tier Comprehensive, per test-plan §1 Test tier) and two PREREQ tests the plan names this route entry the owner of.

## Constraints
- test-plan §1 Coverage triggers, security-vector-coverage (error sanitization and secret logging) requires assertions over all three surfaces (CLI `--json`, MCP `isError`, channel `error.data`) for no absolute path, serde path or value, anyhow chain or tool `input`, and `-32603` exactly `"internal error"` / `data:null`; its layers are integration + E2E (per test-plan §2 Trigger → layer map). Whether the code already satisfies this on every error body is research's question.
- test-plan §6 Error sanitization and secret scan requires these checks after every E2E test through the `events` support helper, the absolute-path check covering a drive letter, `/home/`, `/Users/` and the temp home prefix, a hit reported by place and class and never the matched bytes. Whether the helper reads all three surfaces' payloads today is research's question.
- test-plan §1 Surfaces under test (ipc-internal, wrapper channel and MCP) names closed error shapes that are not all "a code or null": `-32602` with `data:{supported, wrapper}`, `wrapper-fault` with `detail:{code,message,data}`, `invalid-params` with a closed field word. The scope's wording is narrower, so the chunk's proof must either admit these shapes by name or carry a plan amendment; a P4 reading, not an implementation choice.
- test-plan §6 Cross-surface parity requires the MCP `structured_content` `{refusal, detail}` to equal the CLI `--json` `{refusal, detail}` for the same refusal; any change to a refusal or fault body holds both ends.
- test-plan §4 viola-mcp names this route entry the owner, as a PREREQ, of the undriven stdin line past `MAX_FRAME` landing before `initialize` completes (`ServeError::Initialize`, exit 11); test-plan §6 Contract suite (its Owed bullet) names it the owner of a cap-driving case for each named row of the bounded-reader enumeration.
- test-plan §3 `secret-scan` single-sources its canaries: a planted stripped `CLAUDE*` value must be a `canary-<kind>-value-<hex4>` literal in the scan's table, held by `crates/viola-e2e/tests/scan_patterns.rs`; its classes reach diagnostics, the harness capture and JUnit, not the error payloads, which the §6 helper holds.
- test-plan §10 Coverage thresholds gate per OS at lines ≥ 85 %, functions ≥ 95 %, regions ≥ 80 %; test-plan §10 Mutation gate puts no mutation run on this chunk (no boundary in Epoch 6, per the scope's directive); test-plan §10 Zero-flakiness budget keeps `retries = 0`, so the Path 1 watch is never retried away.

## Patterns to follow
- The scope's enumeration of error bodies takes the shape of the bounded-reader enumeration: literal tables, each row naming its site and a defined witness test, a census over product sources with a planted self-check (per test-plan §6 Contract suite). The plan lists the check but names no such table; whether it passes the scope's "no testing past what the plan lists" is for P4.
- Each witness is a negative beside a control, run in both modes, asserting no pid, path or start time in any output, as the server-identity case does (per test-plan §6 Security control negatives, Server identity).
- MCP errors: handler cases drive the transport-free `viola_mcp::Tools` against `TestWrapper` (per test-plan §5 Cross-module patterns, MCP); the full stdio path is an rmcp client over a child the test spawns and owns (per test-plan §6 Drivers per surface).
- Channel errors: a raw `interprocess` stream for malformed frames with shapes validated by jsonschema (per test-plan §5 Driver(s)); the JSON-RPC mapping itself is a unit case (per test-plan §4 viola-channel).
- New binaries and cases take the `<surface>_<topic>.rs` prefix and `<subject>_<condition>_<expected>` names so the layer filtersets select them (per test-plan §2 Test directory + naming conventions); the runs are `scripts/agent-run.sh run --integration` and `run --e2e`, one case through `--filter` (per test-plan §3 `run`).

## Anti-patterns to avoid
- Never take the product's own list as the oracle: expected codes, fixed messages and exit codes are literals in the test, compared by serde value and integer, never by `Debug` string (per test-plan §11 Unit).
- Never assert at E2E what a lower layer can hold: `Display` texts and error mapping belong in §4, shapes in §5, and §6 holds only cross-process behaviour (per test-plan §11 Test Strategy).
- Never read a Linux-only run as proof of a Windows or macOS error path, a path-shaped leak included (per test-plan §11 Test Strategy).

## Contract bindings
- tests §3 Log format ↔ obs-plan: the `channel-response` line telling `-32602` causes apart is a contract read; obs-plan may add fields but not rename or remove the harness's, and a new `event` value needs a Decisions Log entry. The `viola.compat.v_rejected` count is obs-plan §5's.
- tests §6 Error sanitization and secret scan ↔ obs-plan §8 (the content canary, High-class verification) and obs-plan §9 (the scan's step order before any upload).
- tests §1 security-vector-coverage (error sanitization) ↔ security-plan Error-sanitization expectations and Anti-Patterns (Secrets and logging), the trigger's source.
- tests §6 Chaos suite (`.cmd` child: `detail:"batch-script-child"` and the two fixed stderr lines) ↔ architecture [Error Handling]: folding or ratifying `Refusal` must leave those pinned outputs as they stand, or amend them in the same chunk.
- tests §6 Exit-cause matrix ↔ route entry "Exit-cause code catalogue": `cross_exit_causes.rs`, the unwrapped-name hint and the post-preflight `revive --json` documents stay with that entry.

## Acceptance criteria contributions
- A witness per surface drives the failure with a path, an upstream text and a planted stripped `CLAUDE*` canary in play, and its body holds none of them, no serde path and no `Caused by`, with every `-32603` exactly `"internal error"` / `null` (per test-plan §1 security-vector-coverage (error sanitization and secret logging)).
- A test drives a `viola mcp` stdin line past `MAX_FRAME` landing before `initialize` completes and reads `ServeError::Initialize`, exit 11 (per test-plan §4 viola-mcp).
- Each owed row of `tests/contract_bounded_readers.rs` (the status-line command's stdout, the child pipe drain, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep`) names a witness that drives past the row's cap (per test-plan §6 Contract suite).
- `scripts/agent-run.sh run --coverage` then `gate --require coverage,doctest` exit 0 on the three CI OSes, and `secret-scan` prints `ok:true` with no `claude-stripped` or `cookie` hit (per test-plan §10 Coverage thresholds and §3 `secret-scan`).
