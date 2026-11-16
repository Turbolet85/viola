@history_item n=1 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — fixed the prompt normalisation order (classify `harness` on the raw prefix, then unwrap the CLI's same-id paste pair, then un-escape `<\`) and made `hook.event` re-validate `prompt-submitted` text/origin. Matters here: decision/prompt-normalise-order and the hook-side `prompt-submitted.text` are the input any trim tolerance compares against; a tolerance added in the wrong layer would break this order.

@history_item n=2 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — widened `HARNESS_PREFIXES` to four, matched on the raw start "with no trim" (Kept: no trim before the match, the raw start is the narrower class). Matters here: a trim rule added for the driver match must not leak into harness classification (decision/harness-prompt-prefixes, harness-prompts-not-human).

@history_item n=3 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — landed the readiness gate as `src/run/gate.rs` feed thread + pure `viola_agent_claude::screen::Screen`, catch_unwind poisoning, `QUIET_PERIOD`, `GATE_MAX_WAIT`, `CONFIRM_WINDOW_FALLBACK`. Matters here: the gate whose wait window the turn race lives in (decision/vt100-readiness-gate, poisoned-screen-model, gate-timings-compiled).

@history_item n=4 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (confirmed send: kinds, refusals, exits, the relabel) added `send-issued`/`send-confirmed`/`send-refused`, `control-character` first via `validate_paste_text` (CLI and wrapper), `turn-running` also for a second send in flight, the driver relabel only on a prompt whose text equals the in-flight send's exactly, and the partial gate on an unverified CLI. Matters here: this is the exact-equality claim the chunk loosens and the refusal-order/send-records contracts it must keep (contract/refusal-order/send, contract/hook-event/driver-relabel, contract/event-data/send-*).

@history_item n=5 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (fuzz targets, the chaos test home, the send layout) placed `validate_paste_text` and `NotDelivered` in viola-core, the `paste_text` fuzz target with 8 byte-exact seeds, and `src/run/send.rs` as the one-in-flight send. Matters here: "validated as received, never stripped" rests on this validator and its fuzz corpus.

@history_item n=6 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (the wheel as landed) a human-filed `prompt-submitted` moves the wheel before its line is appended; the closed non-editing list; the wheel in `src/run/wheel.rs`. Matters here: an unclaimed driver prompt is exactly what flips the wheel to `human`/`human-input` in the measured trailing-space incident; the claim must settle before this move (decision/human-prompt-confirms-wheel-take).

@history_item n=7 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — built the running-turn state in `WheelSlot`: `append_hook_event` marks it on any `prompt-submitted` (the claimed driver prompt included) before the line is appended, `turn-ended`/session lines end it; the `send` rung decided at arrival. Recorded residual: a turn starting during the readiness gate's wait is not refused. Matters here: the direct precedent and origin of the readiness-gate turn race this chunk closes (decision/send-never-queued).

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — compiled `SIGNATURES`, `run` passing `Some(&SIGNATURES)` exactly when `cli_verified` (full gate verified, partial gate otherwise), `confirm-window` row validating the 10 s window. Matters here: the gate's verified/unverified split and the compiled window the chunk's confirmation path reads (decision/partial-gate-on-unverified-cli, confirm-window-compiled).

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the unwrap removes exactly the CLI's frame "and nothing wider"; `LOCAL_COMMANDS` compiled; `prompt-verbatim`/`long-paste-wrapper` rows; tag escaping as measured. Matters here: precedent that a CLI-side text change is handled by a measured, minimal rule in the agent crate, backed by a ledger row and probe.

@history_item n=10 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — (send consumes the local-command list) classification by exact equality "no trim, no case folding, never a leading-slash test"; `/clear` claimed by its `session-start` post-condition, no `prompt-submitted`; the relabel sentence's one exception. Matters here: classify.rs reads the exact typed text; a trim-tolerant claim must not alter local-command classification (decision/local-commands-by-ledger, clear-confirmed-by-new-session).

@history_item n=11 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (the long-paste frame as measured) a pasted text's last newline never reaches the hook; the beside-text/between-pairs frame compiled with no probe under the ruled limit "a shape no send relies on needs no probe". Matters here: the ruled limit (decision/no-probe-for-unrelied-shapes, capability-ledger-gate except) is the test for whether the trim tolerance, which a send does rely on, needs a ledger row.

@history_item n=12 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (a send whose text ends in a newline is delivered and not claimed) measured: a CLI-dropped trailing byte makes the exact match fail, the prompt is filed `human`, moves the wheel, the send ends `not-delivered`/`no-prompt-submitted`. Kept: no trim added. Matters here: the same failure shape as the trailing-space incident this chunk fixes.

@history_item n=13 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (the paste hint measured on the live CLI, and the founder's decision) the 8.0 s hint timer; the founder chose that the gate waits for the input box with the bound raised to 8.5 s. Matters here: why the gate's wait is long enough for a turn to start inside it (decision/send-waits-out-paste-hint).

@history_item n=14 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` 8.5 s; on a verified CLI a quiet screen with no input-box row waits until the bound; `send` reads the wheel and the running turn twice, at arrival and again after the gate returns ready, retiring the 2026-10-04 residual; the wait is not interrupted when the wheel moves; `send` blocks at most 18.5 s. Trap: a case reading a verified literal-less screen through a clock that never advances waits forever. Matters here: the second read is the existing race guard — the chunk must find the window it leaves (between the post-gate rung and the paste) and keep the send time bound (decision/send-time-bound, contract/refusal-order/send).

@history_item n=15 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — `validate_paste_text` first on the text as received, then `typed_text` taken once and used for classification, the in-flight match, `text_bytes` and the paste; the claim (`SendSlot::claim`) kept exact. Kept: "a tolerant claim in the wrapper was the boundary widening not chosen"; no ledger row because no send relies on the dropped newline. Matters here: the chunk now builds a tolerant claim — exactly the option once declined as a widening — so it needs its own ruling and the ledger question answered.

@history_item n=16 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (the live readings on 2.1.287) a trailing-CR send was submitted without the CR, filed `human`, moved the wheel, ended exit 13; "Trap: the trailing-CR outcome is a known defect, not a designed refusal". Matters here: a prior instance of CLI-side trimming breaking the exact match, fixed on the send side rather than the match side.

@history_item n=17 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (the closed non-editing list gains seven terminal reply shapes) standing rule: the list grows only by measured shapes with exact grammars and negative controls. Matters here: the wheel byte-source claim stays untouched by the scope; a match-tolerance fix must not loosen it (decision/closed-non-editing-byte-list).

@history_item n=18 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (the typed text drops trailing CR and LF) `typed_text` removes trailing CR and LF and nothing else, "not a TAB, a space"; two product callers (wrapper `send`, client `refusal_of`). Matters here: the scope keeps `typed_text` as is ("nothing else is taken off"), so trailing spaces/tabs remain typed and must be tolerated at the match instead.

@history_item n=19 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (an empty typed text is refused `empty-text`) sixth `not-delivered` detail, checked by the client before any frame and by the wrapper before the first wheel read; order `control-character` → `empty-text` → `human-typing`. Matters here: the scope's open question of a text made only of CLI-trimmed whitespace sits right against this rung (contract/send-refusals/empty-text, contract/paste-text/empty).

@history_item n=20 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — `typed_text` types inner CR/CRLF as LF; Kept: "the match stays an exact equality with no tolerance on the submitted side"; the LF-inside-a-paste reliance named with its ledger row owed. Matters here: the latest standing statement of the exact match the chunk changes, and the precedent that a relied CLI shape gets an owed ledger row (decision/typed-text-normalisation, send-text-line-endings).

@history_item n=21 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — added row `paste-newline` and Run B's fifth paste to stamp the relied LF-in-paste shape; every older stamp reads unverified until re-verify. Matters here: the cost model (row + Run B paste + re-stamp, a verify-child-set widening needing the founder) if the trim tolerance is put on a ledger row.

@history_item n=22 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (send and human split by phase) `run/send.rs` root over `run/send/`: `slot.rs` (SendSlot, `append_hook_event`, driver relabel, post-condition claim), `classify.rs`, `params.rs`, `relabel.rs` (cfg(test) cases); `human/hints.rs` holds `send_hint`. Matters here: the file layout of every surface this chunk touches.

@history_item n=23 marker=2026-10-10-session-links
**2026-10-10-session-links** — (the implicit link as landed) the link is recorded after the second wheel and running-turn reads and directly before `send-issued`; a failure is `-32603` with nothing issued. Matters here: any reordering to close the post-gate race must keep this ordering (contract/channel-methods/send-from-link, contract/channel-send/from).

@history_item n=24 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the MCP tools' contract as landed) the MCP client's tool-call timeout exceeds 50 s on 2.1.287, above the 18.5 s send total. Matters here: decision/send-time-bound — closing the race by waiting must stay below the MCP ceiling.

@history_item n=25 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (/compact confirmed by its same-session SessionStart) `SendSlot::window()` is the one bound for `confirm`; a `/compact` send has its own 40 s window; the claim arm on `session-start` cause `compact` with the same id. Matters here: the confirmation window exception and the slot's other claim arm the trim change must leave intact (decision/confirm-window-compiled except, compact-confirmed-by-same-session).

@history_item n=26 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — `SIGNATURES.input_box` gained the mode-independent `(shift+tab to cycle)`; readiness rule unchanged. Matters here: the gate predicate the race window sits behind (decision/screen-signatures-compiled why).

@history_item n=27 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/screen-signatures-compiled: ready when no modal row and a footer literal or the box's own rule row (≥ 40) above a prompt row. Matters here: the current settled-input-box predicate the scope names as the gate's readiness test.

@history_item n=28 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — ([Screen Model] the poisoning episode's parse-rejected detail) one `parse-rejected{vt100-feed}` per episode, `panicked` or `oversize`. Matters here: contract/parse-rejected-event/vt100-feed — a gate change must keep the poisoning record shape.

@history_item n=29 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ contract/refusal-order/send and ~ contract/send-refusal/order: the `budget-paused` rung left; order now control-character → empty-text → human-typing → turn-running → gate → second human-typing/turn-running → no-prompt-submitted; − contract/refusal-reason/budget-paused. Matters here: the current refusal order the race fix must preserve.

@history_item n=30 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/wheel-starts-with-driver: a key typed into the folder-trust dialog before session start takes the wheel. Matters here: wheel rules the scope leaves untouched; a test of the trim match on a fresh session must not trip this.

@history_item n=31 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Standard Contracts) + decision/api-error-turn-end-clears-turn and `turn-ended` `cause?`: a StopFailure ends the running turn like a Stop. Matters here: another turn-end edge of the running-turn state the race fix reads.

@history_item n=32 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Cross-cutting / Established Decisions / Design Philosophy) StopFailure registered provisionally with no ledger row and no verify probe, a named exception to decision/capability-ledger, cli-capability-ledger and capability-ledger-gate (delegate word, PROVISIONAL). Matters here: the nearest precedent for relying on an undocumented CLI behaviour (the trim) without a row — and that it needed a named exception.

@history_item n=33 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — ~ contract/channel-wait/wake-kinds: an activity record and a harness-origin `prompt-submitted` never wake a parked wait; wake by kind alone. Matters here: relabelling a prompt to `driver` must not change what wakes `wait`; the scope leaves the wake rule untouched.
