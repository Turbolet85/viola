# obs · cascade read · group 6 of 9

## Carried
- (.andromeda/obs-plan-detail/span-trace-coverage.md:110) The CLI and MCP witnesses and the enumeration must name the `wait` / `last` refused-check bodies exactly as stated, the CLI document with a null `detail` and the MCP result with no line, and every `detail` on the exit path a closed code.
  > client `process-exit{subject:"self", exit_code, detail}` at WARN, where exit 21 carries `detail:"instance-dead"` plus `during` ∈ `connect|call` (endpoint vanished mid-wait = `during:"call"`), or `detail:"server-verify-failed"` with `during:"connect"` when the serving process is not the recorded wrapper or the check cannot be made (Linux and Windows; macOS on the recorded gap). `wait` / `last`, like `send`, first take the instance check (`client::refuse_untrusted`, before any role file opens), then the liveness-only pre-check and then the server check; a refused instance check exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and sends no frame and writes no line, and MCP `wait` / `last` refused by `check_instance` return `instance-unreachable` with no line.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:109)
  > `channel-response{corr:<id>, conn, result_class, duration_ms}` on both sides, plus `outcome` for method `wait` only: the waking kind (`turn-ended|question|permission|plan|session-end|dialog-expired`, the six of `EventKind::WAIT_WAKE`) or `timed-out`, derived once in `viola-channel` from the result over the shared wake set; the client side alone also writes `instance-unreachable` when the reply never arrived (a closed or reset connection). A `last` response carries no `outcome`, and no result body reaches either line;
  (.andromeda/obs-plan.md:361)
  > first, a home or instance tree the instance check refuses ends a CLI `wait` / `last` before any role file opens (exit 21, the instance pair, or exactly `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`; no line), and MCP's returns `instance-unreachable` with no line.
  (.andromeda/obs-plan.md:333)
  > the rmcp and portable-pty targets are OFF in the sink; channel lines never record `params` or result bodies.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:120) The channel enumeration must name this failed link record as a `-32603` site, answered exactly `"internal error"` with null data, whatever the record's own error held.
  > A record that fails answers `-32603`: the answer is not taken, the dialog stays held and no `dialog-answered` line is written.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:121)
  > CLI `answer` runs `client::refuse_untrusted` before any role file opens, and MCP `answer` runs `check_instance` before `live_endpoint`, each refusal with no frame and no line.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:124) The cap-driving case owed to the dialog hold's `config.json` reader must assert the hold an over-cap file leaves against this range and default.
  > `run.dialog_await`: `deadline_ms` (the hold the slot was started with: `config.json`'s `dialog_hold_s` in ms, 60 000–3 600 000, default 60 000), `deadline_hit`;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:132) `answer`'s refusal bodies must be codes only on the CLI and MCP surfaces, these two named in the enumeration.
  > driver-side refusals: `channel-response{result_class:"refusal", refusal:"not-delivered", detail:"unknown-dialog"}` (exit 13), or `refusal:"unverified-cli"` (exit 12, answer withheld).
  (.andromeda/obs-plan-detail/span-trace-coverage.md:133)
  > a server-verification refusal on CLI `answer`, made before any frame: no `channel-request` / `channel-response` line, `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN; MCP `answer` writes no line.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:134)
  > a refused instance check on CLI `answer` (`client::refuse_untrusted`, before any role file opens): exit 21 with the instance pair, or under `--json` the `instance-unreachable` document, no `channel-request`, no `process-exit` and no line; MCP `answer` refused by `check_instance` returns `instance-unreachable` with `detail: null` and writes no line.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:135)
  > The question, answer and tool `input` never appear.
  (.andromeda/obs-plan.md:369)
  > The question, answer and tool `input` never appear;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:144) The channel enumeration must name a `pause` or `release` whose `wheel` record did not land as a `-32603` site with the fixed body.
  > `channel.dispatch(pause|release)`, answered only after the move's `wheel` record lands (`-32603` if it did not).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:151) The `-32602` carry must tell these causes apart without changing their wire bodies, and the enumeration must reckon with `data: {"reason":"release-from-driver"}` beside the scope's "a code or null", the one `-32602` here whose `data` is an object.
  > when `release` carries a string `from` (CLI `viola release` forwards `VIOLA_NAME`, so a driver session's own `viola release` counts), one `release-from-driver{corr:<rpc id>, conn, from (only when a valid name), from_trust:"self-reported"}` at INFO (D-01), beside the `-32602` `"invalid params"` reply with `data: {"reason":"release-from-driver"}`; a `from` of another type, or a non-bool `budget`, is a plain `-32602` (`data: null`) with no such line, and a plain `release` on a driver-held wheel appends nothing.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:148)
  > `send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}`, with the client `process-exit{exit_code:10}`;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:166) The cap-driving case on the status-line command's stdout must leave the command string out of every span and line it exercises.
  > Never `statusline_command`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:171) The status-line case must assert the decision line its driven path writes, and only these closed `detail` codes on a fail-open.
  > `hook-decision{hook_event:"statusline", budget_written, duration_ms}`, plus `detail` ∈ `oversize-stdin|malformed-json|deadline` on a fail-open path, and `deadline_hit` written only when true. `budget_written` is true only when the payload held a `rate_limits` object and the write succeeded;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:170)
  > `process-start` / `process-exit{subject:"statusline-shell", shell_exit_status, duration_ms}`, only when the snapshot records a `statusline_command` and a shell is built (never on Windows, where `shell_argv` is `None`);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:173)
  > `send-refused{corr:<cursor>, refusal:"budget-paused", detail:"five-hour"|"seven-day", window, used_percentage, resets_at, paused:true}` with client `exit_code:11`.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:174) The status-line case re-asserts this behaviour past the cap: the command's stdout never reaches a line, and the plan must state what passes through once the read is bounded.
  > **Cleanup:** `statusline.shell_out` closes on child exit or at the bound `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`); the bound ends in `hook-decision{detail:"deadline", deadline_hit:true}`. The user's stdout passes through unchanged and is never logged. Oversize or malformed stdin prints nothing and runs no shell.
  (.andromeda/obs-plan.md:385)
  > Never `statusline_command`; the user's stdout passes through unchanged and is never logged; the shell-out is bounded by `STATUSLINE_DEADLINE` (5 s, PROVISIONAL).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:180) The witnesses' planted stripped `CLAUDE*` value must appear on no surface and in no sink, the strip recorded by count and names alone.
  > **E2** (R8 strip): `env_stripped_count`, `env_stripped_known` (the stripped names) and `env_kept` (the persistent-set names kept) on `process-start{subject:"claude-child"}`; names only, never values (D-34).
  (.andromeda/obs-plan-detail/span-trace-coverage.md:181)
  > The strip removes every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) except the persistent set (Windows `HKCU`/`HKLM` `Environment` value names; Unix `config.json` `claude_env_keep`), and always removes the 11 names of the identity floor `IDENTITY_FLOOR`, measured on the Windows host, whatever the persistent set says (a differing Linux dev-host reading on `claude` 2.1.287 stands beside that origin and moves nothing: architecture §Occupied Resources → Environment variables); beyond the floor there is no compile-time list of known names. `env_stripped_known` names every stripped name and `env_kept` every kept name, each comma-joined; no value is read by the strip, the registry reader or any log, and no env map is passed to a span or event (D-34).
  (.andromeda/obs-plan.md:390)
  > names only, never values, and no env map is passed to a span or event.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:193) The `-32602` carry must tell a bad `link` or `unlink` name or pair apart from an unsupported version, with `data: null` and no line of its own kept, and the enumeration must name the `-32603` of a failed `record` or `remove`.
  > **E3**, the explicit methods: the driven wrapper serves `link` / `unlink`, reading `driver` and `driven` as strings through `ViolaName::try_new`; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` with `data: null` and no line written, a `record` / `remove` error is `-32603`, and neither method reads `from`.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:194)
  > Neither method opens a span or writes a process-log line of its own: its trace is the driven wrapper's `channel-request` / `channel-response` pair with `method:"link"|"unlink"`, which holds neither the `driver` nor the `driven` name, then the product line and the snapshot's `links`.
  (.andromeda/obs-plan.md:392)
  > their trace is the call's `channel-request` / `channel-response` pair, the product `link` / `unlink` line in `events.ndjson` and the snapshot's `links`;
- (.andromeda/obs-plan-detail/span-trace-coverage.md:217) The `AgentError` enum the chunk edits must keep the ledger's path and stamp contents out of its messages, whichever way the `Refusal` carry falls.
  > The path and stamp contents never appear.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:216)
  > An unreadable or unparsable ledger, or one the strict-modes check refuses (`run`'s version gate reads it through `viola_state::stamps::read_stamps_strict`), gives `parse-rejected{parser:"ledger-stamps", detail:"unreadable"|"malformed"|"strict-modes-failed", count}` in `run-<name>.ndjson` and is treated as unverified (`cli_verified:false`).
  (.andromeda/obs-plan.md:398)
  > the path and stamp contents never appear.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:227) The CLI enumeration must name `plugin install`'s refused-check document, and its witness must hold no path in it.
  > `plugin install` writes that pair only after its strict-modes tree check has passed (`process-exit` exit 0, no detail); a refused check opens no log and writes no line (exit 21: the stderr pair, or under `--json` the `instance-unreachable` document). Its outcome is the exit code (0 / 1 / 2 / 21) and, under `--json`, `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`; no path in either document or any line.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:226)
  > Refusals go to stderr as the fixed `unable:` + `hint:` pair, and an internal fault prints exactly `error: internal error`.
  (.andromeda/obs-plan-detail/span-trace-coverage.md:225)
  > each a `process-start{subject}` / `process-exit{subject, child_exit_status, duration_ms}` pair: `version-probe` for the `--version` read, `verify-probe` for the print-mode probe, then one `agents-probe` pair for Run B's single `claude agents --json` read (made once its first turn has settled, before the framing pastes; the capture held in memory; no row text, session id, `cwd` or path in a line; chunk 2026-10-16-unwrapped-sessions-on-the-board)
  (.andromeda/obs-plan-detail/span-trace-coverage.md:231)
  > Its home-level lines, when `VIOLA_NAME` is set, fall under G2, G4 and the secret scan like any other.
  (.andromeda/obs-plan.md:400)
  > no line carries row text, a prompt, a session id or a path;

## Bindings
- **The `-32602` causes on the `channel-response` line ↔ the derived metric**: telling the causes of `release` and `link` / `unlink` apart must end the recorded over-count of `viola.compat.v_rejected`, and any field that does it must enter the §6 catalog and its schema, `(.andromeda/obs-plan-detail/span-trace-coverage.md:151)`, `(.andromeda/obs-plan-detail/span-trace-coverage.md:193)` and `(.andromeda/obs-plan.md:418)`.
- **The `-32603` sites ↔ error capture and the ipc surface**: each failed record site must answer the fixed body the error layers require, with no chain reaching `error.data`, `(.andromeda/obs-plan-detail/span-trace-coverage.md:120)`, `(.andromeda/obs-plan-detail/span-trace-coverage.md:144)`, `(.andromeda/obs-plan.md:123)` and `(.andromeda/obs-plan.md:510)`.
- **The `instance-unreachable` bodies ↔ error capture**: a frame tool's refused check must keep the `isError` code its only record, on the MCP witness and in the enumeration, `(.andromeda/obs-plan-detail/span-trace-coverage.md:110)`, `(.andromeda/obs-plan-detail/span-trace-coverage.md:134)` and `(.andromeda/obs-plan.md:537)`.
- **The refusal and fail-open `detail` values ↔ the `detail` code catalog**: every code an error document or line carries must be a member of the closed catalog, never a path or pid, `(.andromeda/obs-plan-detail/span-trace-coverage.md:132)`, `(.andromeda/obs-plan-detail/span-trace-coverage.md:171)`, `(.andromeda/obs-plan-detail/span-trace-coverage.md:173)` and `(.andromeda/obs-plan.md:451)`.
- **The names-only strip ↔ the secret-scan leg of the floor**: the planted stripped value must be found nowhere the scan reads unless a real leak put it there, `(.andromeda/obs-plan-detail/span-trace-coverage.md:181)`, `(.andromeda/obs-plan-detail/span-trace-coverage.md:231)` and `(.andromeda/obs-plan.md:570)`.
- **The ledger's silence and the `Refusal` carry ↔ the §7 enum list**: folding or ratifying `Refusal` must leave every `AgentError` message on the fixed-message list, holding no path or payload, `(.andromeda/obs-plan-detail/span-trace-coverage.md:217)` and `(.andromeda/obs-plan.md:496)`.
