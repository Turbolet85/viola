# obs · cascade read · group 1 of 10

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:19) The re-verified vt100 panic degrade must read its signal through the resolved `event` extension (the chaos signal and the readiness gate's `input-not-ready`), adding no new value outside the §6 catalog's resolution.
  > - the `event` enum extensions (`release-from-driver` and the chaos signals);
  (.andromeda/obs-plan.md:141)
  > (b) These signals have no `event` value today: heartbeat live→stale transitions, torn-line healing, snapshot replay recovery, readiness-gate `input-not-ready`, and skipped-record counters (tests excerpt, Coverage Triggers: Crash-safe state, Parser surfaces, Child spawn). Phase 3 chooses between an additive field (for example `detail` on `process-*`) and an enum extension.
  (.andromeda/obs-plan.md:264)
  > Until a wrap brings a difference current, §3, §6 and §12 win where they differ: diagnostics root (D-08, D-22), `event` enum extensions (D-01…D-05, D-21),
- (.andromeda/obs-plan-detail/obs-scope-summary.md:27) The root bin's readers the chunk bounds (hook stdin, `send`'s `--file` / stdin text, `config.json`, the agents and version reads) must emit their refusal and parse lines there, the instance resolved only from `VIOLA_NAME` or a validated argument, and `config.json` observed only at its load-time parse.
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:111)
  > the instance comes from `VIOLA_NAME` or an argument, never from the working directory;
  (.andromeda/obs-plan.md:99)
  > - **Instrumentability:** not-instrumentable — static configuration and data; only the parse at load time is observable.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:36) The unit witnesses of `ViolaName::try_new` and `validate_paste_text` must expect no log line from `viola-core`; the `send-refused` line is each deciding side's own and carries no text.
  > - **Reason if not fully instrumentable:** It is a pure type crate with no I/O and no runtime behaviour of its own. It supplies the kebab-case values used as log fields (`refusal`, `detail`, event kinds) but emits nothing. Excluded from sections 2 to 4.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:35)
  > - **Instrumentability:** not-instrumentable
  (.andromeda/obs-plan.md:51)
  > - **Instrumentability:** not-instrumentable — a pure type crate with no I/O that supplies log-field values but emits nothing; excluded from sections 2 to 4.
  (.andromeda/obs-plan.md:195)
  > - **Required telemetry signal:** `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:44) The size cap on the child's PTY output into the vt100 feed must log no byte of that stream, an over-cap or panic outcome reaching a line only as a gate outcome or a count.
  > - **Reason if not fully instrumentable:** N/A for the seam. The ConPTY and openpty OS APIs behind it are boundary-only (see the OS PTY entry). The PTY byte stream is user content and is never logged (security Data Classifications, user content; brief §3.2 "Nobody parses the rendered screen for content").
  (.andromeda/obs-plan.md:55)
  > - **Instrumentability:** instrumentable at the seam; the ConPTY and openpty OS APIs behind it are boundary-only, and the PTY byte stream is user content, never logged.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:52) The channel's frame and param bounds on both sides, and the closed method type over the listed methods, must log synchronously with no Tokio in the sync graph, never record `params` or result bodies, answer `-32603` as the fixed `internal error`, and log `from` as unauthenticated.
  > - **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:51)
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan-detail/obs-scope-summary.md:50)
  > - **Source:** arch excerpt, Stack (interprocess 2.4.4), Workspace, Standard Contracts (IPC methods `send · wait · last · answer · pause · release · link · unlink · hook.dialog · hook.event`)
  (.andromeda/obs-plan.md:59)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Tokio is forbidden in this crate's graph unless its feature is enabled.
  (.andromeda/obs-plan.md:123)
  > `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
  (.andromeda/obs-plan.md:190)
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:60) The bounded reads of `snapshot.json`, `events.ndjson`, `budget.json` and `ledger/stamps.json` must keep their lines sync and Tokio-free, copy no `events.ndjson` payload into a process-log line, and give a strict-modes failure a line only on the `ledger-stamps` read in `run`'s version gate.
  > - **Reason if not fully instrumentable:** N/A. Stack bans Tokio here, so instrumentation must be sync.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:59)
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:63)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Stack bans Tokio here.
  (.andromeda/obs-plan.md:95)
  > - **Instrumentability:** instrumentable through the `viola-state` call sites; `events.ndjson` payload content is never copied into process-log lines, and the absolute paths in `snapshot.json` and config values are scrubbed from external output.
  (.andromeda/obs-plan.md:215)
  > a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate
- (.andromeda/obs-plan-detail/obs-scope-summary.md:68) The bounded hook payload and `claude agents --json` parses must send any drift report to `diagnostics/` only, and the `claude --version` and `claude agents --json` spawns under the R8 strip must log names or counts only, never a `CLAUDE*` value.
  > - **Reason if not fully instrumentable:** N/A. Its outputs are the most sensitive in the product:
  >   - Drift reports go to `diagnostics/` only (security Vector 4).
  >   - R8-stripped `CLAUDE*` values are never logged anywhere (security Vector 6).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:67)
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan-detail/obs-scope-summary.md:66)
  > - **Source:** arch excerpt, Workspace; Stack (serde_path_to_error drift reports)
  (.andromeda/obs-plan.md:67)
  > - **Instrumentability:** instrumentable, its outputs the most sensitive in the product: drift reports go to `diagnostics/` only, and R8-stripped `CLAUDE*` values are never logged anywhere.
  (.andromeda/obs-plan.md:210)
  > a never-log floor for every R8-stripped `CLAUDE*` value across all sinks and fixtures; the strip is logged as a count or variable names only, never values,
- (.andromeda/obs-plan-detail/obs-scope-summary.md:77) The `mcp` stdin and tool-argument bounds must log a rejected call codes-only to `mcp.ndjson` (stderr JSON only when that file cannot be opened), with nothing on stdout but MCP frames and no upstream text, absolute path or type name.
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan-detail/obs-scope-summary.md:18)
  > - `mcp` logging to stderr when `VIOLA_DIR` is unset — reconciled by D-09 and landed (chunk 2026-10-17-mcp-server-for-drivers): `mcp` always writes `mcp.ndjson`, stderr JSON only when that file cannot be opened;
  (.andromeda/obs-plan.md:71)
  > - **Instrumentability:** instrumentable; stdout is reserved for MCP frames, so no telemetry may go to stdout.
  (.andromeda/obs-plan.md:127)
  > always the file sink `<home>/diagnostics/mcp.ndjson`, stderr JSON only when that file cannot be opened; stdout is forbidden because it carries MCP frames, so the writer is set explicitly; `isError` results and the log lines for rejected calls carry codes only.
  (.andromeda/obs-plan.md:205)
  > - **Required telemetry signal:** rejected tool calls are logged with codes only, with no upstream text, absolute paths or type names.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:86) The re-verified vt100 panic degrade is the readiness-gate outcome this boundary logs: it must surface as a gate outcome, the child's exit still read on the process handle, and `run` writing nothing of its own to the terminal while the child runs.
  > - **Reason if not fully instrumentable:** It is a third-party binary run unmodified. Instrument only the boundary:
  >   - spawn and exit (`process-start` / `process-exit`), with exit detected on the process handle, never on EOF (Stack; brief §4.1);
  >   - paste writes;
  >   - the vt100 readiness-gate outcome.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:91)
  >   Its internal state and transcripts are never read as a contract (creator Obs Anti-Patterns, brief §4).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:85)
  > - **Instrumentability:** boundary-only
  (.andromeda/obs-plan.md:75)
  > - **Instrumentability:** boundary-only — a third-party binary run unmodified: its spawn and exit (exit detected on the process handle, never on EOF), paste writes and the vt100 readiness-gate outcome; its internal state and transcripts are never read as a contract.
  (.andromeda/obs-plan.md:115)
  > `run` writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns; instrumentation is sync, with no Tokio;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:99) The hook stdin bound under `take(MAX_FRAME + 1)` must fail open with its outcome on `hook-decision` in `hook-<name>.ndjson` only (exit 0, stderr empty), and a hook whose instance does not resolve or is refused must stay a silent no-op.
  > - **Reason if not fully instrumentable:** The caller is Claude Code, so viola sees only the inbound stdin payload and its own response. Instrument `hook-invoked` / `hook-decision` and MCP tool-call handling. No trace context arrives from Claude Code.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:98)
  > - **Instrumentability:** boundary-only
  (.andromeda/obs-plan.md:79)
  > - **Instrumentability:** boundary-only — viola sees only the inbound stdin payload and its own response; no trace context arrives from Claude Code.
  (.andromeda/obs-plan.md:119)
  > without `VIOLA_NAME`, on an instance the instance check refuses, or on a `VIOLA_DIR` naming an instance directory that does not exist, the hook is a silent no-op (exit 0, no file opened, no line, no frame);
  (.andromeda/obs-plan.md:200)
  > - **Required telemetry signal:** fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:106) The bounded `claude agents --json` and `claude --version` reads must be logged only at their boundary (spawn, exit, parse outcome), an oversize or malformed read becoming `"unknown"` plus a counter with the untrusted rows never logged.
  > - **Instrumentability:** boundary-only
  (.andromeda/obs-plan.md:83)
  > - **Instrumentability:** boundary-only — an external CLI: its spawn/exit and parse outcome; a parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
  (.andromeda/obs-plan.md:220)
  > - **Required telemetry signal:** parse failures surface as `unknown` plus a counter, and C0/C1 controls are escaped in any human terminal rendering of logged rows.
- (.andromeda/obs-plan.md:310) The hook stdin bound must keep `hook`'s stdout to its decision body and `viola hook statusline`'s to the command's output, printing nothing on oversize or malformed stdin, with nothing on stderr.
  > `hook` writes only its decision body to stdout and nothing to stderr. The one other thing a `hook` stdout carries is `viola hook statusline`'s: the user's status-line command's output byte for byte (nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden `--capture` arm, which prints nothing). No telemetry reaches stdout on either arm.
- (.andromeda/obs-plan.md:311) The re-verified passthrough must show the human's terminal carrying the child's own bytes through a vt100 panic, with `run` writing nothing of its own while the child runs.
  > `run` writes nothing to the terminal except the child's own output, with one exception before any child is spawned:
  (.andromeda/obs-plan.md:311)
  > While the child runs, neither `run` nor a revived start writes anything of its own.

## Bindings
- **obs-scope-summary (the child's vt100 readiness-gate outcome) ↔ log-coverage, slo-invariants-telemetry-budgets and obs-anti-patterns**: the panic degrade is witnessed through the resolved gap (b) signal and the one `parse-rejected{parser:"vt100-feed"}` per poisoning episode, the forced panic's own `event:"panic"` line counted inside G2's bounded exemption, and no per-chunk line in the feed, (.andromeda/obs-plan-detail/obs-scope-summary.md:86) (.andromeda/obs-plan.md:141)
- **obs-scope-summary (`viola-core` emits nothing) ↔ log-coverage (`send-refused`)**: the paste rule's refusal line is written by the CLI client, the MCP client and the wrapper, never by the validator, and carries no text, (.andromeda/obs-plan-detail/obs-scope-summary.md:36) (.andromeda/obs-plan.md:195)
- **obs-scope-summary (`claude agents --json` boundary-only) ↔ log-coverage and pii-scrubbing-compliance**: an oversize or malformed read is one `parse-rejected{parser:"claude-agents-json"}` and `"unknown"`, the rows never verbatim, the spawn under the R8 floor, (.andromeda/obs-plan-detail/obs-scope-summary.md:106) (.andromeda/obs-plan-detail/obs-scope-summary.md:68)
- **obs-scope-summary (hook boundary-only) ↔ error-capture-reporting and log-coverage**: oversize or malformed hook stdin fails open with a `hook-decision` `detail` in `hook-<name>.ndjson` and no body, (.andromeda/obs-plan-detail/obs-scope-summary.md:99) (.andromeda/obs-plan.md:310)
- **obs-scope-summary (channel sync instrumentation) ↔ metric-coverage**: a new `-32602` from a channel `name` / `driver` / `driven` check adds to `viola.compat.v_rejected`'s over-count while the line does not tell the causes apart, (.andromeda/obs-plan-detail/obs-scope-summary.md:52)
- **obs-scope-summary (`viola-state` sync instrumentation) ↔ log-coverage**: the bounded snapshot read keeps one `state-recovered` at WARN per call and the `events.ndjson` reader keeps its three skip counts, (.andromeda/obs-plan-detail/obs-scope-summary.md:60) (.andromeda/obs-plan.md:95)
- **obs-scope-summary (`mcp` instrumentable) ↔ error-capture-reporting**: a rejected `mcp` call is an `isError` with codes only and `instance-unreachable` gets no line, (.andromeda/obs-plan-detail/obs-scope-summary.md:77) (.andromeda/obs-plan.md:127)
