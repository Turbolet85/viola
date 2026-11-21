@history_item n=1 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the mcp sink is always mcp.ndjson) `viola mcp` always logs to `<home>/diagnostics/mcp.ndjson` (D-09), its home and `diagnostics/` checked before the sink opens; this became decision/mcp-log-sink, decision/mcp-log-file-sink and contract/diagnostics-files/mcp. Any line a new per-tool deadline writes goes there and never to stdout (decision/mcp-stdout-frames-only).

@history_item n=2 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the list tool's lines, the refused home check, invalid-params) the `list` tool writes the `agents-probe` pair and the board read's `parse-rejected` / `state-recovered` / `config-json` lines into `mcp.ndjson`; a refused home check exits 1 with no line (exemption 7); the codes-only rejects add `invalid-params`, its `detail` a closed field word. This is the precedent if a capped caller argument (for example `timeout_ms`) is refused: it is `invalid-params` with a closed field word (contract/mcp-invalid-params/detail already names `timeout_ms`). It also matters because `list`'s longest block includes the agents spawn.

@history_item n=3 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `run::agents::read_logged` writes the `agents-probe` pair for `list`, `revive` and verify's Run B, while `run_bounded` / `run_capped` stay unlogged. The `list` tool's block is the agents read under those bounds. Bounding it below the client timeout must keep this one pair as the record and add no line to the bounded runner.

@history_item n=4 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — scoped the wait channel lines: `after` / `timeout_ms` are `u64` only and ride `wait` alone, on both sides, and `outcome` (the waking kind or `timed-out`) is derived once over the shared wake set. A cap on `wait`'s caller-supplied `timeout_ms` lands on these fields (contract/wait-log-fields/channel-request, contract/wait-log-line/required-fields, contract/wait-span-attributes/wait-block), and a capped expiry must still read `timed-out`, with no new outcome word.

@history_item n=5 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival at the wrapper, so it includes the readiness gate's wait, up to architecture's `GATE_MAX_WAIT` (up to 8.5 s on a delivered send), ahead of the confirm window. The bound is named by its constant, never copied. This is one term of `send`'s longest block, and it is the field that shows that block in the log (contract/send-log-fields/send-confirmed).

@history_item n=6 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL) for a `/compact` confirmed by its session; every other confirmed send waits 10 s, and `window_ms` reads `SendSlot::window()`. This 40 s window plus the gate's wait is `send`'s longest block. It sits closest to any client tool-call timeout and is what the chunk must bound below it (decision/compact-confirm-window, contract/send-span-attributes/window-ms, contract/confirm-window-close/compact).

@history_item n=7 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `run.confirm_window` closes on the confirmation its send waits for, or on expiry (`no-prompt-submitted`). An unconfirmable send opens no window and ends at `pty.paste_write` with `send-confirmed{confirmed:false}`. This sets which `send` paths block for a window and which do not, which matters for the per-path bound.

@history_item n=8 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — an echo-confirmed local command is confirmed by its transcript stdout tag inside `run.confirm_window`, awaits no hook line and expires as `no-prompt-submitted`. The read itself is uninstrumented (+ decision/transcript-echo-read-uninstrumented; ~ contract/send-path-log-events/event-order; ~ contract/send-refused-detail/no-prompt-submitted). This is another confirm-window path inside `send`'s block, and any bound must keep its no-new-line shape.

@history_item n=9 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — + contract/confirm-window-close/context: `/context` closes on a new local-command-stdout tag within the fallback window; unverified, it reads unconfirmable. One more window-bounded `send` path to cover in the bound census.

@history_item n=10 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — ~ contract/confirm-window-close/ordinary-send (prompt equality with a trailing-whitespace tolerance) and ~ contract/send-refused-detail/turn-running (the post-gate turn read, link record, `send-issued` and paste under one flight lock; a running turn or another send in flight refuses at once, client exit 13). Here `send`'s wait sits in the wrapper's gate and window, not in a queue behind another send.

@history_item n=11 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — `turn-running` covers a running turn or another `send` in flight under one closed detail with no new field: no cursor, no `send-issued`, exit 13. A second send never waits for the first. This is a precedent for the bound: a refusal, not a queue.

@history_item n=12 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` (60 000–3 600 000 ms, default 60 000) is read at `run`'s start, the wrapper writes `dialog-expired` at the hold's end, and `woken_kind` / `outcome` gained `dialog-expired`, six kinds in all. This is a configured long wait that a `wait` caller can park on. It is a precedent for a bound read from `config.json` (one `parse-rejected{config-json, malformed}` on a bad value).

@history_item n=13 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the pre-frame server check's place in the span tree) `viola mcp`'s `send`, `wait`, `last` and `answer` verify after `TokioClient::connect` inside `mcp.tool_call` (`server_verified`, the start time read on `tokio::task::spawn_blocking`). A refusal opens no `channel.request`. These pre-frame steps are part of each tool's block inside `mcp.tool_call` (contract/mcp-spans/mcp-tool-call, contract/span-kinds/server).

@history_item n=14 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (MCP's `instance-unreachable` reject writes no line; the `wait` outcome count) a logged MCP reject's line is codes-only, and `instance-unreachable` is logged by no line, the `isError` code its only record. `viola.wait.outcome` counts only `wait` calls answered by a `channel-response`. If a bounded tool times out client-side, the chunk must decide whether that is a codes-only line or a no-line reject (decision/instance-unreachable-not-logged, decision/mcp-rejects-codes-only).

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the MCP frame tools' instance check before the liveness read) `frame` runs `check_instance` before `live_endpoint`; a refusal returns `instance-unreachable`, with no frame and no line. This is the order of the pre-frame steps inside every MCP frame tool's block (contract/mcp-frame-tool-refusal/strict-modes-failed).

@history_item n=16 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the `viola mcp` stdin line cap; D-05 gains `mcp-stdin`) rmcp's unbounded `read_until` was bounded by the reader viola hands it (`CappedLines` at `MAX_FRAME`). A line past it ends the session as a closed stdin with one `parse-rejected{mcp-stdin, oversize}`, and a new closed `parser` value took a Decisions Log record. This is the direct precedent for bounding an rmcp-side behaviour in viola's own code, and for the rule that a new closed word needs a log entry (contract/parse-rejected/mcp-stdin-oversize, contract/mcp-stdin/*).

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the mcp role writes no self process-exit; exit at an oversize line before initialize) `internal_error_exit_line` covers `Run` and `Cli` only, so `mcp` writes no self `process-exit`. A cut before `initialize` exits 1 with both streams empty (11 is the unit child's code alone). A bounded tool must not assume a self exit line on the mcp role (contract/panic-outcome-by-role/mcp-main-thread-panic, contract/mcp-stdin/oversize-before-initialize).

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (a peer's reply screened by `sanitised_fault` / `is_code_word`; `channel-response`'s `detail` closed to three words) `of_reply` (`crates/viola-mcp/src/result.rs`) screens every peer fault to five fixed codes, and `channel-response.detail` is only `unsupported-version|invalid-params|release-from-driver`. A timeout result from a tool must fit these closed shapes, with no upstream text (decision/clients-screen-peer-replies-to-code-words, contract/mcp-iserror/wrapper-fault, contract/channel-response/detail).

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (MCP's `instance-unreachable` body names its cause; exit 21's logged causes and their `during`) the `isError` body is the CLI's document less `v` with one `UnreachableCause` word. In `of_reply`, `Connect` maps to `InstanceDead` and `Closed`/`Io` to `EndpointLost`. A wrapper lost mid-wait reads `endpoint-lost` `during:"call"`. The MCP-side result of a mid-call cut, or of a tool bounded below the client timeout, maps through this closed set (contract/mcp-iserror/instance-unreachable, contract/unreachable-cause/endpoint-lost, contract/cli-exit-21/endpoint-lost).

@history_item n=20 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (MCP's frame tools: the socket-directory and euid refusals) `frame` runs `check_endpoint_dir` after `live_endpoint`, and inside `mcp.tool_call` it runs `peer_admitted` then `server_verified`. Refusals give `socket-dir-refused` / `server-verify-failed` with no frame and no line, and exit 21's causes went from five to six. This completes the list of pre-frame steps in each MCP tool's block and the closed cause set (contract/mcp-frame-tool-refusal/*, contract/mcp-instance-unreachable/*).

@history_item n=21 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — removed the send path's budget rung and retired exit 11 ("11 is retired and never reused", ~ decision/exit-code-log-field). A release with any budget key reads `invalid-params`. If a bounded `send` needs a new exit or detail, 11 is not available, and the `send-refused` detail words are closed to the arch `RefusalReason` set (contract/send-refused-detail/refusal-reason-words).

@history_item n=22 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (the `empty-text` detail) the client-side validation refusals (`control-character`, `empty-text`) are made before any frame, one line per deciding side, never the text. They are the client-side checks at the head of `send`'s whole path that the scope names (contract/send-refused-client/*, decision/send-refused-by-each-deciding-side).

@history_item n=23 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — the three content-free MCP inputs derive neither veil `Redact` nor `Debug`, because veil 0.3.0 refuses a `Redact` derive with no redact field (~ decision/payload-types-derive-redact; ~ decision/veil-redact-payload-types except). A cap or new field on a tool's argument type moves it across this line.

@history_item n=24 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — added `verify-probe` to the spawn subjects; verify logs its spawns at the call site and `run_bounded` stays unlogged. In CI, verify runs only against the fake agent; the real-CLI verify runs only through the local live run (decision/ci-verify-fake-agent-only, decision/real-cli-verify-never-in-ci). This bears on the scope's fork: a 30th relied row with a verify probe of the client's tool-call timeout needs a fake-agent form in CI.

@history_item n=25 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout prints one `[NN/MM]` step line per relied row, MM being the relied count, with the summary summing to it. A 30th relied row moves MM (contract/verify-stdout/step-line, contract/verify-stdout/summary-line), and the rows state it by rule, not by number.

@history_item n=26 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — ~ contract/verify-stdout/step-line, ~ contract/verify-stdout/summary-line and ~ contract/verify-exit-codes/0 now carry the `absent` verdict for the browser-tools-prompt row. This is the latest shape of verify's per-row output and exit-0 rule that a new relied row would join.

@history_item n=27 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a stopped verify starts no further spawn, closes every pair it opened and ends with `process-exit{detail:"interrupted"}` (contract/verify-process-exit/interrupted). Any new verify probe for the MCP bound must follow this stop discipline.

@history_item n=28 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — + contract/diag-subject/verify-sender and + contract/verify-spawn-subject/verify-sender; ~ contract/process-log/child-and-probe-lifecycle; ~ contract/verify-process-exit/child-exit-status-and-duration and ~ contract/diag-line-fields/process-start, now stated by rule, not by count. This is the latest change to verify's child set (scope: "verify's child set"). It is the precedent: a new verify spawn adds one subject word, one row per word and its own pair.

@history_item n=29 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `config.json` known keys went from four to five (`verify_new_cli`), and + contract/diag-subject/auto-verify (the start's verify child pair in the run log). If the bound becomes a configured value, this is the known-keys precedent. A new relied row is also stamped by this automatic verify at the start.
