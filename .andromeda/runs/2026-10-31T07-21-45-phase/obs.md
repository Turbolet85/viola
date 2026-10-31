# obs extract

## Relevance
relevant

## Constraints
- Obs tier Standard (1) with Minimal exporter carve-outs: no external reporter, so scrubbing sits at the source and the local file sink is the only error sink (per obs-plan §1 Obs Scope Summary; per obs-plan §7 Platform pick). Instrumentation depth for this chunk is that tier's.
- Every product error `Display` that can reach a surface must be a fixed message with no field holding a path or payload, on the typed errors §7 names (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `CoreError`) (per obs-plan §7 Scrubbing layer 1). §7 does not name `viola-agent-claude`'s second enum `Refusal`; whether its `Display` and every other type at HEAD already comply is research's question.
- At dispatch, an anyhow chain with a `serde_json::Error` / `serde_path_to_error::Error` source must become the fixed `internal error` before it reaches stderr, `--json`, MCP `isError` or channel `error.data`; the full chain goes only to the instance detail file, and only when an instance resolves (per obs-plan §7 Scrubbing layers 2 and 3). Whether every dispatch path already does so is research's question.
- MCP `isError` results and the log lines of rejected calls must be codes only, and `instance-unreachable` is logged by no line, the `isError` code its only record (per obs-plan §7 Error classes captured; per obs-plan §1 Telemetry surfaces, ipc-internal `viola mcp`).
- `-32603` must be `"internal error"` with `data: null`; channel lines record neither `params` nor result bodies, not even hashed (per obs-plan §1 Telemetry surfaces, ipc-internal `viola-channel`; per obs-plan §6 Boundary-call wrappers).
- Stripped `CLAUDE*` values are never logged anywhere, detail files included; absolute paths and internal type names are scrub-required in external output, and home-level lines stay path-free; drift reports reach only the instance detail file (per obs-plan §8 Data classification rules, detail table).
- Each typed error crossing a crate seam must be logged once at `WARN`/`ERROR` per the level table, with a closed `detail` / `error_code`; any new field must be named in the §6 catalog and admitted by `schemas/diag-line.v1.json` (per obs-plan §10 Module-boundary error logging; per obs-plan §6 Log levels mapping; per obs-plan §8 Default-deny posture).

## Patterns to follow
- The fixed `unable:` / `hint:` refusal pair through `src/human.rs` as the human-stderr form, and the catch-site line of exactly `error: internal error` with no hint, for a failure a verb cannot name (per obs-plan §3 intro; per obs-plan §7 Per-role behaviour).
- The same-shaped `--json` refusal document with a closed code and `detail: null`, as specified for `instance-unreachable` and the revive preflight refusals (per obs-plan §8 Verification, strict-modes refusal row; per obs-plan §6 `detail` code catalog).
- Whole-output witnesses in the manner of `tests/security_negatives_server_identity.rs` and `tests/security_negatives_strict_modes.rs`: read every output with `digit_runs` and assert the line count, as a model for the per-surface proof (per obs-plan §8 Verification).
- `channel-response` emitted from the `Drop` guard in `channel.dispatch`, so an early return, error or contained worker panic still logs one code-only line (per obs-plan §4 Scenario: Confirmed `send`; per obs-plan §7 `run` worker threads).
- A witness home lives under `target/e2e-home/`, so G2, G4 and the secret scan read what it wrote; a gate failure prints file, line and keyword, never the content (per obs-plan §9 Pipeline integration; per obs-plan §9 Gate commands, G4).

## Anti-patterns to avoid
- NEVER print an anyhow chain that still holds a serde source to stderr, `--json`, MCP `isError` or channel `error.data`, and never use `#[instrument(err)]` / `#[instrument(ret)]` on a function whose error or return can carry one (per obs-plan §11 Error Reporting; per obs-plan §11 Spans / Traces).
- NEVER put a path or pid inside a `detail` code, and NEVER log a `CLAUDE*` value at any level in any file (per obs-plan §11 PII Scrubbing).
- NEVER add an `event` value or rename a tests field without a Decisions Log entry, or log a field outside the §6 catalog; this bars telling the `-32602` causes apart by an unrecorded field (per obs-plan §11 Logs; per obs-plan §11 PII Scrubbing).

## Contract bindings
- obs ↔ tests §3 Log format: any change to the `channel-response` line is additive only; obs §6 and `schemas/diag-line.v1.json` are the catalog it answers to, and `channel-response` already catalogues `detail` while whether the schema admits it beside `error_code:-32602` is research's question (per obs-plan §3 Log format JSON schema; per obs-plan §5 Per-surface metrics, `viola.compat.v_rejected`).
- obs ↔ tests §6 secret scan: the canary values and test body are tests-owned, and obs owns the rule that the content canary is in no home-level file and each `CLAUDE*` canary in no `diagnostics/` file (per obs-plan §8 Verification, detail table).
- obs ↔ tests §9 G4: `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json` are obs-owned, the check body tests-owned; a chain or drift report written by this chunk's paths must validate against the detail schema's `chain` / `drift_report` fields (per obs-plan §9 G4; per obs-plan §8 Detail-file scope).
- obs ↔ security Vectors 1, 5 and 6: the codes-only MCP lines, the unauthenticated `from` and the serde-chain replacement are the telemetry signals those vectors require (per obs-plan §1 Telemetry triggers).
- obs ↔ arch Error Handling: folding `Refusal` into `AgentError` or ratifying it as an exception is arch's call; either form stays under the fixed-`Display` rule (per obs-plan §7 Platform pick).
- obs ↔ the "Exit-cause code catalogue" entry: the `process-exit` `detail` catalogue is closed at 13 values and that entry owns its change, so this chunk adds no member to it (per obs-plan §6 `detail` code catalog).

## Acceptance criteria contributions
- With a path, an upstream text and a planted stripped `CLAUDE*` value in play, each surface's driven failure (CLI `--json`, MCP `isError`, channel `error`) shows none of the three in its body, and a serde-sourced failure shows exactly the fixed `internal error` message with `data: null` on the channel (per obs-plan §7 Scrubbing layer 2).
- The same driven failures' chain or drift report appears only in `instances/<name>/diagnostics/detail-<process>.ndjson`; the planted `CLAUDE*` value appears in no `diagnostics/` file, detail files included, and the secret scan and G4 pass over the witness homes (per obs-plan §8 Verification).
- If `viola.compat.v_rejected`'s over-count is closed, a `channel-response` for `send`'s invalid params, `release` carrying `from` and a bad `link` / `unlink` is distinguishable from an unsupported version by a closed code holding no path, validated by `schemas/diag-line.v1.json` (per obs-plan §5 Per-surface metrics).
- A `viola mcp` stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with exit 11, gives no `isError` result, and leaves as its one record `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` holding no byte of the cut line; whether that exit arm writes the line today is research's question (per obs-plan §7 Error classes captured, the `MAX_FRAME` detail row).
