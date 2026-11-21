@history_item n=1 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP tool input validation and error bodies as landed) — each handler validates its raw argument object (never rmcp's typed deserialisation); `wait`'s `timeout_ms` is clamped to `WAIT_CAP` 50 000 ms, also its default; a failure is `isError` `invalid-params` with a closed field word, no frame; any other fault, a reply of no known shape or a panicked call is `-32603`. This clamp is the caller-supplied wait the chunk must hold below the client's tool-call timeout. Any new cap, deadline result or error word must stay a closed code (rows contract/mcp-tool-inputs/timeout-ms, decision/mcp-handlers-validate-raw-arguments, contract/mcp-errors/-32603).

@history_item n=2 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp process spawns claude agents --json) — the MCP `list` tool makes one `claude agents --json` read per call through `list::board_document` on a blocking thread, killed at 5 s and `MAX_FRAME`-bounded. That fixes `list`'s longest block, and it was a ratified widening, so a change to how `list` blocks or spawns needs the same showing.

@history_item n=3 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the eighth dated gap: MCP frame tools after the liveness-only pre-check) — the origin of the `send`/`wait`/`last`/`answer` tools writing frames through `viola-channel`'s Tokio client. It is the path whose block the chunk bounds. Later entries closed both halves of the gap, so the check order now runs before any frame.

@history_item n=4 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the tokio ban's tree form and the rmcp release-graph gate) — `viola-channel`'s Tokio client sits behind its `tokio` feature (`scripts/channel-tokio-free.sh`, the `channel-tokio-edge` probe). rmcp in viola's release graph is exactly `server` + `transport-io`, checked by `scripts/rmcp-features.sh`. A compiled deadline built on tokio timers or new rmcp features (for example progress notifications) must stay inside these gates and the sync-crate tokio ban (decision/no-tokio-in-sync-crates).

@history_item n=5 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (viola mcp's stdin capped at MAX_FRAME per line; the MCP-over-stdio threat vector) — `CappedLines` sits on the reader handed to rmcp, never the codec. An over-long line is refused whole and ends the session as a stdin close does, logging one `parse-rejected{mcp-stdin, oversize}` that holds no byte of it. This is the precedent for bounding the MCP transport in viola's own code around rmcp. A client-side timeout teardown must read the same way: the session ends with no partial dispatch.

@history_item n=6 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — `tests/contract_bounded_readers.rs` requires every product reader call to be bounded in its statement or named in `EXCEPTIONS`, and each boundary to have a witness (decision/bounded-readers-census). Any new reader or wait the chunk adds to the tool handlers falls under this census.

@history_item n=7 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply) — `of_reply` in `crates/viola-mcp/src/result.rs` and `reply_of` read a fault only through `viola_channel::sanitised_fault`, and refusal and detail only as code words (`is_code_word`). Nothing of the peer's but closed values reaches an MCP result. A deadline-expired result built in `result.rs` must keep that form (decision/client-reads-closed-values-only, decision/client-reply-code-words-only).

@history_item n=8 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census) — `tests/contract_error_bodies.rs` (`MCP_RESULTS`, `product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`) fails on an unnamed product error writer. A new timeout error body or error variant in `viola-mcp` must be enumerated with its writer and witness, and its message must interpolate nothing.

@history_item n=9 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21 names its cause in detail; the no-endpoint reading) — MCP's `instance-unreachable` `detail` is a closed cause set, and `endpoint-lost` is "the wrapper lost with the call open". Adding a new cause or error word for a call cut at its bound is a closed-set widening that took the founder's own word last time (picked rows contract/instance-unreachable-cause/*).

@history_item n=10 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`socket-dir-refused`, the sixth cause; the client's peer euid check and the order at a seam) — `frame` in `crates/viola-mcp/src/tools.rs` runs the directory check, the connect, the peer euid decision, then the server verification, each a refusal with no frame and no line. A deadline wrapped around `send`'s whole path covers these steps and must not reorder or skip them (decision/client-seam-check-order, decision/frame-connect-check-order).

@history_item n=11 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the dated gaps keep only their strict-modes half; a failed server check) — `viola mcp`'s tools verify after `TokioClient::connect`, with the start-time read on `tokio::task::spawn_blocking`. A failed check returns `isError` `instance-unreachable` with no frame and no line. These blocking pre-frame reads are part of each tool's block to bound.

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola mcp`'s frame tools check the instance before `live_endpoint`; snapshot readers onto `read_snapshot_or_replay`) — each MCP frame tool runs `check_instance` and then reads the snapshot through the replay. A refusal is quiet, with no line. These pre-frame filesystem steps sit inside the bounded block, and their order is fixed (decision/strict-modes-before-snapshot-read, decision/mcp-frame-tools-refuse-quietly).

@history_item n=13 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (send's input validation as landed; the feed bound closed) — sets out `send`'s path: the client-side `validate_paste_text`, then the frame, then the wrapper re-checking first and typing one bracketed paste. The vt100 feed queue is bounded (`FEED_CAPACITY`), and a poisoned model reads `input-not-ready`. This is the wrapper-side path whose confirmation wait makes `send`'s longest block.

@history_item n=14 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** (the fifth dated gap) — the channel `wait`/`last` params clause: `after` and `timeout_ms` are absent or a `u64`, else `-32602` with `data: null` (contract/channel-frames/wait-last-params). Any cap on a caller's argument must stay consistent with this wire rule and with MCP's `WAIT_CAP` clamp.

@history_item n=15 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (the sixth dated gap: answer and hook.dialog frames) — sets `answer`'s params, the closed per-kind `response`, `validate_paste_text` on free text, and the hook's dialog reply read bounded at a deadline. These are the precedents for `answer`'s path and for a bound read in viola's own code.

@history_item n=16 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` validated into `DialogHold` (60–3600 s, default 60): no value makes it zero or unbounded, and no env var or flag sets it. The hook's reply read is bounded at `DIALOG_HOLD_MAX + 5 s`. It is the precedent that a time bound is a validated, compiled-ceiling surface, and that adding a config key is a widening (contract/config-json/dialog-hold-s, decision/config-cannot-disable-controls).

@history_item n=17 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — the typed text (trailing LF removed after validation) is the only text `send` uses for classification, confirmation, byte count and paste. "Reject, don't strip" stands, and any other removal needs its own ruling. This is a constraint if the chunk touches `send`'s confirmation path.

@history_item n=18 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; an empty typed text refused) — `empty-text` is refused by the client before any frame and by the wrapper before the first wheel read. This fixes the order of the early refusals inside `send`'s path (contract/send-refusal/empty-text, decision/empty-typed-text-refused).

@history_item n=19 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — an inner CR or CR LF is typed as one LF after validation, and the set of accepted texts is unchanged. Typing any character the received text did not hold would be a new class needing a ruling (decision/typed-text-line-ending-normalisation).

@history_item n=20 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — delivery confirmation moved from exact equality to a trailing-whitespace tolerance (decision/delivery-confirmation-trailing-tolerance, replacing decision/delivery-confirmation-exact-match). Ratified as trailing-only by the founder at 2026-11-16T06-05-33-wrap, with a leading trim owed to a live reading. The confirmation wait is `send`'s longest block, and its match rule must not move with the bound.

@history_item n=21 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — local commands are classified by compiled name only (decision/compiled-classification-sources). Their delivery is confirmed through a transcript read that is bounded and count-only (decision/transcript-read-bounded). Verify's echo probe residuals were accepted (decision/verify-echo-probe-residuals). Part of `send`'s confirmation path that a compiled bound must cover.

@history_item n=22 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — the `send` contract now answers `ok` with `confirmed:false` (unconfirmable) for a listed local command before its transcript exists, or on an unverified CLI, never a false confirmation (contract/wrapper-channel-methods/send). It is the precedent for how `send` reads when confirmation cannot complete. It also added Run B's eighth paste, `/context`, with its residual accepted as measured (decision/verify-context-probe-residual).

@history_item n=23 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 degrade: the feed runs under `catch_unwind`, and a poisoned model reads `input-not-ready` to confirmed `send`. It is the gate in `send`'s path before typing (decision/vt100-feed-isolated, decision/vt100-feed-under-panic-catch).

@history_item n=24 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Input Validation; Dependency Security) — added decision/input-box-readiness-from-literals: one predicate decides for verify and for `viola run`'s pre-send gate. It also replaced the empty advisory ignore list with reasoned ignores for disabled rmcp features (decision/rmcp-advisory-ignores). A new rmcp feature taken for progress or timeout handling would meet those ignores' disabled-feature ground.

@history_item n=25 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — the readiness predicate's modal clause became "a standing modal". verify reads compiled signatures: the literals and the compiled shapes over them. The pre-send gate is part of `send`'s block, and any verify probe added for the bound reads screens only this way (decision/verify-reads-only-signatures, decision/verify-screen-read-against-literals).

@history_item n=26 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — the advisory ignore rule was widened to two grounds and a closed list (decision/advisory-ignore-grounds, decision/advisory-ignore-list), re-read at each epoch boundary. If the chunk moves rmcp or tokio, any new advisory ignore needs one of these grounds.

@history_item n=27 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — `[[bans.features]]` holds the `rmcp` ban (`transport-streamable-http-server`, `auth`), proven live by `scripts/deny-probes.sh`. This constrains which rmcp features a bound or progress mechanism may enable; the MCP server stays stdio-only (decision/mcp-over-stdio).

@history_item n=28 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the `claude agents --json` child is a direct spawn under the R8 strip, its stdout through `take(MAX_FRAME + 1)`, killed at 5 s, failing open. That compiled 5 s kill bounds the MCP `list` tool's block (decision/claude-agents-read-fail-open, decision/child-output-bounded).

@history_item n=29 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange recorded by the test-only tap) — the live CLI's MCP client crosses into the test-only `viola-mcp-tap`, which records `Mcp.` fixtures under the scrub and closed refusal codes. A ratified widening (one start), and the precedent for measuring the client's MCP behaviour (its tool-call timeout) live.

@history_item n=30 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** — the tap is now started live by the standing rig under its private CLI config (decision/mcp-fixtures-via-test-only-tap), and the rig's MT arm keeps no CLI output (decision/live-rig-mcp-arm-keeps-no-output). Any rig reading of the client's timeout at its bound must stay inside these rows, shown whole at P3.

@history_item n=31 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — added decision/live-round-rig-keys-own-pty and decision/live-session-screens-kept-local. A live measurement of the bound needs the founder's word anew for the chunk, and its screens stay local.

@history_item n=32 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — the rig's key grant became a closed argv grant, and it added decision/live-rig-credentials-copy and decision/live-rig-isolated-child (a private `CLAUDE_CONFIG_DIR`, a closed kept env list). Any rig arm that drives an MCP call to its timeout runs under these.

@history_item n=33 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — the rig's stop (Ctrl-C into its own pty or a no-key group kill) needs no per-chunk word, while its argv grant does. Its prompt capture keeps closed words only (decision/live-rig-prompt-capture-closed).

@history_item n=34 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** — added decision/live-rig-hb-arm-keeps-attributes-only and widened the kept-stamp seeding (decision/stamps-written-only-by-verify except). These are the rig precedents for a new arm or a seeded stamp, should the bound be measured live.

@history_item n=35 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive children: direct spawns under the R8 strip at 80×24, read only against compiled literals, never a key into a dialog, a modal start killed with no key. Each widening of the verify child set took the founder's live ruling. A 30th relied row with a verify probe is that class of widening (decision/verify-children-direct-spawn, decision/verify-never-answers-dialogs).

@history_item n=36 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's added compiled pastes, with waits up to `PROBE_DEADLINE` (120 s), and the stamp's relied-row count rising with each row. It is the precedent for adding a ledger row plus its probe, and for older stamps reading `cli_verified:false` when the relied set grows.

@history_item n=37 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the sixth Run B paste and the relied-row stamp growth: an older stamp of fewer ids reads unverified. Precedent for the fork the P3 block prices: a new relied row re-gates dialog answers on every home until re-stamped (decision/dialog-answers-need-verify-stamp).

@history_item n=38 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp and contract/cli-exit-codes/12 now read "every relied ledger row counts: pass, or absent on the browser-tools-prompt row alone". A 30th relied row joins this gate. Whether it may read absent is a ruling, not a default.

@history_item n=39 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled on the live reading) — StopFailure stands registered with no capability ledger row and no verify probe, on the founder's word over one contained live reading. This is the precedent for the chunk's fork: a relied row with a probe, or a compiled bound with no ledger row.

@history_item n=40 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — the latest relied-row-plus-probe precedent. It widened verify's child set (a print-mode sender and a compiled accept settings literal: decision/verify-children-direct-spawn), made the uds address the one non-literal prompt part (decision/probe-prompts-synthetic), and accepted residuals (decision/verify-cross-session-probe-residuals). It held `XDG_RUNTIME_DIR` in memory only (decision/verify-runtime-dir-held-in-memory). The capture arm also gained `--names-only`. All of these rows stand PROVISIONAL for the epoch batch.

@history_item n=41 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — the capture arm reads its own process's env names on SessionStart, names only (decision/capture-arms-isolated). The stamp's env probe holds names and counts only (decision/env-probe-names-and-counts-only). The env fixture holds names only (decision/env-fixture-names-only). If verify probes a timeout-moving env name, it may keep the name and never the value.

@history_item n=42 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (`MAX_FRAME` consumers extended, verify's CI split) — the real CLI runs only locally. In CI, verify runs only against the fake agent, with its real-CLI probe and `--record` local. A bound witnessed in CI must be a compiled bound with a fake-agent test, not a real-CLI reading (decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude).

@history_item n=43 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — `FAKE_AGENT_PUMP_DELAY_MS` exists only under the `fake-agent` feature, capped at 5 000 ms and compiled out of release. Another seam or env read needs its own Decisions Log entry. A test that drives a tool past its bound may need a delay seam, and that is a boundary widening (decision/fake-agent-pump-delay-seam, decision/new-env-read-needs-log-entry, decision/test-seams-fake-agent-only).

@history_item n=44 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (founder ratification of three standing boundary widenings) — the pump-delay seam stands as ratified. Standing rule: a widening halts the wrap for a live answer, and an earlier direction never ratifies one. This applies to any new test seam or reading the chunk adds.

@history_item n=45 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the precedent that an environment read (the colour facts) is a boundary widening needing its own ruling and Decisions Log entry. viola must not read the client's timeout env names to set its own bound (decision/env-not-config-channel, decision/controls-not-switchable).

@history_item n=46 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — question option text crosses into CLI `wait --json` and the MCP `wait` (decision/question-option-text-crosses-boundary). It is part of what the MCP `wait` returns when it completes inside its bound.

@history_item n=47 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the human's terminal on `viola run`'s stdin as an attack-surface vector: the closed non-editing list, and typing taking the wheel. This feeds `send`'s wheel read and the `human-typing` refusal inside `send`'s path (decision/human-typing-takes-wheel, decision/stdin-closed-non-editing-list, contract/send-refusal/human-typing).

@history_item n=48 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify's own trust record is the one bounded write into the Claude config, by a guarded writer. verify's reads of `CLAUDE_CONFIG_DIR` happen only when kept. A new verify probe for the timeout must write nothing else under `~/.claude` (decision/nothing-written-under-claude-home, decision/trust-record-writer-guarded).

@history_item n=49 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — widened the Claude-home exceptions to the start's trust flag and the shared guarded writer (decision/no-writes-under-claude-home, decision/viola-writes-nothing-in-claude-dir). These are the current exception set that any new probe or rig write would extend.
