@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the six landed ledger rows, `viola verify`'s probe, the stamps envelope) — founded the closed ledger, `viola verify`'s probe and the stamps envelope; a version is verified only when every relied row passes and the local-command rows were first owed to confirmed send. Matters because an echo this chunk relies on must land as a new `LedgerRow` with a verify probe and stamp, under decision/cli-capability-ledger and decision/capability-ledger-gate.

@history_item n=2 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (the readiness gate as landed and its provisional timing values) — landed the vt100 `Screen` on `run`'s feed thread and `CONFIRM_WINDOW_FALLBACK` = 10 s. Matters for decision/confirm-window-compiled and decision/vt100-readiness-gate: a screen-line echo would have to come through this gate model, whose only consumer is pre-send readiness.

@history_item n=3 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (confirmed send: kinds, refusals, exits, the relabel) — added `send-issued` / `send-confirmed` / `send-refused`, the `send` refusal order and the driver relabel, and recorded that with no local-command row a local command (`/clear` included) ends `not-delivered`. This is the origin of the false `no-prompt-submitted` this chunk fixes for unlisted local commands, and of the event records any echo-confirmed send must write.

@history_item n=4 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the running-turn state as built) — the running turn is started by any `prompt-submitted` and ended by `turn-ended` / `session-start` / `session-end`, keyed on kind alone. Trap: a local command fires no `prompt-submitted`, so it starts no turn, and `/clear` / `/compact` end one through `session-start`. An echo-confirmed command must not leave the turn state or decision/send-never-queued wrong.

@history_item n=5 marker=2026-10-05-t00-09-13-wrap
**2026-10-05-t00-09-13-wrap** (the typed-input probe ratified; its rows owed to "Real-CLI verify probes") — the founder ratified the PTY typed-input `viola verify` probe against the live CLI, run locally only, and the local-command rows rode it. This is the precedent for how a local-command reading becomes a ledger row (decision/real-cli-probe-local-only).

@history_item n=6 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (screen signatures compiled, ten ledger rows, verify's interactive runs) — compiled `SIGNATURES`, made `Screen::rows()` verify-only, passed `Some(&SIGNATURES)` only when `cli_verified` (the partial gate otherwise), added `confirm-window`, and gave verify Runs A and B with no key into a dialog. Trap: decision/run-never-reads-screen-content and decision/screen-rows-read-only-by-verify forbid `run` from reading a screen echo line.

@history_item n=7 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (seventeen ledger rows, Run B at four pastes) — compiled `LOCAL_COMMANDS` and the `local-command-clear` row (`/clear` fires SessionEnd `clear` then SessionStart `clear` with a new id and no UserPromptSubmit), measured by a Run B paste. Each added Run B paste was ratified by the founder as a widening. This is the direct precedent for measuring a local command's echo, and the widening card that pattern needs.

@history_item n=8 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (registry rows for the framing variants, the fake agent's two options) — the fake agent's `--framing` replays the probed local command's recorded `clear-1` variants, and `--record` writes the `SessionEnd.clear-<n>.json` / `SessionStart.clear-<n>.json` stems. This is how a new echo fixture class and its replay would be added under `fixtures/claude/<version>/`.

@history_item n=9 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (send consumes the local-command list) — `send` classifies by exact equality with `LOCAL_COMMANDS` (never a leading slash); a listed command with post-condition none, or on an unverified CLI, is answered `ok` `unconfirmable` at once. `/clear` waits for its post-condition, claimed through the remembered `agent_session_id`; a missed window is `no-prompt-submitted`, with no new detail. These are the classification, unconfirmable and post-condition-claim mechanics this chunk extends (decision/local-commands-by-ledger, contract/send-results/unconfirmable).

@history_item n=10 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (the paste-hint reading, the fake agent's eighth option, the mirror's fourth line) — added the CLI mirror's `[  ] unconfirmable` line with its one fixed note (`src/human/mirror.rs`), and named the remembered session id and the `session-start` claim in `send.rs`. Matters because the scope names this mirror as a surface whose outcomes change.

@history_item n=11 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the long-paste frame as measured beside typed text and between two pairs) — the founder ruled the limit that a CLI shape no `send` relies on needs no probe. This is the source of decision/no-probe-for-unrelied-shapes. An echo that confirms a send is relied on, so it falls outside that limit.

@history_item n=12 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** (the gate waits for the input box, bound 8.5 s) — set `GATE_MAX_WAIT` 8.5 s, the wheel and turn read twice (at arrival and after the gate), and `send` blocking at most 18.5 s. Matters for decision/send-time-bound and decision/send-waits-out-paste-hint: any echo window must keep the send total under the MCP ceiling.

@history_item n=13 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** (send types a text without its trailing LF) — the typed text is the only text `send` uses for the local-command classification, the match, `text_bytes` and the paste, so `/clear` plus a newline is `/clear`. Matters because local-command classification runs on the typed text (decision/typed-text-normalisation).

@history_item n=14 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (the live readings on 2.1.287 are measured) — measured the `/clear` confirmation and `/clear` plus a newline live, and landed the `input-not-ready` hint wording. The `/clear` post-condition is measured and must stay untouched (scope boundary).

@history_item n=15 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the typed text drops trailing CR and LF) — `typed_text` strips every trailing CR and LF, so `/clear` with a CR or a CRLF is `/clear`, and `typed_text` has two callers (the wrapper's `send` and the client's `refusal_of`). The local-command list is matched on this typed text.

@history_item n=16 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (an empty typed text is refused `empty-text`) — `not-delivered`'s closed detail set gained `empty-text`, and the refusal order now opens `control-character` → `empty-text`. Any new outcome detail must extend this closed set in `viola-core` (decision/refusal-details-closed-set, contract/refusal-reason/not-delivered).

@history_item n=17 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** (the typed text types an inner CR or CR LF as one LF) — the full normalisation of decision/typed-text-normalisation and decision/send-text-line-endings, which the local-command classification reads.

@history_item n=18 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the capability ledger: three relied-on dependences with no row, and where each row is owed) — a relied dependence that lands with no row is a named exception with an owner, on the founder's word; it is never a silent change to the rule. This is the precedent if the echo cannot be rowed in this chunk.

@history_item n=19 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — split `src/run/send.rs` into `slot.rs` (the claim, relabel and `session-start` post-condition claim) and `classify.rs` (the local-command decision), and `src/human.rs` into `src/human/mirror.rs` and others. These are the exact files the scope names as touched surfaces.

@history_item n=20 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** (eighteen ledger rows, Run B's fifth paste) — adding a row makes every older stamp read unverified until re-stamped, `MM` in the step lines is the relied count, and a new Run B paste is a ratified widening of verify's child set. Trap: a new echo row forces a re-stamp and an `[NN/MM]` change (contract/viola-verify-output/step-line, decision/version-verified-all-relied-rows).

@history_item n=21 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E and /compact listed) — `LOCAL_COMMANDS` gained `/remote-control` and `/compact` with post-condition none (`unconfirmable` at once), on the founder's answer to list `/compact` and owe its row. This is how an unmeasured local command was handled before this chunk.

@history_item n=22 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tools' contract as landed) — the MCP client's tool-call timeout exceeds 50 s on 2.1.287, above the send total. A longer echo window must stay below it (decision/send-time-bound). The MCP `send` result maps the channel's refusal and ok.

@history_item n=23 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (/compact confirmed by its same-session SessionStart, its own 40 s window) — `PostCondition::CompactedSession`, `COMPACT_CONFIRM_WINDOW` = 40 s (provisional), and `SendSlot::window()` as the one bound. `/remote-control` is left the only command with none. This is the closest precedent: a local command given a measured post-condition, its own window and a claim arm in the tap (decision/compact-confirmed-by-same-session, decision/compact-confirm-window).

@history_item n=24 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (the twenty-fourth ledger row, local-command-compact, and Run B's sixth paste) — the `local-command-compact` row and check, Run B's sixth compiled paste (`PROBE_COMPACT_COMMAND`, a founder-ratified widening), `FRAMING_TURNS` gaining `compact-1`, `--record` stems, `compact_latency_ms` in `typed_probe`, and the fake agent replaying `compact-1`. This is the template for a local-command echo row, its probe paste, fixture and replay.

@history_item n=25 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (Run B's three transcript files) — each local command Run B pastes leaves CLI transcript residue, which was measured and accepted. Another probe paste would add residue that the plan must name.

@history_item n=26 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the second input-box literal and the three-screen input-box-signature row) — `SIGNATURES.input_box` gained `(shift+tab to cycle)`, and Run B's added pastes go only into rows holding an input-box literal. 2.1.287 and 2.1.302 are the stamped versions. Matters for the probe's pastes and for decision/screen-signatures-compiled.

@history_item n=27 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the 2.1.302 fixture set and the long-turn screen switch) — the fake agent's option count (13) is the list's rule, and the 2.1.302 set was recorded whole by one `--record` round. A new echo fixture or fake-agent mode moves these counts (the scope names `src/bin/viola-fake-agent.rs` and `fixtures/fake-scripts/`).

@history_item n=28 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules) — split `src/cmd/verify/typed.rs` into `typed/runs.rs`, `typed/turns.rs` (`framing_turns`, `probe_until`) and `typed/pty.rs` (`box_wait`), and placed the ledger's unit cases in `src/ledger/test_cases.rs`. These are where verify's probe for an echo row lands (scope: `src/cmd/verify/`).

@history_item n=29 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail a code word or null) — a reader treats an unrecognised code-word `detail` as opaque text, never an error. This bounds contract/refusal-reason/detail and contract/peer-reply/detail if the chunk adds an outcome detail.

@history_item n=30 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set: all twenty-four rows on every build) — `LedgerRow::relied()` is every row of `ALL` on every build, so a new row is relied everywhere and moves `MM` for all homes.

@history_item n=31 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Established Decisions: 2 row(s)) — changed `~ decision/screen-signatures-compiled` (the box read by its own rule and prompt rows beside the footer literals) and `~ decision/verify-box-wait-for-literal`. These are the readiness and verify-wait rules any screen-observed echo or new Run B paste must respect.

@history_item n=32 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Standard Contracts: 1 row(s)) — changed `~ contract/ledger-stamps/measured` (adding `box_only_guards`). A new measured reading for an echo extends this list with no `v` bump.

@history_item n=33 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Occupied Resources: 1 row(s)) — changed `~ decision/screen-fixtures-signature-rows-only`: a recorded screen keeps only signature rows. Trap: a screen-line echo cannot be recorded whole as a fixture under this rule.

@history_item n=34 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Conventions: 3 row(s), 2 of 2) — changed `~ contract/refusal-order/send` (the budget rung removed). This is the current send refusal order the chunk's outcomes sit in.

@history_item n=35 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Established Decisions: 7 row(s), 1 of 2) — changed `~ contract/send-refusal/order` and `~ decision/permission-request-question-stays-null` (the ledger count moved). The send order is now `control-character` → `empty-text` → `human-typing` → `turn-running` → the gate.

@history_item n=36 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Established Decisions: 5 row(s), 1 of 2) — changed `~ decision/permission-request-question-stays-null` (the relied rows 23 → 21, so the next row is the 22nd). An echo row would be the 22nd ledger row and re-stamp every version.

@history_item n=37 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Conventions: 2 row(s)) — changed `~ decision/protocol-version-bump-rule` (removed stamp row ids kept `v` at 1). Adding an outcome field or detail is additive, with no `v` bump; removing or redefining one bumps `v`.

@history_item n=38 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Established Decisions: 3 row(s)) — changed `~ decision/cli-capability-ledger` (the StopFailure exception), `~ contract/hook-tiers/ordering-spine` and `~ contract/hook-tiers/sync-spine`. This is the most recent precedent for relying on a CLI behaviour with no ledger row, as a named exception on the founder's word.

@history_item n=39 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Cross-cutting Patterns: 1 row(s)) — changed `~ decision/capability-ledger-gate`'s except (StopFailure). Any echo relied on with no row would need the same explicit except.

@history_item n=40 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (Design Philosophy: 1 row(s)) — changed `~ decision/capability-ledger`'s except (StopFailure, no row, no probe), a picked row this chunk may have to extend.

@history_item n=41 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (Standard Contracts: 2 row(s)) — changed `~ contract/hook-event/driver-relabel` and `~ contract/channel-methods/hook-event-driver-origin` (the trailing-whitespace tolerance). The scope keeps this match untouched for ordinary sends, while the relabel's exception for a post-condition-waiting send stands.

@history_item n=42 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (Established Decisions: 3 row(s)) — replaced `− decision/exact-prompt-match` with `+ decision/submitted-form-prompt-match`, and added `+ decision/send-flight-lock`: the post-gate reads, link, `send-issued` and paste run under one wrapper flight lock that the `hook.event` claim, turn moves and wheel moves also take. An echo claim must take that lock like the existing claims.

@history_item n=43 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Established Decisions: 2 row(s)) — changed `~ decision/cli-capability-ledger` (StopFailure no longer provisional) and `~ decision/submitted-form-prompt-match` (now standing). This is the current state of the ledger rule's exceptions and of the untouched ordinary-send match.

@history_item n=44 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Design Philosophy: 1 row(s)) — changed `~ decision/capability-ledger`'s except to the ratified StopFailure form. This is the current text the chunk extends if the echo lands without a row.

@history_item n=45 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Cross-cutting Patterns: 1 row(s)) — changed `~ decision/capability-ledger-gate`'s except to the ratified StopFailure form. This is the current single-gate rule text for any echo dependence.
