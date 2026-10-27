# obs · cascade read · group 4 of 10

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:328) The chunk's witnesses of the dialog hooks' bounded stdin and of `answer`'s closed decision must leave the dialog path's lines free of question, answer and tool `input` text, and a refused instance check must still write no line.
  > The question, answer and tool `input` text never appear. A refused instance check writes no line and so carries none of these fields: CLI `answer` (`refuse_untrusted`, before any frame) exits 21 with the instance pair, or the `instance-unreachable` document under `--json`; the dialog hook exits 0 at once with empty streams, no frame and no line.
  (.andromeda/obs-plan.md:173)
  > - **Required log fields:** the question, answer and tool `input` text never appear.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:345) The chunk re-asserts the `hook statusline` stdin cap, the statusline command's `take(MAX_FRAME)` stdout and the `budget.json` checked read on the budget path, which must keep exactly these spans and events and add no line.
  > - **Must-trace spans:** `hook.statusline` › `state.budget_write` › `statusline.shell_out`, then `run.budget_eval`. Log events: `hook-invoked`, `hook-decision` for statusline, and `send-refused` with `budget-paused`.
  (.andromeda/obs-plan.md:87)
  > boundary-only — a user-written shell command: its duration and exit status only; the command string is never logged outside `diagnostics/`.
  (.andromeda/obs-plan.md:181)
  > - **Required log fields:** the `statusline_command` string never appears outside `diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:353) The chunk bounds channel frames and closes their methods and params on both sides; a refused over-size or invalid frame must be logged under this scrub, never with `params` or a result body.
  > - **Required telemetry signal:**
  >   - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  >   - `-32603` is logged as the fixed "internal error".
  >   - `from` is logged as unauthenticated.
  (.andromeda/obs-plan.md:190)
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
  (.andromeda/obs-plan.md:123)
  > `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:362) The chunk re-verifies `validate_paste_text` at the CLI `send` client, the MCP `send` client and the wrapper; each deciding side must emit its own `send-refused` with `control-character` or `empty-text` and no text.
  > - **Required telemetry signal:**
  >   - A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  >   - `send-refused` with detail `control-character` or `empty-text` is emitted by each deciding side, client and wrapper, without the text (tests excerpt, Coverage Triggers, Vector 2).
  (.andromeda/obs-plan.md:195)
  > - **Required telemetry signal:** `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:370) The chunk's `take(MAX_FRAME + 1)` hook stdin must fail open on oversize or malformed JSON, logged to `hook-<name>.ndjson` only, exit 0 and stderr empty.
  > - **Required telemetry signal:**
  >   - Fail-open paths are logged to `hook-<name>.ndjson` only: oversize stdin, malformed JSON, channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:373)
  > - Exit is 0 and stderr stays empty.
  (.andromeda/obs-plan.md:200)
  > - **Required telemetry signal:** fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
  (.andromeda/obs-plan.md:119)
  > without `VIOLA_NAME`, on an instance the instance check refuses, or on a `VIOLA_DIR` naming an instance directory that does not exist, the hook is a silent no-op (exit 0, no file opened, no line, no frame)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:380) The chunk closes `mcp` stdin and every tool argument; a rejected call must be logged with codes only (`invalid-params` with its closed field word), and a frame tool's refused instance check must write no line.
  > Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names. One rejected call writes no line: a frame tool (`send`, `wait`, `last`, `answer`) whose instance check (`check_instance`, before `live_endpoint`) refuses returns `instance-unreachable` with `detail: null` and writes no frame.
  (.andromeda/obs-plan.md:205)
  > - **Required telemetry signal:** rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
  (.andromeda/obs-plan.md:127)
  > `isError` results and the log lines for rejected calls carry codes only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:386) The chunk bounds the `claude --version` and `claude agents --json` spawns' output; their `process-start` lines must carry the R8 strip as names or a count only, never a value.
  > - **Required telemetry signal:**
  >   - A never-log floor for `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every R8-stripped `CLAUDE*` value, across all sinks and fixtures.
  >   - The R8 strip is logged as a count or variable-name list only, never values. Phase 3 must confirm names are admissible.
  (.andromeda/obs-plan.md:210)
  > a never-log floor for every R8-stripped `CLAUDE*` value across all sinks and fixtures; the strip is logged as a count or variable names only, never values, and Phase 3 confirms the names are admissible;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:398) The chunk bounds the `ledger/stamps.json` read in `run`'s version gate, which keeps its one line, and the `budget.json` checked read, whose every failure, oversize included, stays no reading and no line; the bounded `events.ndjson` reader keeps its torn-line count.
  > As landed a strict-modes failure has a line in two places: the `ledger-stamps` read in `run`'s version gate,
  (.andromeda/obs-plan-detail/obs-scope-summary.md:398)
  > the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor),
  (.andromeda/obs-plan-detail/obs-scope-summary.md:400)
  > - `skipped.torn_lines` counts.
  (.andromeda/obs-plan.md:215)
  > a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate
- (.andromeda/obs-plan-detail/obs-scope-summary.md:406) The chunk caps the `claude agents --json` and `--version` output and closes the agents row status; an oversize or malformed read must surface as `unknown` plus a counter, the probes keeping their `process-start` / `process-exit`.
  > - **Required telemetry signal:**
  >   - `process-start` / `process-exit` for the `claude` child and for `claude agents --json` / `--version` probes.
  >   - Parse failures surface as `unknown` plus a counter.
  >   - C0/C1 controls are escaped in any human terminal rendering of logged rows.
  (.andromeda/obs-plan.md:83)
  > a parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
  (.andromeda/obs-plan.md:220)
  > - **Required telemetry signal:** parse failures surface as `unknown` plus a counter, and C0/C1 controls are escaped in any human terminal rendering of logged rows.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:440) The chunk's bounded `events.ndjson` and `snapshot.json` reads must keep the torn-line healing and the corrupt or unsupported-`v` replay recovery telemetry, and the degrade witness reads the readiness-gate `input-not-ready` signal through the gap-(b) resolution.
  > - **Required telemetry signal:** Fault telemetry per class:
  >   - torn ndjson line healed, with file and offset;
  >   - corrupt or unsupported-`v` snapshot → replay recovery;
  (.andromeda/obs-plan-detail/obs-scope-summary.md:447)
  > These need `event` values or an additive `detail` field (section 3, gap b).
  (.andromeda/obs-plan.md:141)
  > - (b) These signals have no `event` value today: heartbeat live→stale transitions, torn-line healing, snapshot replay recovery, readiness-gate `input-not-ready`, and skipped-record counters
  (.andromeda/obs-plan.md:95)
  > `events.ndjson` payload content is never copied into process-log lines
- (.andromeda/obs-plan-detail/obs-scope-summary.md:466) The chunk's forced vt100 screen-model panic witness must still produce exactly one single-line `event:"panic"` record, its home placed where the invariant's count admits it.
  > A zero-unlogged-panics invariant for all four process roles. Each panic produces exactly one single-line `event:"panic"` record and no multi-line backtrace, and `hook` still exits 0.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:478) `send-refused` is the logged event the chunk's re-verified paste refusals write on each deciding side's role file.
  > - **Required telemetry signal:**
  >   - Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
  >   - `send-issued` and `send-refused` are logged events.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:488) The chunk closes channel method params, `release` carrying `from` among them; that refusal must stay its own logged event, never a generic `-32602` wrapper fault.
  > - `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:496) The chunk's closed types and caps are this boundary set: unknown kinds and fields skipped and counted, the `RefusalReason` `unknown` fallback, the `-32602` name checks on `link` / `unlink`, and a parse-failure count per parser for paste validation, framing at `MAX_FRAME`, hook stdin, `claude agents --json` and vt100 under `catch_unwind`.
  > - **Required telemetry signal:**
  >   - Counters for unknown event kinds, unknown fields and torn lines (`skipped`).
  >   - The `RefusalReason` `unknown` fallback (exit 14).
  >   - Higher-`v` rejections (-32602; the code also carries `send`'s invalid params, `release` carrying `from`, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`, so it is not a higher-`v` count alone).
  >   - Per-parser parse-failure counts: paste validation, channel framing at `MAX_FRAME`, hook stdin, `claude agents --json`, vt100 under `catch_unwind`.
  (.andromeda/obs-plan.md:111)
  > the instance comes from `VIOLA_NAME` or an argument, never from the working directory
- (.andromeda/obs-plan-detail/obs-scope-summary.md:508) The chunk's cap on the child's PTY output into the vt100 feed must add no per-chunk line and derive no telemetry from screen content; vt100 yields gate outcomes only.
  > - No telemetry is derived from screen content: vt100 is used for gate outcomes only.
  (.andromeda/obs-plan.md:55)
  > the PTY byte stream is user content, never logged.
  (.andromeda/obs-plan.md:75)
  > paste writes and the vt100 readiness-gate outcome; its internal state and transcripts are never read as a contract.

## Bindings
- **obs-scope-summary (agents parse failure as `unknown` plus a counter) ↔ log-coverage (child / shell spawns)**: the bounded agents read's oversize or malformed outcome is one `parse-rejected{parser:"claude-agents-json"}` and `"unknown"` (.andromeda/obs-plan-detail/obs-scope-summary.md:406) ↔ (.andromeda/obs-plan.md:482)
- **obs-scope-summary (channel scrub) ↔ log-coverage (IPC invocation)**: a bounded or refused frame is logged without `params` or result bodies, not even hashed (.andromeda/obs-plan-detail/obs-scope-summary.md:353) ↔ (.andromeda/obs-plan.md:479)
- **obs-scope-summary (per-parser parse-failure counts, fault telemetry) ↔ log-coverage (levels)**: `parse-rejected` and `state-recovered` lines of the bounded readers are WARN, as is `-32600` / `-32602` (.andromeda/obs-plan-detail/obs-scope-summary.md:496) ↔ (.andromeda/obs-plan.md:460)
- **obs-scope-summary (zero-unlogged-panics) ↔ slo-invariants-telemetry-budgets (zero unlogged panics, G2 count)**: the forced vt100 panic witness's one `event:"panic"` line meets the error budget's count (.andromeda/obs-plan-detail/obs-scope-summary.md:466) ↔ (.andromeda/obs-plan.md:637)
- **obs-scope-summary (no screen-derived telemetry) ↔ slo-invariants-telemetry-budgets (deterministic emission)**: the PTY feed cap adds no line and nothing derives from screen content (.andromeda/obs-plan-detail/obs-scope-summary.md:508) ↔ (.andromeda/obs-plan.md:645)
- **obs-scope-summary (fault telemetry, gap b) ↔ the body (§1 alignment gap b)**: the degrade witness reads `input-not-ready` as the signal gap (b) resolved (.andromeda/obs-plan-detail/obs-scope-summary.md:447) ↔ (.andromeda/obs-plan.md:141)
