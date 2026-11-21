@history_item n=1 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tools' contract as landed) — set MCP `wait`'s `timeout_ms` to `min(given, 50 000)` (`WAIT_CAP`, `wait_timeout`, 50 000 by default) because claude 2.1.287's stdio tool-call default was measured live only as "exceeds 50 s", the exact default unmeasured. It also recorded under [Delivery Confirmation] that the client timeout sits above the 18.5 s send total, and set `invalid-params` / `-32603` and `ttlMs` 0 / `cacheScope` `private` on `tools/list`. This is the standing ground the chunk re-measures: `decision/mcp-wait-cap` and `decision/send-time-bound` rest on a reading that gave no exact number, and a new cap or bound replaces or tightens it.

@history_item n=2 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP list builds the board and reads agents; the Tokio client; the plugin registers viola mcp) — MCP `list` runs `list::board_document` on a blocking thread and makes one `claude agents --json` read per call, a ratified widening. The Tokio flavour sits in `viola-channel`'s feature-gated `TokioClient`. Only `viola-mcp` enables tokio (`rt`), with rmcp release features exactly `server` + `transport-io` (`scripts/rmcp-features.sh`), and `viola-e2e` takes rmcp `client` alone. `.mcp.json` registers `viola mcp`, and `mcp.ndjson` gets its lines. Trap: a tokio timeout bound in `viola-mcp` (for example tokio's `time` feature) is a dependency/feature change under `decision/tokio-only-in-mcp-crate`, `sync-core-tokio-only-in-mcp` and the channel tree-form ban. List's longest block is the board read plus the agents read.

@history_item n=3 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added `COMPACT_CONFIRM_WINDOW` = 40 s (PROVISIONAL) beside the 10 s window, through `SendSlot::window()`. It stated that a compact send's blocking total "gains 48.5 s plus the paste, under the MCP tool-call ceiling read at 50 012 ms". This is `send`'s known longest block and the one figure nearest the client bound: a 48.5 s+paste path under a ~50 s reading leaves almost no margin, so it is the first gap the chunk must close or prove (`decision/compact-confirm-window`).

@history_item n=4 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — raised `GATE_MAX_WAIT` to 8.5 s, the founder's ruling to cover the CLI's 8.0 s paste hint. It set `send`'s block to "at most 18.5 s (8.5 s and 10 s)" and added the second wheel/turn read after the gate. This is the origin of the `send-time-bound` figure. Trap it recorded: a case reading a verified, literal-less screen through a clock that never advances waits forever. A bound test must drive a real or advancing clock.

@history_item n=5 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the paste hint measured on the live CLI, and the founder's decision) — the founder chose the 8.5 s wait on a priced card that named `send`'s longest block as 18.5 s. This is the precedent that raising a bound on `send` is priced and ruled by the founder, not by a delegate.

@history_item n=6 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — named the timing constants (`QUIET_PERIOD`, `GATE_MAX_WAIT`, `CONFIRM_WINDOW_FALLBACK`) as compiled PROVISIONAL built-ins. It made `viola_core::SPINE_DEADLINE` = 1 s the product bound and kept the hook's 750 ms `CONNECT_DEADLINE` asserted below it. This is the precedent for the "compiled bound with a test" branch: a constant held below another bound by a unit assertion (`decision/hook-connect-deadline`, `decision/spine-deadline`).

@history_item n=7 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — made `quiet-period` and `confirm-window` ledger rows validate the compiled timing constants per CLI version, with run reading no number from the stamp. This is the precedent for the "relied ledger row with a verify probe" branch of the chunk's fork: a timing value stays compiled and a per-version row validates it (`decision/confirm-window-compiled`, `gate-timings-compiled`).

@history_item n=8 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — built `wait {after?, timeout_ms?, from?}` with `u64` params (else `-32602`), a `checked_add` deadline, and the `WaitFeed` Condvar wake with no file polling. It also built `last`. `wait`'s caller-supplied block is this deadline, so any cap stays inside the u64 / `-32602` contract (`contract/channel-wait/params`, `channel-wait/timeout-ms`).

@history_item n=9 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — moved the hold to `dialog_hold_s` (60–3600 s). It bounded the hook's reply read at `DIALOG_HOLD_MAX + 5 s`, with the `hooks.json` timeout 3615 s set above it so that viola, never Claude Code, ends the wait. It also added `dialog-expired` as a wake kind. This is the precedent for keeping viola's own bound below the host's timeout by a pinned margin (`decision/viola-ends-the-dialog-wait`). A parked MCP `wait` can span a hold, so it must stay capped.

@history_item n=10 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — made `send` consume `LOCAL_COMMANDS`. An unconfirmable command returns at once with no window, and `/clear` on a verified CLI waits the window for its post-condition. These are the per-command branches of `send`'s block that the bound must cover.

@history_item n=11 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added stdout-echo confirmation (a transcript tag count, the read bounded by MAX_FRAME) and the `unconfirmable` case for an unreadable transcript, ratified 2026-11-17. This adds another confirmation path whose wait is part of `send`'s longest block (`decision/local-command-echo-confirmation`, `contract/send-results/unconfirmable`).

@history_item n=12 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — added the `/context` local command with its own relied row and Run B's eighth paste, and extended `unconfirmable` to any listed command on an unverified CLI. This is one more listed command whose window and post-condition feed the send total. It is also the most recent precedent for a widening of verify's pastes, held PROVISIONAL for the epoch batch.

@history_item n=13 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — put `send`'s post-gate section (second wheel/turn read, link, `send-issued`, paste) under one wrapper flight lock that the claim and turn/wheel moves also take, and widened the prompt match to the submitted form. Lock waits add to `send`'s whole path (`decision/send-flight-lock`, `submitted-form-prompt-match`).

@history_item n=14 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — built the confirmed `send`: the `send-issued` / `send-confirmed` / `send-refused` records, the refusal order, one send in flight (`turn-running`), and a paste failing after issue answered `input-not-ready`. A compiled bound or split that ends a send early must keep these records and the refusal order (`contract/send-refusal/order`, `contract/send/paste-failure-after-issue`).

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — removed the `budget-paused` rung from `send`'s refusal order, and retired exit 11 never to be reused (`decision/retired-exit-codes-never-reused`). If the chunk adds a timeout outcome, it may not reuse 11. The refusal-order rows it touches are the ones it last rewrote.

@history_item n=16 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — added the server identity check (peer pid and start time) to every frame client, the MCP frame tools included. In MCP the start time is read on `tokio::task::spawn_blocking`. The MCP result is `instance-unreachable` with no frame and no line. These client-side checks are the first part of `send`'s and `wait`'s whole path on the MCP surface (`decision/frame-client-verifies-server`, `server-start-time-read-through-sysinfo`).

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the instance check first at every CLI frame verb and MCP frame tool) — made each MCP frame tool run `check_instance` before `live_endpoint`, a refusal giving `instance-unreachable` with no frame and no line. It also moved snapshot readers onto the replay. This fixes the order of the MCP frame path a bound must wrap (`decision/frame-client-check-order`, `mcp-frame-tools-strict-modes-check`).

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — added the socket-directory check and the peer euid check to every client, the MCP `frame` among them (`socket-dir-refused`, `server-verify-failed`). It set the check order: strict-modes, liveness, socket dir, euid, then pid. These are the last client-side checks before `send`'s frame on MCP.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — set exit 21 / MCP `instance-unreachable`'s closed cause set and `result::unreachable(cause)`. It read `Closed | Io` as `endpoint-lost`, and the founder ratified the cause crossing to drivers. Any new outcome for a call cut at a bound must fit or extend a closed set through `viola-core`, with the founder's word on a crossing (`contract/unreachable-cause/cause-set`, `decision/mcp-unreachable-result-for-every-cause`).

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — made MCP error results carry codes and fixed messages only, held by the census `tests/contract_error_bodies.rs`. It set `wrapper-fault` through `sanitised_fault`, and `viola mcp` now exits 1 on a stdin cut before `initialize`. A new MCP timeout or split result must be a named row of that census (`decision/sanitised-error-surfaces`).

@history_item n=21 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — capped `viola mcp`'s stdin with `CappedLines` at MAX_FRAME, handed to rmcp 3.4.1 (whose stdio `read_until` is unbounded), and added the bounded-readers census `tests/contract_bounded_readers.rs`. This is the precedent for bounding inside the reader viola hands rmcp rather than patching rmcp. Any new reader the chunk adds must be bounded or named (`decision/bounded-readers`).

@history_item n=22 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange: the tap; the measured 2.1.302 MCP opening) — added the test-only `viola-mcp-tap` (`record` / `fixture`) and the `Mcp.<variant>.json` class with its schema, recorded live on two founder-capped starts. It measured that 2.1.302 opens with `server/discover` and no `initialize`. This is the precedent and instrument for reading the CLI's MCP client side. Recording it again is a ratified boundary crossing, and this phase makes no live start.

@history_item n=23 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — bounded the `claude agents --json` read (`run::agents::read_logged`: `take(MAX_FRAME + 1)`, a 5 s kill) that MCP `list` makes on each call, failing to the `unknown` statuses. This sets `list`'s longest block, which must sit below the client bound (`decision/agents-listing-never-errors`, `mcp-list-reads-agents`).

@history_item n=24 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — measured that a session with both plugins starts the `--plugin-dir` plugin's `viola mcp`, and that an exit-1 `viola mcp` does not block the CLI's start. This tells which server's tool calls the client bound applies to.

@history_item n=25 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — landed the 29th relied ledger row and widened `decision/verify-child-set` with a print-mode sender in Run B, PROVISIONAL for the epoch batch. This is the latest precedent for the scope's "30th row" branch: a new row extends `verify-child-set`, and any new child or read verify makes is a widening shown whole to the founder.

@history_item n=26 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — added a relied row with its own probe and a `measured.env_probe` stamp block, PROVISIONAL. This is the pattern for a new row's measured readings in the stamps envelope (`contract/ledger-stamps/measured`, additive with no `v` bump). It is also a precedent for reading names only from a CLI bundle's strings, which must be called a reading, not a measurement.

@history_item n=27 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — widened verify's child set to six runs (Run F) under the founder's rulings, with a relied row whose rule says what the probe measures per version. This is the precedent that a ledger row states a per-version question, not an assumed fact.

@history_item n=28 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — made a version verified only when every relied row counts (pass, or absent on browser-tools-prompt alone). Every home stamped before a new relied row reads unverified until re-stamped. A 30th row carries that cost, and the scope says no live re-stamp happens in this phase (`decision/version-verified-all-relied-rows`, `capability-ledger`).

@history_item n=29 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — founded the ledger and `viola verify`'s print-mode probe (`-p`, bounded at 120 s) and the `--version` read (`MAX_FRAME`, 5 s kill). This is the probe shape a 30th row reading the CLI's tool-call timeout would most likely extend (`decision/cli-capability-ledger`, `capability-ledger-gate`).

@history_item n=30 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — ratified `viola-mcp`'s `ServeError` beside `McpError` (and four other enums) as named exceptions to one-enum-per-crate. A new timeout error in `viola-mcp` must go into an existing enum or come as a ruled exception (`decision/thiserror-one-enum-per-crate`).

@history_item n=31 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — moved the Tokio pin to `=1.53.3` (`default-features = false`) and kept Tokio contained to `viola-mcp`, the optional channel feature and `viola-e2e`. A tokio feature added for an MCP-side timeout must keep the four sole-root bans and `channel-tokio-free` green.

@history_item n=32 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — pinned that `wait` wakes by event kind alone, so every `turn-ended` wakes it whatever its `background_tasks`, and listed the records that never wake it. A `wait` parked across non-waking records runs to its `timeout_ms`, so the MCP cap is its only bound (`contract/channel-methods/wait-wake-kinds`).
