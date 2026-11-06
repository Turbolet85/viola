@carried n=1 label=obs-plan/span-trace-coverage fact=fct-lohdixpq
why: The overseer's live drive sends with a `from`, so its evidence must find the implicit link line directly before `send-issued`, and v1-13 / v1-36's link clauses (CARRY 4) are ruled against this record.
quote: One operation on this path carries no span and no line of its own, as landed (chunk 2026-10-10-session-links): the implicit link record (E3 below). After the second wheel and running-turn reads and directly before the send is issued, a `send` whose `from` is a name other than the wrapper's own records the link, so a new pair's `link` product line stands directly before that send's `send-issued` in `events.ndjson`, whose `cursor` is still the offset its own line starts at; a send refused at any earlier rung, one with no `from` and one under the wrapper's own name record nothing.
@carried n=2 label=obs-plan/span-trace-coverage fact=fct-xfymn57j
why: The `portable-pty` bump moves the PTY seam the paste write runs on, so paste must still record only the typed-text length, never text.
quote: `pty.paste_write`: `text_bytes` (the length of the typed text: the sent text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters), `paste_mode="bracketed"`;
@carried n=2 run=2 label=obs-plan fact=fct-o5zzhzdu
quote: Never the sent text, only its `text_bytes`; `channel-response` is emitted from a `Drop` guard in `channel.dispatch`, so an early return or error still logs it; the `/compact` confirmation window `COMPACT_CONFIRM_WINDOW` (40 s) is PROVISIONAL.
@carried n=3 label=obs-plan/span-trace-coverage fact=fct-gghzntp4
why: The live drive's evidence is read through `agent-run logs` and `jq`, never by review, so each confirmed send must show its `send-confirmed` line.
quote: `send-confirmed{corr:<cursor>, confirmed, duration_ms}` (`confirmed:false` for `unconfirmable`; `duration_ms` runs from the send's arrival, the readiness gate's wait included);
@carried n=4 label=obs-plan/span-trace-coverage fact=fct-u2zbdxyd
why: An ordinary send in the live drive confirms only on the driver-origin `prompt-submitted`, which the evidence must read before it counts the send confirmed.
quote: `run.confirm_window` closes on the confirmation its send waits for, or on window expiry. For an ordinary send that is the matching `prompt-submitted{origin:"driver"}`.
@carried n=5 label=obs-plan/span-trace-coverage fact=fct-hluu43pb
why: The driver reads the builder's turns through `wait`, so the live evidence reads the `outcome` on its `channel-response` from the closed wake set.
quote: `channel-response{corr:<id>, conn, result_class, duration_ms}` on both sides, plus `outcome` for method `wait` only: the waking kind (`turn-ended|question|permission|plan|session-end|dialog-expired`, the six of `EventKind::WAIT_WAKE`) or `timed-out`, derived once in `viola-channel` from the result over the shared wake set;
@carried n=5 run=2 label=obs-plan/span-trace-coverage fact=fct-ykdl4lpn
quote: `run.wait_dispatch`: `woken_kind` (`turn-ended|question|permission|plan|session-end|dialog-expired`, the six of `EventKind::WAIT_WAKE`) or `timed_out`.
@carried n=5 run=3 label=obs-plan fact=fct-q67chfay
quote: `last_assistant_message` never appears, and no result body reaches a line; a mid-call disconnect is logged once, by the client;
@carried n=6 label=obs-plan/span-trace-coverage fact=fct-u5crvqku
why: The dialog → `answer` scenario is the live hold read's evidence (CARRY 2), and an `answer` with a `from` there records the link the v1-13 / v1-36 rulings read.
quote: Once every refusal has passed and the response's kind matched the pending dialog, and before the `dialog-answered` line and the answer being taken, an `answer` whose `from` is a name other than the wrapper's own records the link (one `link` product line for a new pair, none for a pair the set holds); a refused answer, one with no `from` and one under the wrapper's own name record nothing.
@carried n=7 label=obs-plan/span-trace-coverage fact=fct-uicv6khr
why: The 3600 s ceiling leaves PROVISIONAL only on a live hold read near this bound's top, set through `dialog_hold_s`.
quote: `run.dialog_await`: `deadline_ms` (the hold the slot was started with: `config.json`'s `dialog_hold_s` in ms, 60 000–3 600 000, default 60 000), `deadline_hit`;
@carried n=8 label=obs-plan/span-trace-coverage fact=fct-ffumo3bh
why: The hold read's evidence is the dialog hook's `hook-invoked` written after the `hook.dialog` response, with the question, answer and tool input never logged.
quote: A dialog hook's `hook-invoked` is written only after the `hook.dialog` response, so its `invoked_at` records the hook's true start instant; non-dialog hooks and fail-open paths before `dialog_id` emit `hook-invoked{corr:null}` immediately, and the wrapper's `dialog-raised` gives live visibility during the wait (D-07).
@carried n=8 run=2 label=obs-plan fact=fct-trei2tnd
quote: The question, answer and tool `input` never appear; a dialog hook's `hook-invoked` is written only after the `hook.dialog` response.
@carried n=9 label=obs-plan/span-trace-coverage fact=fct-k233iq5j
why: A live hold near the ceiling ends on an answer or at the hold's end, and the read must tell the wrapper's expiry from the hook's 3605 s backstop.
quote: At the hold's end the wrapper appends one `dialog-expired{dialog_id, dialog_kind}` product line (`source: wrapper`, no question, plan or tool text; it wakes a parked `wait`) and writes one `dialog-answered{corr:<dialog_id>, dialog_kind, deadline_hit:true, duration_ms}` with no `from`; the reply stays `null`, so the hook writes `hook-decision{decision_emitted:false, deadline_hit:false}`, exits 0 with empty stdout, and the dialog renders for the human. A hand-back or an answer writes neither, and a failed append changes nothing else. The hook writes `deadline_hit:true` only when its own read times out at its backstop bound, `DIALOG_HOLD_MAX + 5 s` (3605 s).
@carried n=10 label=obs-plan/span-trace-coverage fact=fct-wovktszp
why: v1-13's budget-release clause (CARRY 4) is proved or deferred here on the founder's word, and a proof reads the budget release by this trace.
quote: A `release` with `budget: true` does not touch the wheel and appends one `budget-gate` record when a pause stood, none when no pause stood, with no span and no process-log line of its own; the string-`from` refusal and the `budget` type check still come first (chunk 2026-10-10-budget-governor).
@carried n=11 label=obs-plan/span-trace-coverage fact=fct-6xa7lf3f
why: A live real-`claude` session's diagnostics are non-synthetic, so its child `process-start` must carry the stripped names only and no `CLAUDE*` value may reach a home-level file.
quote: `env_stripped_count`, `env_stripped_known` (the stripped names) and `env_kept` (the persistent-set names kept) on `process-start{subject:"claude-child"}`; names only, never values (D-34).
@carried n=11 run=2 label=obs-plan fact=fct-xnfonqro
quote: `env_stripped_count`, `env_stripped_known` and `env_kept` on `process-start{subject:"claude-child"}`; names only, never values, and no env map is passed to a span or event.
@carried n=12 label=obs-plan/span-trace-coverage fact=fct-46kjozmt
why: A proof of v1-13's MCP `link` result or v1-36's board link set (CARRY 4) reads the link through this trace, never through a process-log line.
quote: It has no process-log line, no `event` value and no span (`run::links` emits no `obs_event!` and declares no span); its trace is that call's `channel-request` / `channel-response` pair of method `send` or `answer`, the product line and the snapshot's `links`.
@carried n=12 run=2 label=obs-plan fact=fct-3o7hzpx4
quote: the implicit link (recorded by the driven wrapper inside an issued `send` or a taken `answer`) and the explicit `link` / `unlink` methods the driven wrapper serves have no process-log line, no `event` value and no span of their own;
@carried n=13 label=obs-plan/span-trace-coverage fact=fct-6z2owz4q
why: The live real-`claude` sessions run the verified branch (confirmed sends, a held dialog) only on a stamped ledger, so each must start on a stamped CLI and its `cli_verified` is read.
quote: Read inside `run.version_gate` as the child span `state.ledger_read`. Its outcome is the existing `cli_verified` on `process-start{subject:"claude-child"}`. A missing stamp gives `cli_verified:false` and the Scenario 4 `unverified-cli` branch.
@carried n=13 run=2 label=obs-plan fact=fct-g4daqt4h
quote: read inside `run.version_gate`; a missing stamp, or an unreadable, malformed or strict-modes-refused ledger, counts as unverified (`cli_verified:false`); the path and stamp contents never appear.
@carried n=13 run=3 label=obs-plan/span-trace-coverage fact=fct-f3gi2vhc
quote: The real-`claude` verify never runs in CI (arch §CI/CD approach): it runs only through the local `run --local-live`, which refuses under `CI` (`live-in-ci`).
@carried n=14 label=obs-plan/span-trace-coverage fact=fct-f3gi2vhc
why: The bump moves `portable-pty` off `=0.8.1`, so this behaviour measured on the old release must be re-measured on the new one or its line re-worded.
quote: a child of an interactive PTY run killed at the stop closes with `duration_ms` and a `child_exit_status`, the PTY pump's exit code, since portable-pty reports a code for a signalled child, as measured at chunk 2026-10-14-resume-and-compact-ledger-rows by its resume-run stop case;
@carried n=14 run=2 label=obs-plan fact=fct-itztmsse
quote: the child's `process-exit` comes from the handle-wait thread, never on reader EOF.
@binding n=1
**span-trace-coverage PTY spans and exits ↔ log-coverage PTY seam wrapper**: through the `portable-pty` bump, paste keeps `text_bytes` only (obs-plan/span-trace-coverage fct-xfymn57j), the signalled-child exit code measured on `=0.8.1` is re-measured on the new release (obs-plan/span-trace-coverage fct-f3gi2vhc), and the seam's spawn, handle-wait exit and kill-fallback lines hold (obs-plan fct-3epowryi) with the `portable_pty` target kept OFF (obs-plan fct-7e4jvssp).
@binding n=2
**span-trace-coverage dialog hold ↔ the verification matrix's PROVISIONAL ceiling (CARRY 2)**: the live read near 3600 s reads `deadline_ms`'s top (obs-plan/span-trace-coverage fct-uicv6khr), the hold's end and the 3605 s backstop (obs-plan/span-trace-coverage fct-k233iq5j) and the post-response `hook-invoked` (obs-plan/span-trace-coverage fct-ffumo3bh); the ceiling leaves PROVISIONAL only on that read, made within the directive's two live sessions.
@binding n=3
**span-trace-coverage link and budget-release traces ↔ the v1-13 / v1-36 dispositions (CARRY 4)**: a disposition that proves a clause reads the implicit link on `send` / `answer` (obs-plan/span-trace-coverage fct-lohdixpq, obs-plan/span-trace-coverage fct-u5crvqku), the link's channel-pair-and-product-line trace (obs-plan/span-trace-coverage fct-46kjozmt) and the `budget: true` release (obs-plan/span-trace-coverage fct-wovktszp); a deferral or re-worded acceptance leaves these traces standing unchanged.
@binding n=4
**span-trace-coverage live evidence ↔ obs-scope-summary harness `logs` and the strategy pyramid**: the live drive's evidence is the `send` and `wait` lines (obs-plan/span-trace-coverage fct-gghzntp4, obs-plan/span-trace-coverage fct-hluu43pb) read through `agent-run logs` and `jq` (obs-plan fct-q2zh72zo); spans are declared but not serialized (obs-plan fct-i24bcxka), so no span is evidence.
@binding n=5
**span-trace-coverage child start ↔ pii-scrubbing-compliance non-synthetic uploads**: the live real-`claude` sessions start on a CLI stamped through `run --local-live` (obs-plan/span-trace-coverage fct-6z2owz4q, obs-plan/span-trace-coverage fct-f3gi2vhc), their child `process-start` carries strip names only (obs-plan/span-trace-coverage fct-6xa7lf3f), and any upload of their diagnostics first drops `detail-*.ndjson` and `target/agent-run/` (obs-plan fct-tcjwyf2p).
