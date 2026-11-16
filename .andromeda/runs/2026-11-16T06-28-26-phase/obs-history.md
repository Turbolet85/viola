@history_item n=1 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the confirm window's second trigger, and the send that opens none: `send` began consuming the compiled local-command list; an unconfirmable send (a listed command with no post-condition, or any listed command on an unverified CLI) opens no `run.confirm_window`, its chain ends at `pty.paste_write`, one `send-issued` plus one `send-confirmed{confirmed:false}` under one `corr`, no `send-refused`, no hook line awaited; `/clear` on a verified CLI closes on `session-start{cause:clear}`; expiry is `no-prompt-submitted`; `viola.send.confirm_ms` counts only `confirmed` not false. This is the direct precedent and register this chunk extends: a local command's echo becomes a new close trigger (or turns today's `unconfirmable` / `no-prompt-submitted` readings into confirmed ones), and the "no new event, span or field" discipline it kept is the bar for this chunk.

@history_item n=2 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — CL-1's third close condition: `/compact` joined `/clear` on the session-start hook leg (same `agent_session_id`), its own window `COMPACT_CONFIRM_WINDOW` 40 s read by `window_ms` through `SendSlot::window()`, expiry `no-prompt-submitted` for all three, no new event, detail or field; verify's `MM` grew by one relied row (`local-command-compact`) through a sixth Run B paste with no new spawn pair or subject. The template for adding a local-command post-condition: a ledger row with a verify probe, a close rule per command, `window_ms` stated, and verify's relied count and spawn-pair count re-stated (rows confirm-window-close/clear, /compact, send-span-attributes/window-ms, decision/compact-confirm-window).

@history_item n=3 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the fake-agent verify flag list gained `--framing`; verify still logs its spawn pairs and no row text, prompt or path reaches a line. The local-command ledger rows' origin: a new echo row probed against the fake agent (`--local-command-mode`) moves the CI fake-agent verify flags bound to test-plan §3 boot step 4, and the stamped-home row count.

@history_item n=4 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `contract/confirm-window-close/ordinary-send` now closes on the `prompt-submitted{origin:"driver"}` equal to the typed text or to it with a trailing ASCII-whitespace run removed; `contract/send-refused-detail/turn-running` bounded to a turn known before the paste (post-gate turn read, link record, `send-issued` and paste under one flight lock). The scope keeps the ordinary-send match untouched; a local command's echo path must not widen or route through this rule, and the flight-lock order is what a new confirmation arm sits inside.

@history_item n=5 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the send trailing-only tolerance and the StopFailure no-ledger-row answer ratified by the founder; `confirm-window-close/ordinary-send` drops its provisional mark. Fixes the untouched boundary this chunk names, and records the one precedent of an undocumented CLI hook (`stop-failure`) standing with no ledger row and no verify probe, by the founder's word — against CLAUDE.md's every-dependence-is-a-ledger-row rule this chunk's echo must meet.

@history_item n=6 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `contract/diag-line-fields/hook-events`: `hook_event` gains `stop-failure`, ten closed names. If the measured echo is a hook event not yet in the closed `hook_event` enum, it is the same kind of closed-enum change (schema plus row), and `contract/send-log-fields/hook-invoked` would name its hook leg.

@history_item n=7 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — `turn-running`'s two causes (a running turn marked by a `prompt-submitted` of any origin, ended by `turn-ended` / `session-start` / `session-end`; or another send in flight) under one closed detail, no new field, no `send-issued`, client exit 13. A local command that runs fires no `prompt-submitted`, so the running-turn state and the in-flight slot it holds bear on how a local-command send ends and releases the slot.

@history_item n=8 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — `send.client`'s exits (0/10/12/13/14/20/21 and 1 `internal-error`), exit 20 `wrapper-fault`, and `vt100-feed` `oversize` beside `panicked`. Rows cli-exit/1-internal-error, cli-exit/20-wrapper-fault: the CLI/MCP send outcomes this chunk reads only as they already read the wrapper's reply.

@history_item n=9 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — a vt100 panic is caught on run's feed thread, one `parse-rejected{vt100-feed}` per poisoning episode, `send-refused{input-not-ready}` following. If the echo is read from the screen model, it stands against decision/no-telemetry-from-screen and decision/deterministic-emission (vt100 serves gate outcomes only; the gate's verdict is the one named exception), and a poisoned model must not confirm a send.

@history_item n=10 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival at the wrapper, so it includes the readiness gate's wait (up to `GATE_MAX_WAIT`). A local-command confirmation's `send-confirmed` carries the same span (row send-log-fields/send-confirmed).

@history_item n=11 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `empty-text` added to the refusal details, one line per deciding side, never the text; `send-refused.detail` is an open string in `diag-line.v1.json`, so a new detail moved no schema, only the §6 catalog and §1 list. Precedent if this chunk needs a new send detail (rows send-log-line/detail, send-refused-detail/refusal-reason-words), and the `text_bytes` wording it leaves unchanged.

@history_item n=12 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — `text_bytes` is the typed text's length (CR LF and lone CR as LF, trailing CR/LF removed) on `pty.paste_write` and `send-issued`. The typed form of a slash command is what any echo match compares against (row send-span-attributes/text-bytes); the measure stays a length only.

@history_item n=13 marker=2026-10-10-session-links
**2026-10-10-session-links** — the implicit link is recorded inside an issued `send` directly before `send-issued`, one `link` product line for a new pair, a failed record `-32603` with nothing issued; no line or span of its own. A local-command send keeps this step order (row events-log/link-before-send-issued).

@history_item n=14 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — `diag-line.v1.json` requires `corr` on `send-*` when `side` is `wrapper`; a mis-shaped `conn` logged as `srv_conn`. Every wrapper send line a new confirmation path writes keeps `corr` (the send cursor), `conn`/`srv_conn` and `rpc_id` (rows send-line/conn-rpc-id, corr-by-event/send).

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — channel-response's `detail` closed to three `-32602` words, absent on a send refusal's channel line (the refusal's detail stays on `send-refused`); peers' replies screened by `is_code_word`, an unknown refusal word visible with exit 14. A new send outcome or detail reaches the CLI and MCP only through this screen (rows channel-response/detail, mcp-iserror/refusal, refusal-reason/unknown-fallback).

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `contract/diag-line-fields/send-events` lost `window`/`used_percentage`/`resets_at`/`paused`, `contract/send-refused-detail/refusal-reason-words` lost the budget details, exit 11 retired and never reused (decision/exit-code-log-field), and the additive-only rules narrowed to "a field the product still emits". The current send-line field set and refusal words this chunk extends, and the exit-code register a new outcome must not reuse.

@history_item n=17 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — verify's spawns logged as pairs at the call site (`verify-probe` joined the subject enum, a new closed value taking a Decisions Log entry); CI verify only against the fake agent, the real-CLI verify only through the local live run, refused under CI. A new echo probe in `src/cmd/verify/` follows this: drive it on the fake agent in CI, reuse an existing subject or record a new one.

@history_item n=18 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify logs eight child spawn pairs; a new relied row moves the step lines' `MM` while a probe that reuses Run B adds no pair or subject. The current spawn-pair register a local-command echo probe either keeps (a Run B paste, as the compact row did) or extends.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows host arms retired) verify's relied count reads all 24 rows of `ALL` on the one Linux target (was 24 Unix / 21 Windows); `subject` and the `detail` enums close by the rule that a word no build emits leaves the schema. The relied-row count a new ledger row raises, and the rule for any closed word this chunk adds.

@history_item n=20 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout prints one `[NN/MM]` step line per relied row, `MM` the relied count, the summary summing to it; the hidden capture arm uninstrumented. A new relied echo row moves rows verify-stdout/step-line and summary-line; the probe's capture writes no line (decision/hidden-capture-arms-uninstrumented).

@history_item n=21 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — events.ndjson is never rewritten: a reused instance name keeps earlier starts' lines at its head, so a start's own lines are read from where that start begins (decision/events-stream-never-rotated). A trap for any fixture or test reading a local command's echo, or cursor, out of the events log.

@history_item n=22 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each verified against the code; "a master holds what a planner must respect and the code holds the rest". Every picked row (send-line, confirm-window-close, send-refused-detail, verify rows) comes from this conversion, so a change this chunk makes is a row amendment named by handle.
