@history_item n=1 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — (1. Obs Scope Summary, ~ contract/mcp-iserror/instance-unreachable) the closed cause set grew six to seven with `call-deadline`, written by the frame tools alone. This is the direct precedent for a compiled bound ending a call with one closed word and no upstream text. It is also the MCP_CALL_DEADLINE (55 s) that the paste bound plus the rest of `send` must stay under, and the carry this chunk folds says the driver reads `call-deadline`, outcome unknown, when the paste blocks.

@history_item n=2 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — (7. Error Capture & Reporting, ~ decision/mcp-unreachable-carries-one-cause-word · ~ decision/instance-unreachable-not-logged) an MCP frame call past the call deadline carries exactly one UnreachableCause word and writes no line, with the isError code as its only record. A new paste-bound outcome on the MCP `send` tool has to choose between this no-line reject class and a wrapper `send-refused` line. It must keep the one-cause-word rule if it reaches the MCP result as instance-unreachable.

@history_item n=3 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — (1. Obs Scope Summary, ~ decision/claude-internals-not-a-contract) the claude MCP client's tool-call bound is a per-version reading of bundle strings, never a contract. The paste-bound arithmetic against the MCP call deadline must rest on viola's own compiled `MCP_CALL_DEADLINE`, not on claude's internal bound. The row also fixes paste writes as one of the boundary points where claude is instrumented.

@history_item n=4 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (1. Obs Scope Summary, ~ decision/claude-boundary-only-instrumentation) the closed list of claude boundary instrumentation (spawn and exit, paste writes, the readiness-gate outcome, the start's trust-record line) was widened by naming the new boundary operation. A paste-write bound changes the paste-write boundary point. A new telemetry on it, for example a cut-paste outcome, is the kind of change this row's list has had to name before.

@history_item n=5 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — the wrapper `send-refused` detail stays closed at `input-not-ready|no-prompt-submitted|turn-running`. `turn-running` gained a second cause (another send in flight) under the same detail with no new field, no `send-issued` and client exit 13. This is the precedent for putting a new cause under an existing closed detail instead of adding a word. A cut paste comes after `send-issued`, so it does not fit the before-the-paste `turn-running` shape.

@history_item n=6 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — (4. Span / Trace Coverage, ~ contract/send-refused-detail/turn-running) the post-gate turn read, the link record, `send-issued` and the paste now run under one flight lock. `turn-running` covers only a turn known before the paste. A paste blocked inside that lock holds the flight lock, so every later send reads `turn-running` until the bound fires. The bound and the lock's release decide what the next send meets after a cut paste.

@history_item n=7 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — an unconfirmable send opens no `run.confirm_window`: its span chain ends at `pty.paste_write`, and its lines are one `send-issued` and one `send-confirmed{confirmed:false}` under the same corr, with no `send-refused`. Expiry of the window is `no-prompt-submitted`. A paste cut at the bound lands after `send-issued`, so the chunk must say which line closes that corr (`send-refused` with a new or existing detail) on both the confirmed and the unconfirmable path.

@history_item n=8 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (1. Obs Scope Summary, ~ contract/send-path-log-events/event-order · 4. Span / Trace Coverage, ~ contract/send-refused-detail/no-prompt-submitted, + decision/transcript-echo-read-uninstrumented) the send event order is channel-request, send-issued, hook-invoked or none, then send-confirmed or send-refused, then channel-response. `no-prompt-submitted` covers ordinary, /clear, /compact and echo-confirmed local-command expiry with no new detail. A cut paste must fit into this order, and its outcome must close the corr that `send-issued` opened.

@history_item n=9 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the local-command-delivery rows: the send event order, deterministic emission keyed on the transcript tag count, and transcripts not a contract. They now stand unchanged on his own word, so a paste-bound change to the event order amends ratified rows, not provisional ones.

@history_item n=10 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival at the wrapper. It includes the readiness gate's wait up to architecture's `GATE_MAX_WAIT` (up to 8.5 s on a delivered send) ahead of the confirm window. The bound is named by its constant. This is one term of `send`'s compiled total that the paste bound is read against. A paste-write wait now also falls inside `duration_ms`.

@history_item n=11 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `COMPACT_CONFIRM_WINDOW` is 40 s (PROVISIONAL, later ratified), and every other confirmed send waits 10 s. `window_ms` reads `SendSlot::window()`. The 40 s compact window plus the gate's wait is the largest term of `send`'s total. The paste bound added to it must stay under the 55 s MCP call deadline, and the arithmetic the scope asks for starts here.

@history_item n=12 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — `text_bytes` on the `pty.paste_write` span and on `send-issued` is the typed text's length after CR/LF normalisation. `pty.paste_write` is the span this chunk bounds. Any new attribute on it, such as a bound or a cut outcome, must follow the span-field allow-list, and the cut must never log the text or how many bytes were written beyond allowed codes.

@history_item n=13 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (the `empty-text` detail) the refusal details gained `empty-text` with no schema change, because `send-refused.detail` is an open string in `diag-line.v1.json`. This is the precedent for adding a new send-refused detail word. It needs the §6 catalog and the §1 list, plus the RefusalReason words row (contract/send-refused-detail/refusal-reason-words), but no schema enum edit.

@history_item n=14 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — the `send.client` exits are 0/10/12/13/14/20/21 and 1 `internal-error`. Exit 20 carries `wrapper-fault`, and exit 21 carries `instance-dead` with `during` connect or call. This fixes which exit codes a paste-bound outcome may reuse on CLI `send`. Exit 11 was later retired and is never reused.

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1. Obs Scope Summary, ~ decision/exit-code-log-field · 6. Log Coverage, ~ contract/send-refused-detail/refusal-reason-words · − contract/send-refused/budget-paused) exit 11 is retired and never reused. The RefusalReason details now read manual-pause, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text. These are the closed sets a new paste-bound word or exit code would extend, and 11 cannot be reused for it.

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1. Obs Scope Summary, ~ decision/diag-schema-additive-only · ~ decision/log-fields-additive-only · 6. Log Coverage, ~ decision/log-schema-additive-only · 3. Observability Harness Contract, ~ decision/obs-aligns-to-tests-log-format) the operator's Q2 narrowed the additive-only rules to "must not rename or remove a field the product still emits". This is the schema-change rule any new field or enum word for a cut paste must respect, and a new event value still needs a Decisions Log entry.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21's document names its cause; exit 21's logged causes and their `during`; the `process-exit` `detail` enum's count) exit 21 prints `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` from the closed UnreachableCause set. `during` is `call` for `endpoint-lost` alone. The `process-exit` detail enum is counted by its members. If the paste bound ends CLI `send` with a new closed word, this is the catalogue and the count it extends, with `during` call for a bound reached mid-call.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (MCP's `instance-unreachable` body names its cause) the MCP isError body is the CLI's document less `v`. The map from client errors to causes lives in `frame` and `of_reply` (`Closed`/`Io` read `EndpointLost`), and rejected calls write no line. What MCP `send` returns at the paste bound runs through this mapping, so a wrapper-side closed word has to reach `of_reply` as a refusal or a cause.

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (exit 21's closed causes, five to six; `socket-dir-refused` in the log catalogue, `process-exit` 19 to 20) this is the latest full precedent for adding one closed word. The word went into viola_core's UnreachableCause with no serde path and no RefusalReason, into both schema detail enums with recounts, into exit 1 and exit 21 lists, and into the `--json` document. The founder answered the widening himself. A new paste-bound word walks the same set of sites.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the closed words that left `diag-line.v1.json` with their emitter) `process-exit` detail closes at 19, and a new rule says a closed schema word no build can emit leaves the schema with its emitter (decision/unemittable-schema-words-removed). A new paste-bound word enters only with its emitter, and the count is re-measured.

@history_item n=21 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (a peer's reply screened by sanitised_fault and is_code_word) clients keep a peer's refusal and detail only as code words. An unknown code word stays visible and exits 14 by `reason_of`. A new wrapper-side word for a cut paste therefore reads as `unknown`/exit 14 on any client not taught it, including the pinned driver until the re-pin, and the scope's boundary on the pinned product (`a59da9da`) bears on this.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (channel-response's detail closed to three words) `channel-response.detail` is closed to unsupported-version, invalid-params and release-from-driver, on a -32602 only. A refusal's own detail stays on `send-refused`. A cut-paste outcome must not ride `channel-response.detail`, and its word belongs on the send line.

@history_item n=23 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (MCP's `instance-unreachable` reject writes no line) a logged MCP reject's line is codes-only, and `instance-unreachable` is logged by no line, with the isError code its only record. Together with the call-deadline entry, this fixes the logging class the MCP `send` tool's result at the paste bound falls into.

@history_item n=24 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the wheel scenario: `run.wheel_transition` cause `human-input|manual-pause|release` is a point-in-time span on the wheel's worker thread, and pause and release are traced as channel pairs. The scope requires the wheel to stay unmoved at the bound and asks what the human's keys do during and after a blocked paste. A cut paste must open no wheel transition, and human input during it stays the existing `human-input` path.

@history_item n=25 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — (+ decision/wheel-typed-mark-uninstrumented) the stdin typed mark and the held wakes count are in-memory wrapper state with no span or line, carrying no key or prompt text. Anything this chunk adds to track human keys pressed while a paste is blocked falls under this row, and typed keys are never logged.

@history_item n=26 marker=2026-10-10-session-links
**2026-10-10-session-links** — the implicit link is recorded inside an issued `send` after the wheel and running-turn reads, just before `send-issued`, with no line or span. A failed record is -32603 with nothing issued. Since the link lands before the paste, a paste later cut at the bound leaves the link recorded, and the chunk must say whether that stands.

@history_item n=27 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — `run`'s worker threads: the vt100 feed thread is a contained worker. The ban on a worker panic that leaves no `process-exit` covers the PTY pump and the handle-wait thread. If this chunk bounds the paste by moving the `write_all` onto a thread or timer in the pump (`crates/viola-pty/src/pump.rs`), that thread falls under the pump-thread catch_unwind and panic rules (decision/run-threads-inside-catch-unwind, decision/run-worker-panic-ends-run).

@history_item n=28 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the `viola mcp` stdin line cap) rmcp's unbounded `read_until` meant the bound belonged to the reader viola hands it (`CappedLines`). A cut line is refused whole, never truncated, with one codes-only `parse-rejected` holding no byte of the line. This is the nearest precedent for bounding a third-party blocking I/O at viola's seam. A cut paste is the write-side twin: its record holds no text byte, only codes.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the ConPTY sideload and the Windows kill fallback retired; openpty the one PTY backend) the `viola-pty` seam is portable-pty alone over openpty on Linux, with no Windows arms. A paste-write bound in `crates/viola-pty` needs a Linux-only mechanism and does not need per-OS telemetry.

@history_item n=30 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the portable-pty pin at =0.9.0) the `viola-pty` entity is the PTY seam (spawn, read, write, resize, wait, kill) over portable-pty `=0.9.0`. Its `log` target is OFF and failures are captured at the seam (D-11). A paste write timing out inside portable-pty leaves no third-party record, so the bound's outcome must be captured at viola's seam.

@history_item n=31 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — §7 records that `PtyError`'s fixed Display is hand-written because `viola-pty` takes no thiserror (decision/typed-errors-fixed-display, except). A new paste-timeout error variant in `viola-pty` follows that hand-written fixed-message form with no interpolated fields.

@history_item n=32 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the thiserror derive census on the product sources) the census reads 9 `thiserror::Error` derives over the product sources (`src` and `crates` outside `crates/viola-e2e`). A new error type added for the bound moves that count and must be re-measured. A new variant of an existing type does not, which is the 2026-11-01 `Refused::NotADirectory` precedent.
