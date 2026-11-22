@history_item n=1 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (send's input validation as landed; the feed bound closed) — set the paste as one bracketed `write_all` through `viola-pty`'s `PasteHandle`, with `validate_paste_text` first in the wrapper's `send`, and bounded the vt100 tee to `sync_channel(FEED_CAPACITY)` 256 messages with `try_send` after the human's write. That one blocking `write_all` is the unbounded write this chunk bounds (carry cry-x63fwx7w, `crates/viola-pty/src/pump.rs:77`). The feed-queue precedent is the one to follow: a full queue drops and poisons the model and never blocks passthrough (decision/vt100-feed-bounded-queue, decision/vt100-feed-isolated).

@history_item n=2 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — landed the tee on `run`'s pump: the human's bytes are written first and a feed thread runs under `catch_unwind`, a poisoned model reading `input-not-ready`. The bound was owed to confirmed `send` and is closed in n=1. This is the passthrough-first rule that a blocked or cut paste must not break, and it is what the human's keys rest on while a paste is blocked (decision/vt100-feed-under-panic-catch).

@history_item n=3 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the vt100 passthrough after a caught feed panic, witnessed on the outer terminal) — `chaos_feed_panic_refuses_send_and_keeps_passthrough` reads the child's bytes on the outer terminal and finds no `panic`/`parse-rejected`/`unable`/`hint:` text. It is the guard of decision/vt100-feed-under-panic-catch and the pattern for a witness that a cut paste keeps passthrough and writes no upstream text on the terminal.

@history_item n=4 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — the typed text is taken once after `validate_paste_text`, and that text alone feeds the local-command classification, the confirmation match, `text_bytes` and the paste. It kept the one-write bracketed paste. Standing rule: any other removal from a sent text needs its own ruling. If the paste is cut partway (scope: «text with no Enter»), nothing may later re-type or trim that text without a ruling (decision/bridge-send-one-bracketed-paste, decision/send-text-is-one-bracketed-paste).

@history_item n=5 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; an empty typed text refused) — added the `empty-text` refusal, `not-delivered`, exit 13. The wrapper's `send` checks it right after the typed text is taken and before the first wheel read. This fixes where in `send`'s order the existing refusals sit relative to the wheel, which matters when placing the paste bound's ending «the wheel unmoved» (contract/send-refusal/empty-text).

@history_item n=6 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — every CR LF and every other CR in the typed text becomes one LF, pinned by a 512-case property. A change that types a character the received text did not hold needs its own ruling. This is the origin of decision/typed-text-line-ending-normalisation. A bounded write must still write exactly this typed text inside one bracketed paste.

@history_item n=7 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — added the human-terminal stdin vector: the stdin classifier's closed non-editing list was widened once, by exact grammar, for measured terminal replies, and every other byte is typing that takes the wheel. This governs «what the human's keys do while a paste is blocked and after it is cut» (decision/human-typing-takes-wheel, decision/stdin-closed-non-editing-list). No reply may be added on a guess.

@history_item n=8 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the seventh dated gap: pause and release frames) — served `pause`/`release` and added the `release-from-driver` refusal (`-32602`, a `from` string). The wheel's holder and its moves come from here (contract/wheel-event/human-input, contract/send-refusal/human-typing). The chunk's «wheel not moved» at the bound has to keep these holder rules.

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (channel-response's closed detail word) — `ProtocolError::cause` gives exactly three `-32602` detail words on the WARN `channel-response` line, closed in `schemas/diag-line.v1.json`, and no other line carries a detail. A new send outcome word must not widen this enum by accident (contract/channel-response-log/detail, contract/channel-response-line/release-from-driver).

@history_item n=10 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply) — `sanitised_fault` and `viola_core::is_code_word` (1–64 bytes `[a-z0-9-]`, a letter first) screen every refusal and detail. A word outside the closed list stays visible with exit 14 «unknown». The new closed word for a cut paste must pass `is_code_word` and join the closed lists, or CLI and MCP will show it as unknown (decision/client-reads-closed-values-only, decision/reply-code-words-validated, contract/mcp-iserror/refusal, contract/mcp-iserror/detail).

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census and what it does not drive) — `tests/contract_error_bodies.rs` names every error body with its writer and witness. `product_error_writers_are_each_named` fails on an unnamed writer, and `thiserror_messages_interpolate_nothing` fails on an interpolating message. The `UNUSABLE_TEXT` exit-2 stderr line is a named exception. Trap: a new send outcome body or error writer must be added to the census or the census reads red (decision/every-error-writer-named, decision/unusable-response-line-on-stderr).

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the product error enums, by census) — there are nine `thiserror` derives plus the hand-written `PtyError`. Display messages are fixed, and path or payload fields stay out of Display. A new `PtyError` or pump-write error variant for the bound has to keep a fixed message (decision/error-messages-interpolate-nothing, decision/fixed-error-display-messages, decision/error-display-no-upstream-text).

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21 names its cause in detail; the cause hints; the exit-cause table) — made exit 21's `detail` a closed cause set, added the `unwrapped-name`/`endpoint-lost` hints, and added `tests/cross_exit_causes.rs` (`every_code_is_a_code_word`, `no_two_codes_share_a_hint`). Precedent and trap: a new closed word with an exit and a hint enters this table. The cause in `detail` was a widening the founder answered live (contract/cli-instance-unreachable/json-document, contract/cli-json-error/strict-modes-failed).

@history_item n=14 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused`, the sixth cause and the eleventh code) — the last precedent for adding a closed word across `viola_core`, CLI `--json`, MCP `isError` and `revive --json`. The new word was the founder's own answer, live, after the widening was shown, and the rows were PROVISIONAL until his batch. A closed word for a cut paste is planned the same way (scope: «a new closed word» in `crates/viola-core`).

@history_item n=15 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (Error Handling: 2 rows) — added `call-deadline` as the seventh unreachable cause, written by the MCP frame tools alone at `MCP_CALL_DEADLINE`, with the CLI exit-21 set staying six (contract/instance-unreachable-cause/call-deadline, contract/mcp-iserror/instance-unreachable). This is the deadline the paste bound plus the rest of `send` must stay under. It is also the outcome-unknown reading this chunk replaces with a closed send word.

@history_item n=16 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (Data Protection: the MB arm) — added decision/live-rig-mb-arm-keeps-no-output: the rig's MB arm keeps only closed words of the client and carries the long-call predicate. The scope's facts name the MB arm as the measured long call behind `MCP_CALL_DEADLINE`. This chunk makes no live start, so the arm is evidence only.

@history_item n=17 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP tool input validation and error bodies as landed) — the MCP `send` tool runs `validate_paste_text` and the empty-typed-text check client-side before any frame, with closed `invalid-params` field words. Every `isError` text reads `refused: <code>`, and any other fault is `-32603`. The bound's result on the MCP `send` tool must fit this closed shape (contract/mcp-tools/send, contract/mcp-tool-inputs/invalid-params, contract/mcp-errors/invalid-params, contract/mcp-iserror/text, contract/mcp-structured-content/codes).

@history_item n=18 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the tokio ban's tree form and the rmcp release-graph gate) — the sole-root `deny-sync.toml` tokio ban runs over the crates of `scripts/sync-crates.txt`, and viola-channel's Tokio client sits behind its feature (`scripts/channel-tokio-free.sh`). `viola-pty` is a sync crate, so a time bound on the paste write must not pull tokio into it (decision/no-tokio-in-sync-crates, decision/sync-crate-tokio-ban, decision/sync-crates-tokio-free).

@history_item n=19 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — introduced the tokio ban in `deny-sync.toml` per sync crate as sole root, with `scripts/deny-probes.sh` proving every ban live. This is the origin of the sync-crate tokio rows that any new timing mechanism in `viola-pty` must respect.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (one licence exception and one deny target triple) — `deny-sync.toml`'s tokio ban now runs for the one triple `x86_64-unknown-linux-gnu`. This is the current form of the sync-crate gate a `viola-pty` change is checked against.

@history_item n=21 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty pinned =0.9.0, the swap path retired) — `crates/viola-pty` builds on portable-pty `=0.9.0` with `pty_backend()` always `openpty`, viola-pty 33 of 33 green. The bounded write is built on this PTY writer API, so a new crate or a pin move needs its own audit.

@history_item n=22 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — added the `FAKE_AGENT_PUMP_DELAY_MS` seam: `fake-agent`-only, capped, compiled out of release, not `VIOLA_*`, configures nothing. Another seam needs its own Decisions Log entry. A fake agent that stops reading its PTY (the scope's measuring child), if driven by an env var, is such a new seam (decision/test-seams-fake-agent-only, decision/new-env-read-needs-log-entry, decision/test-seams-compiled-out).

@history_item n=23 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (founder ratification of three standing boundary widenings) — the pump-delay seam was ratified live by the founder. Standing rule: a boundary widening halts the wrap for a live answer, and an earlier direction never ratifies one. A new fake-agent seam or env read in this chunk is that class.

@history_item n=24 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption) — the second test seam was ratified live by the founder. The ban now names another seam as needing its own entry. This is the closest precedent for a third fake-agent behaviour (decision/no-control-switch-off, decision/controls-not-switchable: the two named seams switch off nothing).

@history_item n=25 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — admitted the colour facts beside the test seams and wrote the NEVER: another seam or environment read without its own Decisions Log entry. This is the current wording of the env-read ban a new fake-agent switch would meet (decision/no-stored-secrets, decision/env-not-config-channel).

@history_item n=26 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — `release-check` refuses `test-support`/`fake-agent` artifacts (probe 5/5). A new fake-agent behaviour must stay under the test-only feature, where this gate catches it (decision/release-build-no-test-binaries, decision/release-build-without-test-features).

@history_item n=27 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration U35) — moved into the body `scripts/release-check.sh` judging the build's own artifact records and refusing any non-`viola` executable or `test-support`/`fake-agent` artifact, with `test-support` enabled only by dev-dependencies. This is the guard text of decision/release-build-without-test-features.

@history_item n=28 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` was validated into `DialogHold` (60–3600) so that no value makes the hold zero or unbounded, and no env var or flag sets it. This is the origin of decision/config-cannot-disable-controls. The paste bound is a compiled constant (scope), and the precedent says no config value may switch it off.

@history_item n=29 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Input Validation; Authentication & Authorization) — retired the budget gate and `budget-paused`. It narrowed decision/config-cannot-disable-controls to the dialog hold and claude_env_keep, and set `release`'s string-`from` refusal ahead of any other param check (decision/driver-release-refused, contract/channel-release/params). These are the current send-refusal set and wheel-release rules beside a new send word.

@history_item n=30 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (Input Validation; Threat Model Summary) — replaced exact-match confirmation with decision/delivery-confirmation-trailing-tolerance and dropped «stays an exact equality» from decision/typed-text-line-ending-normalisation's why. It bears on how a cut paste and the next send's confirmation are read: only a trailing whitespace run is tolerated, never a leading trim.

@history_item n=31 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (the send trailing-only tolerance ratified by the founder) — the trailing-only tolerance stands on the founder's own word (state standing). A leading trim waits for a live reading. The disposition of a cut paste's leftover box text must not lean on any wider confirmation tolerance.

@history_item n=32 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (Threat Model Summary: 1 row) — changed contract/wrapper-channel-methods/send: a listed local command sent before its transcript exists, or on an unverified CLI, answers ok with confirmed false (unconfirmable), never a confirmation. This is the nearest send-outcome precedent. The paste bound's ending must say where it sits against «never presumed delivered».

@history_item n=33 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Security Anti-Patterns: 1 row) — decision/compiled-classification-sources: a typed text is a local command only by its compiled name. Classification runs on the typed text the paste writes, so a partially written paste must not be re-classified from screen or upstream text.

@history_item n=34 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Input Validation: 1 row) — added decision/input-box-readiness-from-literals: one predicate over compiled literals decides readiness for verify and for `viola run`'s pre-send gate. The readiness gate's bound is one of the totals the paste bound is read against, and a cut paste's leftover text in the box meets this predicate on the next send.

@history_item n=35 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** (Input Validation: rule and except) — the readiness predicate's modal clause moved to «a standing modal is up», with the quote exception and its residual (a stand-alone quote line still refuses). This is the current form of the gate that comes before the paste write and whose bound is in the arithmetic.

@history_item n=36 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** (Authentication & Authorization: 1 row) — added decision/live-rig-kr-arm-keeps-attributes-only for the KR arm (own-pty resizes, key counts, kept readings). This is the predecessor chunk that this chunk's carries fold from. The KR arm stays outside this chunk's no-live-start boundary.

@history_item n=37 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (`MAX_FRAME` consumers extended, verify's CI split) — set the rule that the real CLI runs only locally and that `verify` runs in CI only against the fake agent. It underlies the scope's fake-agent-only measuring child (decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude).

@history_item n=38 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — removed the CI and pre-push mutation legs and moved mutation testing to the epoch-boundary audit (decision/mutation-testing-at-audit). This matches the scope's «no mutation run». The `viola-pty` test comment's `mutants` profile kill (carry cry-mui36h42) belongs to that audit.

@history_item n=39 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — made Run B's compiled pastes go only into rows holding an input-box literal, with a modal, a poisoned screen or the deadline ending the added pastes with no key. Verify's pastes are another caller of the PTY paste path, so a write bound in `viola-pty`'s contract reaches them too.

@history_item n=40 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the diagnostics opens refuse a symlink or a widened path) — `open_role_file`/`write_detail` run `check_diagnostics` before narrowing anything, and a refusal is a failed open to a sink. Any new diagnostics line for a cut paste writes through this path (decision/diagnostics-open-fails-to-sink, decision/diagnostics-strict-modes-fail-open).

@history_item n=41 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — the canary scan skips only the exact-path `chunk.diff`, which is never uploaded (decision/ci-artifact-canary-scan except). The new tests' diagnostics and harness capture stay under the scan's gate, so a cut-paste test must leave no upstream text in them.

@history_item n=42 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — CI runs the artifact canary scan against the NEVER-log floor before every test-home upload and never prints the matched bytes. This is the origin of decision/scan-gated-ci-uploads, which every new test of this chunk's uploads go through.

@history_item n=43 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (Dependency Security: 3 rows; the advisory rule widened) — added decision/advisory-ignore-grounds and decision/advisory-ignore-list (RUSTSEC-2026-0543, chrono, on the unmaintained-upstream ground), each such ignore re-read at each epoch boundary. Carry cry-3un3aeio rides this chunk as Epoch 4's last entry. The re-read is the founder's, at the boundary.

@history_item n=44 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the first-start race on the pinned copy closed) — the pinned binary is placed once and never replaced, and the write handle is closed before the name appears. That is decision/pinned-binary-placed-once. The scope's pinned product is untouched, and the `send` change reaches the driver only at the re-pin.

@history_item n=45 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Threat Model Summary: 3 rows) — widened decision/cwd-only-in-snapshot and decision/spawn-cwd-snapshot-only to the trust record key in the Claude config. Still on no log line, error body or stdout: a cut-paste line or result must not carry the spawn directory.
