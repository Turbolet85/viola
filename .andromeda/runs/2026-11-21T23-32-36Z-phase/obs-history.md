@history_item n=1 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — set where a vt100 panic is caught: on `run`'s feed thread (`src/run/gate.rs`), with a `catch_unwind` around each feed and each resize. Each poisoning writes one `parse-rejected{parser:"vt100-feed"}` at WARN with no `corr`, the model stays poisoned until the host size changes, and `run` continues. This is the resize path's one standing telemetry (decision/vt100-panics-contained, decision/vt100-feed-panic-contained). Any change to how a resize reaches the screen model or the child's PTY must keep the per-resize catch and the one-line-per-episode rule.

@history_item n=2 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (feed overflow) — `oversize` also covers `vt100-feed`: when the bounded feed queue drops a copy, the model is poisoned, one line per episode, and a size message carries the drop count at its offer. A burst of redraw output after a resize can hit this path. A settle wait or a read-back that leans on the screen model inherits the poisoned-until-resize state.

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the G2 question, F4) — `tests/chaos_feed_panic.rs` witnesses one `parse-rejected{vt100-feed, panicked}` and one `panic` line, then `viola send` refused `input-not-ready`, while a typed key still reaches the child. Its home is the `TestHome::outside_scan()` carve-out under the system temp dir, outside G2, G4 and the secret scan. This is the existing test that types a key through a poisoned model; it is the precedent for a test asserting key passthrough around a resize.

@history_item n=4 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (feed-panic passthrough half) — the same chaos test also asserts that none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and that the child's bytes reach it. This holds the run-silent-terminal rows (decision/run-silent-terminal, contract/run-terminal-output/child-output): a resize fix must write nothing of viola's own to the terminal.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — retired the ConPTY sideload and the Windows kill fallback. openpty is the one PTY backend (`pty_backend` `openpty`, `os` closed to `linux`), the OS APIs are instrumented only at viola's call sites, and the forced feed-panic witness has no OS split. The resize path is Linux openpty alone: no per-OS arm and no new backend word to add (contract/diag-line-schema/os, contract/diag-line-schema/pty-backend, decision/os-apis-at-call-sites).

@history_item n=6 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty pin) — the `viola-pty` seam (spawn, read, write, resize, wait, kill) is pinned at portable-pty `=0.9.0`. Its `log` target stays OFF and failures are captured at the seam (decision/portable-pty-log-target-off, decision/third-party-crate-logs-off). A resize failure or a write after a resize must be captured at the `viola-pty` seam, never by turning the crate's log target on.

@history_item n=7 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival and so includes the readiness gate's wait, up to `GATE_MAX_WAIT` (8.5 s on a delivered send, by the founder's ruling), named by its constant and never copied. If the fix is a settle wait after a resize on the send path, it lands inside this sample, and the row says whose wait it is (contract/send-log-fields/send-confirmed).

@history_item n=8 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the wheel's cause vocabulary is `human-input|manual-pause|release`, and `run.wheel_transition` is a point-in-time span opened once per change. The keys a human types are never logged; the `wheel` product event is their audit record (decision/typed-keys-never-logged, decision/wheel-event-is-audit-record, decision/no-typed-keys-in-telemetry). Measuring a key lost after a resize must tell lost from kept by a closed word, never by logging the key bytes.

@history_item n=9 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — the live round on 2.1.325 measured the folder-trust keys, passed through the wrapper, writing `wheel{human-input}` before `session-start`, with a later human key adding no wheel line. The start order is read from that start's own lines, because `events.ndjson` is never rewritten (contract/run-start-events/order, decision/events-stream-never-rotated). A live measuring start that types keys into the CLI changes the wheel, and its start-order assertions must read from where that start begins.

@history_item n=10 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — added decision/wheel-typed-mark-uninstrumented. The stdin typed mark and the held wakes count are in-memory wrapper state that open no span, write no line and carry no key or prompt text. Any typed-after-resize state that a fix adds (a pending key, a re-type flag) has this precedent: uninstrumented by plan, named by a row.

@history_item n=11 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — widened decision/claude-internals-not-a-contract so that the CLI's bundle strings are never read as a contract either. The scope's reading of what 2.1.328 and 2.1.330 do on a resize (redraw, input-mode reset, debounce) is a per-version reading of bundle strings, never a contract that a fix can key on.

@history_item n=12 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — the one standing precedent for confirming delivery by a read-back. It added the transcript echo exceptions to decision/claude-internals-not-a-contract, decision/transcripts-not-a-contract and decision/deterministic-emission, and the row decision/transcript-echo-read-uninstrumented: the read adds no span or line, and its result shows only on `send-confirmed` inside `run.confirm_window`. A "read-back and one re-type" fix would need the same named exceptions, and decision/no-telemetry-from-screen-content forbids deriving it from the screen beyond gate outcomes. The founder ratified these rows on 2026-11-17 (2026-11-17T05-21-35Z-wrap).

@history_item n=13 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — added decision/browser-tools-prompt-reading-uninstrumented: a new reading of the CLI's screen that adds no log line, span, event or detail code, and whose record is the ledger stamp and verify's stdout. This is the precedent if the answer becomes a relied ledger row or a new screen reading around a resize.

@history_item n=14 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `run.confirm_window` closes on the `prompt-submitted{origin:"driver"}` whose prompt equals the typed text, or equals it with a trailing run of whitespace removed; turn-running covers only a turn the wrapper knows of before the paste. A key lost after a resize would show as a prompt mismatch and then as `send-refused{detail:"no-prompt-submitted"}` at expiry, with no new detail (contract/confirm-window-close/ordinary-send, contract/send-refused-detail/no-prompt-submitted).

@history_item n=15 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance. A leading trim was deferred to a live reading carried on «Standing live rig with a screen model». This is a precedent for a match rule widened only after a live measurement on the rig, the same order the scope sets for «fixed if it does»: choose only after the reading.

@history_item n=16 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — `text_bytes` on `pty.paste_write` and `send-issued` is the typed text's length: each CR LF pair and each other CR as one LF, trailing CR and LF removed (contract/send-span-attributes/text-bytes, contract/pty-seam-log/paste). A re-type fix writes to the PTY again, and its telemetry must keep the paste logged as `text_bytes` only, never as text or as a second count field.

@history_item n=17 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `window_ms` reads the send's own deadline: 40 000 for a `/compact` send (`COMPACT_CONFIRM_WINDOW`, PROVISIONAL), else 10 000 (contract/send-span-attributes/window-ms, decision/compact-confirm-window). A settle wait or a re-type inside the confirm window must not move these bounds silently.

@history_item n=18 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — the hyprctl spawn is instrumented at viola's own call site. The span-area list gained `hyprland`, and a hyprctl reply failing its parse reads oversize or malformed with one `parse-rejected{parser:"hyprctl-json"}` (decision/span-naming, decision/os-apis-at-call-sites, decision/parse-failures-unknown-counter). If the resize comes from the tiled console's window, its source is this seam. A new span stays inside the closed `<area>` list, and a new event needs a Decisions Log entry (decision/event-values-closed).

@history_item n=19 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — closed decision/claude-boundary-only-instrumentation's list at spawn and exit, paste writes, the vt100 readiness-gate outcome, and the start's one trust-record line. A resize or a typed-key outcome is not on that list. Instrumenting it at the claude boundary means amending this closed row through a planned row change, not a silent addition.

@history_item n=20 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify's interactive runs went from five to six (Run F), and contract/verify-spawn-subject/verify-pty-probe states that count. If the answer becomes a relied ledger row measured by a new verify PTY run, this row's run count moves again (crates/viola-agent-claude, the capability ledger).

@history_item n=21 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — the latest ledger-row chunk. The verify spawn-pair count and the `process-start` subject list are now stated by rule (one word of the diag-line subject enum, one row per subject word) instead of as a count, so a new verify spawn subject adds its own `contract/diag-subject/<word>` row and leaves the generic rows unchanged.
