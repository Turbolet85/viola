# obs · cascade read · group 2 of 10

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:107) The bounded `claude agents --json` and `claude --version` readers and the closed row-status type must turn every failed parse, an oversize or malformed read included, into `"unknown"` plus a counter, and treat the rows as untrusted.
  > External CLI. Instrument the spawn/exit and the parse outcome. A parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
  (.andromeda/obs-plan.md:83)
  > boundary-only — an external CLI: its spawn/exit and parse outcome; a parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
  (.andromeda/obs-plan.md:220)
  > parse failures surface as `unknown` plus a counter, and C0/C1 controls are escaped in any human terminal rendering of logged rows.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:115) The status line command's stdout, bounded by this chunk's `take(MAX_FRAME)`, must stay a boundary-only shell-out: its duration and exit status are the only telemetry, and the command string is logged nowhere outside `diagnostics/`.
  > A user-written shell command. Instrument the shell-out duration and exit status only. The command string is never logged outside `diagnostics/` (security Data Classifications, `statusline_command`).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:114)
  > boundary-only
  (.andromeda/obs-plan.md:87)
  > boundary-only — a user-written shell command: its duration and exit status only; the command string is never logged outside `diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:132) The bounded reads of `events.ndjson` and `snapshot.json` must copy no `events.ndjson` payload into a process-log line, keep the snapshot's absolute paths out of external output, and keep a replayed snapshot endpoint-less, with no `endpoint`, `pid`, `started_at` or `statusline_command`.
  > `events.ndjson` is itself the product audit stream (brief §7, "the event log as the audit trail"). Its payload content is never copied into process-log lines.
  >   - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  >   - A `snapshot.json` the replay rebuilds from `events.ndjson` (`read_snapshot_or_replay`) carries no `endpoint`, `pid`, `started_at` or `statusline_command`: `live_endpoint` over it is `None` (unreachable), hook `connect` is unreachable (fail open, `channel-unreachable`), `hook statusline` runs and prints nothing, and `run`'s collision check reads the heartbeat alone.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:130)
  > instrumentable (through the `viola-state` call sites)
  (.andromeda/obs-plan.md:95)
  > instrumentable through the `viola-state` call sites; `events.ndjson` payload content is never copied into process-log lines, and the absolute paths in `snapshot.json` and config values are scrubbed from external output.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:143) The closed-type `config.json` read must be observable only at its load-time parse, its telemetry the rejected-key lines and no other.
  > Static configuration and data. Only the parse at load time is observable, and it is logged through `viola-state` / root bin (skipped-key counts).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:142)
  > not-instrumentable
  (.andromeda/obs-plan.md:99)
  > not-instrumentable — static configuration and data; only the parse at load time is observable.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:159) CLI `send`'s client-side `control-character` / `empty-text` refusal must log as `cli` only once its instance check has passed, and a name that resolves no instance must open no process-log file and write no line.
  > A file sink only: JSON-per-line to `diagnostics/` when an instance is resolved (security Vector 6: "write the full error only to `diagnostics/` when an instance is resolved"). Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:159)
  > `send`, `wait`, `last`, `answer`, `pause` and `release` log as `cli` once their instance check (`refuse_untrusted`: `check_instance` on the home and `instances/<name>`) has passed; a refused check exits 21 with the instance pair, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, opens nothing under the home and writes no line.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:162) Every name the chunk validates before a path join must give a log `instance` that is a `ViolaName` from an explicit argument or `VIOLA_NAME`, never from the working directory.
  > The instance comes from `VIOLA_NAME` or an argument, never from the working directory (arch Instance identity rule).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:160)
  > The instance is `ViolaName` from an explicit argument or `VIOLA_NAME`.
  (.andromeda/obs-plan.md:111)
  > the instance comes from `VIOLA_NAME` or an argument, never from the working directory;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:171) The re-verified vt100 degrade must surface as a sync, logged readiness-gate outcome, and the channel server handlers that check over-size frames stay sync with no Tokio.
  > `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
  (.andromeda/obs-plan.md:75)
  > boundary-only — a third-party binary run unmodified: its spawn and exit (exit detected on the process handle, never on EOF), paste writes and the vt100 readiness-gate outcome; its internal state and transcripts are never read as a contract.
  (.andromeda/obs-plan.md:55)
  > the PTY byte stream is user content, never logged.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:173) A vt100 screen-model panic, forced in the chunk's witness or met live, must write nothing to the terminal while the child runs, so the child's bytes keep passing through to the human unharmed and the replaced panic hook sends its line to the file sink, never to stderr.
  > A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:172)
  > N/A. The terminal carries the child's screen.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:176)
  > Rust's default panic hook prints to stderr, which is the child's terminal. It must be replaced before the child spawns.
  (.andromeda/obs-plan.md:115)
  > `run` writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:184) The hook's stdin parse, bounded through `take(MAX_FRAME + 1)`, must keep its required `hook-invoked` / `hook-decision` lines, with an oversize or malformed stdin failing open, logged only in `hook-<name>.ndjson`, and a panic caught as one ERROR `panic` line.
  > `tracing` spans around stdin parse → channel call (`hook.dialog` / `hook.event`) → decision emit, through the tracing-subscriber JSON logger. `hook-invoked` / `hook-decision` lines are required, and panics are caught with `catch_unwind` and emitted as one `level:"ERROR", event:"panic"` line (tests excerpt, Log format, Agent parsing).
  (.andromeda/obs-plan.md:200)
  > fail-open paths log to `hook-<name>.ndjson` only, exit 0 with stderr empty, and drift reports go to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:186) On an oversize or malformed hook stdin, stdout must carry no body and stderr nothing, `hook statusline` printing nothing, and the bounded status line output must be the only thing `hook statusline` prints.
  > A file sink only, to `diagnostics/hook-<name>.ndjson`. stderr is forbidden on both. stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output (arch Standard Contracts, Hook contract); the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body. A failed log write is swallowed, and the hook still exits 0 (tests excerpt, Log format Constraints).
  (.andromeda/obs-plan.md:119)
  > a file sink only, to `diagnostics/hook-<name>.ndjson`; stderr is forbidden, and stdout carries only the decision body (for `viola hook statusline`, the user's status-line command's output); a failed log write is swallowed and the hook still exits 0;
- (.andromeda/obs-plan-detail/obs-scope-summary.md:187) At the hook, the instance must resolve and pass its check before the bounded stdin is read, and a missing, refused or non-existent instance must be a silent exit 0 that reads no stdin and writes no line.
  > If `VIOLA_NAME` is missing, `instance` is null and the hook is a silent no-op (tests excerpt, E1 R5 unwrapped session hooks; Coverage Triggers, Vector 4 "missing `VIOLA_NAME`"). The event and dialog paths resolve the instance directory (`canonical`) and run its strict-modes check (`trusted`: `check_instance`) before `read_diagnostics_level` and obs init: a refused or absent instance is a silent exit 0 with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:191)
  > A `VIOLA_DIR` naming an instance directory that does not exist resolves none, so the hook writes nothing.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:197) The closed-type reading of channel params must skip unknown fields rather than refuse them, and declares no trace-context field.
  > Channel `params` declare no trace-context field (arch Standard Contracts, `send`: "No trace-context field is declared"). Readers skip unknown fields (arch Mixed-version tolerance), so an additive envelope field would be wire-compatible, but none is required by upstream.
  (.andromeda/obs-plan.md:123)
  > `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.

## Bindings
- **scope-entities ↔ log-coverage (`parse-rejected` catalog)**: the `"unknown"` plus counter for a failed `claude agents --json` read (.andromeda/obs-plan-detail/obs-scope-summary.md:107) and the skipped-key counts of the `config.json` parse (.andromeda/obs-plan-detail/obs-scope-summary.md:143) are what the `parse-rejected{parser:"claude-agents-json"}` and per-key `parse-rejected{parser:"config-json"}` lines carry; the chunk's closed-type reads must emit those lines and add no other parser value outside the §6 catalog.
- **scope-entities ↔ scope-surfaces**: the store scrub rules for bounded `events.ndjson` / `snapshot.json` reads (.andromeda/obs-plan-detail/obs-scope-summary.md:132) hold on every surface that reads them, and a replayed snapshot leaves hook `connect` unreachable and `hook statusline` printing nothing (.andromeda/obs-plan-detail/obs-scope-summary.md:186).
- **scope-surfaces (run) ↔ span-trace-coverage / error-capture**: the logged readiness-gate outcome of the vt100 degrade (.andromeda/obs-plan-detail/obs-scope-summary.md:171) and the forced panic kept off the terminal (.andromeda/obs-plan-detail/obs-scope-summary.md:176) bind together: the witness reads the gate's `input-not-ready` signal and the panic hook's one line in the file sink, with the child's bytes still on the terminal (.andromeda/obs-plan-detail/obs-scope-summary.md:173).
- **scope-surfaces (cli, hook) ↔ security-plan names before path joins**: the instance a log line names (.andromeda/obs-plan-detail/obs-scope-summary.md:162) and the no-instance silence of the cli and hook surfaces (.andromeda/obs-plan-detail/obs-scope-summary.md:159, .andromeda/obs-plan-detail/obs-scope-summary.md:191) are the observable side of `ViolaName::try_new` before every path join: a refused name opens no role file and writes no line.
- **scope-surfaces (ipc-internal) ↔ standing mixed-version rule**: channel params readers skip unknown fields (.andromeda/obs-plan-detail/obs-scope-summary.md:197), so the chunk's closed-type params check refuses an invalid value, never an unknown field.
