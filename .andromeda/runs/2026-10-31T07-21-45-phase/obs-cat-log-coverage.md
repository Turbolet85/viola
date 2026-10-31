# obs · Log Coverage — category read

## Carried
- (.andromeda/obs-plan-detail/log-coverage.md:10) The strip's log fields must stay counts and names only, so the planted stripped `CLAUDE*` value of the proof appears in no `process-start` field.
  > `env_stripped_count`, `env_stripped_known`, `env_kept` (claude-child: the kept `CLAUDE*` names, comma-joined, names only)
- (.andromeda/obs-plan-detail/log-coverage.md:13) The `channel-response` line is the contract the `-32602` carry reads against: any telling apart of the causes must use this row's fields, and `error_code` must stay on its closed list.
  > | `channel-response` | `method` (only a name on the channel's closed method list, `link` and `unlink` among them; any other name logs no `method` key), `conn` / `srv_conn`, `result_class` (`ok|refusal|error`), `refusal`, `detail`, `error_code` (`-32700|-32600|-32601|-32602|-32603`), `outcome`, `duration_ms` |
- (.andromeda/obs-plan-detail/log-coverage.md:17) A handler fault that panics must keep payload and backtrace out of the home-level line and out of the error bodies of the three surfaces.
  > | `panic` | `panic_location` (workspace-relative, or `<crate>-<ver>/src/...` for dependencies), `thread`. Payload and backtrace go to `detail-<process>.ndjson` only |
- (.andromeda/obs-plan-detail/log-coverage.md:18) `release-from-driver` is one of the `-32602` causes the carry must tell apart, and its own event and fields must stand unchanged.
  > | `release-from-driver` (D-01) | `conn`, `from` (only when a valid `ViolaName`), `from_trust` (`self-reported`) |
- (.andromeda/obs-plan-detail/log-coverage.md:20) The cap-driving cases owed by the bounded-inputs prerequisite must log under this closed `parser` list and add no parser without the schema moving with it.
  > | `parse-rejected` (D-05) | `parser` (`claude-agents-json|vt100-feed|config-json|ledger-stamps|hook-stdin|channel-frame|mcp-stdin`, `prompt-submitted|paste-text|state-entry`), `detail`, `count` |
- (.andromeda/obs-plan-detail/log-coverage.md:26) The `--json` error enumeration must name this document shape for the run and revive collision refusal, and the logged line must stay unchanged by it.
  > or under `--json` the `{"v":1,"error":"already-live","detail":null}` document — revive's preflight refusals under `--json` are the same-shaped document with their code, the logged line unchanged (chunk 2026-10-17-cli-machine-contract)
- (.andromeda/obs-plan-detail/log-coverage.md:26) Folding or ratifying the agent crate's `Refusal` enum must leave these exit-1 `detail` codes as they are, `batch-script-child` among them.
  > `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `internal-error`,
- (.andromeda/obs-plan-detail/log-coverage.md:26) The `--json` enumeration must hold the `strict-modes-failed` document of this start-arm refusal, which opens no log.
  > its refusal is the stderr `unable:` / `hint:` pair (or under `--json` the `strict-modes-failed` document, chunk 2026-10-17-cli-machine-contract) and exit 1 with no log opened, no `process-start`, no `process-exit` and no child.
- (.andromeda/obs-plan-detail/log-coverage.md:27) A refused home tree must stay a `--json` document with no `process-exit` detail, so a witness on it reads the document and no line.
  > (revive under `--json` prints the `strict-modes-failed` document), so a refused home tree carries no `process-exit` detail either.
- (.andromeda/obs-plan-detail/log-coverage.md:28) The closed `process-exit` `detail` count is a value the chunk must match, and any new code must move the schema and this count together.
  > `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 13 values (the enum's members; 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).
- (.andromeda/obs-plan-detail/log-coverage.md:30) A witness driving a server-verification failure on a frame verb must find this exact line, holding no pid, start time, path or endpoint.
  > `server-verify-failed` is written by the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows): `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson` (`link` / `unlink`: the driven instance's), with no pid, start time, path or endpoint.
- (.andromeda/obs-plan-detail/log-coverage.md:31) A wrapper fault must log this closed code alongside its `wrapper-fault` error body.
  > - `process-exit` exit 20: `wrapper-fault`.
- (.andromeda/obs-plan-detail/log-coverage.md:32) The status-line cap-driving case must write only these three `detail` codes on its arm.
  > `viola hook statusline` uses three of the seven (the rule: the `detail` codes the arm's fail-open paths write): `oversize-stdin`, `malformed-json`, and `deadline`, which on that arm is the 5 s PROVISIONAL bound on the user's command, not a dialog deadline.
- (.andromeda/obs-plan-detail/log-coverage.md:34) The owed cap-driving cases must assert the `parse-rejected` detail this list gives each parser, `oversize` only where it names one, and the list stays closed by the schema.
  > - `parse-rejected` (D-05) details, per `parser`: `malformed` (any parser), `unreadable` (`config-json`, `ledger-stamps`, `state-entry`), `oversize` (`channel-frame` beyond `MAX_FRAME`, `hook-stdin`, `mcp-stdin` (a `viola mcp` stdin line past `MAX_FRAME`), `vt100-feed` when the bounded feed queue drops a copy — one line per poisoning episode, `claude-agents-json` for an answer past `MAX_FRAME`, `AgentsFailure::Oversize`), `unknown-keys` (`config-json` skipped keys, counted in `count`), `panicked` (`vt100-feed` under `catch_unwind`), `control-character` (`paste-text`), `symlink-ignored` (`state-entry`), `strict-modes-failed` (`ledger-stamps`: `run`'s version gate refused the stamps file on the strict-modes check, and the read counts as unverified). `diag-line.v1.json` constrains `parse-rejected.detail` to this list (D-31).
- (.andromeda/obs-plan-detail/log-coverage.md:35) The owed test of a stdin line past `MAX_FRAME` before `initialize` must hold this line's terms: the sink, the once-per-session write, and no byte of the cut line.
  > `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` is written at `WARN` into `mcp.ndjson` by `viola_mcp::serve` through `obs_event!`, once per session, after the session ends, only when the stdin reader cut a line past `MAX_FRAME`; it holds no byte of the cut line.
- (.andromeda/obs-plan-detail/log-coverage.md:36) The closed refusal codes the CLI, MCP and channel surfaces carry must be these verbatim, and `send-refused.detail` being an open string in the schema must not become a door for free text.
  > - Refusal details are the arch `RefusalReason` details verbatim: `manual-pause`, `five-hour`, `seven-day`, `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character`, `empty-text` (an empty typed text on `send`; it adds no field, and `send-refused.detail` is an open string in `diag-line.v1.json`).
- (.andromeda/obs-plan-detail/log-coverage.md:37) The `--json` enumeration must hold revive's `strict-modes-failed` document on this refusal, with no log line.
  > `viola revive`'s start arm and `viola revive --list` both refuse on `viola_state::strict::check_instance` with the stderr pair, or under `--json` the `strict-modes-failed` document, exit 1 and no log line;
- (.andromeda/obs-plan-detail/log-coverage.md:37) The `--json` enumeration must hold the `instance-unreachable` document of `plugin install`, `link` and `unlink`, each writing no line.
  > `viola plugin install`'s refused `check_path` of its home or a `marketplace/` tree dir exits 21 and writes no line either, under `--json` the `instance-unreachable` document (chunk 2026-10-17-optional-plugin-install); `viola link` / `viola unlink`'s refused `check_instance` of the driven instance exits 21 and writes no line either — no `process-start`, no `process-exit` — the fixed instance-state pair on stderr, or under `--json` the `instance-unreachable` document), `pinned-hash-mismatch` (on `process-exit` exit 1), `symlink-ignored`.
- (.andromeda/obs-plan-detail/log-coverage.md:37) A symlink's name and target must stay out of every line, the same bar the chunk holds its error bodies to.
  > The entry name and the link target are never logged.
- (.andromeda/obs-plan-detail/log-coverage.md:39) The witnesses of the CLI and MCP surfaces must find these refusals as codes with no line, `detail: null` on the MCP frame tools.
  > The other refused strict-modes checks write no line either — no `process-start`, no `process-exit`, no `strict-modes-failed`: `send`, `wait`, `last`, `answer`, `pause` and `release` through `client::refuse_untrusted` (exit 21, the instance pair, or under `--json` the `instance-unreachable` document); `viola mcp`'s frame tools (`instance-unreachable`, `detail: null`); `viola verify`'s home-tree check (exit 1, the home pair); and the home-tree check of `viola run` and `viola revive`'s start arm (exit 1, the home or the instance pair).
- (.andromeda/obs-plan-detail/log-coverage.md:45) The owed cap-driving case on `claude agents --json` must assert this outcome, one line and the value `"unknown"`, with no row text.
  > `claude agents --json` parse failure (oversize or malformed) gives one `parse-rejected{parser:"claude-agents-json"}` and the value `"unknown"`.
- (.andromeda/obs-plan.md:67) The planted stripped value must be absent from every sink, not only the three error surfaces.
  > drift reports go to `diagnostics/` only, and R8-stripped `CLAUDE*` values are never logged anywhere.
- (.andromeda/obs-plan.md:123) Every wrapper `-32603` must answer this fixed body, and `from` must not be logged as authenticated.
  > `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
- (.andromeda/obs-plan.md:127) MCP `isError` results and the lines for rejected calls must hold codes only.
  > `isError` results and the log lines for rejected calls carry codes only.
- (.andromeda/obs-plan.md:205) A rejected tool call's line must hold no upstream text, absolute path or type name, which the MCP witness must drive with each in play.
  > rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan.md:210) The never-log floor spans all sinks and fixtures, and no switch may lift redaction for any witness or fault path.
  > a never-log floor for every R8-stripped `CLAUDE*` value across all sinks and fixtures; the strip is logged as a count or variable names only, never values, and Phase 3 confirms the names are admissible; a serde-sourced anyhow chain becomes a fixed message before stderr; no env, flag or config may disable redaction.
- (.andromeda/obs-plan.md:253) Each exit 21 and exit 1 cause must keep a distinct pathless code, and a refusal before any log opens must have only its stderr pair or `--json` document as its cause.
  > `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid, except a strict-modes check refused before any log opens, which writes no line (its cause is the fixed stderr pair or the `--json` document).
- (.andromeda/obs-plan.md:418) The carried `-32602` over-count retires only when the line tells these causes apart, and this sentence must move with that change.
  > - `viola.compat.v_rejected` over-counts: `-32602` also answers `send`'s invalid params, `release` carrying `from` and a bad `link` / `unlink` name or pair, until the line tells the causes apart.
- (.andromeda/obs-plan.md:432) A change to a log line must add and never rename or remove.
  > This section only **adds** fields and the D-01, D-03, D-05 and D-06 enum values; it renames and removes nothing.
- (.andromeda/obs-plan.md:449) Any field added for the cause split must be snake_case with a kebab-case, code-only value.
  > **Additive field catalog** (snake_case; values kebab-case; all codes-only):
- (.andromeda/obs-plan.md:451) Every `detail` value a surface's failure logs must be a closed kebab-case code, never a path or pid.
  > **`detail` code catalog** (closed, kebab-case, never a path or pid; Founder Direction 4):
- (.andromeda/obs-plan.md:459) The levels must hold on every error line the chunk touches, `-32603` at `error`, and `-32600` and `-32602` at `warn`.
  > | `error` | `panic`; `process-exit` with `exit_code` 1 or 20; `channel-response` with `error_code` -32603 |
  > | `warn` | `state-recovered`; `parse-rejected`; `hook-decision` with a fail-open `detail`; `process-exit` exit 21; `channel-response` with `-32600`/`-32602` |
- (.andromeda/obs-plan.md:479) The channel's error witness and the cause split must record no `params` or result body, not even a digest of one.
  > **No argument digest**: `params` and result bodies are never recorded, not even hashed.
- (.andromeda/obs-plan.md:576) A field the chunk adds to a line must be named in the Section 6 catalog and allowed by the schema, or the build must fail.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:643) A typed error reaching one of the surfaces must log once with a closed code, its chain in the instance detail file alone.
  > **Module-boundary error logging:** every typed error that crosses a crate seam is logged once at `WARN`/`ERROR` with a closed `detail` / `error_code`. The full chain goes only to the instance detail file.
- (.andromeda/obs-plan.md:715) A new `event` value for the cause split or any failure must have a Decisions Log entry first, and no test field may be renamed.
  > - NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:731) The default-deny rule stands as a ban on this chunk's lines.
  > - NEVER log any field not named in the Section 6 catalog (default-deny).
- (.andromeda/obs-plan.md:734) The canary and the planted value must meet these bans at every level and file, detail files included for `CLAUDE*` values.
  > - NEVER log `CLAUDE*` values or a `Cookie` header, at any level, in any file, detail files included. NEVER log `statusline_command`, send text, tool `input` or `last_assistant_message` in any home-level file. They may reach only the instance-scoped `detail-<process>.ndjson`, and only inside a chain or drift report (§8).
- (.andromeda/obs-plan.md:735) A path driven into a failure by the proof may appear only in an instance detail file.
  > - NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.

## Bindings
- **`channel-response` `-32602` causes ↔ Metric Coverage**: the line's cause split on the log side and `viola.compat.v_rejected`'s over-count on the metric side must change as one reading, the field under the Section 6 catalog and the schema (.andromeda/obs-plan-detail/log-coverage.md:13), (.andromeda/obs-plan.md:418), (.andromeda/obs-plan.md:576)
- **`mcp-stdin` oversize and `claude-agents-json` lines ↔ test-plan §4 viola-mcp and §6 Contract suite**: the test plan owes the cap-driving cases, and this category fixes the line each must find (.andromeda/obs-plan-detail/log-coverage.md:35), (.andromeda/obs-plan-detail/log-coverage.md:34), (.andromeda/obs-plan-detail/log-coverage.md:45)
- **`--json` documents and no-line refusals ↔ Exit-cause code catalogue**: the enumeration must name these documents while the catalogue and the revive start-refusal documents owed to it stay with that entry (.andromeda/obs-plan-detail/log-coverage.md:26), (.andromeda/obs-plan-detail/log-coverage.md:37), (.andromeda/obs-plan-detail/log-coverage.md:39)
- **Refusal details ↔ architecture `RefusalReason` and [Error Handling]**: the log's codes are the arch details verbatim, and the `Refusal` enum's fold or ratification on the arch side must leave the logged `batch-script-child` code unchanged (.andromeda/obs-plan-detail/log-coverage.md:36), (.andromeda/obs-plan-detail/log-coverage.md:26)
- **`detail` codes and never-log floor ↔ PII Scrubbing and security Vectors 5 and 6**: the codes-only lines here and the floor there are one property, proved by the witnesses and the secret scan (.andromeda/obs-plan.md:451), (.andromeda/obs-plan.md:210), (.andromeda/obs-plan.md:734)
- **Log levels ↔ Error Capture & Reporting**: the boundary-error rule's `WARN`/`ERROR` line takes its level from the Section 6 table (.andromeda/obs-plan.md:643), (.andromeda/obs-plan.md:459)
