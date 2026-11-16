@history_item n=1 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — (wait channel lines scoped) set `after` / `timeout_ms` (u64 only) and `outcome` on `wait` alone, both sides; `outcome` is the waking kind or `timed-out`, derived ONCE in `viola-channel` over the shared wake set, the client alone adding `instance-unreachable`; `last` carries none. The root of contract/wait-log-fields/channel-request, contract/wait-log-fields/channel-response-outcome and contract/wait-log-line/outcome: any change to which kinds wake must move that one shared wake set, never a per-side list.

@history_item n=2 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — (the expiry is the wrapper's) widened `woken_kind` and the `channel-response` `outcome` to six kinds (`turn-ended|question|permission|plan|session-end|dialog-expired`, the six of `EventKind::WAIT_WAKE`), was five; the wrapper appends `dialog-expired` that wakes a parked wait. Precedent for this chunk: a wake-set change is stated in contract/wait-span-attributes/run-wait-dispatch and contract/events-log/dialog-expired together, keyed to `EventKind::WAIT_WAKE`; a notification-class kind staying log-only must stay out of that list.

@history_item n=3 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (6. Log Coverage) ~ contract/diag-line-fields/hook-events: `hook_event` gained `stop-failure`, nine to ten closed names, PROVISIONAL (delegate overseer, «no ledger row»). The previous chunk's turn-end work this chunk rests on; a new hook name or field from the wake work takes the same closed-enum row edit and schema change.

@history_item n=4 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/events-stream-never-rotated (never rewritten; a reused name keeps earlier starts' lines at the head, so a start's lines are read from where it begins), ~ contract/run-start-events/order and ~ decision/product-order-from-events-log (wheel{human-input} may precede session-start). `wait`'s `after` is a byte offset into this stream: a wake test over the fake agent must arm after the start's own offset, not assume the log's first lines.

@history_item n=5 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — stated `turn-running`'s two causes: the running turn is in-memory wrapper state, marked by a `prompt-submitted`, ended by `turn-ended` / `session-start` / `session-end`, cleared by a wheel-returning `release`, writing no event, span or line of its own. The turn-end semantics a background-task turn end touches; a change to when a turn counts as ended must keep this no-new-line shape or take a Decisions Log entry.

@history_item n=6 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the `wait` outcome count; «server-verify-failed cannot occur» retired; the check's place in the span tree; the scenarios' lines) `viola.wait.outcome` counts only waits answered by a `channel-response`; a CLI `wait` refused before any frame ends inside `wait.block` with one exit-21 `process-exit` during connect and no `channel-request`; MCP's `wait` writes no line. Source of contract/cli-wait-last-connect/server-verify-failed, contract/mcp-wait-last-connect/server-verify-failed and contract/wait-span-attributes/wait-block: wake tests must not count refusals as outcomes.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (frame verbs' instance check through `client::refuse_untrusted`; MCP frame tools' check; `check_diagnostics`; home-tree checks of run, revive and verify; pre-init window; refusal-telemetry record) a refused instance check ends `wait` / `last` first, exit 21, no line, no outcome counted; `viola verify` checks the home tree before its log. Behind contract/cli-wait-last-connect/strict-modes-failed, contract/mcp-wait-last-connect/strict-modes-failed, decision/diagnostics-checked-at-open, decision/home-tree-check-before-start-log, decision/refused-home-tree-writes-no-line and contract/viola-verify/home-tree-refused — the order (instance check, liveness, server check) a re-touched `wait` must keep.

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21's document names its cause; MCP's body names its cause; exit 21's logged causes and `during`; `verify` writes its refusal's code; Founder Direction 4's codes) a wrapper killed mid-wait reads `endpoint-lost` during `call`, the other exit-21 causes during `connect`; the CLI/MCP documents carry one `UnreachableCause` word; verify's refused `update_stamps` logs `strict-modes-failed`. Source of contract/wait-log-line/exit-code-21, contract/exit-21-document/*, contract/process-exit-detail/verify-refusal and decision/exit-1-detail-codes.

@history_item n=9 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (exit 21's causes five to six; MCP frame tools' socket-dir and euid refusals; `socket-dir-refused` in the catalogue; no strict-modes failure; the frame verbs' socket-directory refusal; refused peer dropped without a line; one code; Founder Direction 4) `wait.block` also closes on a refused socket directory, one exit-21 `process-exit` during connect, no `channel-request`; a refused peer euid is dropped with no line. Source of contract/wait-log-line/exit-code-21's socket-dir clause, contract/cli-wait-last-connect/socket-dir-refused, decision/socket-dir-one-code and decision/refused-peer-dropped-without-line.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (server check on Linux; strict-modes line in two places; closed words leave the schema with their emitter; one CI leg) dropped per-OS qualifiers from the `wait` / `last` server check; set decision/strict-modes-failure-lines to the `ledger-stamps` read and verify's refused stamp write; added the rule that a closed `diag-line.v1.json` word no build emits leaves the schema; CI is `ubuntu-latest` alone — the one leg the carried `path1_run_start_one_home_in_order` timeout sits on.

@history_item n=11 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — made `corr` required in `diag-line.v1.json` on `channel-request` / `channel-response` (null only for id-less frames) and logged a malformed `conn` as `srv_conn`. Binds contract/wait-log-line/required-fields (corr = JSON-RPC id) and decision/corr-conn-correlation for any new wait-path line.

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (`channel-response`'s detail closed to three words; a peer's reply screened by `sanitised_fault` / `is_code_word`; the mcp role writes no self `process-exit`) the `channel-response` `detail` is closed to the three `-32602` words; peer replies are screened to code words with no added span or line. Bears on decision/channel-log-scrub-rule, decision/clients-screen-peer-replies-to-code-words and decision/single-sanitising-fault-reader: a wait reply change must not push a body or new word through these.

@history_item n=13 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (route entries cited by title) the `wait` / `last` scenario's dated gap is cited by route-entry title, not bare route numbers: the citation rule a new wait-scenario statement follows.

@history_item n=14 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/diag-schema-additive-only (narrowed to «a field the product still emits», operator Q2), ~ decision/exit-code-log-field (11 retired, never reused), ~ contract/run-start-events/order. Constrains any field or kind this chunk adds to the wait path (e.g. carrying `background_tasks`): additive only, through a Decisions Log entry (decision/event-values-closed).

@history_item n=15 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (1. Obs Scope Summary) ~ decision/parse-failures-unknown-counter gained its except (a hyprctl reply's parse failure reads a closed error, never unknown). The row is picked: a wake classification of an unparsed or unknown hook payload falls under the unknown-plus-counter rule (decision/skipped-counters).

@history_item n=16 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the mcp sink is always mcp.ndjson) `viola mcp` always logs to `<home>/diagnostics/mcp.ndjson`, never stdout. Behind decision/mcp-log-file-sink, decision/mcp-stdout-frames-only and contract/diagnostics-files/mcp: MCP `wait` tool work stays inside that sink.

@history_item n=17 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — put CI `viola verify` on the fake agent alone (`tests/cli_verify.rs`, harness `boot` step 4, `stamped_home`), the real-CLI verify local only and refused under CI; verify logs its spawns as pairs. Source of decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci — the `stamped_home` fixture whose unbounded verify the carried CI timeout measures.

@history_item n=18 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify logs five `verify-pty-probe` runs (A–E); CI's fake-agent verify drives five interactive runs; a PTY child killed at a stop still carries `child_exit_status`. The cost the stamped-home fixture pays per test (contract/verify-spawn-subject/verify-pty-probe), weighed when choosing between sharing a stamped home and sizing the `ci` kill.

@history_item n=19 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify's eighth spawn pair, `agents-probe` around Run B's agents read. Completes contract/verify-spawn-subject/agents-probe and the eight-pair count a shared or copied stamped home must still produce once.

@history_item n=20 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — CI's fake-agent verify passes `--framing`; boot step 4 and the stamped-home fixture stamp through the framing replay, bound to test-plan §3's boot step 4. Names the stamped-home fixture as a tests↔obs bind: changing how `stamped_home` stamps moves both masters.

@history_item n=21 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by SIGINT / SIGTERM ends `process-exit` exit 1 `detail:"interrupted"` after closing every pair it opened. If the fixture's verify is bounded (a `within`) and stopped, this is the line it must leave (contract/verify-process-exit/interrupted).

@history_item n=22 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — (CI keeps every home) no harness cleanup or test-home drop deletes a home before G2, G4, the secret scan and the uploads (`AGENT_RUN_KEEP_HOMES=1`). A stamped home shared or copied across tests must still be kept and scanned like any test home.

@history_item n=23 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — G2's start point is `target/e2e-home/` with the trailing slash (a linked scope descended); on the dev host test homes live on tmpfs. Trap for option (a): a shared stamped home placed outside `target/e2e-home/` falls out of G2 / G4 / the scan.

@history_item n=24 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (the chaos home outside the scans) the named test-data carve-out: one `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the scan and upload, by founder ruling. Precedent: placing a test home outside the scanned root needs a named carve-out, not a quiet move.

@history_item n=25 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `parse-rejected{parser:"ledger-stamps"}` gained `strict-modes-failed`, read as unverified (`cli_verified:false`). Source of contract/run-log-parse-rejected/ledger-stamps and decision/unreadable-ledger-is-unverified, touched if stamped homes are shared or copied (a copy must keep the stamps file's modes).

@history_item n=26 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — an absent pinned copy is placed no-clobber; a start losing the race re-hashes, reusing an equal copy, refusing a different one as `pinned-hash-mismatch`, adding no line. Behind decision/pinned-copy-never-replaced and decision/pinned-copy-race-adds-no-line: a stamped home shared between concurrent tests meets this race.

@history_item n=27 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (753 records → 1077 rows), each verified against the code; the row handles in the picked set date from it, and a later edit names its row by handle (`+` / `~` / `−`).
